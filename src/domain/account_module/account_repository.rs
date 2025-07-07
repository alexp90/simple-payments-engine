use std::collections::HashMap;
use crate::domain::account_module::account::{Account, AccountId};

pub struct AccountRepository {
    accounts: HashMap<AccountId, Account>
}

impl AccountRepository {

    pub fn new() -> Self {
        Self {
            accounts: HashMap::new()
        }
    }

    pub fn find(&self, account_id: AccountId) -> Option<&Account> {
        self.accounts.get(&account_id)
    }

    pub fn all(&self) -> impl Iterator<Item = &Account> {
        self.accounts.values()
    }

    pub fn store(&mut self, account: Account) {
        self.accounts.insert(account.id(), account);
    }
}