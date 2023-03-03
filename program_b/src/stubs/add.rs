use crate::state::*;
use solana_program::entrypoint::ProgramResult;

pub fn add(accumulator_data: &mut Accumulator) -> ProgramResult {
    // Place your custom code here...
    accumulator_data.number += 1;
    Ok(())
}
