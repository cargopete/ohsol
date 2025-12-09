mod cli;
mod decoder;
mod idl;
mod log_parser;
mod output;
mod rpc;

use clap::Parser;
use cli::{Cli, Commands};
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Decode { code, program, stdin } => {
            if *stdin {
                handle_decode_stdin(&cli, program.as_deref())?;
            } else {
                handle_decode(&cli, code, program.as_deref())?;
            }
        }
        Commands::Tx { signature } => {
            handle_transaction(&cli, signature)?;
        }
        Commands::Idl { program_id, output } => {
            handle_idl(&cli, program_id, output.as_ref())?;
        }
        Commands::List { program } => {
            handle_list(&cli, program)?;
        }
    }

    Ok(())
}

fn handle_decode(cli: &Cli, code_str: &str, program_id: Option<&str>) -> anyhow::Result<()> {
    let code = decoder::anchor::parse_error_code(code_str)?;
    let error = decoder::decode_error(code, program_id);
    output::format_error(&error, cli.json);
    Ok(())
}

fn handle_decode_stdin(cli: &Cli, program_id: Option<&str>) -> anyhow::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Try to extract error codes from the line
        if let Some(hex_code) = extract_hex_error(&line) {
            if let Ok(code) = u32::from_str_radix(&hex_code, 16) {
                let error = decoder::decode_error(code, program_id);
                output::format_error(&error, cli.json);
            }
        } else if let Ok(code) = decoder::anchor::parse_error_code(trimmed) {
            let error = decoder::decode_error(code, program_id);
            output::format_error(&error, cli.json);
        }
    }
    Ok(())
}

fn extract_hex_error(line: &str) -> Option<String> {
    // Look for "0x" followed by hex digits
    if let Some(pos) = line.find("0x") {
        let rest = &line[pos + 2..];
        let hex_end = rest
            .chars()
            .take_while(|c| c.is_ascii_hexdigit())
            .count();
        if hex_end > 0 {
            return Some(rest[..hex_end].to_string());
        }
    }
    None
}

fn handle_transaction(cli: &Cli, signature: &str) -> anyhow::Result<()> {
    let rpc_url = rpc::get_rpc_url(cli.rpc_url.clone());
    let errors = rpc::fetch_transaction_errors(&rpc_url, signature)?;

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&errors)?);
    } else {
        println!("Transaction: {}", signature);
        println!("Found {} error(s):\n", errors.len());

        for (i, error) in errors.iter().enumerate() {
            if i > 0 {
                println!();
            }
            output::format_error(error, false);
        }
    }

    Ok(())
}

fn handle_idl(cli: &Cli, program_id_str: &str, output_path: Option<&std::path::PathBuf>) -> anyhow::Result<()> {
    let program_id = decoder::parse_program_id(program_id_str)?;
    let rpc_url = rpc::get_rpc_url(cli.rpc_url.clone());

    println!("Fetching IDL for program: {}", program_id);

    let idl = idl::fetch_idl_with_url(&rpc_url, &program_id)?;

    let json = serde_json::to_string_pretty(&idl)?;

    if let Some(path) = output_path {
        std::fs::write(path, &json)?;
        println!("IDL saved to: {}", path.display());
    } else {
        println!("{}", json);
    }

    println!("\nProgram: {} v{}", idl.get_name(), idl.get_version());
    println!("Format: {}", if idl.is_modern_format() { "Modern" } else { "Legacy" });
    println!("Errors: {}", idl.errors.len());

    Ok(())
}

fn handle_list(cli: &Cli, program: &str) -> anyhow::Result<()> {
    if let Some(errors) = decoder::registry::list_program_errors(program) {
        if cli.json {
            println!("{}", serde_json::to_string_pretty(&errors)?);
        } else {
            if let Some(name) = decoder::registry::get_program_name(program) {
                println!("Known errors for {} ({}):\n", name, program);
            } else {
                println!("Known errors for {}:\n", program);
            }
            output::format_error_list(&errors, false);
        }
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Unknown program: {}. Try using a program ID or known name like 'spl-token' or 'jupiter'",
            program
        ))
    }
}
