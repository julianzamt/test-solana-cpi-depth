use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum Program3Error {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Already Initialized")]
    AlreadyInitialized,

    #[error("Not The Expected Account Address")]
    NotExpectedAddress,

    #[error("Invalid String")]
    InvalidString,
}

impl From<Program3Error> for ProgramError {
    fn from(e: Program3Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
