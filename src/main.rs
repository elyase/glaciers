use std::fs;
use std::path::PathBuf;

use decoding::abi_reader;
use decoding::decoder::process_log_files;
use decoding::decoder::DecodeError;
use thiserror::Error;

const TOPIC0_FILE_PATH: &str = "ABIs/ethereum__abis_topic0.parquet";
const RAW_LOGS_FOLDER_PATH: &str = "data/logs";

#[derive(Error, Debug)]
enum AppError {
    #[error("Decode error: {0}")]
    DecodeError(#[from] DecodeError),
    #[error("Polars error: {0}")]
    PolarsError(#[from] polars::prelude::PolarsError),
    #[error("Join error: {0}")]    
    JoinError(#[from] tokio::task::JoinError),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Read ABI list
    let abi_list_df = abi_reader::read_abis_topic0(TOPIC0_FILE_PATH)?;

    // Collect log files' paths from RAW_LOGS_FOLDER_PATH
    let log_files: Vec<PathBuf> = fs::read_dir(RAW_LOGS_FOLDER_PATH)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();

    // process the log files concurrently
    process_log_files(log_files, abi_list_df).await?;

    Ok(())
}
