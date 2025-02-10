mod prover;
mod state;

use prover::generate_proof;
use state::MerkleTreeState;

fn main() {
    let mut rollup_state = MerkleTreeState::new();

    let txs = vec![
        "farmer_1 sells 10 tokens",
        "investor_2 stakes 100 USDC",
    ];
    
    // Update state and generate ZK proof
    let new_root = rollup_state.update_state(&txs);
    let proof = generate_proof(&txs);

    println!("Nova raiz Merkle: {:?}", new_root);
    println!("Prova gerada: {:?}", proof);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
