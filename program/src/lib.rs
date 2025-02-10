use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::processor::process_instruction;

mod processor;
mod proof;
mod state;

entrypoint!(process_instruction);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy() {
        assert_eq!(1 + 1, 2);
    }
}
