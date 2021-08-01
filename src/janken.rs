use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Te {
    Rock,
    Scissors,
    Paper,
    Draw,
}

trait Solver {
    fn add(&mut self, te: Te);
    fn solve(&self) -> Te;
}

fn new() -> impl Solver {
    SolverImpl::new()
}

struct SolverImpl {
    tes: HashMap<Te, bool>,
}

impl SolverImpl {
    fn new() -> SolverImpl {
        let mut tes = HashMap::new();
        tes.insert(Te::Paper, false);
        tes.insert(Te::Rock, false);
        tes.insert(Te::Scissors, false);
        SolverImpl{tes}
    }
}

impl Solver for SolverImpl {
    fn add(&mut self, te:Te){
        self.tes.insert(te, true);
    }

    fn solve(&self) -> Te {
        let p: bool = *self.tes.get(&Te::Paper).unwrap();
        let r: bool = *self.tes.get(&Te::Rock).unwrap();
        let s: bool = *self.tes.get(&Te::Scissors).unwrap();
        if p & r & s {
            return Te::Draw;
        } else if p & r {
            return Te::Paper;
        } else if p & s {
            return Te::Scissors;
        } else if r & s {
            return Te::Rock;
        } else if p {
            return Te::Draw;
        } else if r {
            return Te::Draw;
        } else if s {
            return Te::Draw;
        } else {
            unimplemented!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;
    use super::Te;

    #[test]
    fn test0(){
        let mut solver = super::new();
        solver.add(Te::Paper);
        solver.add(Te::Paper);
        solver.add(Te::Rock);
        let want = Te::Paper;
        let got = solver.solve();
        assert_eq!(want, got);
    }

    #[test]
    fn test1(){
        let mut solver = super::new();
        solver.add(Te::Rock);
        solver.add(Te::Paper);
        solver.add(Te::Rock);
        solver.add(Te::Scissors);
        let want = Te::Draw;
        let got = solver.solve();
        assert_eq!(want, got);
    }
}