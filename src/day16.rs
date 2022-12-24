use std::collections::{HashMap, HashSet};
use termion::{color, style,cursor};

#[derive(Debug)]
#[derive(Clone)]
struct Valve {
    id: String,
    flow_rate: u32,
    tunnels: HashMap<String, u32>,
    open: bool,
}

fn print_valves (valves: &HashMap<String, Valve>) {
    for (key, value) in valves {
        println!("{}{}{}: {:?}", color::Fg(color::Blue), key, color::Fg(color::Reset),  value);
    }
}

const num_of_days: u32 = 30;

// Breath first search
// Look at the valve if the valve value 

#[derive(Clone)]
#[derive(Debug)]
struct ValveState {
    valves: HashMap<String, Valve>,
    open_valves: Vec<String>,
    time_remaining: u32,
    current_flow: u32,
    current_valve: String,
    path: Vec<String>,
}

fn take_step(valve_states: &Vec<ValveState>, seen_states: &mut HashMap<(String,Vec<String>), u32>) -> Vec<ValveState> {
    let mut new_valve_states = Vec::new();
    for valve_state in valve_states.iter() {
        if valve_state.time_remaining == 0 {
            new_valve_states.push(valve_state.clone());
            continue;
        }
        if valve_state.valves.get(&valve_state.current_valve).unwrap().flow_rate == 0 {
            for tunnel in valve_state.valves.get(&valve_state.current_valve).unwrap().tunnels.iter() {
                let mut new_valve_state = valve_state.clone();
                new_valve_state.current_valve = tunnel.0.to_string();
                new_valve_state.time_remaining = new_valve_state.time_remaining - tunnel.1;
                new_valve_state.path.push(format!("{}{}T-{}{}{}: /{} {} {}", color::Fg(color::Green), style::Bold, tunnel.1, color::Fg(color::Reset), style::Reset,  new_valve_state.current_valve.clone(), new_valve_state.current_flow, new_valve_state.current_flow));
                new_valve_states.push(new_valve_state.clone());
            }
            continue;
        }
        let mut state_clone = valve_state.clone();
        let added_flow = state_clone.valves.iter().filter(|v| v.1.open == true).map(|v| v.1.flow_rate).sum::<u32>();
        state_clone.current_flow = state_clone.current_flow + added_flow;
        state_clone.time_remaining = state_clone.time_remaining - 1;
        state_clone.valves.get_mut(&state_clone.current_valve).unwrap().open = true;
        state_clone.path.push(format!("{}{}O-1{}{}: {} {} {}", color::Fg(color::Blue), style::Bold, color::Fg(color::Reset), style::Reset, state_clone.current_valve.clone(), added_flow, state_clone.current_flow));
        let filtered_tunnels = 
        state_clone.valves.get(&state_clone.current_valve).unwrap().tunnels.iter()
            .filter(|t| 
                *t.1 < state_clone.time_remaining && 
                state_clone.valves.get(t.0).unwrap().open == false).collect::<Vec<_>>();
        if filtered_tunnels.len() == 0 {
            let added_flow = state_clone.valves.iter().filter(|v| v.1.open == true).map(|v| v.1.flow_rate).sum::<u32>();
            state_clone.current_flow = state_clone.current_flow + (added_flow * state_clone.time_remaining);
            state_clone.path.push(format!("{}{}T-{}{}{}: {} {} {}", color::Fg(color::LightYellow), style::Bold,state_clone.time_remaining, color::Fg(color::Reset), style::Reset, state_clone.current_valve.clone(), added_flow, state_clone.current_flow));
            state_clone.time_remaining = 0;
            new_valve_states.push(state_clone.clone());
            continue;
        }
        for tunnel in filtered_tunnels.iter() {
            let mut new_valve_state = state_clone.clone();
            new_valve_state.current_valve = tunnel.0.to_string();
            new_valve_state.time_remaining = new_valve_state.time_remaining - tunnel.1;
            let added_flow = new_valve_state.valves.iter().filter(|v| v.1.open == true).map(|v| v.1.flow_rate).sum::<u32>();
            new_valve_state.current_flow = new_valve_state.current_flow + added_flow * tunnel.1;
            new_valve_state.path.push(format!("{}{}T-{}{}{}: {} {} {}", color::Fg(color::Green), style::Bold, tunnel.1, color::Fg(color::Reset), style::Reset,  new_valve_state.current_valve.clone(), added_flow, new_valve_state.current_flow));
            new_valve_states.push(new_valve_state.clone());
        }


    }
    new_valve_states
}

fn calculate_travel_time_to_interesting_valves(
    valves: &HashMap<String, Valve>,
    interesting_valves: &Vec<String>
) -> HashMap<String,Valve> {
    let mut result = HashMap::new();
    for valve in valves.iter() {
        let mut new_valve = valve.1.clone();
        new_valve.tunnels = HashMap::new();
        for interesting_valve in interesting_valves.iter() {
            if valve.0 == interesting_valve {
                continue;
            }
            let mut current_valves = HashSet::new();
            current_valves.insert(valve.0.clone());
            let mut time = 1;
            'external: loop {
                let mut new_current_valves = HashSet::new();
                for current_valve in current_valves.iter() {
                    let current_valve = valves.get(current_valve).unwrap();
                    if current_valve.tunnels.contains_key(interesting_valve) {
                        new_valve.tunnels.insert(interesting_valve.clone(), time);
                        break 'external;
                    } else {
                        for tunnel in current_valve.tunnels.iter() {
                            new_current_valves.insert(tunnel.0.clone());
                        }
                    }
                }
                current_valves = new_current_valves;
                time = time + 1;
            }
        }
        result.insert(valve.0.clone(), new_valve);
    }
    result
}

fn filter_only(
    valves: &HashMap<String, Valve>,
    start_values: &str,
) -> HashMap<String,Valve> {
    let mut result = HashMap::new();
    for valve in valves.iter() {
        if start_values.contains(valve.0) || valve.1.flow_rate != 0 {
            result.insert(valve.0.clone(), valve.1.clone());
        }
    }
    result
}

pub fn run(input: &str) -> String {
    let mut valves = HashMap::new();
    for line in input.lines() {
        let result = 
        line.replace("Valve ", "")
        .replace("has flow rate=", "")
        .replace("; tunnels lead to valves", "")
        .replace("; tunnel leads to valve", ",")
        .replace(",", "");
        let valve = Valve {
            id: result.split(" ").next().unwrap().to_string(),
            flow_rate: result.split(" ").nth(1).unwrap().parse().unwrap(),
            tunnels: result.split(" ").skip(2).map(|s| (s.to_string(), 1)).collect(),
            open: false,
        };        
        valves.insert(valve.clone().id,valve);
    }
    let interesting_valves = valves.iter().filter(|v| v.1.flow_rate > 0).map(|v| v.0.clone()).collect::<Vec<String>>();
    let mut valves = calculate_travel_time_to_interesting_valves(&valves, &interesting_valves);
    valves = filter_only(&valves, "AA");
    print_valves(&valves);
    
    let mut valve_states = Vec::new();
    let valve_state = ValveState {
        valves: valves.clone(),
        open_valves: Vec::new(),
        time_remaining: num_of_days,
        current_flow: 0,
        path: vec![format!("{}{}S{}{}: {}/{} {} {}", color::Fg(color::Yellow), style::Bold, color::Fg(color::Reset), style::Reset, num_of_days, "AA".to_string(), 0, 0)],
        current_valve: "AA".to_string(),
    };

    valve_states.push(valve_state);
    let mut seen_states = HashMap::new();
    let mut completed_valve_states = vec![]; 
    loop {
        if valve_states.clone().iter().all(|v| v.time_remaining == 0) {
            break;
        }
        valve_states = take_step(&valve_states, &mut seen_states);
        completed_valve_states.append(&mut valve_states.clone().iter().filter(|v| v.time_remaining == 0).map(|v| v.clone()).collect::<Vec<ValveState>>());
        valve_states = valve_states.into_iter().filter(|v| v.time_remaining > 0).collect::<Vec<ValveState>>();
        if valve_states.len() == 0 {
            break;
        }
        let max_days = valve_states.iter().map(|v| v.time_remaining).max().unwrap();
        println!("{}Days passed{}: {}{}{} States: {}{}{} Seen {}{}{}", color::Fg(color::Yellow), color::Fg(color::Reset), color::Fg(color::Red), max_days, color::Fg(color::Reset), color::Fg(color::LightGreen), valve_states.len(), color::Fg(color::Reset), color::Fg(color::Green), seen_states.len(), color::Fg(color::Reset));
    }
    completed_valve_states.sort_by(|a, b| b.current_flow.cmp(&a.current_flow));
    let max_flow = completed_valve_states[0].current_flow;
    // let mut max_path = valve_states[0].path.clone();
    
    for i in 0..completed_valve_states[0].path.len() {
        let value = completed_valve_states[0].path[i].clone();
        print!("{value}");
        for j in 0..1 {
            if value != completed_valve_states[j].path[i] {
                print!("/ {j} {}", completed_valve_states[j].path[i]);
            }
        }
        println!()
    }


    max_flow.to_string()
}