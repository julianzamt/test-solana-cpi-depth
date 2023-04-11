use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
};

use crate::instructions::rust_utils::*;

pub fn cpi_program_0<'a>(
    number: u8,
    program_0_account_info: &AccountInfo<'a>,
    program_1_account_info: &AccountInfo<'a>,
    program_2_account_info: &AccountInfo<'a>,
    program_3_account_info: &AccountInfo<'a>,
    add_program_account_info: &AccountInfo<'a>,
    adder_account_info: &AccountInfo<'a>,
    signer_account_info: &AccountInfo<'a>,
    system_account_info: &AccountInfo<'a>,
) -> ProgramResult {
    msg!("cpi to program_0 from program_-1...");

    let buf: &mut [u8] = &mut [0; 1];

    // pack data
    pack_u8(buf, number);

    let b_ix = Instruction::new_with_bytes(
        *program_0_account_info.key,
        buf,
        [
            AccountMeta::new_readonly(*program_1_account_info.key, false),
            AccountMeta::new_readonly(*program_2_account_info.key, false),
            AccountMeta::new_readonly(*program_3_account_info.key, false),
            AccountMeta::new_readonly(*add_program_account_info.key, false),
            AccountMeta::new(*adder_account_info.key, false),
            AccountMeta::new(*signer_account_info.key, true),
            AccountMeta::new_readonly(*system_account_info.key, false),
        ]
        .to_vec(),
    );
    invoke(
        &b_ix,
        &[
            program_0_account_info.clone(),
            program_1_account_info.clone(),
            program_2_account_info.clone(),
            program_3_account_info.clone(),
            add_program_account_info.clone(),
            adder_account_info.clone(),
            signer_account_info.clone(),
            system_account_info.clone(),
        ],
    )?;
    Ok(())
}
