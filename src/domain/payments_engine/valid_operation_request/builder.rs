use crate::domain::account_module::account::{Account, ActiveAccount};
use crate::domain::account_module::account_repository::AccountRepository;
use crate::domain::payments_engine::operation_request::{ChargebackOperationRequest, DepositOperationRequest, DisputeOperationRequest, ResolveOperationRequest, WithdrawalOperationRequest};
use crate::domain::payments_engine::valid_operation_request::validator::{validate_existing_and_active_account, validate_existing_transaction, validate_positive_amount, validate_transaction_is_deposit, validate_transaction_is_disputed_deposit, validate_unique_transaction_id, OperationValidationError};
use crate::domain::payments_engine::valid_operation_request::ValidOperationRequest;
use crate::domain::transaction_module::transaction::{DepositTransaction, WithdrawalTransaction};
use crate::domain::transaction_module::transaction_repository::TransactionRepository;

/*
  The approach is simple: errors are collected in the vec[], and if there is at least one, then return an Err, otherwise an Ok with
  the correct type.
  The error collecting could have been done in a more elegant way by implementing manually various "compose" functions for different number of arguments - something
  already present in Scala (cats) or Kotlin (arrow) libraries
*/

pub fn build_deposit(deposit_operation_request: &DepositOperationRequest, account_repository: &AccountRepository, transaction_repository: &TransactionRepository) -> Result<ValidOperationRequest, Vec<OperationValidationError>>  {
    let mut errors: Vec<OperationValidationError> = vec![];

    let default_new_account = Account::Active(ActiveAccount::new(deposit_operation_request.account_id));
    let account = account_repository
        .find(deposit_operation_request.account_id)
        .unwrap_or(&default_new_account);

    let validated_account_result = validate_existing_and_active_account(Some(account));
    let validated_amount_result = validate_positive_amount(deposit_operation_request.amount);
    let validated_transaction_id = validate_unique_transaction_id(deposit_operation_request.transaction_id, transaction_repository);

    if let Err(error) = validated_account_result.clone() {
        errors.push(error)
    }
    if let Err(error) = validated_amount_result.clone() {
        errors.push(error)
    }
    if let Err(error) = validated_transaction_id.clone() {
        errors.push(error)
    }

    match (validated_account_result, validated_amount_result, validated_transaction_id) {
        (Ok(account), Ok(amount), Ok(transaction_id)) => {
            let new_transaction = DepositTransaction::new(transaction_id, account.id(), amount);
            Ok(ValidOperationRequest::Deposit { new_transaction, to_account: account.clone() })
        },
        _ => Err(errors)
    }
}

pub fn build_withdrawal(withdrawal_operation_request: &WithdrawalOperationRequest, account_repository: &AccountRepository, transaction_repository: &TransactionRepository) -> Result<ValidOperationRequest, Vec<OperationValidationError>>  {
    let mut errors: Vec<OperationValidationError> = vec![];

    let maybe_account = account_repository.find(withdrawal_operation_request.account_id);

    let validated_account_result = validate_existing_and_active_account(maybe_account);
    let validated_amount_result = validate_positive_amount(withdrawal_operation_request.amount);
    let validated_transaction_id = validate_unique_transaction_id(withdrawal_operation_request.transaction_id, transaction_repository);

    if let Err(error) = validated_account_result.clone() {
        errors.push(error)
    }
    if let Err(error) = validated_amount_result.clone() {
        errors.push(error)
    }
    if let Err(error) = validated_transaction_id.clone() {
        errors.push(error)
    }

    match (validated_account_result, validated_amount_result, validated_transaction_id) {
        (Ok(account), Ok(amount), Ok(transaction_id)) => {
            let new_transaction = WithdrawalTransaction::new(transaction_id, account.id(), amount);
            Ok(ValidOperationRequest::Withdrawal { new_transaction, from_account: account.clone() })
        },
        _ => Err(errors)
    }
}

pub fn build_dispute(dispute_operation_request: &DisputeOperationRequest, account_repository: &AccountRepository, transaction_repository: &TransactionRepository) -> Result<ValidOperationRequest, Vec<OperationValidationError>> {
    let mut errors: Vec<OperationValidationError> = vec![];

    let validated_transaction_and_account_result = validate_existing_transaction(dispute_operation_request.transaction_id, transaction_repository)
        .and_then(validate_transaction_is_deposit)
        .and_then(|deposit_transaction| {
            let maybe_account = account_repository.find(deposit_transaction.to_account_id());
            validate_existing_and_active_account(maybe_account).map (|active_account| (deposit_transaction, active_account))
        });


    if let Err(error) = validated_transaction_and_account_result.clone() {
        errors.push(error)
    }

    match validated_transaction_and_account_result {
        Ok((deposit_transaction, active_account)) => {
            Ok(ValidOperationRequest::OpenDispute { on_transaction: deposit_transaction.clone(), account: active_account.clone() })
        },
        _ => Err(errors)
    }
}

pub fn build_resolve(resolve_operation_request: &ResolveOperationRequest, account_repository: &AccountRepository, transaction_repository: &TransactionRepository) -> Result<ValidOperationRequest, Vec<OperationValidationError>> {
    let mut errors: Vec<OperationValidationError> = vec![];

    let validated_transaction_and_account_result = validate_existing_transaction(resolve_operation_request.transaction_id, transaction_repository)
        .and_then(validate_transaction_is_disputed_deposit)
        .and_then(|disputed_deposit_transaction| {
            let maybe_account = account_repository.find(disputed_deposit_transaction.to_account_id());
            validate_existing_and_active_account(maybe_account).map (|active_account| (disputed_deposit_transaction, active_account))
        });


    if let Err(error) = validated_transaction_and_account_result.clone() {
        errors.push(error)
    }

    match validated_transaction_and_account_result {
        Ok((disputed_deposit_transaction, active_account)) => {
            Ok(ValidOperationRequest::ResolveDispute { on_transaction: disputed_deposit_transaction.clone(), account: active_account.clone() })
        }
        _ => Err(errors)
    }
}

pub fn build_chargeback(chargeback_operation_request: &ChargebackOperationRequest, account_repository: &AccountRepository, transaction_repository: &TransactionRepository) -> Result<ValidOperationRequest, Vec<OperationValidationError>> {
    let mut errors: Vec<OperationValidationError> = vec![];

    let validated_transaction_and_account_result = validate_existing_transaction(chargeback_operation_request.transaction_id, transaction_repository)
        .and_then(validate_transaction_is_disputed_deposit)
        .and_then(|disputed_deposit_transaction| {
            let maybe_account = account_repository.find(disputed_deposit_transaction.to_account_id());
            validate_existing_and_active_account(maybe_account).map (|active_account| (disputed_deposit_transaction, active_account))
        });


    if let Err(error) = validated_transaction_and_account_result.clone() {
        errors.push(error)
    }

    match validated_transaction_and_account_result {
        Ok((disputed_deposit_transaction, active_account)) => {
            Ok(ValidOperationRequest::ChargeBack { on_transaction: disputed_deposit_transaction.clone(), account: active_account.clone() })
        }
        _ => Err(errors)
    }

}