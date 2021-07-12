struct Box {
    height: u32,
    width: u32,
    depth : u32,
}

impl Box {
    fn new(height:u32, width:u32, depth:u32) -> Box {
        Box{height,width,depth}
    }

    fn can_put_ball(&self, radius:u32) -> bool {
        let r2 = radius * 2;
        r2 <= self.height && r2 <= self.width && r2 <= self.depth
    }
}

fn solve(radius:u32, boxes:Vec<Box>) -> Vec<usize> {
    let mut results = Vec::new();
    for i in 0..boxes.len(){
        let b = &boxes[i];
        if b.can_put_ball(radius){
            results.push(i);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::Box;
    #[test]
    fn test0(){
        let radius = 2;
        let boxes = vec![
            Box::new(6,6,6),
            Box::new(4,6,4),
            Box::new(6,1,1),
            Box::new(4,4,4),
        ];
        let want = vec![0,1,3];
        let got = super::solve(radius, boxes);
        assert_eq!(want, got);
    }

    #[test]
    fn test1(){
        let radius = 5;
        let boxes = vec![
            Box::new(2,184, 12),
            Box::new(135, 169, 37),
            Box::new(99,121,41),
        ];
        let want = vec![1,2];
        let got = super::solve(radius, boxes);
        assert_eq!(want,got);
    }

    fn test2(){
        let radius = 7;
        let boxes = vec![
            Box::new(197, 78, 14),
            Box::new(14, 197, 62),
            Box::new(80, 14, 109),
            Box::new(138, 145, 147),
            Box::new(9, 130, 175),
        ];
        let want = vec![0,1,2,3];
        let got = super::solve(radius, boxes);
        assert_eq!(want,got);
    }
}