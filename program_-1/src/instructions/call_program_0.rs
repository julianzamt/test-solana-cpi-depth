use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instructions::{program_0_cpis::*, rust_utils};

entrypoint!(call_program_0);

fn call_program_0(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Executing call_program_0 from program_1...");

    let accounts_iter = &mut accounts.iter();
    let program_0_account_info = next_account_info(accounts_iter)?;
    let program_1_account_info = next_account_info(accounts_iter)?;
    let program_2_account_info = next_account_info(accounts_iter)?;
    let program_3_account_info = next_account_info(accounts_iter)?;
    let add_program_account_info = next_account_info(accounts_iter)?;
    let adder_account_info = next_account_info(accounts_iter)?;
    let signer_account_info = next_account_info(accounts_iter)?;
    let system_account_info = next_account_info(accounts_iter)?;

    let (_, number) = rust_utils::unpack_u8(instruction_data);

    cpi_program_0(
        number,
        program_0_account_info,
        program_1_account_info,
        program_2_account_info,
        program_3_account_info,
        add_program_account_info,
        adder_account_info,
        signer_account_info,
        system_account_info,
    )
}
