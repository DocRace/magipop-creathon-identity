use linear_sdk::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use linear_sdk::program::{decode_mint_proof, decode_transfer_proof};

#[derive(Debug, PartialEq)] 
struct NFT {
    pub owner: Pubkey,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let command = instruction_data[0];

    match command {
        0 => {
            // Initialize NFT
            let nft_account = &accounts[0]; 
            let nft = NFT {
                owner: *nft_account.key,
            };
            
            let mint_proof = decode_mint_proof(accounts, instruction_data)?;
            mint_proof.verify()?;

            nft_account.set_packed(&nft)?;

            msg!("NFT initialized!");
        }

        1 => {
             // Transfer NFT
             let nft_account = &accounts[0];
             let new_owner_account = &accounts[1];

             let transfer_proof = decode_transfer_proof(accounts, instruction_data)?;
             transfer_proof.verify()?;
             
             let mut nft = nft_account.unpack::<NFT>()?;
             nft.owner = *new_owner_account.key;
             
             nft_account.set_packed(&nft)?;
             
             msg!("NFT transferred!");
        }

        _ => {
            msg!("Invalid instruction");
            return Err(linear_sdk::error::Error::InvalidInstruction.into());
        }
    }

    Ok(())
}