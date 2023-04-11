use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum Program2Error {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Already Initialized")]
    AlreadyInitialized,

    #[error("Not The Expected Account Address")]
    NotExpectedAddress,

    #[error("Invalid String")]
    InvalidString,
}

impl From<Program2Error> for ProgramError {
    fn from(e: Program2Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
