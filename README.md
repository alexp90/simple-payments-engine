# Simple Payments Engine
![Rust](https://img.shields.io/badge/Rust-🦀-informational)
![Coverage](https://img.shields.io/badge/Coverage-90.73%25-brightgreen)
![Status](https://img.shields.io/badge/Status-MVP-blue)

This project is a simplified payments engine, built to showcase robust type safety and error handling - all in 🦀 `Rust`.

## How it works

This is a simple payments engine which accepts a CSV of this form:

```
type,       client,  tx,    amount
deposit,    1,       3,     2.0
withdrawal, 1,       4,     1.5
dispute,    999,     3,
resolve,    1,       6,
chargeback, 87,      3,
```

and processes it, outputting the balance of all the clients involved.

It supports five different types of operations:
| Operation     | Description|
|---------------|---------------------------------------------------------------------------------------------------|
| `deposit`     | add funds to a client's account |
| `withdrawal`  | remove funds from a client's account. It's skipped and deemed failed if there is not enough money |
| `dispute`     | starts the process of disputing a deposit operation. It's ignored if the transaction is already disputed. <br>The money deposited through the disputed transaction is removed from the available amount and held, until a `resolve` or a `chargeback` operation is received |
| `resolve`     | releases the money held until now and closes the `dispute` |
| `chargeback`  | removes for good the held money from the client's account and freezes it. When an account is frozen, no operations can be executed on it. |

If there is any error in the input CSV, the affected row is skipped. 
If the CSV row is syntactically correct but invalid from a business perspective, the operation is skipped.

## How to run
Make sure you have `rustup` installed. It comes with `cargo`.

You can then run it by executing

```shell
cargo run -- <csv_file_path>
```


When executing it, it will print the errors in the stderr while printing the result in the stdout.
This means that if you execute

```shell
cargo run -- <csv_file_path> > output.csv
```
will redirect only the output to `output.csv`, while keeping the errors in the terminal.

I have left a couple of test files inside [fixtures folder](fixtures):
- [comprehensive_test_with_errors.csv](fixtures/comprehensive_test_with_errors.csv) should address all the edge cases and possible errors
- [benchmark_1.csv](fixtures/benchmark_1.csv) is a big CSV file with 263510 rows generated automatically, to test performance
- [benchmark_2.csv](fixtures/benchmark_2.csv) is a bigger CSV file with 1428839 rows generated automatically, to test even more performance

you can use them to test that everything works on your machine as well!

## General architecture and error management

TL;DR: It's basically a layered design: CSV → OperationRequest → ValidOperationRequest → PaymentsEngine.

This system currently supports only CSVs. 
When the program is executed, the path provided as CLI argument is passed to the [process_from_csv_use_case](src/use_case/process_from_csv_use_case.rs), which exposes a method for processing operations from a CSV. 
The [use_case](src/use_case) module contains all the entry points to business-level features. In this case, it includes only the `process_from_csv_use_case`, which reads the CSV and delegates the actual processing to [payments engine](src/domain/payments_engine.rs). 
The `PaymentsEngine` operates on a generic [OperationRequest](src/domain/payments_engine/operation_request.rs) and not directly on CSVs.

The method offered by  [process_from_csv_use_case](src/use_case/process_from_csv_use_case.rs) processes the CSV line by line (reads with an asynchronous reader) and
converts each row to the `OperationRequest` needed by the `PaymentsEngine`. If there is an issue when reading the CSV, it will print the error.

I preferred to separate concerns (CSV processing AND operation processing) so that the `PaymentsEngine` contains only the real business logic, 
and in the future we can accept other types of input (other CSVs formats, API calls, etc.), requiring only to convert the data to an `OperationRequest`. This approach is
future-proof.

### How does the PaymentsEngine work?
The only way to interact with it is by providing an `OperationRequest`. The `OperationRequest` specifies
the operation we want to process, but `PaymentsEngine` does not process directly them - instead, it processes
[ValidOperationRequest](src/domain/payments_engine/valid_operation_request.rs), which are built only after successful
`OperationRequest` validation. 

The approach being followed is "**parse, don't validate**", which is very similar to the Factory as defined in Domain Driven Design: our business logic
operates only on valid structs, which are valid `by definition`. 

Runtime errors are prevented by design: `ValidOperationRequest` is only created after successful validation, and the validation logic provides exactly the data needed to construct it (as for business logic).
<br>I'm fully leveraging the type system, so we're pretty close to the utopian mantra: `if it compiles, it works!` (as long as business logic is implemented correctly).

Right now the validation phase (which happens in the `ValidOperationRequest` constructor) collects all the validation outcomes, and then
 - returns an `Err` with the list of validation errors, if there are any
 - returns an `Ok(ValidOperationRequest)` if all validations passed

The errors are just printed, but the code already collects all of them (instead of stopping at the first one), so, whenever the need arises, we can manage errors in a different way.
As long as we model our code correctly, and we don't lose data, we can always change the logic without too many issues.

When the `PaymentsEngine` completes the processing, it returns the outcome (which is itself - the reason why is better explained in the "Other thoughts" section) and it's printed
through [output_printer](src/output_printer.rs)

### Additional details

The idea is that domain structs like [Account](src/domain/account_module/account.rs), [Transaction](src/domain/transaction_module/transaction.rs) and [ValidOperationRequest](src/domain/payments_engine/valid_operation_request.rs) can't be created on their own, 
but they need to either be created by the validation flow (a Factory in DDD), or returned by the repository. 
<br> I've achieved it by locking the `Account` and `Transaction` constructors to the `domain`, while `ValidOperationRequest` is not even visible from outside the `PaymentsEngine`. 
I'm fine with `Account` and `Transaction` being constructable inside the domain, as this may be needed by future developments


You may have noticed that in the [ValidOperationRequest validation and building phase](src/domain/payments_engine/valid_operation_request/builder.rs) there is a bit of "duplication".
I'm perfectly fine with it, because even if different type of operations are sharing the same validations, it's not correct to abstract them into a single method.
Different operation types have different life cycles and may evolve differently, so it's correct from a "business" point of view that they are not grouped
under the same code. TL;DR; don't over optimize for acronyms (in this case, DRY)

## Assumptions

A couple assumptions have been made while developing this engine:
- a `dispute` can only be requested for `deposit` transactions
- for `dispute`, `resolve` and `chargeback` operations the client is always ignored - the only valuable information is the transaction_id. I could have also removed the constraint on client being always present, but I just preferred to keep things simpler and not add other edge cases to manage.
- while a `withdrawal` can't let a client balance go in negative, I've decided that the `dispute` can. From a Bank point of view, a transaction can always be disputed and a `dispute` can't just be ignored, but if that would make the client's balance go in negative, 
  it's an alarm: something is going wrong (a fraud?). It may signal that the client first deposited some money, then withdraw it and then opened a `dispute` on the first deposit, to double the money.
- I'm treating `Operation` validation errors as errors from the third party's side, so invalid operations are not persisted. Because of this, if a CSV contains an invalid Operation with a specific `id` 7 followed by a valid one with the same `id`, the second one will be accepted.
  <br>The uniqueness check only applies to successfully processed operations.

## Potential issues
By being a simple payments engine, it's fine for what it's doing. If we would move it to production though, there are a couple
potential issues that should be looked at first.

First of all, we're processing a CSV but we're just skipping errors. <br>The CSV is hypothetically provided by a third party, but if there
are issues, I would expect the whole processing to be discarded without persisting anything, as the risk of having partial updates
is just too high. <br>If the third party, for any reason, has put wrong data in some CSV rows, there may be other operations in the same CSV that rely 
on the failed ones, and they would fail as well. This approach is prone to inconsistencies.

Secondarily, the [process_from_csv_use_case](src/use_case/process_from_csv_use_case.rs) supports multiple asynchronous calls, so it can be called concurrently
from different callers, uploading different CSVs.
This is safe in this project, because there is no real persistence and the accounts exist only during the binary execution. No concurrency issue can happen.

In a real world scenario, we need to take care of it, as multiple threads are trying to update
the same account or even the same transaction. In this case, the solution depends from the business logic and the scenario:
- when CSVs contain *different* set of clients between each other: there should be no harm in processing them in parallel, we can even have different repository instances for each third party allowed to upload a CSV. If a third party tries to upload multiple CSVs one after the other, then we will treat it like the next scenario
- when the CSVs share the same set of clients: we may need to manage concurrency or we may even opt for processing the CSVs in sequence, and not in parallel. We would still accept multiple CSVs uploads,
  but we would build some infrastructure to then process the operation in sequence. This depends again on the product and its business logic (are the CSVs ordered by time? do they contain the whole day of transactions? are the CSV grouped by "tenant"? etc.)

## Other thoughts

We can think about the [payments_engine](src/domain/payments_engine.rs) as an in-memory outcome of a block of work. While in this
project this is also mimicking a bit the persistence (the repositories), in a real world scenario it would work a bit differently.<br>
In this engine, every operation is persisted row after row, while in a real world scenario it may be better to persist only at the end of the processing, if there are no errors. <br> This helps avoid any inconsistent state and allows retrying the CSV without needing to track where it failed. <br>
This solution has a higher memory footprint, but it's probably simpler and safer. It would be something that I would observe and see how it behaves, iterating and changing the solution accordingly with the results.

Obviously there are other solutions in the middle, like processing the operations in chunks and persisting chunk after chunk, or we can even continue saving operation after operation,
but a bit more infrastructural work is needed to be able to resume from where it stopped (for example, through some queue).


## What's next?

While this project was intended to be built in 2-3 hours, I've found myself spending a bit more time on it while having fun in writing a type safe implementation. 
For this reason, I've thought that something would need to be postponed after this first MVP.
Here's the list of the next things to do:
- adding tests in the missing places. I've implemented just one integration test that processes a CSV with all the cases in [process_from_csv_use_case_test](src/use_case/process_from_csv_use_case/process_from_csv_use_case_test.rs). 
  Coverage is already quite high (tarpaulin reports *90.73%*) covering all the important cases. <br>
  Still, I'd like to cover the generic [payments_engine](src/domain/payments_engine.rs) with extensive integration tests, so that in the future the specific processors (like the csv_payments_engine) can be unit tested with the payments_engine mocked out. Again, right now the coverage is high enough
  to be happy. This also opens up another discussion point: testing pyramid or diamond testing?

- improve logging: right now it just focuses on errors for debugging. Having some `info` log would improve observability.
