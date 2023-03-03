use crate::*;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum AdderInstruction {
    /// Accounts expected:
    ///
    /// 0. `[writable]` accumulator
    /// 1. `[]` signer
    /// 2. `[]` system_program
    Add {},
}

impl AdderInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpacking instruction...");

        let (tag, _) = input
            .split_first()
            .ok_or(errors::AdderError::InvalidInstruction)?;

        Ok(match tag {
            0 => {
                Self::Add {}
            }

            _ => return Err(errors::AdderError::InvalidInstruction.into()),
        })
    }
}
