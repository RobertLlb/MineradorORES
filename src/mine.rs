use std::sync::Arc;

use colored::*;
use drillx::{
    equix::{self},
    Hash, Solution,
};
use ore_api::{
    consts::{BUS_ADDRESSES, BUS_COUNT, EPOCH_DURATION},
    state::{Config, Proof},
};
use rand::Rng;
use solana_program::pubkey::Pubkey;
use solana_rpc_client::spinner;
use solana_sdk::signer::Signer;

use crate::{
    args::MineArgs,
    send_and_confirm::ComputeBudget,
    utils::{amount_u64_to_string, get_clock, get_config, get_proof_with_authority, proof_pubkey},
    Miner,
};

impl Miner {
    pub async fn mine(&self, args: MineArgs) {
        // Register, if needed.
        let signer = self.signer();
        self.open().await;

        // Check num threads
        self.check_num_cores(args.threads);

        // Start mining loop
        let mut global_best_difficulty = 0;
        loop {
            // Fetch proof
            let config = get_config(&self.rpc_client).await;
            let proof = get_proof_with_authority(&self.rpc_client, signer.pubkey()).await;
            println!(
                "\nStake: {} ORE\n  Multiplier: {:12}x",
                amount_u64_to_string(proof.balance),
                calculate_multiplier(proof.balance, config.top_balance)
            );

            // Run drillx
            let (solution, best_difficulty) = Self::find_hash_par(
                proof,
                args.threads,
                16,  // Set the minimum difficulty to 20
            )
            .await;

            // Update global best difficulty if the current best difficulty is greater
            if best_difficulty > global_best_difficulty {
                global_best_difficulty = best_difficulty;
            }

            // Verifica se a dificuldade encontrada é menor que 20 e continua se for o caso
            if global_best_difficulty < 16 {
                println!("Found difficulty below 20, retrying...");
                continue;
            }

            // Submit most difficult hash
            let mut compute_budget = 500_000;
            let mut ixs = vec![ore_api::instruction::auth(proof_pubkey(signer.pubkey()))];
            if self.should_reset(config).await && rand::thread_rng().gen_range(0..100).eq(&0) {
                compute_budget += 100_000;
                ixs.push(ore_api::instruction::reset(signer.pubkey()));
            }
            ixs.push(ore_api::instruction::mine(
                signer.pubkey(),
                signer.pubkey(),
                find_bus(),
                solution,
            ));
            self.send_and_confirm(&ixs, ComputeBudget::Fixed(compute_budget), false)
                .await
                .ok();
        }
    }

    async fn find_hash_par(
        proof: Proof,
        threads: u64,
        min_difficulty: u32,
    ) -> (Solution, u32) {
        // Dispatch job to each thread
        let progress_bar = Arc::new(spinner::new_progress_bar());
        
        let handles: Vec<_> = (0..threads)
            .map(|i| {
                std::thread::spawn({
                    let proof = proof.clone();
                    let progress_bar = progress_bar.clone();
                    let mut memory = equix::SolverMemory::new();
                    move || {
                        let mut nonce = u64::MAX.saturating_div(threads).saturating_mul(i);
                        let mut local_best_nonce = nonce;
                        let mut local_best_difficulty = 0;
                        let mut local_best_hash = Hash::default();
                        loop {
                            progress_bar.set_message(format!(
                                "Mining... (Best Difficulty Found: {} ) (Minimum Difficulty: {} )",
                                local_best_difficulty, min_difficulty
                            ));
                            
                            // Create hash
                            if let Ok(hx) = drillx::hash_with_memory(
                                &mut memory,
                                &proof.challenge,
                                &nonce.to_le_bytes(),
                            ) {
                                let difficulty = hx.difficulty();
                                if difficulty > local_best_difficulty {
                                    local_best_nonce = nonce;
                                    local_best_difficulty = difficulty;
                                    local_best_hash = hx;
                                }
                            }
                            if local_best_difficulty >= min_difficulty {
                                // Mine until min difficulty has been met
                                break;
                            }
                  
                            // Increment nonce
                            nonce += 1;
                        }

                        // Return the best nonce
                        (local_best_nonce, local_best_difficulty, local_best_hash)
                    }
                })
            })
            .collect();

        // Join handles and return best nonce
        let mut best_nonce = 0;
        let mut best_difficulty = 0;
        let mut best_hash = Hash::default();
        for h in handles {
            if let Ok((nonce, difficulty, hash)) = h.join() {
                if difficulty > best_difficulty {
                    best_difficulty = difficulty;
                    best_nonce = nonce;
                    best_hash = hash;
                }
            }
        }

        // Update log
        progress_bar.finish_with_message(format!(
            "Best hash: {} (difficulty: {})",
            bs58::encode(best_hash.h).into_string(),
            best_difficulty
        ));

        (Solution::new(best_hash.d, best_nonce.to_le_bytes()), best_difficulty)
    }

    pub fn check_num_cores(&self, threads: u64) {
        // Check num threads
        let num_cores = num_cpus::get() as u64;
        if threads > num_cores {
            println!(
                "{} Number of threads ({}) exceeds available cores ({})",
                "WARNING".bold().yellow(),
                threads,
                num_cores
            );
        }
    }

    async fn should_reset(&self, config: Config) -> bool {
        let clock = get_clock(&self.rpc_client).await;
        config
            .last_reset_at
            .saturating_add(EPOCH_DURATION)
            .saturating_sub(5) // Buffer
            .le(&clock.unix_timestamp)
    }
}

fn calculate_multiplier(balance: u64, top_balance: u64) -> f64 {
    1.0 + (balance as f64 / top_balance as f64).min(1.0f64)
}

// TODO Pick a better strategy (avoid draining bus)
fn find_bus() -> Pubkey {
    let i = rand::thread_rng().gen_range(0..BUS_COUNT);
    BUS_ADDRESSES[i]
}
