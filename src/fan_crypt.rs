use std::collections::HashMap;

trait Decryptor {
    fn decrypt(&self, encrypted:&str) -> String;
}

fn new(rule:&str, count:i32) -> impl Decryptor {
    DecryptorImpl::new(rule, count)
}

struct DecryptorImpl {
    rule: HashMap<char, char>,
    count: i32,
}

const Alphas:&str = "abcdefghijklmnopqrstuvwxyz";

impl DecryptorImpl {
    fn new(rule_str:&str, count:i32) -> DecryptorImpl {
        let mut rule: HashMap<char, char> = HashMap::new();
        for (c0, c1) in rule_str.chars().zip(Alphas.chars()) {
            rule.insert(c0, c1);
        }
        DecryptorImpl{rule, count}
    }

    fn decrypt_once(&self, encrypted:&str) -> String {
        let mut result = String::new();
        for ch in encrypted.chars(){
            let c = match ch {
                ' ' => ' ',
                _ => *self.rule.get(&ch).unwrap(),
            };
            result.push(c);
        }
        result
    }
}

impl Decryptor for DecryptorImpl {
    fn decrypt(&self, encrypted:&str) -> String {
        let mut s: String = encrypted.to_string();
        for _i in 0..self.count {
            let t = self.decrypt_once(&s);
            s = t;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::fan_crypt::Decryptor;

    #[test]
    fn test_crypt1(){
        let dec = super::new("qwertyuiopasdfghjklzxcvbnm", 1);
        let input = "hqomq gfsoft iqeaqzigf";
        let want = "paiza online hackathon";
        let got = dec.decrypt(input);
        assert_eq!(want, got);        
    }

    #[test]
    fn test_crypt2(){
        let dec = super::new("poiuytrewqlkjhgfdsamnbvcxz", 100);
        let input = "snn xufu ngebmv qwtg";
        let want = "cpp java python ruby";
        let got = dec.decrypt(input);
        assert_eq!(want, got);
    }
}