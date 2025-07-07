use std::error::Error;
use csv_async::{AsyncReaderBuilder, Trim};
use futures::StreamExt;
use log::error;
use tokio::fs::File;
use tokio::task::JoinHandle;
use tokio_util::compat::TokioAsyncReadCompatExt;

use serde::Deserialize;
use crate::domain::account_module::account::AccountId;
use crate::domain::Amount;
use crate::domain::payments_engine::operation_request::OperationRequest;
use crate::domain::payments_engine::PaymentsEngine;
use crate::domain::transaction_module::transaction::TransactionId;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CsvOperationType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Deserialize, Clone)]
pub struct OperationCsvRow {
    #[serde(rename = "type")]
    pub operation_type: CsvOperationType,
    pub client: AccountId,
    pub tx: TransactionId,
    pub amount: Option<Amount>
}



/*
  This method expects a CSV path, reads it row by row and produces an OperationRequest, needed by the
  generic PaymentsEngine.
  If there is an error, it's just printed.
*/
pub async fn process_from_csv(file_path: String, mut payments_engine: PaymentsEngine) -> JoinHandle<Result<PaymentsEngine, Box<dyn Error + Send + Sync>>> {
    tokio::spawn ( async move {

        let file = File::open(file_path).await?;
        let reader = file.compat();

        let mut csv_reader = AsyncReaderBuilder::new()
            .trim(Trim::All)
            .create_deserializer(reader);


        let mut records = csv_reader.deserialize::<OperationCsvRow>();
        let mut index = 0;

        while let Some(record) = records.next().await {
            match record {
                Ok(operation_csv_row) => {
                    let operation_request_result = OperationRequest::new_from_csv(operation_csv_row);
                    match operation_request_result {
                        Ok(operation_request) => payments_engine.process(operation_request, index),
                        Err(error_description) => {
                            error!("Error while converting CSV row to Operation Request at index {index} - {error_description}");
                        }
                    }
                }
                Err(error) => {
                    error!("Error while deserializing CSV row at index {index} - Error: {error}")
                }
            }
            index += 1;
        }

        Ok(payments_engine)
    })
}

#[cfg(test)]
mod process_from_csv_use_case_test;
