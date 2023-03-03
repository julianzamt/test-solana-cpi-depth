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
            AdderInstruction::Add {} => {
                Self::process_add(accounts, program_id)?;
            }
        }

        Ok(())
    }

    pub fn process_add(
        accounts: &[AccountInfo],
        program_id: &Pubkey
    ) -> ProgramResult {
        msg!("process_add ix...");
        let account_info_iter = &mut accounts.iter();

        let accumulator = next_account_info(account_info_iter)?;
        let signer = next_account_info(account_info_iter)?;
        let _system_program = next_account_info(account_info_iter)?;

        if accumulator.lamports() == 0 && *accumulator.owner == solana_program::system_program::id()
        {
            let rent = Rent::get()?;
            let (_, bump) = Pubkey::find_program_address(&[b"accumulator"], program_id);
            let space = Accumulator::LEN;
            let rent_minimum_balance = rent.minimum_balance(space);

            invoke_signed(
                &create_account(
                    &signer.key,
                    &accumulator.key,
                    rent_minimum_balance,
                    space as u64,
                    program_id,
                ),
                &[signer.clone(), accumulator.clone()],
                &[&[b"accumulator".as_ref(), &[bump]]],
            )?;
        }

        let mut accumulator_data = Accumulator::unpack_unchecked(&accumulator.try_borrow_data()?)?;

        add::add(&mut accumulator_data)?;

        Accumulator::pack(accumulator_data, &mut accumulator.try_borrow_mut_data()?)?;

        Ok(())
    }
}
