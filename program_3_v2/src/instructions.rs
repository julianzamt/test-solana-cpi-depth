use crate::*;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum Program3Instruction {
    /// Accounts expected:
    ///
    /// 0. `[]` counter_program_id
    /// 1. `[writable]` counter
    /// 2. `[writable]` signer
    /// 3. `[]` system_program
    CallCounter {},
}

impl Program3Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpacking instruction...");

        let (tag, rest) = input
            .split_first()
            .ok_or(errors::Program3Error::InvalidInstruction)?;

        Ok(match tag {
            0 => {
                Self::CallCounter { }
            }

            _ => return Err(errors::Program3Error::InvalidInstruction.into()),
        })
    }
}
