use crate::domain::account_module::account::{Account, ActiveAccount};
use crate::domain::Amount;
use crate::domain::transaction_module::transaction::{DepositTransaction, DisputedDepositTransaction, Transaction, TransactionId};
use crate::domain::transaction_module::transaction_repository::TransactionRepository;

pub fn validate_existing_and_active_account(maybe_account: Option<&Account>) -> Result<&ActiveAccount, OperationValidationError> {
    if let Some(account) = maybe_account {
        match account {
            Account::Active(active_account) => Ok(active_account),
            Account::Frozen(_) => Err(OperationValidationError::AccountFrozen)
        }
    } else {
        Err(OperationValidationError::AccountNotFound)
    }
}

pub fn validate_positive_amount(amount: Amount) -> Result<Amount, OperationValidationError> {
    if amount.is_sign_negative() {
        Err(OperationValidationError::NegativeAmount)
    } else {
        Ok(amount)
    }
}

pub fn validate_unique_transaction_id(transaction_id: TransactionId, transaction_repository: &TransactionRepository) -> Result<TransactionId, OperationValidationError> {

    if transaction_repository.find(transaction_id).is_some() {
        Err(OperationValidationError::TransactionIdAlreadyExisting)
    } else {
        Ok(transaction_id)
    }
}

pub fn validate_existing_transaction(transaction_id: TransactionId, transaction_repository: &TransactionRepository) -> Result<&Transaction, OperationValidationError> {

    if let Some(transaction) = transaction_repository.find(transaction_id) {
        Ok(transaction)
    } else {
        Err(OperationValidationError::TransactionNotFound)
    }
}

pub fn validate_transaction_is_deposit(transaction: &Transaction) -> Result<&DepositTransaction, OperationValidationError> {
    match transaction {
        Transaction::Deposit(deposit_transaction) => Ok(deposit_transaction),
        _ => Err(OperationValidationError::ReferencedTransactionIsNotDeposit)
    }
}

pub fn validate_transaction_is_disputed_deposit(transaction: &Transaction) -> Result<&DisputedDepositTransaction, OperationValidationError> {
    match transaction {
        Transaction::DisputedDeposit(disputed_deposit_transaction) => Ok(disputed_deposit_transaction),
        _ => Err(OperationValidationError::ReferencedTransactionIsNotDisputedDeposit)
    }
}

#[derive(Clone)]
pub enum OperationValidationError {
    AccountNotFound,
    AccountFrozen,
    NegativeAmount,
    TransactionIdAlreadyExisting,
    TransactionNotFound,
    ReferencedTransactionIsNotDeposit,
    ReferencedTransactionIsNotDisputedDeposit,
}

impl std::fmt::Display for OperationValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            OperationValidationError::AccountNotFound => "AccountNotFound",
            OperationValidationError::AccountFrozen => "AccountFrozen",
            OperationValidationError::NegativeAmount => "NegativeAmount",
            OperationValidationError::TransactionIdAlreadyExisting => "TransactionIdAlreadyExisting",
            OperationValidationError::TransactionNotFound => "TransactionNotFound",
            OperationValidationError::ReferencedTransactionIsNotDeposit => "ReferencedTransactionIsNotDepositWithoutDispute",
            OperationValidationError::ReferencedTransactionIsNotDisputedDeposit => "ReferencedTransactionIsNotDisputedDeposit"
        };
        write!(f, "{error}")
    }
}
