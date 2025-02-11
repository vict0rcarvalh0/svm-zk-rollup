use crate::models::{state::State, transaction::Transaction};
// use crate::transaction::Transaction;
use serde_json;
use std::fs;
use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;

lazy_static! {
    static ref STATE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));
}

pub fn execute_transaction(tx: Transaction) -> bool {
    let mut state = STATE.lock().unwrap();
    let success = state.apply_transaction(&tx.sender, &tx.receiver, tx.amount);

    if success {
        save_state(&state);
    }

    success
}

pub fn save_state(state: &State) {
    let state_json = serde_json::to_string_pretty(state).unwrap();
    fs::write("state.json", state_json).expect("Failed to save state");
}

pub fn load_state() {
    let data = fs::read_to_string("state.json").unwrap_or_else(|_| "{}".to_string());
    let loaded_state: State = serde_json::from_str(&data).unwrap_or_else(|_| State::new());

    let mut state = STATE.lock().unwrap();
    *state = loaded_state;
}
