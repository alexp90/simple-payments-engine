mod output_printer;
mod domain;
mod use_case;

use std::env;
use std::error::Error;
use crate::domain::payments_engine::PaymentsEngine;
use crate::output_printer::print_outcome_to_stdout;
use crate::use_case::process_from_csv_use_case::process_from_csv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        eprintln!("No arguments passed!\nUsage: {} <input_csv_file>", &arguments[0]);
        std::process::exit(1);
    }

    env_logger::init();

    let payments_engine = PaymentsEngine::new();

    let payments_engine_after_processing = process_from_csv(arguments[1].to_owned(), payments_engine).await.await??;

    print_outcome_to_stdout(payments_engine_after_processing.accounts());

    Ok(())
}

