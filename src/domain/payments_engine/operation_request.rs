use crate::domain::Amount;
use crate::domain::account_module::account::AccountId;
use crate::domain::transaction_module::transaction::TransactionId;
use crate::use_case::process_from_csv_use_case::{CsvOperationType, OperationCsvRow};

#[derive(Clone)]
pub enum OperationRequest {
    Deposit(DepositOperationRequest),
    Withdrawal(WithdrawalOperationRequest),
    Dispute(DisputeOperationRequest),
    Resolve(ResolveOperationRequest),
    Chargeback(ChargebackOperationRequest)
}

impl OperationRequest {
    pub fn new_from_csv(operation_csv_row: OperationCsvRow) -> Result<Self, String> {
        match operation_csv_row.operation_type {

            CsvOperationType::Deposit => {
                if let Some(amount) = operation_csv_row.amount {
                    Ok(OperationRequest::Deposit(DepositOperationRequest {
                        account_id: operation_csv_row.client,
                        transaction_id: operation_csv_row.tx,
                        amount: amount.trunc_with_scale(4)
                    }))
                } else {
                    Err("Amount not found for Deposit transaction_module request".to_owned())
                }
            }
            CsvOperationType::Withdrawal => {
                if let Some(amount) = operation_csv_row.amount {
                    Ok(OperationRequest::Withdrawal(WithdrawalOperationRequest {
                        account_id: operation_csv_row.client,
                        transaction_id: operation_csv_row.tx,
                        amount: amount.trunc_with_scale(4)
                    }))
                } else {
                    Err("Amount not found for Withdrawal transaction_module request".to_owned())
                }
            }
            CsvOperationType::Dispute => {
                Ok(OperationRequest::Dispute(DisputeOperationRequest{transaction_id: operation_csv_row.tx}))
            }
            CsvOperationType::Resolve => {
                Ok(OperationRequest::Resolve(ResolveOperationRequest{transaction_id: operation_csv_row.tx}))
            }
            CsvOperationType::Chargeback => {
                Ok(OperationRequest::Chargeback(ChargebackOperationRequest{transaction_id: operation_csv_row.tx}))
            }
        }
    }

}

#[derive(Clone)]
pub struct DepositOperationRequest {
    pub account_id: AccountId,
    pub transaction_id: TransactionId,
    pub amount: Amount
}

#[derive(Clone)]
pub struct WithdrawalOperationRequest {
    pub account_id: AccountId,
    pub transaction_id: TransactionId,
    pub amount: Amount
}

#[derive(Clone)]
pub struct DisputeOperationRequest {
    pub transaction_id: TransactionId
}

#[derive(Clone)]
pub struct ResolveOperationRequest {
    pub transaction_id: TransactionId
}

#[derive(Clone)]
pub struct ChargebackOperationRequest {
    pub transaction_id: TransactionId
}


