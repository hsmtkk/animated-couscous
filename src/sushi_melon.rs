trait Solver {
    fn solve(&self, netas:Vec<&str>) -> u32;
}

fn new() -> impl Solver {
    SolverImpl::new()
}

struct SolverImpl {}

impl SolverImpl {
    fn new() -> SolverImpl {
        SolverImpl{}
    } 
}

impl Solver for SolverImpl {
    fn solve(&self, netas:Vec<&str>) -> u32{
        let mut melon = 0;
        let mut count = 0;
        for neta in netas {
            count -= 1;
            if count > 0 {
                continue;
            }
            if "melon" == neta {
                melon += 1;
                count = 10;
            }
        }
        melon
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;
    #[test]
    fn test() {
        let inputs = vec![
            "melon",
            "ikura",
            "sake",
            "kappa",
            "maguro",
            "melon",
            "uni",
            "toro",
            "ebi",
            "amaebi",
            "tamagoyaki",
            "ika",
            "anago",
            "melon",
            "hamachi",
        ];
        let solver = super::new();
        let want = 2;
        let got = solver.solve(inputs);
        assert_eq!(want, got);
    }
}
