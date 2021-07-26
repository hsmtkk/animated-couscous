#[derive(Debug)]
struct Answer {
    m: u32,
    n: u32,
    p: u32,
}

impl Answer {
    fn new(m:u32, n:u32, p:u32) -> Answer {
        Answer{m, n, p}
    }
}

fn solve() -> Vec<Answer>{
    let mut answers = Vec::new();
    for m in 1..30 {
        for n in 1..30 {
            let p2 = m * m + n * n;
            if let Some(p) = is_square_number(p2) {
                let ans = Answer::new(m, n, p);
                answers.push(ans);
            }
        }
    }
    answers
}

fn is_square_number(n:u32) -> Option<u32> {
    for i in 1..n {
        if n == i * i {
            return Some(i)
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve(){
        let answers = super::solve();
        println!("{:?}", answers);
    }
}