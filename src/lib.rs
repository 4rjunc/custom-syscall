use solana_program::{
    account_info::AccountInfo, compute_units::sol_remaining_compute_units, declare_id, entrypoint,
    entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey,
};

#[cfg(test)]
mod tests;

declare_id!("HnMfkc4LRNnw3kXEg6Fg9ezSMucWL3GseZDaBLqr8R5E");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::ID) {
        return Err(ProgramError::IncorrectProgramId);
    }
    msg!("Hello");
    Ok(())
}
