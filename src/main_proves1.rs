use rand::{Rng, random, rng, seq::SliceRandom};

fn main() {
    let mut v=vec![3,5,6];
	let mut my_rng=rng();
    v.shuffle(&mut my_rng);
    println!("{:?}",v);
}