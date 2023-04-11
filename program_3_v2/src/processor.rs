use crate::{instructions::*, stubs::*};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = Program3Instruction::unpack(instruction_data)?;

        match instruction {
            Program3Instruction::CallCounter {} => {
                Self::process_call_counter(accounts)?;
            }
        }

        Ok(())
    }

    pub fn process_call_counter(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("process_call_counter in program1...");

        let account_info_iter = &mut accounts.iter();

        let counter_program = next_account_info(account_info_iter)?;
        let counter = next_account_info(account_info_iter)?;
        let signer = next_account_info(account_info_iter)?;
        let _system_program = next_account_info(account_info_iter)?;

        call_counter::call_counter(counter_program, counter, signer, _system_program)?;

        Ok(())
    }
}
