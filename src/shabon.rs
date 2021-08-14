struct Wind {
    x: i32,
    y: i32,
}

impl Wind {
    fn new(x:i32, y:i32) -> Wind {
        Wind{x, y}
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x:i32, y:i32) -> Point {
        Point{x, y}
    }
}

trait Solver {
    fn solve(&self, winds:Vec<Wind>) -> i32;
}

struct SolverImpl {
    point: Point,
}

impl SolverImpl {
    fn new(point:Point) -> SolverImpl{
        SolverImpl{point}
    }
}

impl Solver for SolverImpl {
    fn solve(&self, winds:Vec<Wind>) -> i32{
        let mut drop = false;
        let mut max_x = self.point.x;
        let mut current_point = Point::new(self.point.x, self.point.y);
        for wind in winds {
            if drop {
                break;
            }
            current_point.x += wind.x;
            current_point.y += wind.y;
            if current_point.y < 0 {
                drop = true;
            }
            if current_point.x > max_x {
                max_x = current_point.x;
            }
        }
        max_x
    }
}

fn new(point:Point) -> impl Solver {
    SolverImpl::new(point)
}

#[cfg(test)]
mod tests {
    use super::Solver;
    use super::Point;
    use super::Wind;
    #[test]
    fn test0(){
        let winds = vec![
            Wind::new(4,2),
            Wind::new(-5,-4),
            Wind::new(3,3),
            Wind::new(3,3),
        ];
        let solver = super::new(Point::new(1, 1));
        let got = solver.solve(winds);
        let want = 5;
        assert_eq!(want, got);
    }
}