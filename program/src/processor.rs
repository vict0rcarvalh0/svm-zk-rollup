use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::proof::verify_proof;
use crate::state::StateAccount;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    proof_data: &[u8],
) -> ProgramResult {
    let account = accounts.first().ok_or(ProgramError::InvalidArgument)?;
    
    // Verify ZK proof
    if !verify_proof(proof_data) {
        msg!("Prova inválida");
        return Err(ProgramError::InvalidInstructionData);
    }

    // Update state in account
    let mut state_account = StateAccount::load(account)?;
    state_account.state = proof_data.to_vec();
    state_account.save(account)?;

    Ok(())
}
