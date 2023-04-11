use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instructions::{add_cpis::*, rust_utils};

entrypoint!(call_add);

fn call_add(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Executing call_add in program 3...");

    let accounts_iter = &mut accounts.iter();
    let add_program_account_info = next_account_info(accounts_iter)?;
    let adder_account_info = next_account_info(accounts_iter)?;
    let signer_account_info = next_account_info(accounts_iter)?;
    let system_account_info = next_account_info(accounts_iter)?;

    let (_, number) = rust_utils::unpack_u8(instruction_data);

    cpi_add(
        number,
        add_program_account_info,
        adder_account_info,
        signer_account_info,
        system_account_info,
    )
}
