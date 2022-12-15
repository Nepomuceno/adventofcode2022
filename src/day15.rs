use regex::Regex;


fn calculate_taxy_distance(x: (i64, i64), y: (i64,i64)) -> i64 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

fn check_if_beacon_can_be_present(x: i64, y: i64, sensors: &Vec<((i64, i64),i64)>) -> bool {
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
        let sensor = (captures[1].parse::<i64>().unwrap(), captures[2].parse::<i64>().unwrap());
        let beacon = (captures[3].parse::<i64>().unwrap(), captures[4].parse::<i64>().unwrap());
        let distance = calculate_taxy_distance(sensor, beacon);
        sensors.push((sensor, distance));
        beacons.push(beacon);
    }
    let max_size = 4000000;
    sensors.sort_by(|a, b| a.0.0.cmp(&b.0.0));
    beacons.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result = (0,0);
    'external: for y in 0..=max_size {
        let mut intersections = vec![];
        for (sensor, distance) in &sensors {
            let sensor_distance = calculate_taxy_distance(*sensor, (sensor.0, y));
            if sensor_distance > *distance {
                continue;
            }
            let mut sensor_left = sensor.0 - distance + sensor_distance;
            let mut sensor_right = sensor.0 + distance - sensor_distance;
            sensor_left = sensor_left.max(0);
            sensor_right = sensor_right.min(max_size);
            if sensor_left == 0 && sensor_right == max_size {
                continue 'external;
            }
            intersections.push((sensor_left, sensor_right));
        }
        intersections.sort_by(|a, b| a.0.cmp(&b.0));
        let min_x = intersections[0].0;
        if min_x > 0 {
            result = (min_x+1, y);
            break 'external;
        }
        let mut max_x = intersections[0].1;
        for i in 1..intersections.len()-1 {
            if max_x < intersections[i].0 {
                result = (max_x+1, y);
                break 'external;
            }
            if intersections[i].1 > max_x {
                max_x = intersections[i].1;
            }            
        }
    }
    let worked = check_if_beacon_can_be_present(result.0, result.1, &sensors);
    println!("Result: {:?} worked: {}", result, worked);
    ((result.0*4000000) + result.1).to_string()
}