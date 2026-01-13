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

    let v1=(1..=5).map(|_| rng().random::<u8>()).collect::<Vec<u8>>();
    println!("v1={v1:?}");

    let random_number=rng().random_range(1..=10);
    println!("{random_number:?}");

    let mut ttff=vec![];
    for _ in 1..=20 {  ttff.push(rng().random::<u8>()); }
    println!("{ttff:?}");

}