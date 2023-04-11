use crate::*;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum Program2Instruction {
    /// Accounts expected:
    ///
    /// 0. `[]` program_3_id
    /// 1. `[]` counter_program_id
    /// 2. `[writable]` counter
    /// 3. `[writable]` signer
    /// 4. `[]` system_program
    CallProgram3 {},
}

impl Program2Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpacking instruction...");

        let (tag, _) = input
            .split_first()
            .ok_or(errors::Program2Error::InvalidInstruction)?;

        Ok(match tag {
            0 => Self::CallProgram3 {},

            _ => return Err(errors::Program2Error::InvalidInstruction.into()),
        })
    }
}
