use crate::domain::account_module::account::Account;

pub fn print_outcome_to_stdout<'a>(accounts_iterator: impl Iterator<Item = &'a Account>) {
    println!("client,available,held,total,locked");
    for account in accounts_iterator {
        println!(
            "{},{},{},{},{}",
            account.id(),
            account.available_amount(),
            account.held_amount(),
            account.total_amount(),
            is_account_locked(account)
        );
    }
}

fn is_account_locked(account: &Account) -> bool {
    match account {
        Account::Active(_) => false,
        Account::Frozen(_) => true
    }
}