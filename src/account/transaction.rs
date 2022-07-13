use csv::{StringRecord};

use super::TransactionError;

#[derive(Debug, Clone)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}


pub type TransactionID = u32;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client_id: u32,
    pub tx_id: TransactionID,
    pub amount: Option<f32>,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    NotYetCommitted,
    Committed,
    Disputed,
    Chargeback,
    Error(TransactionError),
}

pub enum TransactionValidationError {
    MissingTransactionType,
    MissingAmountForWithdrawal,
    MissingAmountForDeposit,
    MissingClientID,
    MissingTransactionID,
    InvalidTransactionType,
    InvalidAmount,
    InvalidIDFormat
}

impl TryFrom<StringRecord> for Transaction {
    type Error = TransactionValidationError;

    fn try_from(record: StringRecord) -> Result<Self, Self::Error> {
        let transaction_type = parse_and_validate_transaction_type(&record)?;
        let client_id = parse_and_validate_id(&record, 1)?;
        let tx_id = parse_and_validate_id(&record, 2)?;
        let amount = parse_and_validate_amount(&record, &transaction_type)?;
        Ok(Transaction {
            transaction_type,
            client_id,
            tx_id,
            amount,
            status: TransactionStatus::NotYetCommitted,
        })
    }
}

fn parse_and_validate_transaction_type(record: &StringRecord) -> Result<TransactionType, TransactionValidationError> {
    if let Some(parsed_transaction_type) = record.get(0) {
        return match parsed_transaction_type.trim().to_lowercase().as_str() {
            "deposit" => Ok(TransactionType::Deposit),
            "withdrawal" => Ok(TransactionType::Withdrawal),
            "dispute" => Ok(TransactionType::Dispute),
            "resolve" => Ok(TransactionType::Resolve),
            "chargeback" => Ok(TransactionType::Chargeback),
            _ => Err(TransactionValidationError::InvalidTransactionType)
        }
    }
    Err(TransactionValidationError::MissingTransactionType)
}

fn parse_and_validate_amount(record: &StringRecord, transaction_type: &TransactionType) -> Result<Option<f32>, TransactionValidationError> {
    if let Some(amnt) = record.get(3) {
        if let Ok(parsed_amount) = format!("{:.4}",amnt.trim()).parse::<f32>() {
            Ok(Some(parsed_amount))
        } else {
            Err(TransactionValidationError::InvalidAmount)
        }
    } else {
        match transaction_type {
            TransactionType::Deposit => Err(TransactionValidationError::MissingAmountForDeposit),
            TransactionType::Withdrawal => Err(TransactionValidationError::MissingAmountForWithdrawal),
            _ => Ok(None)
        }    
    }
}

fn parse_and_validate_id(record: &StringRecord, record_index: usize) -> Result<u32, TransactionValidationError> {
    if let Some(id_str) = record.get(record_index) {
        if let Ok(id) = id_str.trim().parse::<u32>()  {
            Ok(id)   
        } else {
            Err(TransactionValidationError::InvalidIDFormat)
        }
    } else {
        match record_index {
            1 => Err(TransactionValidationError::MissingClientID),
            _ => Err(TransactionValidationError::MissingTransactionID)
        }
    }
}