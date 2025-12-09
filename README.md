# ohsol

A Rust CLI tool for decoding Solana program errors. Turns cryptic error codes like `0x1771` into human-readable messages.

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
./target/release/ohsol --help
```

## Usage

### Decode error codes

Decode hex or decimal error codes:

```bash
# Hex format
ohsol decode 0x1771

# Decimal format
ohsol decode 6001

# With program context
ohsol decode 17 --program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
```

### Parse transaction errors

Fetch and decode errors from a transaction signature:

```bash
ohsol tx 5h6xBEauJ3PK6iKvmVCQfVwJyXKPhvwSeNada5ZpW8cGzLkPreF3r5mNVUXqkJ8G5gZq7mW8y7HX8p9vFxM6p3No
```

### List known errors

List all errors for a known program:

```bash
# By program name
ohsol list spl-token
ohsol list jupiter

# By program ID
ohsol list TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
```

### Fetch program IDL

Download and display a program's IDL:

```bash
# Print to stdout
ohsol idl 6khKp4BeJpCjBY1Eh39ybiqbfRnrn2UzWeUARjQLXYRC

# Save to file
ohsol idl 6khKp4BeJpCjBY1Eh39ybiqbfRnrn2UzWeUARjQLXYRC -o program.json
```

### Pipe from logs

Extract and decode errors from piped input:

```bash
solana logs --program YourProgram | ohsol decode --stdin
```

### JSON output

All commands support JSON output with `--json`:

```bash
ohsol decode 0x1771 --json
ohsol list spl-token --json
```

## Supported Programs

Built-in error databases for:

- **SPL Token** (`spl-token`)
- **Jupiter** (`jupiter`)
- **Anchor Framework** (all standard errors)

For other programs, ohsol will attempt to fetch the IDL on-chain.

## Error Code Ranges

- `0-99`: Anchor instruction errors
- `100-999`: Anchor instruction errors
- `1000-1999`: IDL errors
- `2000-2999`: Constraint errors (ConstraintMut, ConstraintSeeds, etc.)
- `3000-4099`: Account errors
- `4100-4999`: Miscellaneous Anchor errors
- `6000+`: Custom program errors

## Configuration

Set RPC URL via environment variable or flag:

```bash
export SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
```

Or use the flag:

```bash
ohsol tx <signature> --rpc-url https://api.devnet.solana.com
```

## Examples

```bash
# Decode a constraint error
$ ohsol decode 2006
Error 2006 (0x7d6)
  Program: Anchor Framework
  Name: ConstraintSeeds
  Message: A seeds constraint was violated

# List SPL Token errors
$ ohsol list spl-token
Known errors for spl-token (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA):

┌──────┬──────┬───────────────────┬─────────────────────────────────────────┐
│ Code │ Hex  │ Name              │ Message                                 │
├──────┼──────┼───────────────────┼─────────────────────────────────────────┤
│ 0    │ 0x0  │ NotRentExempt     │ Lamport balance below rent-exempt...    │
│ 1    │ 0x1  │ InsufficientFunds │ Insufficient funds                      │
│ 17   │ 0x11 │ AccountFrozen     │ Account is frozen                       │
└──────┴──────┴───────────────────┴─────────────────────────────────────────┘
```

## License

MIT
