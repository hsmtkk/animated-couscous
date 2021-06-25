use anyhow::{bail, Result};

fn split_token(formula:&str) -> Vec<&str> {
    formula.split('+').collect()
}

fn parse_token(token:&str) -> Result<i32> {
    let mut val = 0;
    for ch in token.chars(){
        match ch {
            '<' => { val += 10 },
            '/' => { val += 1 },
            _ => bail!("unknown character {}", ch),
        }
    }
    Ok(val)
}

fn eval(formula:&str) -> Result<i32>{
    let tokens = split_token(formula);
    let mut answer = 0;
    for t in tokens {
        answer += parse_token(t)?;
    }
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    fn test_split_token() {
        let mut formula_want = HashMap::new();
        formula_want.insert("///+////", vec!["///", "////"]);
        formula_want.insert("//+//////", vec!["//", "//////"]);
        formula_want.insert("<///////+<<</+////", vec!["<///////", "<<</", "////"]);
        for (formula, want) in formula_want {
            let got = super::split_token(formula);
            assert_eq!(want, got);
        }
    }

    fn test_parse_token(){
        let mut token_value = HashMap::new();
        token_value.insert("<///////", 17);
        token_value.insert("<<</", 31);
        token_value.insert("////", 4);
        for (token, value) in token_value {
            let got = super::parse_token(token).unwrap();
            assert_eq!(value, got);
        }
    }

    fn test_eval(){
        let mut formula_answer = HashMap::new();
        formula_answer.insert("///+////", 7);
        formula_answer.insert("<///////+<<</+////", 52);
        formula_answer.insert("<<<<<<<<</////////+<<<<<<<<</////////", 198);
        for (formula, answer) in formula_answer{
            let got = super::eval(formula).unwrap();
            assert_eq!(answer, got);
        }
    }
}