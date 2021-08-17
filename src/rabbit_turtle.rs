struct Race {
    road: u32,
    rabbit_min: u32,
    rabbit_span: u32,
    rabbit_rest: u32,
    turtle_min: u32,
}

impl Race {
    fn new(road:u32, rabbit_min:u32, rabbit_span:u32, rabbit_rest:u32, turtle_min:u32) -> Race {
        Race{road, rabbit_min, rabbit_span, rabbit_rest, turtle_min}
    }

    fn start(&self) -> RaceResult {
        let kame = self.road * self.turtle_min;
        let rabbit_rest_times: u32 = self.road / self.rabbit_span - 1;
        let usagi = self.road * self.rabbit_min + self.rabbit_rest * rabbit_rest_times;
        if usagi < kame {
            return RaceResult::USAGI;
        } else if usagi == kame {
            return RaceResult::DRAW;
        } else {
            return RaceResult::KAME;
        }
    }
}

#[derive(Debug, PartialEq)]
enum RaceResult {
    USAGI,
    KAME,
    DRAW,
}

#[cfg(test)]
mod tests {
    use super::{Race, RaceResult};
    #[test]
    fn test0(){
        let race = Race::new(6, 2, 2 ,5, 4);
        let want = RaceResult::USAGI;
        let got = race.start();
        assert_eq!(want, got);
    }
}