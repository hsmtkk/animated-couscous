#[derive(Debug, PartialEq)]
enum Attendee {
    AtA,
    AtB,
}

trait Auctioner {
    fn execute(&self) -> (Attendee, u32);
}

struct AuctionerImpl {
    init_price: u32,
    a_limit: u32,
    b_limit: u32,
}

impl AuctionerImpl{
    fn new(init_price:u32, a_limit:u32, b_limit:u32) -> AuctionerImpl {
        AuctionerImpl{init_price, a_limit, b_limit}
    }
}

fn new(init_price:u32, a_limit:u32, b_limit:u32) -> impl Auctioner {
    AuctionerImpl::new(init_price, a_limit, b_limit)
}

impl Auctioner for AuctionerImpl {
    fn execute(&self) -> (Attendee, u32){
        let mut price = self.init_price;
        let mut i = 0;
        loop {
            if i % 2 == 0 {
                let next_price = price + 10;
                if next_price < self.a_limit {
                    price = next_price;
                } else {
                    return (Attendee::AtB, price);
                }
            } else {
                let next_price = price + 1000;
                if next_price < self.b_limit {
                    price = next_price;
                } else {
                    return (Attendee::AtA, price)
                }
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Auctioner;
    #[test]
    fn test(){
        let auctioner = super::new(1, 1500, 2050);
        let (winner, price) = auctioner.execute();
        assert_eq!(winner, super::Attendee::AtB);
        assert_eq!(price, 2021);
    }
}
