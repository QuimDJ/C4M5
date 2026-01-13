use rand::{Rng, rng};

fn main(){
    let mut n=rng();
    let m=n.random::<f64>();
    println!("m={:?}", m);
    //let h=(1..=5).map(|_| n.random::<u8>());
    let mut v=vec![];
    for x in (1..=5).map(|_| n.random::<u8>()) {
        v.push(x);
    }
    println!("v={v:?}");
    for elem in v.iter() { println!("{elem}"); }
    

}