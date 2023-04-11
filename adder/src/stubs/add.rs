use crate::state::*;
use solana_program::{entrypoint::ProgramResult, msg};

pub fn add(number: u8, adder_data: &mut Adder) -> ProgramResult {
    // Place your custom code here...
    msg!("Number: {}", number);
    adder_data.number += number;
    Ok(())
}
