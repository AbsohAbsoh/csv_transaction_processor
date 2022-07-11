use std::collections::HashMap;

use csv::StringRecord;

use crate::account::transaction::{
    Transaction, TransactionError, TransactionID, TransactionStatus, TransactionType,
};

pub type ClientID = u32;

pub struct ClientAccountDTO {
    pub client_id: ClientID,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}

impl From<&ClientAccount> for ClientAccountDTO {
    fn from(account: &ClientAccount) -> Self {
        ClientAccountDTO {
            client_id: account.client_id,
            available: account.available,
            held: account.held,
            total: account.total,
            locked: account.locked,
        }
    }
}

impl Into<StringRecord> for ClientAccountDTO {
    fn into(self) -> StringRecord {
        todo!()
    }
}

pub struct ClientAccount {
    client_id: ClientID,
    available: f32,
    held: f32,
    total: f32,
    transactions: HashMap<TransactionID, Transaction>,
    locked: bool,
}

impl ClientAccount {
    pub fn new(id: ClientID) -> Self {
        ClientAccount {
            client_id: id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            transactions: HashMap::new(),
            locked: false,
        }
    }

    // TODO ENSURE TRANSACTIONS ARE APPLIED WITH EXACTLY FOUR LEVELS OF DECIMAL PRECISION
    // TODO consider using a linked list or some other DS as a way to deal with transaction application?
    pub fn apply_transaction(&mut self, transaction: Transaction) -> Result<(), TransactionError> {
        if self.locked {
            return Err(TransactionError::AccountLocked);
        }
        let result = match transaction.transaction_type {
            TransactionType::Deposit => self.deposit(&transaction),
            TransactionType::Withdrawal => self.withdrawal(&transaction.amount.unwrap()),
            TransactionType::Dispute => self.dispute(&transaction.tx_id),
            TransactionType::Resolve => self.resolve(&transaction.tx_id),
            TransactionType::Chargeback => self.chargeback(&transaction.tx_id),
        };
        self.transactions
            .insert(transaction.tx_id.clone(), transaction);
        result
    }

    fn deposit(&mut self, transaction: &Transaction) -> Result<(), TransactionError> {
        if let Some(amount) = transaction.amount {
            self.total = self.total + amount;
            self.available = self.available + amount;
            Ok(())
        } else {
            Err(TransactionError::MissingDepositAmount)
        }
    }

    fn withdrawal(&mut self, amount: &f32) -> Result<(), TransactionError> {
        if self.available.lt(amount) {
            Err(TransactionError::InsufficientFundsForWithdrawal)
        } else {
            self.total -= amount;
            self.available -= amount;
            Ok(())
        }
    }

    // TODO handle transaction id not found
    // TODO handle if dispute would make balance negative
    fn dispute(&mut self, transaction_id: &TransactionID) -> Result<(), TransactionError> {
        if let Some(transaction) = self.transactions.get_mut(transaction_id) {
            transaction.status = TransactionStatus::Disputed;
            if let Some(amount) = transaction.amount {
                self.available -= amount;
                self.held += amount;
            }
            Ok(())
        } else {
            Err(TransactionError::ResolvedTransactionNotDisputed)
        }
    }

    fn resolve(&mut self, transaction_id: &TransactionID) -> Result<(), TransactionError> {
        if let Some(transaction) = self.transactions.get_mut(transaction_id) {
            if let TransactionStatus::Disputed = transaction.status {
                transaction.status = TransactionStatus::Committed;
                if let Some(amount) = transaction.amount {
                    self.available += amount;
                    self.held -= amount;
                }
                return Ok(());
            }
            Err(TransactionError::ResolvedTransactionNotDisputed)
        } else {
            Err(TransactionError::ResolvedTransactionNotFound)
        }
    }

    fn chargeback(&mut self, transaction_id: &TransactionID) -> Result<(), TransactionError> {
        if let Some(transaction) = self.transactions.get_mut(transaction_id) {
            if let TransactionStatus::Disputed = transaction.status {
                transaction.status = TransactionStatus::Chargeback;
                if let Some(amount) = transaction.amount {
                    self.total -= amount;
                    self.held -= amount;
                }
                self.locked = true;
                Ok(())
            } else {
                Err(TransactionError::ChargebackWasNotDisputed)
            }
        } else {
            Err(TransactionError::ChargebackTransactionNotFound)
        }
    }
}
