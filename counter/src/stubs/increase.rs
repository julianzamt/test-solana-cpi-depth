use crate::state::*;
use solana_program::{entrypoint::ProgramResult, msg};

pub fn increase(counter_data: &mut Counter) -> ProgramResult {
    // Place your custom code here...
    msg!("Increase() in Counter...");
    counter_data.number += 1;
    Ok(())
}
