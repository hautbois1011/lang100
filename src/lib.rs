/// 文字列の逆順を返します。
pub fn reverse_str(s: &str) -> String {
    s.chars().rev().collect()
}

/// step毎に文字を抜き出します。
pub fn step_str(s: &str, step: usize) -> String {
    s.chars().step_by(step).collect()
}

/// s1とs2の文字を交互に連結します。
pub fn alternate_str(s1: &str, s2: &str) -> String {
    let mut ret = String::new();
    s1.chars().zip(s2.chars()).for_each(|(c1, c2)| {
        ret.push(c1);
        ret.push(c2);
    });

    ret
}

/// 文字列を単語に分け、文字数を単語毎に格納したベクトルを返します。
pub fn list_of_len_of_words(s: &str) -> Vec<usize> {
    s.chars()
        .filter(|&c| c != '.')
        .filter(|&c| c != ',')
        .collect::<String>()
        .split_whitespace()
        .map(|word| word.len())
        .collect::<Vec<usize>>()
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

    #[test]
    fn alternate_str_test() {
        assert_eq!("パタトクカシーー".to_string(), alternate_str("パトカー", "タクシー"))
    }

    #[test]
    fn list_of_len_of_words_test() {
        let text = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
        println!("{:?}", list_of_len_of_words(&text));
    }

}
