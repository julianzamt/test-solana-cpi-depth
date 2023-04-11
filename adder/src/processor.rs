use crate::{instructions::*, state::*, stubs::*};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = AdderInstruction::unpack(instruction_data)?;

        match instruction {
            AdderInstruction::Add {
                number,
            } => {
                Self::process_add(number, accounts, program_id)?;
            }
            AdderInstruction::LogAdder {} => {
                Self::process_log_adder(accounts, program_id)?;
            }
        }

        Ok(())
    }

    pub fn process_add(number: u8, accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        msg!("process_add ix...");
        let account_info_iter = &mut accounts.iter();

        let adder = next_account_info(account_info_iter)?;
        let signer = next_account_info(account_info_iter)?;
        let _system_program = next_account_info(account_info_iter)?;

        if adder.lamports() == 0 && *adder.owner == solana_program::system_program::id() {
            let rent = Rent::get()?;
            let (_, bump) = Pubkey::find_program_address(&[b"adder"], program_id);
            let space = Adder::LEN;
            let rent_minimum_balance = rent.minimum_balance(space);

            invoke_signed(
                &create_account(
                    &signer.key,
                    &adder.key,
                    rent_minimum_balance,
                    space as u64,
                    program_id,
                ),
                &[signer.clone(), adder.clone()],
                &[&[b"adder".as_ref(), &[bump]]],
            )?;
        }

        let mut adder_data = Adder::unpack_unchecked(&adder.try_borrow_data()?)?;

        add::add(number, &mut adder_data)?;

        Adder::pack(adder_data, &mut adder.try_borrow_mut_data()?)?;

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
