use std::collections::HashMap;

use csv::StringRecord;
use tokio::{sync::mpsc, task};

use crate::csv_io::create_client_output_csv;

use super::{
    client_account::{ClientAccount, ClientID},
    transaction::{Transaction},
};

pub struct AccountActor {
    accounts: HashMap<ClientID, ClientAccount>,
}

impl AccountActor {
    pub fn spawn(receiver_buffer_size: usize) -> mpsc::Sender<TransactionCommand> {
        let (sender, mut receiver) = mpsc::channel(receiver_buffer_size);
        let mut actor = AccountActor {
            accounts: HashMap::new(),
        };
        task::spawn(async move {
            while let Some(transaction_dto) = receiver.recv().await {
                match transaction_dto {
                    TransactionCommand::ProcessTransaction(transaction) => {
                        if let Err(transaction_process_error) = actor.process_transaction(transaction) {
                            debug!("Transaction process error: {:?}", transaction_process_error);
                        }
                    }
                    TransactionCommand::WriteClientsToStdout => {
                        let mut writer = create_client_output_csv();
                        for (_, account) in &actor.accounts {
                            let record: StringRecord = account.into();
                            let _ = writer.write_record(&record);
                        }
                    }
                }
            }
        });
        sender
    }

    fn process_transaction(&mut self, transaction: Transaction) -> Result<(), TransactionError> {
        let client_id = transaction.client_id.clone();
        let account = self.get_or_create_account(&client_id);
        account.apply_transaction(transaction)
    }

    fn get_or_create_account(&mut self, client_id: &ClientID) -> &mut ClientAccount {
        if let None = self.accounts.get_mut(client_id) {
            let account = ClientAccount::new(client_id.clone());
            self.accounts.insert(client_id.clone(), account);
        }
        self.accounts.get_mut(client_id).unwrap()
    }
}

#[derive(Debug)]
pub enum TransactionCommand {
    ProcessTransaction(Transaction),
    /// Okay.. in a real system the ClientAccountActor would probably not be responsible for writing to Stdout
    WriteClientsToStdout,
}


#[derive(Debug, Clone)]
pub enum TransactionError {
    AccountLocked,
    InsufficientFundsForWithdrawal,
    MissingDepositAmount,
    MissingWithdrawalAmount,
    ChargebackWasNotDisputed,
    ChargebackTransactionNotFound,
    DisputedTransactionNotFound,
    ResolvedTransactionNotFound,
    ResolvedTransactionNotDisputed,
}