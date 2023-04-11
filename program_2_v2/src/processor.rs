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
        let instruction = Program2Instruction::unpack(instruction_data)?;

        match instruction {
            Program2Instruction::CallProgram3 {} => {
                Self::process_call_program_3(accounts)?;
            }
        }

        Ok(())
    }

    pub fn process_call_program_3(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("process_call_program_3 in program_2...");

        let account_info_iter = &mut accounts.iter();

        let program_3 = next_account_info(account_info_iter)?;
        let counter_program = next_account_info(account_info_iter)?;
        let counter = next_account_info(account_info_iter)?;
        let signer = next_account_info(account_info_iter)?;
        let _system_program = next_account_info(account_info_iter)?;

        call_program_3::call_program_3(
            program_3,
            counter_program,
            counter,
            signer,
            _system_program,
        )?;

        Ok(())
    }
}
