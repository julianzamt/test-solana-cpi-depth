use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg};

use crate::stubs::cpi_counter;

pub fn call_counter<'a>(
    counter_account_info: &AccountInfo<'a>,
    counter_program_account_info: &AccountInfo<'a>,
    signer_account_info: &AccountInfo<'a>,
    system_account_info: &AccountInfo<'a>,
) -> ProgramResult {
    // Place your custom code here...
    msg!("call_counter stub in program 1");

    cpi_counter::cpi_counter(
        counter_account_info,
        counter_program_account_info,
        signer_account_info,
        system_account_info,
    );

    Ok(())
}
