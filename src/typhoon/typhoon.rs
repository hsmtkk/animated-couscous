struct Typhoon {
    xc: i32,
    yc: i32,
    r1: i32,
    r2: i32,
}

impl Typhoon {
    fn new (xc:i32, yc:i32, r1:i32, r2:i32) -> Typhoon {
        Typhoon{xc, yc, r1, r2}
    }
}
#[derive(Hash, Eq, PartialEq)]
struct Person {
    x: i32,
    y: i32,
}

impl Person {
    fn new(x:i32, y:i32) -> Person {
        Person{x,y}
    }
}

fn pow(n:i32) -> i32 {
    n*n
}

fn in_typhoon(ty:&Typhoon, p:Person) -> bool {
    let r2 = pow(p.x - ty.xc) + pow(p.y - ty.yc);
    (pow(ty.r1) <= r2) && (r2 <= pow(ty.r2))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    fn test_in_typhoon(){
        let ty = super::Typhoon::new(0, 0, 1, 2);
        let mut person_want = HashMap::new();
        person_want.insert(super::Person::new(0,0), false);
        person_want.insert(super::Person::new(1,1), true);
        person_want.insert(super::Person::new(4,2), false);
        for (person, want) in person_want {
            let got = super::in_typhoon(&ty, person);
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_in_typhoon2(){
        let ty = super::Typhoon::new(47, 19, 57, 80);
        let mut person_want = HashMap::new();
        person_want.insert(super::Person::new(62,-52), true);
        person_want.insert(super::Person::new(35,-70), false);
        person_want.insert(super::Person::new(-81,2), false);
        for (person, want) in person_want {
            let got = super::in_typhoon(&ty, person);
            assert_eq!(want, got);
        }
    }

}