/// 文字列の逆順を返します。
pub fn reverse_str(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn step_str(s: &str, step: usize) -> String {
    s.chars().step_by(step).collect()
}

#[cfg(test)]
mod chap1_tests {
    use super::*;

    #[test]
    fn reverse_str_test() {
        assert_eq!("desserts".to_string(), reverse_str("stressed"));
    }

    #[test]
    fn step_str_test() {
        assert_eq!("パトカー".to_string(), step_str("パタトクカシーー", 2))
    }

}
