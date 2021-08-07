struct TimeLocation {
    time: u32,
    location: u32,
}

impl TimeLocation {
    fn new(time: u32, location: u32) -> TimeLocation {
        TimeLocation { time, location }
    }
}

fn calc_speeds(tls: Vec<TimeLocation>) -> Vec<u32> {
    let mut prev_time = 0;
    let mut prev_location = 0;
    let mut speeds = Vec::new();
    for tl in tls {
        let diff_time = tl.time - prev_time;
        let diff_location = tl.location - prev_location;
        if diff_time == 0 {
            continue;
        }
        let speed = diff_location / diff_time;
        speeds.push(speed);
        prev_time = tl.time;
        prev_location = tl.location;
    }
    speeds
}

fn is_speed_violation(limit: u32, speeds: Vec<u32>) -> bool {
    for s in speeds {
        if s > limit {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let inputs = vec![
            super::TimeLocation::new(0, 0),
            super::TimeLocation::new(1, 30),
            super::TimeLocation::new(2, 80),
        ];
        let want = vec![30, 50];
        let speeds = super::calc_speeds(inputs);
        assert_eq!(want, speeds);
        assert_eq!(true, super::is_speed_violation(40, speeds));
    }

    #[test]
    fn test1() {
        let inputs = vec![
            super::TimeLocation::new(1, 50),
            super::TimeLocation::new(2, 100),
            super::TimeLocation::new(3, 150),
            super::TimeLocation::new(5, 200),
            super::TimeLocation::new(8, 250),
        ];
        let want = vec![50, 50, 50, 25, 16];
        let speeds = super::calc_speeds(inputs);
        assert_eq!(want, speeds);
        assert_eq!(false, super::is_speed_violation(60, speeds));
    }
}
