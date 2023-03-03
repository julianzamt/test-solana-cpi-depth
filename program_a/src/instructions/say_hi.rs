use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey, program::invoke, instruction::{Instruction, AccountMeta}
};

use crate::instructions::{program_b_cpis::*, rust_utils::*};


entrypoint!(say_hi);

fn say_hi(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hi! ");

    // let accounts_iter = &mut accounts.iter();
    // msg!("Accounts iter: {:?}", accounts_iter);
    // let _program_b_account_info = next_account_info(accounts_iter)?;
    // let accumulator_account_info = next_account_info(accounts_iter)?;
    // let signer_account_info = next_account_info(accounts_iter)?;
    // let system

    cpi_add(accounts)
}

