use crate::output::DecodedError;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Deserialize, Debug)]
pub struct ErrorEntry {
    pub code: u32,
    pub name: String,
    pub msg: String,
}

#[derive(Deserialize, Debug)]
pub struct ProgramErrors {
    pub name: String,
    pub errors: Vec<ErrorEntry>,
}

#[derive(Deserialize, Debug)]
pub struct ErrorDatabase {
    pub programs: HashMap<String, ProgramErrors>,
    pub anchor_errors: Vec<ErrorEntry>,
}

static ERROR_DB: LazyLock<ErrorDatabase> = LazyLock::new(|| {
    let json_data = include_str!("../../data/errors.json");
    serde_json::from_str(json_data).expect("Failed to parse error database")
});

pub fn lookup_program_error(program_id: &str, code: u32) -> Option<DecodedError> {
    let db = &*ERROR_DB;

    if let Some(program) = db.programs.get(program_id) {
        if let Some(error) = program.errors.iter().find(|e| e.code == code) {
            return Some(
                DecodedError::new(code)
                    .with_program(program_id.to_string())
                    .with_name(error.name.clone())
                    .with_message(error.msg.clone()),
            );
        }
    }

    None
}

pub fn lookup_anchor_error(code: u32) -> Option<DecodedError> {
    let db = &*ERROR_DB;

    if let Some(error) = db.anchor_errors.iter().find(|e| e.code == code) {
        return Some(
            DecodedError::new(code)
                .with_program("Anchor Framework".to_string())
                .with_name(error.name.clone())
                .with_message(error.msg.clone()),
        );
    }

    None
}

pub fn list_program_errors(program_id_or_name: &str) -> Option<Vec<DecodedError>> {
    let db = &*ERROR_DB;

    // Try as program ID first
    if let Some(program) = db.programs.get(program_id_or_name) {
        return Some(
            program
                .errors
                .iter()
                .map(|e| {
                    DecodedError::new(e.code)
                        .with_program(program_id_or_name.to_string())
                        .with_name(e.name.clone())
                        .with_message(e.msg.clone())
                })
                .collect(),
        );
    }

    // Try as program name
    for (pid, program) in db.programs.iter() {
        if program.name == program_id_or_name {
            return Some(
                program
                    .errors
                    .iter()
                    .map(|e| {
                        DecodedError::new(e.code)
                            .with_program(pid.clone())
                            .with_name(e.name.clone())
                            .with_message(e.msg.clone())
                    })
                    .collect(),
            );
        }
    }

    None
}

pub fn get_program_name(program_id: &str) -> Option<String> {
    let db = &*ERROR_DB;
    db.programs.get(program_id).map(|p| p.name.clone())
}
