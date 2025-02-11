mod models;
mod executor;

use executor::{execute_transaction, load_state};
use models::transaction::Transaction;
use std::io;

fn main() {
    load_state();

    loop {
        println!("Digite uma transação (formato: sender receiver amount) ou 'exit' para sair:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("Formato inválido. Use: sender receiver amount");
            continue;
        }

        let sender = parts[0];
        let receiver = parts[1];
        let amount: u64 = match parts[2].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Valor inválido!");
                continue;
            }
        };

        let tx = Transaction::new(sender, receiver, amount);
        if execute_transaction(tx) {
            println!("✅ Transação executada com sucesso!");
        } else {
            println!("❌ Falha na transação (saldo insuficiente ou erro).");
        }
    }
}
