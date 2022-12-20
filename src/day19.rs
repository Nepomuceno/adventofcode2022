use std::{collections::HashSet, vec, time::Instant};

use regex::Regex;


const PRINT_ENABLED: bool = true;

#[derive(Clone)]
#[derive(Debug)]
struct Factory {
    index: usize,
    robots_count: Vec<usize>,
    robots_cost: Vec<Vec<usize>>,
    items: Vec<usize>,
    robots_max: Vec<usize>,
}

impl Factory {
    fn configure_new(blueprint: (usize, usize, usize, usize, usize, usize,usize)) -> Factory {
        Factory {
            index: blueprint.0,
            robots_count: vec![1, 0, 0, 0],
            robots_cost: vec![
                vec![blueprint.1, 0, 0, 0], 
                vec![blueprint.2, 0, 0, 0], 
                vec![blueprint.3, blueprint.4, 0, 0], 
                vec![blueprint.5, 0, blueprint.6, 0]],
            items: vec![0,0,0,0],
            robots_max: vec![blueprint.1.max(blueprint.2).max(blueprint.3).max(blueprint.5),blueprint.4,blueprint.6,5000],
        }
    }
}

fn is_viable(factory: &Factory, num_of_days_remaining: usize, max_geode: usize) -> bool {
    if factory.robots_count[3] * num_of_days_remaining > max_geode {
        return true;
    }
    
    let mut clone_ore = factory.clone();
    let mut i = 0;
    let mut items_need = vec![0,0,0,max_geode+1];

    for _ in i..num_of_days_remaining {
        clone_ore.items[3] += clone_ore.robots_count[3];
        i += 1;
        if clone_ore.items[3] >= items_need[3] {
            break;
        } else {
            items_need[2] += clone_ore.robots_cost[3][2];
            items_need[0] += clone_ore.robots_cost[3][0];
            clone_ore.robots_count[3] += 1;
        }
    }
    let mut days_before_ore = 0;
    for _ in i..num_of_days_remaining {
        clone_ore.items[2] += clone_ore.robots_count[2];
        i += 1;
        days_before_ore += 1;
        if clone_ore.items[2] >= items_need[2] {
            break;
        } else {
            items_need[1] += clone_ore.robots_cost[2][1];
            items_need[0] += clone_ore.robots_cost[2][0];
            clone_ore.robots_count[2] += 1;
        }
    }
    for _ in i..num_of_days_remaining {
        clone_ore.items[1] += clone_ore.robots_count[1];
        i += 1;
        days_before_ore += 1;
        if clone_ore.items[1] >= items_need[1] {
            break;
        } else {
            items_need[0] += clone_ore.robots_cost[1][0];
            clone_ore.robots_count[1] += 1;
        }
    }
    for _ in i..num_of_days_remaining {
        clone_ore.items[0] += clone_ore.robots_count[0] + days_before_ore;
        i += 1;
        if clone_ore.items[0] >= items_need[0] {
            break;
        }
        clone_ore.robots_count[0] += 1;
    }
    let result =  i <= num_of_days_remaining;

    let mut clone_ore = factory.clone();

    for _ in 0..num_of_days_remaining {
        clone_ore.items[0] += clone_ore.robots_count[0];
        clone_ore.robots_count[0] += 1;
    }
    let mut clone_clay = factory.clone();
    clone_clay.items[0] = clone_ore.items[0];
    for _ in 0..num_of_days_remaining {
        clone_clay.items[1] += clone_clay.robots_count[1];
        if clone_clay.items[0] > clone_clay.robots_cost[1][0] {
            clone_clay.items[0] -= clone_clay.robots_cost[1][0];
            clone_clay.robots_count[1] += 1;
        }
    }
    let mut clone_obsidian = factory.clone();
    clone_obsidian.items[0] = clone_clay.items[0];
    clone_obsidian.items[1] = clone_clay.items[1];
    for _ in 0..num_of_days_remaining {
        clone_obsidian.items[2] += clone_obsidian.robots_count[2];
        if clone_obsidian.items[0] > clone_obsidian.robots_cost[2][0] && clone_obsidian.items[1] > clone_obsidian.robots_cost[2][1] {
            clone_obsidian.items[0] -= clone_obsidian.robots_cost[2][0];
            clone_obsidian.items[1] -= clone_obsidian.robots_cost[2][1];
            clone_obsidian.robots_count[2] += 1;
        }
    }
    let mut clone_geode = factory.clone();
    clone_geode.items[0] = clone_obsidian.items[0];
    clone_geode.items[1] = clone_obsidian.items[1];
    clone_geode.items[2] = clone_obsidian.items[2];
    for _ in 0..num_of_days_remaining {
        clone_geode.items[3] += clone_geode.robots_count[3];
        if clone_geode.items[0] > clone_geode.robots_cost[3][0] && clone_geode.items[2] > clone_geode.robots_cost[3][2] {
            clone_geode.items[0] -= clone_geode.robots_cost[3][0];
            clone_geode.items[2] -= clone_geode.robots_cost[3][2];
            clone_geode.robots_count[3] += 1;
        }
    }
    let result2 = clone_geode.items[3] >= max_geode;
    if false {
        if !result2 && result {
            println!("Filtered by 2 {:?}, days: {}", factory, num_of_days_remaining);
        }
        if result && !result2 {
            println!("Filtered by 1 {:?}, days: {}", factory, num_of_days_remaining);
        }
    }
    return result && result2;
}

fn add_a_day(factory: &Factory, num_of_days_remaining: usize) -> Vec<Factory> {
    let mut result = vec![];
    let mut robots_to_build = vec![0,0,0,0];
    for i in (0..4).rev() {
        if factory.robots_count[i] == factory.robots_max[i] {
            continue;
        }
        let mut can_build = true;
        for j in 0..4 {
            if factory.items[j] < factory.robots_cost[i][j] {
                can_build = false;
                break;
            }
        }
        if can_build {
            robots_to_build[i] += 1;
        }
    }
    for x in robots_to_build.iter().enumerate() {
        if *x.1 == 0 {
            continue;
        }
        let mut new_factory = factory.clone();
        for i in 0..4 {
            new_factory.items[i] -= factory.robots_cost[x.0][i];
        }
        for i in 0..4 {
            new_factory.items[i] += factory.robots_count[i];
        }
        new_factory.robots_count[x.0] += 1;
        result.push(new_factory);
    }
    let mut clone = factory.clone();
    for i in 0..4 {
        clone.items[i] += clone.robots_count[i];
    }
    for i in 0..4 {
        let surplus = clone.items[i] as i32 + (num_of_days_remaining * clone.robots_count[i]) as i32 - (factory.robots_max[i] * num_of_days_remaining) as i32;
        if surplus > 0 {
            clone.items[i] = clone.items[i].saturating_sub(surplus as usize);
        }
    }
    result.push(clone);
    result
}

pub fn run(input: &str) -> String {
    let blueprint_regex = Regex::new(r"(\d+)").unwrap();
    let blueprints_input = input.lines().map(|x| {
        let content = blueprint_regex.captures_iter(x).map(|x| x[1].parse::<usize>().unwrap()).collect::<Vec<usize>>();
        (content[0], content[1], content[2], content[3], content[4], content[5],content[6])
    }).collect::<Vec<(usize, usize, usize, usize, usize, usize,usize)>>();
    let mut factories = blueprints_input.iter().map(|x| Factory::configure_new(*x)).collect::<Vec<Factory>>();
    
    for i in 0..factories.len() {
        let start = Instant::now();
        let max_efficience = 0;
        let mut factory_states = HashSet::new();
        factory_states.insert((factories[i].clone().items,factories[i].clone().robots_count));
        let days_to_calculate = 32;
        let mut test_factories = vec![factories[i].clone()];
        for j in 0..days_to_calculate {
            let mut new_factories_temp = vec![];
            for factory in test_factories.iter() {
                let temp_factory = add_a_day(factory, days_to_calculate - j);
                new_factories_temp.extend(temp_factory);
            }
            let factories_added = new_factories_temp.len() as i32 - test_factories.len() as i32;
            test_factories = vec![];
            let temp_max_geodes = new_factories_temp.iter().map(|x| x.items[3]).max().unwrap();
            let mut factories_removed = 0;
            for factory in new_factories_temp.clone().iter() {
                if factory_states.contains(&(factory.items.clone(), factory.robots_count.clone())) {
                    factories_removed += 1;
                    continue;
                }
                factory_states.insert((factory.items.clone(), factory.robots_count.clone()));
                if is_viable(factory, days_to_calculate - j, temp_max_geodes) {
                    test_factories.push(factory.clone());
                } else {
                    factories_removed += 1;
                }
            }
            if PRINT_ENABLED {
                println!("BluePrint: {}, Day: {}, Factories added: {}, Factory states: {}", factories[1].index, j, factories_added - factories_removed, factory_states.len());
            }
        }
        test_factories.sort_by(|a, b| b.items[3].cmp(&a.items[3]));
        if PRINT_ENABLED {
            println!("{}: {}", i, test_factories[0].items[3] * test_factories[0].index);
            println!("Items: {:?}", test_factories[0].items);
            println!("Robots: {:?}", test_factories[0].robots_count);
            println!("Duration: {:?}", start.elapsed());
        }
        factories[i] = test_factories[0].clone();
    }
    
    let mut total = 1;
    for i in 0..factories.len() {
        total *= factories[i].items[3];
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string(format!("data/day19.txt"))
        .expect("Something went wrong reading the file");
        assert_eq!("33", run(&file));
    }
}
