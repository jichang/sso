use std::io::Error as IoError;
use rand::{Rng, StdRng};

pub fn gen_rand_bytes(len: usize) -> Result<Vec<u8>, IoError> {
    let mut rng = StdRng::new()?;
    let random_vector: Vec<u8> = rng.gen_iter::<u8>().take(len).collect();

    Ok(random_vector)
}
