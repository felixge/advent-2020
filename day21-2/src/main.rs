use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let re = Regex::new(r"^(.+?) \(contains (.+?)\)$").unwrap();
    let mut allergens_potential_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.trim().split("\n") {
        let caps = re.captures(line).ok_or(anyhow!("bad line: {}", line))?;
        let ingredients: HashSet<String> = caps[1].split(" ").map(String::from).collect();
        let allergens: HashSet<String> = caps[2].split(", ").map(String::from).collect();
        for allergen in allergens {
            allergens_potential_ingredients
                .entry(allergen)
                .and_modify(|x| *x = x.intersection(&ingredients).map(String::from).collect())
                .or_insert(ingredients.clone());
        }
    }

    let mut allergen_ingredients: HashMap<String, String> = HashMap::new();
    while allergens_potential_ingredients.len() > 0 {
        let mut new_list = HashMap::new();
        for (allergen, ingredients) in allergens_potential_ingredients.iter() {
            let allergen = allergen.clone();
            let matched_ingredients = allergen_ingredients
                        .iter()
                        .map(|(_, x)| x.clone())
                        .collect();
            let diff: HashSet<_> = ingredients
                .difference(&matched_ingredients)
                .map(|x| x.clone())
                .collect();
            if diff.len() == 1 {
                let ingredient = diff.iter().next().unwrap().clone();
                allergen_ingredients.insert(allergen, ingredient);
            } else {
                new_list.insert(allergen, ingredients.clone());
            }
        }
        allergens_potential_ingredients = new_list;
    }

    
    let mut sorted_allergens: Vec<String> = allergen_ingredients.iter().map(|(x, _)| x.clone() ).collect();
    let mut sorted_ingredients: Vec<String> = vec![];
    sorted_allergens.sort();
    for allergen in sorted_allergens.iter() {
        sorted_ingredients.push(allergen_ingredients[allergen].clone());
    }
    println!("{}", sorted_ingredients.join(","));
    Ok(())
}
