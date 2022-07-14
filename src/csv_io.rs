use csv::{Reader, ReaderBuilder, StringRecord, Writer};
use std::error::Error;
use std::fs::File;
use std::io::{stdout, Stdout};

pub fn import(path: &String) -> Result<Reader<File>, Box<dyn Error>> {
    let rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path(path)?;
    Ok(rdr)
}

#[derive(Debug)]
pub enum ReaderHeaderValidationError {
    NoHeaders,
    MissingHeaders,
    HeaderReadFailure
}

pub fn validate_headers(reader: &mut Reader<File>) -> Result<(), ReaderHeaderValidationError> {
    if reader.has_headers() == false {
        return Err(ReaderHeaderValidationError::NoHeaders);
    }
    if let Ok(headers) = reader.headers() {
        let required_headers = vec![
            TYPE_HEADER,
            CLIENT_HEADER,
            TX_HEADER,
            AMOUNT_HEADER
        ];
        let has_proper_headers = required_headers.iter().enumerate().all(|(index, header)| {
            if let Some(compare_header) = headers.get(index) {
                compare_header.trim().to_lowercase() == *header
            } else {
                false
            }
        });
        if has_proper_headers {
            return Ok(());
        } else {
            return Err(ReaderHeaderValidationError::MissingHeaders); 
        }
    }
    return Err(ReaderHeaderValidationError::HeaderReadFailure);
} 

pub fn create_client_output_csv() -> Writer<Stdout> {
    let mut writer = Writer::from_writer(stdout());
    let _ = writer.write_record(&get_output_headers());
    writer
}

fn get_output_headers() -> StringRecord {
    let mut headers = StringRecord::new();
    headers.push_field(CLIENT_HEADER);
    headers.push_field(AVAILABLE_HEADER);
    headers.push_field(HELD_HEADER);
    headers.push_field(TOTAL_HEADER);
    headers.push_field(LOCKED_HEADER);
    headers
}

static CLIENT_HEADER: &str = "client";
static AVAILABLE_HEADER: &str = "available";
static HELD_HEADER: &str = "held";
static TOTAL_HEADER: &str = "total";
static LOCKED_HEADER: &str = "locked";
static TX_HEADER: &str = "tx";
static TYPE_HEADER: &str = "type";
static AMOUNT_HEADER: &str = "amount";

