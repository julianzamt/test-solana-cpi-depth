use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
};

use crate::rust_utils::*;

pub fn cpi_program_3<'a>(
    program_3_account_info: &AccountInfo<'a>,
    counter_program_account_info: &AccountInfo<'a>,
    counter_account_info: &AccountInfo<'a>,
    signer_account_info: &AccountInfo<'a>,
    system_account_info: &AccountInfo<'a>,
) -> ProgramResult {
    msg!("cpi to program_3 in program_2...");

    let buf: &mut [u8] = &mut [0; 1];

    // pack Ix Number
    pack_u8(buf, 0);

    let b_ix = Instruction::new_with_bytes(
        *program_3_account_info.key,
        buf,
        [
            AccountMeta::new_readonly(*counter_program_account_info.key, false),
            AccountMeta::new(*counter_account_info.key, false),
            AccountMeta::new(*signer_account_info.key, true),
            AccountMeta::new_readonly(*system_account_info.key, false),
        ]
        .to_vec(),
    );
    invoke(
        &b_ix,
        &[
            program_3_account_info.clone(),
            counter_program_account_info.clone(),
            counter_account_info.clone(),
            signer_account_info.clone(),
            system_account_info.clone(),
        ],
    )?;
    Ok(())
}
