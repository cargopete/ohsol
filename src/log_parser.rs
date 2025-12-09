use regex::Regex;
use std::sync::LazyLock;

static LOG_PATTERNS: LazyLock<LogPatterns> = LazyLock::new(|| LogPatterns::new());

pub struct LogPatterns {
    invoke: Regex,
    success: Regex,
    failed: Regex,
    consumed: Regex,
    anchor_error: Regex,
    custom_error: Regex,
}

impl LogPatterns {
    fn new() -> Self {
        Self {
            invoke: Regex::new(r"^Program ([1-9A-HJ-NP-Za-km-z]{32,}) invoke \[(\d+)\]$").unwrap(),
            success: Regex::new(r"^Program ([1-9A-HJ-NP-Za-km-z]{32,}) success$").unwrap(),
            failed: Regex::new(r"^Program ([1-9A-HJ-NP-Za-km-z]{32,}) failed: (.*)$").unwrap(),
            consumed: Regex::new(
                r"^Program ([1-9A-HJ-NP-Za-km-z]{32,}) consumed (\d+) of (\d+) compute units$",
            )
            .unwrap(),
            anchor_error: Regex::new(
                r"AnchorError thrown in ([^:]+):(\d+)\. Error Code: (\w+)\. Error Number: (\d+)\. Error Message: ([^.]+)",
            )
            .unwrap(),
            custom_error: Regex::new(r"custom program error: 0x([0-9a-fA-F]+)").unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LogEntry {
    Invoke { program_id: String, depth: u8 },
    Success { program_id: String },
    Failed { program_id: String, error: String },
    Consumed { program_id: String, used: u64, total: u64 },
    AnchorError {
        file: String,
        line: u32,
        code_name: String,
        code_number: u32,
        message: String,
    },
    CustomError { hex_code: String },
    Other { message: String },
}

pub fn parse_log_line(line: &str) -> LogEntry {
    if let Some(caps) = LOG_PATTERNS.invoke.captures(line) {
        return LogEntry::Invoke {
            program_id: caps[1].to_string(),
            depth: caps[2].parse().unwrap_or(0),
        };
    }

    if let Some(caps) = LOG_PATTERNS.success.captures(line) {
        return LogEntry::Success {
            program_id: caps[1].to_string(),
        };
    }

    if let Some(caps) = LOG_PATTERNS.failed.captures(line) {
        return LogEntry::Failed {
            program_id: caps[1].to_string(),
            error: caps[2].to_string(),
        };
    }

    if let Some(caps) = LOG_PATTERNS.consumed.captures(line) {
        return LogEntry::Consumed {
            program_id: caps[1].to_string(),
            used: caps[2].parse().unwrap_or(0),
            total: caps[3].parse().unwrap_or(0),
        };
    }

    if let Some(caps) = LOG_PATTERNS.anchor_error.captures(line) {
        return LogEntry::AnchorError {
            file: caps[1].to_string(),
            line: caps[2].parse().unwrap_or(0),
            code_name: caps[3].to_string(),
            code_number: caps[4].parse().unwrap_or(0),
            message: caps[5].to_string(),
        };
    }

    if let Some(caps) = LOG_PATTERNS.custom_error.captures(line) {
        return LogEntry::CustomError {
            hex_code: caps[1].to_string(),
        };
    }

    LogEntry::Other {
        message: line.to_string(),
    }
}

pub fn parse_logs(logs: &[String]) -> Vec<LogEntry> {
    logs.iter().map(|line| parse_log_line(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_invoke() {
        let line = "Program 11111111111111111111111111111111 invoke [1]";
        match parse_log_line(line) {
            LogEntry::Invoke { program_id, depth } => {
                assert_eq!(program_id, "11111111111111111111111111111111");
                assert_eq!(depth, 1);
            }
            _ => panic!("Expected Invoke"),
        }
    }

    #[test]
    fn test_parse_anchor_error() {
        let line = "Program log: AnchorError thrown in programs/myprogram/src/lib.rs:42. Error Code: AmountTooLarge. Error Number: 6001. Error Message: Amount must be less than or equal to 100";
        match parse_log_line(line) {
            LogEntry::AnchorError {
                file,
                line: line_num,
                code_name,
                code_number,
                message,
            } => {
                assert_eq!(file, "programs/myprogram/src/lib.rs");
                assert_eq!(line_num, 42);
                assert_eq!(code_name, "AmountTooLarge");
                assert_eq!(code_number, 6001);
                assert_eq!(message, "Amount must be less than or equal to 100");
            }
            _ => panic!("Expected AnchorError"),
        }
    }

    #[test]
    fn test_parse_custom_error() {
        let line = "Program failed: custom program error: 0x1771";
        match parse_log_line(line) {
            LogEntry::CustomError { hex_code } => {
                assert_eq!(hex_code, "1771");
            }
            _ => panic!("Expected CustomError"),
        }
    }
}
