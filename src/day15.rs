use regex::Regex;


fn calculate_taxy_distance(x: (i32, i32), y: (i32,i32)) -> i32 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

fn check_if_beacon_can_be_present(x: i32, y: i32, sensors: &Vec<((i32, i32),i32)>) -> bool {
    for (sensor, distance) in sensors {
        let sensor_distance = calculate_taxy_distance(*sensor, (x, y));
        if sensor_distance <= *distance {
            return false;
        }
    }
    return true;
}



pub fn run(input: &str) -> String {
    let mut beacons = vec![];
    let mut sensors = vec![];
    let re = Regex::new(r"Sensor at x=([\-\d]+), y=([\-\d]+): closest beacon is at x=([\-\d]+), y=([\-\d]+)").unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let sensor = (captures[1].parse::<i32>().unwrap(), captures[2].parse::<i32>().unwrap());
        let beacon = (captures[3].parse::<i32>().unwrap(), captures[4].parse::<i32>().unwrap());
        let distance = calculate_taxy_distance(sensor, beacon);
        sensors.push((sensor, distance));
        beacons.push(beacon);
        println!("{:?} {:?} {:?}", sensor, beacon, distance);
    }
    sensors.sort_by(|a, b| a.0.0.cmp(&b.0.0));
    beacons.sort_by(|a, b| a.0.cmp(&b.0));
    //min and max x of sensors and beacons
    let min_x = if sensors[0].0.0 > beacons[0].0 { beacons[0].0 } else  { sensors[0].0.0 }; 
    let max_x = if sensors[sensors.len()-1].0.0 > beacons[beacons.len()-1].0 { sensors[sensors.len()-1].0.0 } else  { beacons[beacons.len()-1].0 };

    let y_test = 2000000;
    
    let mut queue_possible = vec![];
    let mut queue_not_possible = vec![];
    for x in min_x-2000000..max_x+20000000 {
        if check_if_beacon_can_be_present(x, y_test, &sensors) {
            queue_possible.push((x, y_test));
            //print!(".");
        } else {
            if beacons.contains(&(x, y_test)) {
                //print!("B");
            } else {
                queue_not_possible.push((x, y_test));
                //print!("X");
            }
        }

    }
    //println!("queue_possible: {:?} queue_not_possible {:?}", queue_possible, queue_not_possible); 
    
    queue_not_possible.len().to_string()
}

// > 4254102
// current 4502209