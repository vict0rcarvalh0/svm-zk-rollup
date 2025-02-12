// src/zk_prover/zk_prover.rs

// Certifique-se de que no Cargo.toml você tenha:
// bellman = "0.9"
// pairing = "0.20"
// rand = "0.8"

use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bellman::groth16;
use bellman::gadgets::num;
use pairing::bls12_381::{Bls12, Fr};
use rand::thread_rng;

/// Estrutura do circuito para validar uma transação.
/// 
/// **Entradas privadas:**  
/// - `sender_balance`: saldo atual do remetente  
/// - `amount`: valor da transação  
/// - `receiver_balance`: saldo atual do destinatário
///
/// **Entradas públicas:**  
/// - `new_sender_balance`: saldo do remetente após a transação  
/// - `new_receiver_balance`: saldo do destinatário após a transação
#[derive(Clone)]
pub struct TransactionCircuit {
    pub sender_balance: Option<Fr>,
    pub amount: Option<Fr>,
    pub receiver_balance: Option<Fr>,
    pub new_sender_balance: Option<Fr>,
    pub new_receiver_balance: Option<Fr>,
}

impl Circuit<Bls12> for TransactionCircuit {
    fn synthesize<CS: ConstraintSystem<Bls12>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Aloca as variáveis privadas.
        let sender_balance_var = num::AllocatedNum::alloc(cs.namespace(|| "sender_balance"), || {
            self.sender_balance.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let amount_var = num::AllocatedNum::alloc(cs.namespace(|| "amount"), || {
            self.amount.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let receiver_balance_var = num::AllocatedNum::alloc(cs.namespace(|| "receiver_balance"), || {
            self.receiver_balance.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Aloca as variáveis públicas.
        let new_sender_balance_var = num::AllocatedNum::alloc_input(cs.namespace(|| "new_sender_balance"), || {
            self.new_sender_balance.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let new_receiver_balance_var = num::AllocatedNum::alloc_input(cs.namespace(|| "new_receiver_balance"), || {
            self.new_receiver_balance.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Restrição 1: sender_balance - amount = new_sender_balance
        cs.enforce(
            || "sender_balance - amount = new_sender_balance",
            |lc| lc + sender_balance_var.get_variable() - amount_var.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + new_sender_balance_var.get_variable(),
        );
        
        // Restrição 2: receiver_balance + amount = new_receiver_balance
        cs.enforce(
            || "receiver_balance + amount = new_receiver_balance",
            |lc| lc + receiver_balance_var.get_variable() + amount_var.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + new_receiver_balance_var.get_variable(),
        );
        
        Ok(())
    }
}

/// Configura os parâmetros do zk-SNARK.
/// Essa função gera os parâmetros aleatórios necessários para criar provas.
pub fn setup_zk() -> (groth16::Parameters<Bls12>, groth16::VerifyingKey<Bls12>) {
    let rng = &mut thread_rng();
    let params = groth16::generate_random_parameters(
        TransactionCircuit {
            sender_balance: None,
            amount: None,
            receiver_balance: None,
            new_sender_balance: None,
            new_receiver_balance: None,
        },
        rng,
    ).unwrap();
    let vk = groth16::prepare_verifying_key(&params.vk);
    (params, vk)
}

/// Gera uma prova para uma transação.
/// 
/// - `sender_balance`: saldo atual do remetente  
/// - `amount`: valor da transação  
/// - `receiver_balance`: saldo atual do destinatário  
/// 
/// A função calcula os novos saldos e cria o circuito com esses valores.
pub fn generate_proof(
    params: &groth16::Parameters<Bls12>,
    sender_balance: u64,
    amount: u64,
    receiver_balance: u64,
) -> groth16::Proof<Bls12> {
    // Verifica (fora do circuito) se há saldo suficiente.
    let new_sender_balance = sender_balance.checked_sub(amount)
        .expect("Saldo insuficiente para a transação");
    let new_receiver_balance = receiver_balance.checked_add(amount)
        .expect("Erro ao adicionar saldo ao destinatário");
    
    let circuit = TransactionCircuit {
        sender_balance: Some(Fr::from(sender_balance)),
        amount: Some(Fr::from(amount)),
        receiver_balance: Some(Fr::from(receiver_balance)),
        new_sender_balance: Some(Fr::from(new_sender_balance)),
        new_receiver_balance: Some(Fr::from(new_receiver_balance)),
    };
    
    let rng = &mut thread_rng();
    groth16::create_random_proof(circuit, params, rng).unwrap()
}

/// Verifica uma prova dada os saldos e o valor da transação.
/// A função calcula os novos saldos e usa-os como entradas públicas para a verificação.
pub fn verify_proof(
    vk: &groth16::VerifyingKey<Bls12>,
    proof: &groth16::Proof<Bls12>,
    sender_balance: u64,
    amount: u64,
    receiver_balance: u64,
) -> bool {
    let new_sender_balance = sender_balance.checked_sub(amount)
        .expect("Saldo insuficiente para a transação");
    let new_receiver_balance = receiver_balance.checked_add(amount)
        .expect("Erro ao adicionar saldo ao destinatário");
    
    // As entradas públicas (inputs) devem ser os novos saldos.
    let public_inputs = vec![
        Fr::from(new_sender_balance),
        Fr::from(new_receiver_balance),
    ];
    
    match groth16::verify_proof(vk, proof, &public_inputs) {
        Ok(()) => true,  // Se a verificação passar, Ok retorna unit, e consideramos isso como true.
        Err(_) => false, // Se ocorrer um erro, retorna false.
    }
}
