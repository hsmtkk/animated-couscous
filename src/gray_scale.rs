fn shrink_image(orig:Vec<Vec<i32>>, block_size:usize) -> Vec<Vec<i32>>{
    let mut results = Vec::new();
    for block in split_block(orig, block_size) {
        results.push(block_average(block));
    }
    results
}

fn split_block(orig:Vec<Vec<i32>>, block_size:usize) -> Vec<Vec<Vec<i32>>> {
    let mut results = Vec::new();
    let rows = orig.len();
    let columns = orig[0].len();
    for i in 0..(rows/block_size) {
        for j in 0..(columns/block_size) {
            let i2 = i * 3;
            let j2 = j * 3;
            results.push(get_block(&orig, block_size, i2, j2));
        }
    }
    results
}

fn get_block(orig:&Vec<Vec<i32>>, block_size:usize, i2:usize, j2:usize, ) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    for i in i2..i2+block_size {
        let mut row = Vec::new();
        for j in j2..j2+block_size {
            row.push(orig[i][j]);
        }
        results.push(row);
    }
    results
}

fn block_average(block:Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    let mut sum = 0;
    for row in block {
        for n in row {
            sum += n;
            count += 1;
        }
    }
    sum / count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0(){
        let orig = vec![
            vec![1,2,3,4,5,6],
            vec![1,2,3,4,5,6],
            vec![1,2,3,4,5,6],
            vec![4,5,6,1,2,3],
            vec![4,5,6,1,2,3],
            vec![4,5,6,1,2,3],
        ];
        let want = vec![
            vec![2,5],
            vec![5,2],
        ];
        let got = super::shrink_image(orig, 3);
        assert_eq!(want,got);
    }

    #[test]
    fn test_split_block(){
        let orig = vec![
            vec![1,2,3,4,5,6],
            vec![1,2,3,4,5,6],
            vec![1,2,3,4,5,6],
            vec![4,5,6,1,2,3],
            vec![4,5,6,1,2,3],
            vec![4,5,6,1,2,3],
        ];
        let mut want = Vec::new();
        want.push(vec![
            vec![1,2,3],
            vec![1,2,3],
            vec![1,2,3],
        ]);
        want.push(vec![
            vec![4,5,6],
            vec![4,5,6],
            vec![4,5,6],
        ]);
        want.push(vec![
            vec![4,5,6],
            vec![4,5,6],
            vec![4,5,6],
        ]);
        want.push(vec![
            vec![1,2,3],
            vec![1,2,3],
            vec![1,2,3],
        ]);
        let got = super::split_block(orig, 3);
        assert_eq!(want, got);
    }
}