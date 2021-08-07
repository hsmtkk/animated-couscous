use std::collections::HashMap;

trait Inputter {
    fn input(&mut self, s:&str) -> String;
}

struct Keyboard {
    char_limit: HashMap<char, u32>,
    char_counter: HashMap<char, u32>,
}

impl Keyboard {
    fn new(limits:Vec<u32>) -> Keyboard {
        let mut char_limit = HashMap::new();
        let mut char_counter = HashMap::new();
        let alphas = "abcdefghijklmnopqrstuvwxyz";
        for i in 0..26 {
            char_limit.insert(alphas.chars().nth(i).unwrap(), limits[i]);
            char_counter.insert(alphas.chars().nth(i).unwrap(), 0);
        }
        Keyboard{char_limit, char_counter}
    }
}

impl Inputter for Keyboard {
    fn input(&mut self, s:&str) -> String{
        let mut result = String::new();
        for ch in s.chars() {
            let count = self.char_counter.get(&ch).unwrap();
            let limit = self.char_limit.get(&ch).unwrap();
            if count < limit {
                result.push(ch);
            }
            self.char_counter.insert(ch, count + 1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Inputter;
    #[test]
    fn test0(){
        let limits = vec![1,3,2,6,3,5,5,6,2,6,0,4,5,2,4,2,1,2,4,0,4,2,2,5,0,2];
        let mut kb = super::Keyboard::new(limits);
        let input = "abcabcabcabc";
        let want = "abcbcb";
        let got = kb.input(input);
        assert_eq!(want, got);
    }
}
