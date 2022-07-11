use std::collections::HashMap;

use tokio::{
    sync::{mpsc, oneshot},
    task,
};

use super::{
    client_account::{ClientAccount, ClientID},
    transaction::{Transaction, TransactionDTO, TransactionError},
};

pub struct AccountActor {
    accounts: HashMap<ClientID, ClientAccount>,
    listener: mpsc::Receiver<TransactionDTO>,
}

impl AccountActor {
    pub fn spawn(receiver_buffer_size: usize) -> mpsc::Sender<TransactionDTO> {
        let (sender, receiver) = mpsc::channel(receiver_buffer_size);
        let mut actor = AccountActor {
            accounts: HashMap::new(),
            listener: receiver,
        };
        task::spawn(async move {
            while let Some(transaction_dto) = actor.listener.recv().await {
                match transaction_dto {
                    TransactionDTO::ProcessTransaction(transaction, cb) => {
                        let result = actor.process_transaction(transaction);
                        let _ = cb.send(result);
                    }
                    TransactionDTO::GetAllClients(cb) => {
                        for (_, account) in &actor.accounts {
                            let _ = cb.send(account.into()).await;
                        }
                    }
                }
            }
        });
        sender
    }

    /// Assuming processing order is important and that process_transaction should NOT be parallelizable
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
