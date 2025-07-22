use solana_program::{
    account_info::AccountInfo, compute_units::sol_remaining_compute_units, declare_id, entrypoint,
    entrypoint::ProgramResult, magic_number::sol_get_magic_number, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

#[cfg(test)]
mod tests;

declare_id!("r8p3kwsDxPyTu1KyacFxJcP5b98GRn9wocBUsTToWTd");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::ID) {
        return Err(ProgramError::IncorrectProgramId);
    }
    msg!(
        "sol_remaining_compute_units {:?}",
        sol_remaining_compute_units()
    );
    msg!("sol_get_magic_number {:?}", sol_get_magic_number());
    Ok(())
}
