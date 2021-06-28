fn sum(array:&[i32]) -> i32 {
    let mut sum = 0;
    for i in array {
        sum += i;
    }
    sum
}

fn split_row(row:Vec<i32>) -> Option<i32> {
    for i in 1..row.len() {
        let left = &row[..i];
        let right = &row[i..];
        if sum(&left) == sum(&right){
            return Some(i as i32);
        }
    }
    None
}

fn choco_split(choco: Vec<Vec<i32>>) -> Option<Vec<i32>> {
    let mut result = Vec::new();
    for row in choco {
        if let Some(i) = split_row(row){
            result.push(i);
        } else {
            return None;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_choco_split() {
        let choco = vec![
            vec![3, 7, 4, 5, 1],
            vec![6, 9, 1, 8, 8],
            vec![11, 2, 5, 2, 2],
        ];
        let want = vec![2, 3, 1];
        let got = super::choco_split(choco).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test_choco_split2() {
        let choco = vec![
            vec![18, 40, 22, 16],
            vec![2, 38, 10, 10],
            vec![38, 18, 8, 36],
            vec![6, 18, 24, 34],
        ];
        let got = super::choco_split(choco);
        assert_eq!(None, got);
    }

    #[test]
    fn test_split_row(){
        let row = vec![3,7,4,5,1];
        let got = super::split_row(row).unwrap();
        assert_eq!(2, got);
        let row = vec![6,9,1,8,8];
        let got = super::split_row(row).unwrap();
        assert_eq!(3, got);
        let row = vec![18, 40, 22, 16];
        let got = super::split_row(row);
        assert_eq!(None, got);
    }
}
