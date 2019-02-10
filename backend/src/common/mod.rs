use rand::distributions::Standard;
use rand::prelude::StdRng;
use rand::FromEntropy;
use rand::Rng;
use std::io::Error as IoError;

pub fn gen_rand_bytes(len: usize) -> Result<Vec<u8>, IoError> {
    let mut rng = StdRng::from_entropy();
    let random_vector: Vec<u8> = rng.sample_iter(&Standard).take(len).collect();

    Ok(random_vector)
}
