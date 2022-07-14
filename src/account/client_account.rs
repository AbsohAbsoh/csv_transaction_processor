use std::collections::HashMap;

use csv::StringRecord;

use crate::account::transaction::{
    Transaction, TransactionID, TransactionStatus, TransactionType,
};

use super::TransactionError;

pub type ClientID = u16;

impl Into<StringRecord> for &ClientAccount {
    fn into(self) -> StringRecord {
        let mut record = StringRecord::new();
        record.push_field(&self.client_id.to_string());
        record.push_field(&format!("{:.4}", &self.available));
        record.push_field(&format!("{:.4}", &self.held));
        record.push_field(&format!("{:.4}", &self.total));
        record.push_field(&self.locked.to_string());
        record
    }
}

#[derive(Debug, Clone)]
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

    pub fn apply_transaction(&mut self, transaction: Transaction) -> Result<(), TransactionError> {
        if self.locked {
            return Err(TransactionError::AccountLocked);
        }
        let result = match transaction.transaction_type {
            TransactionType::Deposit => self.deposit(transaction),
            TransactionType::Withdrawal => self.withdrawal(transaction),
            TransactionType::Dispute => self.dispute(&transaction.tx_id),
            TransactionType::Resolve => self.resolve(&transaction.tx_id),
            TransactionType::Chargeback => self.chargeback(&transaction.tx_id),
        };
        // TODO we cant use the transaction id here, since for disputed/resolved transactions it is just referencing the prior transactions
        // TODO however we can simply update their statuses
        // TODO these trasaction inserts should probably happen on the individual deposit/withdrawl/dispute/resolve functions since they're unique to them
        result
    }

    fn deposit(&mut self, mut transaction: Transaction) -> Result<(), TransactionError> {
        if let Some(amount) = transaction.amount {
            let amount_with_precision = format!("{:.4}", amount).parse::<f32>().unwrap();
            self.total = self.total + amount_with_precision;
            self.available = self.available + amount_with_precision;
            transaction.status = TransactionStatus::Committed;
            self.transactions.insert(transaction.tx_id.clone(), transaction);
            Ok(())
        } else {
            Err(TransactionError::MissingDepositAmount)
        }
    }

    // TODO handle amount unwrap
    fn withdrawal(&mut self, mut transaction: Transaction) -> Result<(), TransactionError> {
        let amount_with_precision = format!("{:.4}", transaction.amount.unwrap().clone()).parse::<f32>().unwrap();
        if self.available.lt(&amount_with_precision) {
            Err(TransactionError::InsufficientFundsForWithdrawal)
        } else {
            self.total -= amount_with_precision;
            self.available -= amount_with_precision;
            transaction.status = TransactionStatus::Committed;
            self.transactions.insert(transaction.tx_id.clone(), transaction);
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
            Err(TransactionError::DisputedTransactionNotFound)
        }
    }

    fn resolve(&mut self, transaction_id: &TransactionID) -> Result<(), TransactionError> {
        if let Some(transaction) = self.transactions.get_mut(transaction_id) {
            debug!("{:?}", transaction.status);
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
