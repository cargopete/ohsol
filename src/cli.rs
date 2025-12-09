use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ohsol", version, about = "Decode Solana program errors")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true, env = "SOLANA_RPC_URL")]
    pub rpc_url: Option<String>,

    #[arg(long, global = true)]
    pub json: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Decode a hex or decimal error code
    Decode {
        /// Error code (0x1771 or 6001)
        code: String,

        /// Program ID for context
        #[arg(short, long)]
        program: Option<String>,

        /// Read error codes from stdin
        #[arg(long)]
        stdin: bool,
    },

    /// Parse a transaction and explain errors
    Tx {
        /// Transaction signature
        signature: String,
    },

    /// Fetch and cache a program's IDL
    Idl {
        /// Program ID
        program_id: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// List known errors for a program
    List {
        /// Program ID or name (e.g., "spl-token", "jupiter")
        program: String,
    },
}
