use csv::{Reader, ReaderBuilder, StringRecord, Writer};
use std::error::Error;
use std::fs::File;
use std::io::{stdout, Stdout};

pub fn import(path: &String) -> Result<Reader<File>, Box<dyn Error>> {
    let rdr = ReaderBuilder::new().from_path(path)?;
    Ok(rdr)
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

type ParseResult = Result<u32, u32>;