use sp1::Verifier;

pub fn verify_proof(proof: &[u8]) -> bool {
    let verifier = Verifier::new();
    verifier.verify(proof)
}
