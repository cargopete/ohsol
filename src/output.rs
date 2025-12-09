use colored::Colorize;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct DecodedError {
    pub program_id: String,
    pub error_code: u32,
    pub error_hex: String,
    pub error_name: Option<String>,
    pub error_message: Option<String>,
    pub source_file: Option<String>,
    pub source_line: Option<u32>,
    pub cpi_depth: Option<u8>,
}

impl DecodedError {
    pub fn new(code: u32) -> Self {
        Self {
            program_id: String::new(),
            error_code: code,
            error_hex: format!("0x{:x}", code),
            error_name: None,
            error_message: None,
            source_file: None,
            source_line: None,
            cpi_depth: None,
        }
    }

    pub fn with_program(mut self, program_id: String) -> Self {
        self.program_id = program_id;
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.error_name = Some(name);
        self
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.error_message = Some(message);
        self
    }

    pub fn with_source(mut self, file: String, line: u32) -> Self {
        self.source_file = Some(file);
        self.source_line = Some(line);
        self
    }

    pub fn with_cpi_depth(mut self, depth: u8) -> Self {
        self.cpi_depth = Some(depth);
        self
    }
}

pub fn format_error(error: &DecodedError, json_mode: bool) {
    if json_mode {
        println!("{}", serde_json::to_string_pretty(error).unwrap());
    } else {
        println!(
            "{} {} ({})",
            "Error".red().bold(),
            error.error_code,
            error.error_hex.yellow()
        );

        if !error.program_id.is_empty() {
            println!("  {} {}", "Program:".bold(), error.program_id);
        }

        if let Some(name) = &error.error_name {
            println!("  {} {}", "Name:".bold(), name.cyan());
        }

        if let Some(msg) = &error.error_message {
            println!("  {} {}", "Message:".bold(), msg);
        }

        if let Some(file) = &error.source_file {
            if let Some(line) = error.source_line {
                println!("  {} {}:{}", "Source:".bold(), file.dimmed(), line);
            }
        }

        if let Some(depth) = error.cpi_depth {
            println!("  {} {}", "CPI Depth:".bold(), depth);
        }
    }
}

pub fn format_error_list(errors: &[DecodedError], json_mode: bool) {
    if json_mode {
        println!("{}", serde_json::to_string_pretty(errors).unwrap());
    } else {
        use comfy_table::{Table, presets::UTF8_FULL};

        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_header(vec!["Code", "Hex", "Name", "Message"]);

        for error in errors {
            table.add_row(vec![
                error.error_code.to_string(),
                error.error_hex.clone(),
                error.error_name.clone().unwrap_or_else(|| "Unknown".to_string()),
                error.error_message.clone().unwrap_or_else(|| "-".to_string()),
            ]);
        }

        println!("{}", table);
    }
}
