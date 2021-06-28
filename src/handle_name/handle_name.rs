fn handle_name(name:&str) -> String{
    let mut result: String = name.to_string();
    let boin = vec!["a", "i", "u", "e", "o", "A", "I", "U", "E", "O"];
    for b in boin {
        result = result.replace(b, "");
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    fn test_handle_name(){
        let mut want_orig = HashMap::new();
        want_orig.insert("Trvlds", "Torvalds");
        want_orig.insert("PZ", "PAIZA");
        for (want, orig) in want_orig {
            let got = super::handle_name(orig);
            assert_eq!(want,got);
        }
    }
}