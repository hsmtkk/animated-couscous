fn calculate(hit:Vec<Vec<bool>>, score:Vec<Vec<u32>>) -> u32 {
    let mut result = 0;
    let row = hit.len();
    let column = hit[0].len();
    for i in 0..row {
        for j in 0..column {
            if hit[i][j] {
                result += score[i][j];
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test(){
        let hit = vec![
            vec![true, false ,true],
            vec![true, true, false],
            vec![true, false, true],
            vec![false, false, false],
        ];
        let score = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
            vec![10,11,12],
        ];
        let want = 29;
        let got = super::calculate(hit, score);
        assert_eq!(want, got);
    }
}