struct Carrot {
    mass: u32,
    sugar: u32, 
}

impl Carrot {
    fn new(mass:u32, sugar:u32) -> Carrot {
        Carrot{mass, sugar}
    }
}

trait Tester {
    fn test(&self, carrot:&Carrot) -> bool;
}

fn new_tester(sugar_std:u32, accepted_error:u32) -> impl Tester {
    TesterImpl::new(sugar_std, accepted_error)
}

struct TesterImpl {
    sugar_std: u32,
    accepted_error: u32,
}

impl TesterImpl {
    fn new(sugar_std:u32, accepted_error:u32) -> TesterImpl {
        TesterImpl{sugar_std, accepted_error}
    }
}

impl Tester for TesterImpl {
    fn test(&self, carrot:&Carrot) -> bool{
        (self.sugar_std - self.accepted_error < carrot.sugar) && (carrot.sugar < self.sugar_std + self.accepted_error)
    }
}

fn select(tester:impl Tester, carrots:Vec<Carrot>) -> Option<usize>{
    let mut max_mass = 0;
    let mut max_index = 0;
    for i in 0..carrots.len() {
        let carrot = &carrots[i];
        if tester.test(carrot) {
            if carrot.mass > max_mass {
                max_index = i;
                max_mass = carrot.mass;
            }
        }
    }
    if max_mass == 0 {
        return None;
    } else {
        return Some(max_index);
    }
}

#[cfg(test)]
mod tests {
    use super::Tester;
    use super::Carrot;

    #[test]
    fn test_test(){
        let tester = super::new_tester(5, 2);
        assert_eq!(false, tester.test(&Carrot::new(8, 10)));
        assert_eq!(true, tester.test(&Carrot::new(7, 6)));
        assert_eq!(true, tester.test(&Carrot::new(7, 4)));
    }

    #[test]
    fn test_select0(){
        let tester = super::new_tester(5, 2);
        let carrots = vec![
            Carrot::new(8, 10),
            Carrot::new(7, 6),
            Carrot::new(7, 4),
        ];
        let want = 1;
        let got = super::select(tester, carrots).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test_select1(){
        let tester = super::new_tester(64, 10);
        let carrots = vec![
            Carrot::new(84, 75),
            Carrot::new(73, 53),
            Carrot::new(56, 34),
        ];
        let want = None;
        let got = super::select(tester, carrots);
        assert_eq!(want, got);
    }

}