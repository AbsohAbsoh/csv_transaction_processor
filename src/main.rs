#[macro_use]
extern crate log;

mod account;
mod csv_io;

use tokio::runtime::Runtime;
use tokio_stream::{self, StreamExt};

use pretty_env_logger::env_logger::Target;

use account::{AccountActor, TransactionCommand};

fn main() {
    initialize_logger();
    Runtime::new().unwrap().block_on(async {
        if let Ok(mut reader) = csv_io::import(&get_input_path()) {
            let transaction_sender = AccountActor::spawn(10);
            let mut send_handles = tokio_stream::iter(reader.records()).filter_map(|record_result| {
                if let Ok(record) = record_result {
                    let transaction_result = record.try_into();
                    match transaction_result {
                        Ok(transaction) => {
                            let execution_handle = transaction_sender
                                .send(TransactionCommand::ProcessTransaction(transaction));
                            return Some(execution_handle);
                        },
                        Err(_) => {
                            // handle error
                        },
                    }
                }
                None
            });
            while let Some(send_handle) = send_handles.next().await {
                let _ = send_handle.await;
            }
            let _ = transaction_sender.send(TransactionCommand::WriteClientsToStdout).await;
        }
    });
}

fn initialize_logger() {
    pretty_env_logger::formatted_builder()
        .target(Target::Stderr)
        .filter_level(log::LevelFilter::Debug)
        .init();
}

fn get_input_path() -> String {
    std::env::args().nth(1).unwrap_or_else(|| {
        panic!("{}", MISSING_INPUT_PATH_ERROR);
    })
}

static MISSING_INPUT_PATH_ERROR: &str = "Input path was not provided.";