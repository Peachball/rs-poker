extern crate rs_poker;
use rs_poker::core::{Card, Hand, Suit, Value};
use rs_poker::holdem::MonteCarloGame;

fn main() {
    let hands = ["Adkh", "8c8s"]
        .iter()
        .map(|s| Hand::new_from_str(s).expect("Should be able to create a hand."))
        .collect();
    let board = vec![
        Card {
            value: Value::Jack,
            suit: Suit::Diamond,
        },
        Card {
            value: Value::Eight,
            suit: Suit::Club,
        },
        Card {
            value: Value::Three,
            suit: Suit::Diamond,
        },
    ];
    let mut g =
        MonteCarloGame::new_with_hands(hands, board).expect("Should be able to create a game.");
    let mut wins: [u64; 2] = [0, 0];
    for _ in 0..2_000_000 {
        let r = g.simulate();
        g.reset();
        wins[r.0.ones().next().unwrap()] += 1
    }
    println!("Wins = {:?}", wins);
}
