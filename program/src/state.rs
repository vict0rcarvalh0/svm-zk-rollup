use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StateAccount {
    pub state: Vec<u8>,
}

impl StateAccount {
    pub fn load(account: &AccountInfo) -> Result<Self, ProgramError> {
        let data = &account.data.borrow();
        StateAccount::try_from_slice(data).map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn save(&self, account: &AccountInfo) -> ProgramResult {
        let mut data = account.data.borrow_mut();
        self.serialize(&mut &mut data[..])?;
        Ok(())
    }
}
