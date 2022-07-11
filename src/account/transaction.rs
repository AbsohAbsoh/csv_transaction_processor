use csv::StringRecord;
use tokio::sync::{mpsc, oneshot};

use super::client_account::ClientAccountDTO;

pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

pub enum TransactionDTO {
    ProcessTransaction(Transaction, ProcessTransactionCallback),
    GetAllClients(GetAllClientsCallback),
}

type GetAllClientsCallback = mpsc::Sender<ClientAccountDTO>;
type ProcessTransactionCallback = oneshot::Sender<Result<(), TransactionError>>;

#[derive(Debug)]
pub enum TransactionError {
    AccountLocked,
    InsufficientFundsForWithdrawal,
    MissingDepositAmount,
    ChargebackWasNotDisputed,
    ChargebackTransactionNotFound,
    DisputedTransactionNotFound,
    ResolvedTransactionNotFound,
    ResolvedTransactionNotDisputed,
}

pub type TransactionID = u32;

// TODO maybe these shouldn't be pub?
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client_id: u32,
    pub tx_id: TransactionID,
    pub amount: Option<f32>,
    pub status: TransactionStatus,
}

pub enum TransactionStatus {
    NotYetCommitted,
    Committed,
    Disputed,
    Chargeback,
    Error(TransactionError),
}

pub enum TransactionSerializeError {
    MissingAmountForWithdrawal,
    MissingAmountForDeposit,
    MissingClientID,
    MissingTransactionID,
    InvalidTransactionType,
    InvalidRecordFormat,
}

/// For simplicity's sake we use this pure function instead of serde / serializer lib
pub fn deserialize_transaction(
    serialized_record: StringRecord,
) -> Result<Transaction, TransactionSerializeError> {
    let parsed_transaction_type = serialized_record.get(0).unwrap().trim().to_lowercase();
    let client_id: u32 = serialized_record
        .get(1)
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();
    let tx_id: TransactionID = serialized_record
        .get(2)
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();
    let amount: Option<f32> = if let Some(amnt) = serialized_record.get(3) {
        Some(amnt.trim().parse::<f32>().unwrap())
    } else {
        None
    };
    let transaction_type = match parsed_transaction_type.as_str() {
        "deposit" => TransactionType::Deposit,
        "withdrawl" => TransactionType::Withdrawal,
        "dispute" => TransactionType::Dispute,
        "resolve" => TransactionType::Resolve,
        "chargeback" => TransactionType::Chargeback,
        _ => panic!("Undefined transaction type todo handle this better"),
    };
    Ok(Transaction {
        transaction_type,
        client_id,
        tx_id,
        amount,
        status: TransactionStatus::NotYetCommitted,
    })
}
