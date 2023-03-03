use crate::*;
use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};

// Stores a number increased by the number sent in say_bye_and_add(number).
#[derive(Debug, Default)]
pub struct Accumulator {
    // The number increased by the say_bye_and_add method.
    pub number: u32,
}

impl Sealed for Accumulator {}

impl Pack for Accumulator {
    const LEN: usize = 4;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let number: u32;

        // Deserialize
        let new_src: &[u8] = src;
        (_, number) = rust_utils::unpack_u32(new_src);

        Ok(Accumulator { number })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let new_dst: &mut [u8] = dst;

        // Destructure self
        let Accumulator { number } = self;

        // Serialize each field
        rust_utils::pack_u32(new_dst, *number);
    }
}
