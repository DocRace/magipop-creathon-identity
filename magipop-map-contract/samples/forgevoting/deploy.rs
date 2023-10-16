// deploy.rs

use linear_sdk::{
    account_info::AccountInfo,
    entrypoint, 
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program::encode_initialize_mint_proof
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]  
) -> ProgramResult {

    let token_mint_proof = encode_initialize_mint_proof(
        &accounts[0].key, 
        &accounts[1].key,
        1000000
    )?;

    let (mint_proof_accounts, mint_proof_data) = token_mint_proof.deconstruct();

    process_instruction(
        program_id,
        &mint_proof_accounts,
        &mint_proof_data, 
    )?;

    msg!("Token and Votes accounts initialized!");

    Ok(())
}