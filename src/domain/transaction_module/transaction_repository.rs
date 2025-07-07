use std::collections::HashMap;
use crate::domain::transaction_module::transaction::{Transaction, TransactionId};

pub struct TransactionRepository {
    transactions: HashMap<TransactionId, Transaction>
}

impl TransactionRepository {

    pub fn new() -> Self {
        Self {
            transactions: HashMap::new()
        }
    }
    pub fn find(&self, transaction_id: TransactionId) -> Option<&Transaction> {
        self.transactions.get(&transaction_id)
    }

    pub fn store(&mut self, transaction: Transaction) {
        self.transactions.insert(transaction.id(), transaction);
    }
}