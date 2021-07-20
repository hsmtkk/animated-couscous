struct Kouji {
    x: u32,
    y: u32,
    r: u32,
}

impl Kouji {
    fn new(x:u32, y:u32, r:u32) -> Kouji {
        Kouji{x, y, r}
    }

    fn solve(&self, p:Point) -> Noise {
        if (p.x - self.x) * (p.x - self.x) + (p.y - self.y) * (p.y - self.y) >= self.r * self.r {
            Noise::Silent
        } else {
            Noise::Noisy
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x:u32, y:u32) -> Point {
        Point{x,y}
    }
}

#[derive(Debug, PartialEq)]
enum Noise {
    Noisy,
    Silent,
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::Kouji;
    use super::Point;
    use super::Noise;

    #[test]
    fn test0(){
        let kouji = Kouji::new(20, 10, 10);
        let mut wants = HashMap::new();
        wants.insert(Point::new(25, 10), Noise::Noisy);
        wants.insert(Point::new(20, 15), Noise::Noisy);
        wants.insert(Point::new(70, 70), Noise::Silent);
        for (want_point, want_noise) in wants {
            let got = kouji.solve(want_point);
            assert_eq!(want_noise, got);
        }
    }
}