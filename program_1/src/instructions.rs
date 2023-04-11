use crate::*;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum Program1Instruction {
    /// Accounts expected:
    ///
    /// 0. `[writable]` counter
    /// 1. `[]` signer
    /// 2. `[]` system_program
    CallCounter {},
}

impl Program1Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpacking instruction...");

        let (tag, rest) = input
            .split_first()
            .ok_or(errors::Program1Error::InvalidInstruction)?;

        Ok(match tag {
            0 => {
                Self::CallCounter { }
            }

            _ => return Err(errors::Program1Error::InvalidInstruction.into()),
        })
    }
}
