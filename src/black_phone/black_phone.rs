fn distance(digit:u32) -> u32 {
    match digit {
        0 => 12,
        _ => digit + 2,
    }
}

fn sum_distance(digits: Vec::<u32>) -> u32 {
    let mut sum = 0;
    for d in digits {
        sum += distance(d);
    }
    sum * 2
}

fn eval(numbers:&str) -> u32{
    let mut digits = Vec::new();
    for n in numbers.chars(){
        match n {
            '-' => {},
            _ => {
                let d = n.to_digit(10).unwrap();
                digits.push(d);
            },
        }
    }
    sum_distance(digits)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_distance(){
        assert_eq!(12, super::distance(0));
        assert_eq!(5, super::distance(3));
        assert_eq!(9, super::distance(7));
    }

    #[test]
    fn test_sum_distance(){
        let want = 146;
        let got = super::sum_distance(vec![9,3,1,5,3,5,7,3,9,8]);
        assert_eq!(want, got);
    }

    #[test]
    fn test_eval(){
        let want = 134;
        let got = super::eval("043-2303-3412");
        assert_eq!(want, got);
    }
}