use csv::{Reader, ReaderBuilder, StringRecord, Writer, WriterBuilder};
use std::error::Error;
use std::fs::File;

use crate::account::ClientAccountDTO;

pub fn import(path: &String) -> Result<Reader<File>, Box<dyn Error>> {
    let rdr = ReaderBuilder::new().from_path(path)?;
    Ok(rdr)
}

pub fn create_client_output_csv(path: &String) -> Result<Writer<File>, Box<dyn Error>> {
    let writer = WriterBuilder::new().from_path(path)?;
    // TODO include writer output
    Ok(writer)
}

// TODO - swallowing the error here
pub fn write_client_record(writer: &mut Writer<File>, record: ClientAccountDTO) {
    let string_record: StringRecord = record.into();
    let result = writer.write_record(&string_record);
}
