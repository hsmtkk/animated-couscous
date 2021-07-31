trait Categorizer {
    fn categorize(&self, weight: u32) -> u32;
}

struct CategorizerImpl {
    box_weight: u32,
}

impl CategorizerImpl {
    fn new(box_weight: u32) -> CategorizerImpl {
        CategorizerImpl { box_weight }
    }
}

impl Categorizer for CategorizerImpl {
    fn categorize(&self, weight: u32) -> u32 {
        let x : f32 = weight as f32 / self.box_weight as f32;
        let n: u32 = x.round() as u32;
        match n {
            0 => self.box_weight,
            _ => { n * self.box_weight},
        }
    }
}

fn new(box_weight: u32) -> impl Categorizer {
    CategorizerImpl::new(box_weight)
}

#[cfg(test)]
mod tests {
    use super::Categorizer;

    #[test]
    fn test_solve0() {
        let cat = super::new(10);
        assert_eq!(20, cat.categorize(24));
        assert_eq!(40, cat.categorize(35));
        assert_eq!(10, cat.categorize(3));
    }

    #[test]
    fn test_solve1() {
        let cat = super::new(50);
        assert_eq!(50, cat.categorize(40));
        assert_eq!(100, cat.categorize(90));
        assert_eq!(50, cat.categorize(10));
    }

    #[test]
    fn test_solve2() {
        let cat = super::new(3);
        assert_eq!(9, cat.categorize(9));
        assert_eq!(6, cat.categorize(5));
    }
}
