mod account;
mod cli;
mod csv_io;

use std::{sync::Arc, time::Duration};

use csv_io::{create_client_output_csv, write_client_record};
// use futures::future;
// use std::env;
use futures::future::join_all;

use account::{deserialize_transaction, AccountActor, TransactionDTO};
use tokio::{
    sync::{mpsc, oneshot},
    task,
    time::{sleep, sleep_until},
};

// TAKE 1: non-pipe style

#[tokio::main]
async fn main() {
    let (input_path, output_path) = cli::get_io_paths();
    let reader_result = csv_io::import(&input_path);

    // // Can you stream values through
    // // memory as opposed to loading the entire data set upfront? What if your
    // // code was bundled in a server, and these CSVs came from thousands of
    // // c&oncurrent TCP streams?
    // TODO use Rayon to parellelize everything?

    if let Ok(mut reader) = reader_result {
        let transaction_sender = AccountActor::spawn(10);
        let send_handles = reader.records().filter_map(|record_result| {
            if let Ok(record) = record_result {
                let transaction_result = deserialize_transaction(record);
                if let Ok(transaction) = transaction_result {
                    let (tx, rx) = oneshot::channel();
                    let _ = transaction_sender
                        .send(TransactionDTO::ProcessTransaction(transaction, tx));
                    return Some(rx);
                }
            }
            None
        });
        let mut transaction_responses = join_all(send_handles).await;
        transaction_responses
            .drain(..)
            .for_each(|transaction_response| {
                // Eschewing receiver errors for now
                if let Err(transaction_error) = transaction_response.unwrap() {
                    println!("Transaction error: {:?}", transaction_error);
                }
            });
        let (tx, mut rx) = mpsc::channel(100);
        let mut writer = create_client_output_csv(&output_path).unwrap(); // handle unwrap
        task::spawn(async move {
            // TODO bad - find a better way to process these threads
            sleep(Duration::from_millis(100)).await;
            let _ = transaction_sender.send(TransactionDTO::GetAllClients(tx));
        });
        while let Some(client_record) = rx.recv().await {
            write_client_record(&mut writer, client_record)
        }
    }
}
