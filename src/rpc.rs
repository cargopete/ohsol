use crate::log_parser::{parse_logs, LogEntry};
use crate::output::DecodedError;
use crate::decoder;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use std::str::FromStr;

pub fn get_rpc_url(provided: Option<String>) -> String {
    provided.unwrap_or_else(|| {
        std::env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string())
    })
}

pub fn fetch_transaction_errors(
    rpc_url: &str,
    signature: &str,
) -> anyhow::Result<Vec<DecodedError>> {
    let sig = Signature::from_str(signature)
        .map_err(|e| anyhow::anyhow!("Invalid signature: {}", e))?;

    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let tx = client
        .get_transaction_with_config(
            &sig,
            solana_client::rpc_config::RpcTransactionConfig {
                encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
                commitment: Some(CommitmentConfig::confirmed()),
                max_supported_transaction_version: Some(0),
            },
        )
        .map_err(|e| anyhow::anyhow!("Failed to fetch transaction: {}", e))?;

    let meta = tx
        .transaction
        .meta
        .ok_or_else(|| anyhow::anyhow!("Transaction has no metadata"))?;

    let logs = match meta.log_messages {
        solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
        _ => vec![],
    };
    let parsed_logs = parse_logs(&logs);

    let mut errors = Vec::new();
    let mut current_program: Option<String> = None;
    let mut current_depth: u8 = 0;

    for entry in parsed_logs {
        match entry {
            LogEntry::Invoke { program_id, depth } => {
                current_program = Some(program_id);
                current_depth = depth;
            }
            LogEntry::AnchorError {
                file,
                line,
                code_name,
                code_number,
                message,
            } => {
                let mut error = DecodedError::new(code_number)
                    .with_name(code_name)
                    .with_message(message)
                    .with_source(file, line)
                    .with_cpi_depth(current_depth);

                if let Some(ref pid) = current_program {
                    error = error.with_program(pid.clone());
                }

                errors.push(error);
            }
            LogEntry::CustomError { hex_code } => {
                let code = u32::from_str_radix(&hex_code, 16).unwrap_or(0);
                let mut error = decoder::decode_error(code, current_program.as_deref());
                error = error.with_cpi_depth(current_depth);
                errors.push(error);
            }
            LogEntry::Failed { program_id, error: error_msg } => {
                // Extract error code from the error message if it's a custom error
                if let Some(hex_code) = error_msg.strip_prefix("custom program error: 0x") {
                    let code = u32::from_str_radix(hex_code, 16).unwrap_or(0);
                    let mut error = decoder::decode_error(code, Some(&program_id));
                    error = error.with_cpi_depth(current_depth);
                    errors.push(error);
                }
            }
            _ => {}
        }
    }

    if errors.is_empty() {
        return Err(anyhow::anyhow!("No errors found in transaction"));
    }

    Ok(errors)
}
