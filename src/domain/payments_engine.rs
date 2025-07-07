mod valid_operation_request;
pub mod operation_request;

use log::error;
use valid_operation_request::ValidOperationRequest;
use crate::domain::account_module::account::{Account, ActiveAccount};
use crate::domain::account_module::account_repository::AccountRepository;
use crate::domain::payments_engine::operation_request::OperationRequest;
use crate::domain::transaction_module::transaction::{DepositTransaction, DisputedDepositTransaction, Transaction, WithdrawalTransaction};
use crate::domain::transaction_module::transaction_repository::TransactionRepository;
pub struct PaymentsEngine {
    account_repository: AccountRepository,
    transaction_repository: TransactionRepository
}

impl PaymentsEngine {
    pub fn new() -> Self {
        Self {
            account_repository: AccountRepository::new(),
            transaction_repository: TransactionRepository::new()
        }
    }


    /*
        This is the only entry point for the PaymentsEngine. It expects an OperationRequest and tries
        to validate it and produce a ValidOperationRequest.
        ValidOperationRequest represents a validated operation, which will be processed by the engine.
        I'm following the "parse, don't validate" approach, so whenever we have a ValidOperationRequest,
        logic will run flawlessly and will not generate any Runtime issue. The processing logic is "pure".
    */
    pub fn process(&mut self, operation_request: OperationRequest, index: usize) {
        let valid_operation_request = ValidOperationRequest::new(&operation_request, &self.account_repository, &self.transaction_repository);
        match valid_operation_request {

            Ok(valid_operation_request) => process_valid_operation_request(valid_operation_request, &mut self.account_repository, &mut self.transaction_repository),

            Err(errors) => {
                let errors_as_string  = errors.iter().map(|error| error.to_string()).collect::<Vec<_>>().join(", ");
                error!("Impossible to process operation request number {index} - Errors: {errors_as_string}");
            },

        }
    }

    pub fn accounts(&self) -> impl Iterator<Item = &Account> {
        self.account_repository.all()
    }

}



fn process_valid_operation_request(valid_operation_request: ValidOperationRequest, account_repository: &mut AccountRepository, transaction_repository: &mut TransactionRepository) {

    let updated_account = match valid_operation_request {
        ValidOperationRequest::Deposit { new_transaction, to_account } => process_deposit(new_transaction, to_account, transaction_repository),
        ValidOperationRequest::Withdrawal { new_transaction, from_account } => process_withdrawal(new_transaction, from_account, transaction_repository),
        ValidOperationRequest::OpenDispute { on_transaction, account } => process_open_dispute(on_transaction, account, transaction_repository),
        ValidOperationRequest::ResolveDispute { on_transaction, account } => process_resolve_dispute(on_transaction, account, transaction_repository),
        ValidOperationRequest::ChargeBack { on_transaction, account } => process_chargeback(on_transaction, account, transaction_repository),
    };

    account_repository.store(updated_account);
}

fn process_deposit(deposit_transaction: DepositTransaction, to_account: ActiveAccount, transaction_repository: &mut TransactionRepository) -> Account {
    let updated_account = to_account.deposit(&deposit_transaction);
    transaction_repository.store(Transaction::Deposit(deposit_transaction));
    Account::Active(updated_account)
}

fn process_withdrawal(withdrawal_transaction: WithdrawalTransaction, from_account: ActiveAccount, transaction_repository: &mut TransactionRepository) -> Account {
    let updated_account_result = from_account.withdraw(&withdrawal_transaction);
    match updated_account_result {
        Ok(updated_account) => {
            transaction_repository.store(Transaction::Withdrawal(withdrawal_transaction));
            Account::Active(updated_account)
        }
        Err(active_account) => {
            Account::Active(active_account)
        }
    }

}

fn process_open_dispute(deposit_transaction: DepositTransaction, account: ActiveAccount, transaction_repository: &mut TransactionRepository) -> Account {
    let updated_account = account.hold_amount(deposit_transaction.amount());
    let updated_transaction = deposit_transaction.open_dispute();
    transaction_repository.store(Transaction::DisputedDeposit(updated_transaction));

    Account::Active(updated_account)
}

fn process_resolve_dispute(disputed_deposit_transaction: DisputedDepositTransaction, account: ActiveAccount, transaction_repository: &mut TransactionRepository) -> Account {
    let updated_account = account.release_held_amount(disputed_deposit_transaction.amount());
    let updated_transaction = disputed_deposit_transaction.resolve_dispute();
    transaction_repository.store(Transaction::Deposit(updated_transaction));

    Account::Active(updated_account)
}

fn process_chargeback(disputed_deposit_transaction: DisputedDepositTransaction, account: ActiveAccount, transaction_repository: &mut TransactionRepository) -> Account {
    let updated_account = account.charge_back_amount(disputed_deposit_transaction.amount());
    let updated_transaction = disputed_deposit_transaction.charge_back();
    transaction_repository.store(Transaction::ChargedBackDeposit(updated_transaction));

    Account::Frozen(updated_account)
}