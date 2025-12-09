use super::parse::{parse_idl, Idl};
use flate2::read::ZlibDecoder;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::io::Read;

pub fn get_idl_address(program_id: &Pubkey) -> Pubkey {
    let (idl_address, _) = Pubkey::find_program_address(&[b"anchor:idl", program_id.as_ref()], program_id);
    idl_address
}

pub fn fetch_idl(rpc_client: &RpcClient, program_id: &Pubkey) -> anyhow::Result<Idl> {
    let idl_address = get_idl_address(program_id);

    let account = rpc_client
        .get_account(&idl_address)
        .map_err(|e| anyhow::anyhow!("Failed to fetch IDL account: {}", e))?;

    if account.data.len() < 44 {
        return Err(anyhow::anyhow!("IDL account data too short"));
    }

    // IDL data structure:
    // - 8 bytes: discriminator
    // - 32 bytes: authority pubkey
    // - 4 bytes: data length (u32 little-endian)
    // - N bytes: zlib-compressed IDL JSON

    let compressed_data = &account.data[44..];

    let mut decoder = ZlibDecoder::new(compressed_data);
    let mut decompressed = String::new();
    decoder
        .read_to_string(&mut decompressed)
        .map_err(|e| anyhow::anyhow!("Failed to decompress IDL: {}", e))?;

    parse_idl(&decompressed)
}

pub fn fetch_idl_with_url(rpc_url: &str, program_id: &Pubkey) -> anyhow::Result<Idl> {
    let client = RpcClient::new(rpc_url.to_string());
    fetch_idl(&client, program_id)
}
