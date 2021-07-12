fn find_top(map: Vec<Vec<i32>>) -> Vec<i32> {
    let mut results = Vec::new();
    let rows = map.len();
    let columns = map[0].len();
    for i in 0..rows {
        for j in 0..columns {
            if is_top(&map, i, j) {
                results.push(map[i][j])
            }
        }
    }
    results.sort();
    results
}

fn is_top(map:&Vec<Vec<i32>>, i:usize, j:usize) -> bool {
    let this_val = map[i][j];
    if let Some(up) = get_if_exist(map, i-1, j) {
        if this_val <= up {
            return false;
        }
    }
    if let Some(down) = get_if_exist(map, i+1, j){
        if this_val <= down {
            return false;
        }
    }
    if let Some(left) = get_if_exist(map, i, j-1){
        if this_val <= left {
            return false;
        }
    }
    if let Some(right) = get_if_exist(map, i, j+1){
        if this_val <= right {
            return false;
        }
    }
    true
}

fn get_if_exist(map:&Vec<Vec<i32>>, i:usize, j:usize) -> i32 {
    if let Some(row) = map.get(i) {
        if let Some(v) = row.get(j) {
            return *v;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        let input = vec![vec![90, 10, 10], vec![10, 30, 20], vec![10, 10, 20]];
        let want = vec![90, 30];
        let got = super::find_top(input);
        assert_eq!(want, got);
    }

    fn test1() {
        let input = vec![
            vec![50, 20, 50, 20],
            vec![20, 50, 20, 50],
            vec![40, 20, 40, 20],
            vec![30, 30, 30, 30],
        ];
        let want = vec![50, 50, 50, 50, 40, 40];
        let got = super::find_top(input);
        assert_eq!(want, got);
    }
}
