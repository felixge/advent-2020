use anyhow::Result;
use std::{collections::HashMap, io::{self, Read}};

type Card = i32;
type Cards = Vec<Card>;

fn main() -> Result<()> {
    // Parse Input Data
    let mut decks: [Cards; 2] = [vec![], vec![]];
    let input = &mut String::new();
    io::stdin().read_to_string(input)?;
    let mut cards: Cards = vec![];
    let mut player_id = 0;
    for line in (input.trim().to_owned()+"\n").split("\n") {
        if line.starts_with("Player ") {
            cards = vec![];
        } else if line != "" {
            cards.push(line.parse()?);
        } else {
            decks[player_id] = cards.clone();
            player_id += 1;
        }
    }

    // Determine Winner
    let winner = game(&mut decks);

    // Calculate Result
    let mut factor = 1;
    let winner_deck = &mut decks[winner];
    let mut result = 0;
    while winner_deck.len() > 0 {
        let card = winner_deck.remove(winner_deck.len()-1);
        result += card * factor;
        factor += 1;
    }
    println!("answer: {}", result);
    Ok(())
}

// game plays rounds until there is a winner (0 or 1).
fn game(decks: &mut[Cards; 2]) -> usize {
    let mut seen0: HashMap<Cards, bool> = HashMap::new();
    let mut seen1: HashMap<Cards, bool> = HashMap::new();
    loop {
        if seen0.contains_key(&decks[0]) && seen1.contains_key(&decks[1]) {
            return 0;
        }

        seen0.insert(decks[0].clone(), true);
        seen1.insert(decks[1].clone(), true);
        let c0 = decks[0].remove(0);
        let c1 = decks[1].remove(0);

        let winner;
        if c0 as usize <= decks[0].len() && c1 as usize <= decks[1].len() {
            let d0 = decks[0][0..c0 as usize].to_vec();
            let d1 = decks[1][0..c1 as usize].to_vec();
            winner = game(&mut [d0, d1]);
        } else if c0 > c1 {
            winner = 0;
        } else if c1 > c0 {
            winner = 1;
        } else {
            panic!("impossible");
        }

        if winner == 0 {
            decks[0].append(&mut vec![c0, c1]);
            if decks[1].len() == 0 {
                break 0;
            }
        } else if winner == 1 {
            decks[1].append(&mut vec![c1, c0]);
            if decks[0].len() == 0 {
                break 1;
            }
        } else {
            panic!("impossible");
        }
    }
}
