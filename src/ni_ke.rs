use crate::config::simulation;
use crate::multilinear::MultiLinearGroup;
use ark_ff::PrimeField;
use std::sync::Arc;

pub struct NIKEProtocol<F: PrimeField> {
    pub group: Arc<MultiLinearGroup<F>>,
}

impl<F: PrimeField> NIKEProtocol<F> {
    pub fn new(group: Arc<MultiLinearGroup<F>>) -> Self {
        Self { group }
    }

    pub async fn run_simulation(&self, num_participants: usize) -> Result<bool, Box<dyn std::error::Error>> {
        println!(
            "Starting simulation for {} participants...",
            num_participants
        );

        let mut secrets = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..num_participants {
            secrets.push(self.group.ring.sample_error(&mut rng, simulation::ERROR_STD_DEV)?);
        }

        let mut public_keys = Vec::new();
        for i in 0..num_participants {
            public_keys.push(self.group.encode(&secrets[i], 1)?);
        }

        let shared_encoding = self.group.pair(&public_keys)?;
        let shared_secret = self.group.extract(&shared_encoding)?;

        println!(
            "Shared secret derived (first 16 bytes hex): {}",
            hex::encode(&shared_secret[..16.min(shared_secret.len())])
        );

        Ok(true)
    }
}
