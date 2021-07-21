trait Drawer {
    fn draw(&self, sale:u32) -> String;
}

fn new(div:u32, width:usize) -> impl Drawer {
    DrawerImpl::new(div, width)
}

struct DrawerImpl {
    div: u32,
    width: usize,
}

impl DrawerImpl {
    fn new(div:u32, width:usize) -> DrawerImpl {
        DrawerImpl{div, width}
    }
}

impl Drawer for DrawerImpl {
    fn draw(&self, sale:u32) -> String {
        let marks: usize = (sale / self.div) as usize;
        "*".repeat(marks) + &".".repeat(self.width - marks)
    }
}

#[cfg(test)]
mod tests {
    use super::Drawer;
    #[test]
    fn test0(){
        let drawer = super::new(5, 3);
        assert_eq!("*..", drawer.draw(5));
        assert_eq!("***", drawer.draw(15));
        assert_eq!("**.", drawer.draw(10));
    }

    #[test]
    fn test1(){
        let drawer = super::new(2, 8);
        assert_eq!("******..", drawer.draw(12));
        assert_eq!("****....", drawer.draw(8));
        assert_eq!("********", drawer.draw(16));
        assert_eq!("*****...", drawer.draw(10));
    }
}
