use anyhow::Result;
use rand::Rng;
mod error;
mod merge;
mod thread_pool;
use std::cmp::Ordering;

/// Generate a random vector of size capacity filled with random i64s
fn random_vec(capacity: usize) -> Vec<i64> {
    let mut vec = vec![0; capacity];
    rand::thread_rng().fill(&mut vec[..]);
    vec
}

fn main() -> Result<()> {
    let data: Vec<i64> = random_vec(1_000_000);
    //let data: Vec<i64> = random_vec(1_000);

    merge::merge_sort(data, 50);
    Ok(())
}
