use std::collections::HashSet;
use std::env;
use std::fs;
use std::collections::HashMap;
use std::cell::RefCell;
use std::hash::Hash;
use std::rc::Rc;

use pathfinding::prelude::build_path;
use pathfinding::prelude::dijkstra_all;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1 = solve(&input);
    // let part2 = solve2(&input);
    println!("solution to part 1: {part1}");
    // println!("solution to part 2: {part2}");
}

#[derive(Clone, Eq, PartialOrd, Ord)]
struct Valve {
    name: String,
    flow_rate: u32,
    adjacent_valves: Vec<Rc<RefCell<Valve>>>
}

impl core::fmt::Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Valve")
            .field("name", &self.name)
            .field("flow_rate", &self.flow_rate)
            .field("adjacent_valves", &self.adjacent_valves.iter()
                .map(|valve| valve.borrow().name.clone()).collect::<Vec<_>>())
            .finish()
    }
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Valve {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Valve {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            flow_rate: 0,
            adjacent_valves: vec![],
        }
    }
}

fn floyd_warshall(valves: HashSet<&Valve>) -> HashMap<(&str, &str), Vec<Valve>> {
    let valves = valves.into_iter().collect::<Vec<_>>();
    let valve_idx_map = (0..).zip(valves.into_iter())
        .collect::<HashMap<_, _>>();
    let mut dist = vec![vec![i32::MAX; valves.len()]; valves.len()];
    let mut next = vec![vec![Option::<usize>::None; valves.len()]; valves.len()];
    let mut edges = HashSet::new();

    // generate graph edges
    for valve in valves {
        for neighbor in valve.adjacent_valves {
            edges.insert((valve.name, neighbor.borrow().name));
        }
    }

    // build connections
    for edge in edges {
        dist[&edge] = 1; // set weights to 1 to start
        next[&edge] = edge.1;
    }

    for

    todo!()
}

fn solve(input: &str) -> usize {
    let mut valves = HashMap::new();
    for line in input.lines() {
        let (valve_desc, adjacent_valves_desc) = line.split_once(';').unwrap();
        let valve_desc = valve_desc.split(' ').collect::<Vec<&str>>();

        let name = valve_desc[1];

        let flow_rate = valve_desc[4]
            .split('=')
            .nth_back(0).unwrap()
            .parse().unwrap();

        valves
            .entry(name)
            .or_insert(Rc::new(RefCell::new(Valve::new(name))))
            .borrow_mut()
            .flow_rate = flow_rate;

        let adjacent_valves_names = if let Some(valves) = adjacent_valves_desc.split_once("valves ") {
            valves.1
        } else {
            adjacent_valves_desc.split_once("valve ").unwrap().1
        };

        for adjacent_valve_name in adjacent_valves_names.split(", ") {
            let adj_valve = Rc::clone(valves
                .entry(adjacent_valve_name)
                .or_insert(Rc::new(RefCell::new(Valve::new(adjacent_valve_name)))));

            valves.entry(name).and_modify(|e| e.borrow_mut().adjacent_valves.push(adj_valve));
        }
    }

    dbg!(&valves);

    let path

    // let starting_valve = valves.get("AA").unwrap().borrow().clone();
    // let reachables = dijkstra_all(&starting_valve, successors);
    // let all_reachables = valves
    //     .iter()
    //     .map(|(name, valve)| (name, dijkstra_all(&valve.borrow().clone(), successors)));

    // let all_paths = all_reachables
    //     .map(|(name, reachables)| (*name, build_path(&valves.get(name).unwrap().borrow().clone(), &reachables)))
    //     .collect::<HashMap<_, _>>();
    // let max = all_paths
    //     .map(|path| path.iter().map(|valve| valve.flow_rate))
    //     .sum()
    // let mut
    // for path in all_paths {
    //     for valve in path {
    //         let mut time = 0;
    //         if
    //     }
    // }
    // dbg!(dijkstra_all(&starting_valve, successors).keys());
    // for reachable_target in dijkstra_all(&starting_valve, successors).iter() {
    //     dbg!(build_path(&reachable_target, &dijkstra_all(&starting_valve, successors)));
    // }
    // dbg!(&all_paths["AA"]);

    0
}

fn successors(valve: &Valve) -> Vec<(Valve, usize)> {
    valve.adjacent_valves.iter().map(|adj_valve| (adj_valve.borrow().clone(), 10)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../example.txt");

        let part1 = solve(&input.to_string());
        assert_eq!(part1, 1651);
    }
}
