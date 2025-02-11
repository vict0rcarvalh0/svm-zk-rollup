use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::bls12_381::{Bls12, Fr};
use bellman::gadgets::{boolean, num};
use bellman::groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof};
use rand::thread_rng;
use bls12_381::Bls12;

#[derive(Clone)]
struct TransactionCircuit {
    sender_balance: Option<Fr>,
    amount: Option<Fr>,
    receiver_balance: Option<Fr>,
}

impl Circuit<Bls12> for TransactionCircuit {
    fn synthesize<CS: ConstraintSystem<Bls12>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let sender_balance = num::AllocatedNum::alloc(cs.namespace(|| "sender_balance"), || {
            self.sender_balance.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        let amount = num::AllocatedNum::alloc(cs.namespace(|| "amount"), || {
            self.amount.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let receiver_balance = num::AllocatedNum::alloc(cs.namespace(|| "receiver_balance"), || {
            self.receiver_balance.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let sufficient_funds = boolean::Boolean::from(
            sender_balance.get_value().map(|b| b >= amount.get_value().unwrap_or(Fr::zero()))
        );

        cs.enforce(
            || "sender_balance - amount = new_sender_balance",
            |lc| lc + sender_balance.get_variable(),
            |lc| lc + sufficient_funds.lc(CS::one(), Fr::one()),
            |lc| lc + amount.get_variable(),
        );

        cs.enforce(
            || "receiver_balance + amount = new_receiver_balance",
            |lc| lc + receiver_balance.get_variable(),
            |lc| lc + amount.get_variable(),
            |lc| lc + receiver_balance.get_variable(),
        );

        Ok(())
    }
}


pub fn setup_zk() -> (groth16::Parameters<Bls12>, groth16::VerifyingKey<Bls12>) {
    let rng = &mut thread_rng();
    let params = generate_random_parameters::<Bls12, _, _>(TransactionCircuit {
        sender_balance: None,
        amount: None,
        receiver_balance: None,
    }, rng).unwrap();

    let vk = prepare_verifying_key(&params.vk);
    (params, vk)
}

pub fn generate_proof(params: &groth16::Parameters<Bls12>, sender_balance: u64, amount: u64, receiver_balance: u64) -> groth16::Proof<Bls12> {
    let rng = &mut thread_rng();
    let proof = create_random_proof(
        TransactionCircuit {
            sender_balance: Some(Fr::from(sender_balance)),
            amount: Some(Fr::from(amount)),
            receiver_balance: Some(Fr::from(receiver_balance)),
        },
        params,
        rng
    ).unwrap();

    proof
}

pub fn verify_proof(vk: &groth16::VerifyingKey<Bls12>, proof: &groth16::Proof<Bls12>) -> bool {
    verify_proof(vk, &proof, &[]).unwrap_or(false)
}
