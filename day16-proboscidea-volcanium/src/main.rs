use std::collections::HashSet;
use std::env;
use std::fs;
use std::collections::HashMap;

use petgraph::{prelude::*, Graph};
use petgraph::algo::floyd_warshall;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve {
    name: String,
    flow_rate: u32,
    node_idx: Option<NodeIndex>,
    adjacent_valves: HashSet<String>,
    bitmask: Option<u32>,
}

impl Valve {
    fn new(name: &str, flow_rate: u32) -> Self {
        Self {
            name: name.to_string(),
            flow_rate,
            node_idx: None,
            adjacent_valves: HashSet::new(),
            bitmask: None,
        }
    }
}

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

fn solve(input: &str) -> u32 {
    // let mut adjacent_map = HashMap::new();
    // let mut flow_map = HashMap::new();
    let mut graph = Graph::<Valve, u32>::new();
    let mut valves = HashMap::<&str, Valve>::new();
    let mut bitmask_map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let (valve_desc, adjacent_valves_desc) = line.split_once(';').unwrap();
        let valve_desc = valve_desc.split(' ').collect::<Vec<&str>>();

        let name = valve_desc[1];

        let flow_rate = valve_desc[4]
            .split('=')
            .nth_back(0).unwrap()
            .parse::<u32>().unwrap();

        // flow_map.insert(name, flow_rate);
        let valve = Valve::new(name, flow_rate);

        let adjacent_valves_names = if let Some(valves) = adjacent_valves_desc.split_once("valves ") {
            valves.1
        } else {
            adjacent_valves_desc.split_once("valve ").unwrap().1
        };

        // let mut adjacent_set = HashSet::new();
        for adjacent_valve_name in adjacent_valves_names.split(", ") {
            // adjacent_set.insert(adjacent_valve_name);
            valve.adjacent_valves.insert(adjacent_valve_name.to_string());
        }

        valve.node_idx = Some(graph.add_node(valve));
        valves.insert(name, valve);

        // adjacent_map.insert(name, adjacent_set);
        bitmask_map.insert(name, 1 << i);
    }

    // once all nodes are added to the graph, add the edges
    for valve in valves.values() {
        for adjacent_valve in valve.adjacent_valves {
            graph.add_edge(valve.node_idx.unwrap(), valves[adjacent_valve.as_str()].node_idx.unwrap(), 1);
        }
    }

    let dist = floyd_warshall(&graph, |_| u32::MAX).unwrap();

    // floyd warshall
    // let mut dist = vec![vec![u32::MAX; valves.len()]; valves.len()];
    // for (i, &valve_from) in valves.keys().enumerate() {
    //     for (j, &valve_to) in valves.keys().enumerate() {
    //         dist[i][j] = if valves_vec[i].borrow().contains(valve_to) { 1 } else { u32::MAX };
    //     }
    // }

    // for i in 0..dist.len() {
    //     dist[i][i] = 0;
    // }

    // for k in 0..dist.len() {
    //     for i in 0..dist.len() {
    //         for j in 0..dist.len() {
    //             dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
    //         };
    //     }
    // }

    let val = valves
        .iter()
        .filter(|(_name, valve)| valve.flow_rate == 0)
        .cloned()
        .collect();

    // set bitmasks
    // for (i, valve) in flows.values_mut().enumerate() {
        // valve.bitmask = 1 << 
    // }

    *visit(
        &valves,
        &flows,
        &dist,
        &valves["AA"],
        30,
        0,
        0,
        HashMap::new())
        .values().max().unwrap()
}

fn visit(
    valves: &HashMap<&str, Valve>,
    flows: &HashMap<&str, Valve>,
    dist: &HashMap<(NodeIndex, NodeIndex), u32>,
    current_valve: &Valve,
    budget: u32,
    state: u32,
    flow: u32,
    answer: HashMap<u32, u32>) -> HashMap<u32, u32>
{
    let best_answer_for_state = answer.entry(state).or_insert(0);
    best_answer_for_state = best_answer_for_state.max(&mut flow);

    for next_valve in flows.values() {
        let new_budget = budget - dist[&(current_valve.node_idx.unwrap(), next_valve.node_idx.unwrap())] - 1;
        let valve_open = next_valve.bitmask.unwrap() & state;
        if valve_open || new_budget <= 0 {
            continue;
        } else {
            return visit(valves, flows, *next_valve, new_budget, state | BITMASK_MAP[next_valve], flow + new_budget * next_valve.flow_rate, answer);
        }
    }

    answer
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
