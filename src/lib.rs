/// 文字列の逆順を返します。
pub fn reverse_str(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod chap1_tests {
    use super::*;

    #[test]
    fn reverse_str_test() {
        assert_eq!("desserts".to_string(), reverse_str("stressed"));
    }

}
