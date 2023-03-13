use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    pubkey,
    pubkey::Pubkey,
};

use crate::instructions::rust_utils::*;

pub fn cpi_add(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("cpi_add execution...");

    let accounts_iter = &mut accounts.iter();
    // msg!("Accounts iter: {:?}", accounts_iter);
    let _program_b_account_info = next_account_info(accounts_iter)?;
    let accumulator_account_info = next_account_info(accounts_iter)?;
    let signer_account_info = next_account_info(accounts_iter)?;
    let system_account_info = next_account_info(accounts_iter)?;

    let buf: &mut [u8] = &mut [0; 4];
    pack_u32(buf, 0);

    // Completar el chequeo
    let (accumulator_pubkey, _) = Pubkey::find_program_address(
        &[b"counter"],
        &pubkey!("7cKiYqQhh12atTVhsZqKy4E12bN3kFbDdHqPo4FSuYwe"),
    );
    // if accumulator_pubkey != *accumulator_account_info.key {
    //     return Err(ProgramNameError::InvalidPDA);
    // }

    let b_ix = Instruction::new_with_bytes(
        pubkey!("7cKiYqQhh12atTVhsZqKy4E12bN3kFbDdHqPo4FSuYwe"),
        buf,
        [
            AccountMeta::new(accumulator_pubkey, false),
            AccountMeta::new(*signer_account_info.key, true),
            AccountMeta::new_readonly(*system_account_info.key, false)
        ]
        .to_vec(),
    );
    invoke(&b_ix, accounts)?;
    Ok(())
}
