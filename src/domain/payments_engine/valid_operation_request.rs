mod validator;
mod builder;

use crate::domain::account_module::account::ActiveAccount;
use crate::domain::account_module::account_repository::AccountRepository;
use crate::domain::payments_engine::operation_request::OperationRequest;
use crate::domain::payments_engine::valid_operation_request::builder::{build_chargeback, build_deposit, build_dispute, build_resolve, build_withdrawal};
use crate::domain::transaction_module::transaction::{DepositTransaction, DisputedDepositTransaction, WithdrawalTransaction};
use crate::domain::transaction_module::transaction_repository::TransactionRepository;
use crate::domain::payments_engine::valid_operation_request::validator::OperationValidationError;

pub enum ValidOperationRequest {
    Deposit { new_transaction: DepositTransaction, to_account: ActiveAccount },
    Withdrawal { new_transaction: WithdrawalTransaction, from_account: ActiveAccount },
    OpenDispute { on_transaction: DepositTransaction, account: ActiveAccount },
    ResolveDispute { on_transaction: DisputedDepositTransaction, account: ActiveAccount },
    ChargeBack { on_transaction: DisputedDepositTransaction, account: ActiveAccount },
}

impl ValidOperationRequest {

    pub fn new(operation_request: &OperationRequest, account_repository: &AccountRepository, transaction_repository: &TransactionRepository) -> Result<Self, Vec<OperationValidationError>> {
        match operation_request {
            OperationRequest::Deposit(deposit_operation_request) => build_deposit(deposit_operation_request, account_repository, transaction_repository),
            OperationRequest::Withdrawal(withdrawal_operation_request) => build_withdrawal(withdrawal_operation_request, account_repository, transaction_repository),
            OperationRequest::Dispute(dispute_operation_request) => build_dispute(dispute_operation_request, account_repository, transaction_repository),
            OperationRequest::Resolve(resolve_operation_request) => build_resolve(resolve_operation_request, account_repository, transaction_repository),
            OperationRequest::Chargeback(chargeback_operation_request) => build_chargeback(chargeback_operation_request, account_repository, transaction_repository)
        }
    }
}

