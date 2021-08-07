fn calc_tax(income:u32) -> u32 {
    return (income as f32 * get_rate(income)) as u32;
}

fn get_rate(income:u32) -> f32 {
    if 0 <= income && income < 100001 {
        return 0.0;
    } else if 100001 <= income && income < 750001 {
        return 0.1;
    } else if 750001 <= income && income < 1500001 {
        return 0.2;
    } else {
        return 0.4;
    }
}

fn sum(ns:Vec<u32>) -> u32 {
    let mut s = 0;
    for n in ns {
        s += n;
    }
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0(){
        let incomes = vec![450000, 200000, 100, 2000000, 1234567];
        let mut taxes = Vec::new();
        for income in incomes {
            taxes.push(super::calc_tax(income));
        }
        let want = 621913;
        let got = super::sum(taxes);
        assert_eq!(want, got);
    }
}