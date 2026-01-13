use rand::{seq::SliceRandom, Rng, rng};

#[derive(Debug, Clone, Copy)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}
impl Suit {
    const ALL:[Suit;4]=[Suit::Spades, Suit::Hearts,Suit::Diamonds,Suit::Clubs];
}

#[derive(Debug, Clone, Copy)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}
impl Rank {
    const ALL:[Rank;13]=[Rank::Two,Rank::Three,Rank::Four,Rank::Five,Rank::Six,Rank::Seven,
    Rank::Eight,Rank::Nine,Rank::Ten,Rank::Jack,Rank::Queen,Rank::King,Rank::Ace];
}
#[derive(Debug, Copy, Clone)]
struct Card {
    rank:Rank,
    suit:Option<Suit>,
}

#[derive(Debug)]
struct Deck {
    cards:Vec<Card>,
}

impl Deck{
    fn new(&mut self){   
        for itemSuit in Suit::ALL.iter(){
            for itemRank in Rank::ALL.iter(){
                self.cards.push(Card{rank:*itemRank, suit:Some(*itemSuit)})
            }
        }
    }
    fn baraja_mazo(&mut self) {
        self.cards.shuffle(&mut rng());
    }
    
    fn insertar_comodines_2(&mut self, joker:Vec<Card>) {
        let my_rng=rng();
        let pos1=(0..2).map(|_| my_rng.random_range(0..52)).collect::<Vec<usize>>();
        println!("{pos1:?}");
        self.cards.insert(pos1[0],joker[0]);
        self.cards.insert(pos1[1], joker[1]);
    }
    fn delete_random(&mut self){
        let mut my_rng=rng();
        //if rng().random_ratio(65,100){
        if my_rng.random_bool(0.65){
            let pos_to_delete:usize=my_rng.random_range(0..52);
            self.cards.remove(pos_to_delete);
        }
    }
}

fn main(){

    // Creamos el mazo
    let mut d1=Deck{cards:vec![]};
    // Lo llenamos de cartas
    d1.new();
    // Comprobamos que estan las 52 cartas
    println!("{:?}",d1);
    // Barajamos el mazo
    d1.baraja_mazo();
    println!("{:?}",d1);
    // insertamos los jokers=> Card{rank:Rank::Joker, suit:None}
    let jokers=vec![Card{rank:Rank::Joker, suit:None},Card{rank:Rank::Joker, suit:None}];
    d1.insertar_comodines_2(jokers);
    println!("{:?}",d1);
    for _ in 1..=10 {
        d1.delete_random();
        println!("{:?}", d1.cards.len());
    }

}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn chequeo_mazo_sin_jokers(){
        let mut d1=Deck{cards:vec![]};
        d1.new();
        assert_eq!(d1.cards.len(),52,"El mazo no tiene 52 cartas");

    }
    #[test]
    fn chequeo_mazo_con_jokers(){
        let mut d1=Deck{cards:vec![]};
        d1.new();
        assert_eq!(d1.cards.len(),54,"El mazo no tiene 54 cartas");

    }
    

}