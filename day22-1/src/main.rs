use anyhow::Result;
use std::io::{self, Read};

type Card = i32;
type Cards = Vec<Card>;

fn main() -> Result<()> {
    // Parse Input Data
    let mut players: [Cards; 2] = [vec![], vec![]];
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
            players[player_id] = cards.clone();
            player_id += 1;
        }
    }

    // Determine Winner
    let winner = loop {
        let c1 = players[0].remove(0);
        let c2 = players[1].remove(0);
        if c1 > c2 {
            players[0].append(&mut vec![c1, c2]);
            if players[1].len() == 0 {
                break 0;
            }
        } else if c2 > c1 {
            players[1].append(&mut vec![c2, c1]);
            if players[0].len() == 0 {
                break 1;
            }
        } else {
            panic!("impossible");
        }
    };

    // Calculate Result
    let mut factor = 1;
    let winner_cards = &mut players[winner];
    let mut result = 0;
    while winner_cards.len() > 0 {
        let card = winner_cards.remove(winner_cards.len()-1);
        result += card * factor;
        factor += 1;
    }
    println!("answer: {}", result);
    Ok(())
}
