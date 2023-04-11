use crate::{instructions::*, state::*, stubs::*};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_pack::Pack,
    pubkey::Pubkey,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = Program1Instruction::unpack(instruction_data)?;

        match instruction {
            Program1Instruction::CallCounter {} => {
                Self::process_call_counter(accounts, program_id)?;
            }
        }

        Ok(())
    }

    pub fn process_call_counter(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        msg!("process_call_counter in program1...");

        let account_info_iter = &mut accounts.iter();

        let counter = next_account_info(account_info_iter)?;
        let counter_program = next_account_info(account_info_iter)?;
        let signer = next_account_info(account_info_iter)?;
        let _system_program = next_account_info(account_info_iter)?;

        call_counter::call_counter(counter, counter_program, signer, _system_program)?;

        Ok(())
    }

    pub fn process_log_adder(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        msg!("process_log_adder ix...");
        let account_info_iter = &mut accounts.iter();

        let adder = next_account_info(account_info_iter)?;

        let adder_data = Adder::unpack_unchecked(&adder.try_borrow_data()?)?;

        msg!("Adder number: {}", adder_data.number);

        Adder::pack(adder_data, &mut adder.try_borrow_mut_data()?)?;

        Ok(())
    }
}
