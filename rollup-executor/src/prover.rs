use sp1::Prover;

pub fn generate_proof(txs: &[&str]) -> Vec<u8> {
    let prover = Prover::new();
    prover.generate_proof(txs)
}
