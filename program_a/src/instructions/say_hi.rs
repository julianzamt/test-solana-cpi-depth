use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instructions::program_b_cpis::*;

entrypoint!(say_hi);

fn say_hi(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    msg!("Hi! ");

    let accounts_iter = &mut accounts.iter();
    let _program_b_account_info = next_account_info(accounts_iter)?;
    let counter_account_info = next_account_info(accounts_iter)?;
    let signer_account_info = next_account_info(accounts_iter)?;
    let system_account_info = next_account_info(accounts_iter)?;

    // cpi_add(_program_b_account_info, counter_account_info, signer_account_info, system_account_info)?;
    cpi_log_counter(
        _program_b_account_info,
        counter_account_info,
        signer_account_info,
        system_account_info,
    )
}
