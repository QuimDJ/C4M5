use rand::{Rng, SeedableRng, random_range, rng};
use rand::rngs::StdRng;
fn main(){

    let n=(0..25).map(|_| random_range(0..=10)).collect::<Vec<u8>>();
    println!("{:?}",n);
    println!(" ");
    let mut mi_rng=rng();
    let n=(0..25).map(|_| mi_rng.random_range(0..=10)).collect::<Vec<u8>>();
    println!("{:?}",n);

    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    let n=(0..25).map(|_| rng.random_range(0..=10)).collect::<Vec<u64>>();
    println!("{:?}",n);

    
}