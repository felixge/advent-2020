use anyhow::{anyhow, Result};
use itertools::any;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let re = Regex::new(r"^(.+?) \(contains (.+?)\)$").unwrap();
    let mut all_ingredients = HashMap::new();
    let mut allergens_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.trim().split("\n") {
        let caps = re.captures(line).ok_or(anyhow!("bad line: {}", line))?;
        let ingredients: HashSet<String> = caps[1].split(" ").map(String::from).collect();
        let allergens: HashSet<String> = caps[2].split(", ").map(String::from).collect();
        for allergen in allergens {
            allergens_ingredients
                .entry(allergen)
                .and_modify(|x| *x = x.intersection(&ingredients).map(String::from).collect())
                .or_insert(ingredients.clone());
        }
        for ingredient in ingredients {
            all_ingredients
                .entry(ingredient)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }
    let mut answer = 0;
    for (ingredient, count) in all_ingredients {
        if !any(&allergens_ingredients, |(_, ingredients)| {
            ingredients.contains(&ingredient)
        }) {
            answer += count;
        }
    }
    println!("{:?}", answer);
    Ok(())
}
