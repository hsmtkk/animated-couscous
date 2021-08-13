trait Decryptor {
    fn decrypt(&self, encrypted:&str) -> String;
}

fn new(shift:u32) -> impl Decryptor {
    DecryptorImpl::new(shift)
}

struct DecryptorImpl {
    shift: u32,
}

impl DecryptorImpl {
    fn new(shift:u32) -> DecryptorImpl{
        DecryptorImpl{shift}
    }

    fn forward(&self, ch: char) -> char {
        'A'
    }

    fn backward(&self, ch:char) -> char {
        'B'
    }
}

impl Decryptor for DecryptorImpl {
    fn decrypt(&self, encrypted:&str) -> String{
        let result = String::new();
        for i in 0..encrypted.len() {
            if i % 2 == 0 {
                result += self.backward(encrypted[i]);
            } else {
                result += self.forward(encrypted[i]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tets {
    use super::Decryptor;
    #[test]
    fn test0(){
        let decryptor = super::new(4);
        let want = "MILK";
        let got = decryptor.decrypt("QEPG");
        assert_eq!(want, got);
    }

    #[test]
    fn test1(){
        let decryptor = super::new(19);
        let want = "RQRITJCB";
        let got = decryptor.decrypt("KXKPMQVI");
        assert_eq!(want, got);
    }
}