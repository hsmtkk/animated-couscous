const MARK: &str = "+";
const NEW_LINE: &str = "\n";

fn deco_string(s: &str) -> String {
    let mut result = String::new();
    result += &MARK.repeat(s.len() + 2);
    result += NEW_LINE;
    result += MARK;
    result += s;
    result += MARK;
    result += NEW_LINE;
    result += &MARK.repeat(s.len() + 2);
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deco_string() {
        let want = r#"+++++++
+Paiza+
+++++++"#;
        let got = super::deco_string("Paiza");
        assert_eq!(want, got);
    }

    #[test]
    fn test_deco_string2() {
        let want = r#"+++++++++++
+TicTacToe+
+++++++++++"#;
        let got = super::deco_string("TicTacToe");
        assert_eq!(want, got);
    }
}
