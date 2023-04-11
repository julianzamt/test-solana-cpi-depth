use crate::*;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum Program1Instruction {
    /// Accounts expected:
    ///
    /// 0. `[]` program_2_id
    /// 1. `[]` program_3_id
    /// 2. `[]` counter_program_id
    /// 3. `[writable]` counter
    /// 4. `[writable]` signer
    /// 5. `[]` system_program
    CallProgram2 {},
}

impl Program1Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpacking instruction...");

        let (tag, _) = input
            .split_first()
            .ok_or(errors::Program1Error::InvalidInstruction)?;

        Ok(match tag {
            0 => Self::CallProgram2 {},

            _ => return Err(errors::Program1Error::InvalidInstruction.into()),
        })
    }
}
