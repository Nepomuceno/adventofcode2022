// use std::collections::{HashMap};

// #[derive(Debug)]
// #[derive(Clone)]
// struct Valve {
//     id: String,
//     flow_rate: u32,
//     tunnels: Vec<(String,bool)>,
//     open: bool,
// }

// fn calculate_flow_remaining(valves: &mut HashMap<String, &mut Valve>, current_valve: &str, time_remaining: u32, path: &mut Vec<String>) -> (u32, Vec<String>) {
//     path.push(current_valve.to_string());

//     let valve = valves[current_valve];
//     let mut max_flow = 0;

//     if time_remaining == 0 || valve.tunnels.is_empty() {
//         // Return the maximum flow and the path when the time runs out or when we reach a leaf node
//         return (max_flow, path.clone());
//     }

//     if path.len() >= 30 {
//         // Return the maximum flow and the path when the path becomes too long
//         return (max_flow, path.clone());
//     }

//     // Go to the next tunnel
//     for tunnel in valve.tunnels.iter_mut().enumerate() {
//         if tunnel.1.1 == true {
//             continue;
//         }

//         let mut flow_remaining = 0;
//         let mut path_clone = path.clone();

//         tunnel.1.1 = true;
//         if time_remaining > 1 {
//         //    flow_remaining = calculate_flow_remaining(valves, tunnel.1.0.as_str(), time_remaining - 1, &mut path_clone).0;
//         }
//         tunnel.1.1 = false;

//         if flow_remaining > max_flow {
//             max_flow = flow_remaining;
//             *path = path_clone;
//         }
//     }

//     if valve.flow_rate > 0 && valve.open == false {
//         let time_after_open = time_remaining - 1;
//         let current_flow = valve.flow_rate * time_after_open;
//         valve.open = true;
//         for tunnel in valve.tunnels.iter_mut() {
//             tunnel.1 = false;
//         }
//         path.push("$$".to_string());
//         let flow_remaining = calculate_flow_remaining(valves, current_valve, time_after_open, path).0;
//         return (current_flow + flow_remaining, path.clone());
//     }

//     (max_flow, path.clone())
// }



pub fn run(input: &str) -> String {
    // let mut valves = HashMap::new();
    // for line in input.lines() {
    //     let result = 
    //     line.replace("Valve ", "")
    //     .replace("has flow rate=", "")
    //     .replace("; tunnels lead to valves", "")
    //     .replace("; tunnel leads to valve", ",")
    //     .replace(",", "");
    //     let valve = Valve {
    //         id: result.split(" ").next().unwrap().to_string(),
    //         flow_rate: result.split(" ").nth(1).unwrap().parse().unwrap(),
    //         tunnels: result.split(" ").skip(2).map(|s| (s.to_string(), false)).collect(),
    //         open: false,
    //     };        
    //     // valves.insert(valve.id,&valve);
    // }
    let flow = 0; //calculate_flow_remaining(&mut valves, "AA", 30, &mut vec![]);
    println!("{:?}", flow);
    input.to_string()
}