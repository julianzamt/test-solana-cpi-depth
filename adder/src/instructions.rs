use crate::*;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum AdderInstruction {
    /// Accounts expected:
    ///
    /// 0. `[writable]` adder
    /// 1. `[]` signer
    /// 2. `[]` system_program
    Add { number: u8 },
    /// Accounts expected:
    ///
    /// 0. `[]` adder
    LogAdder {},
}

impl AdderInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpacking instruction...");

        let (tag, rest) = input
            .split_first()
            .ok_or(errors::AdderError::InvalidInstruction)?;

        Ok(match tag {
            0 => {
                let (_, number) = rust_utils::unpack_u8(rest);

                Self::Add { number }
            }

            1 => Self::LogAdder {},

            _ => return Err(errors::AdderError::InvalidInstruction.into()),
        })
    }
}
