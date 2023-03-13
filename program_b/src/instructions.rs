use crate::*;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum CounterInstruction {
    /// Accounts expected:
    ///
    /// 0. `[writable]` counter
    /// 1. `[]` signer
    /// 2. `[]` system_program
    Increase {},
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpacking instruction...");

        let (tag, _) = input
            .split_first()
            .ok_or(errors::AdderError::InvalidInstruction)?;

        Ok(match tag {
            0 => Self::Increase {},

            _ => return Err(errors::AdderError::InvalidInstruction.into()),
        })
    }
}
