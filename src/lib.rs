pub mod chap1;

#[cfg(test)]
mod chap1_tests {
    use super::*;

    #[test]
    fn reverse_str_test() {
        assert_eq!("desserts".to_string(), chap1::reverse_str("stressed"));
    }

    #[test]
    fn step_str_test() {
        assert_eq!("パトカー".to_string(), chap1::step_str("パタトクカシーー", 2))
    }

    #[test]
    fn alternate_str_test() {
        assert_eq!("パタトクカシーー".to_string(), chap1::alternate_str("パトカー", "タクシー"))
    }

    #[test]
    fn list_of_len_of_words_test() {
        let text = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
        println!("{:?}", chap1::list_of_len_of_words(&text));
    }

    #[test]
    fn hash_of_elements_test() {
        let text = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
        let v = vec![1, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 2, 2, 1, 2];

        println!("{:?}", chap1::hash_of_elements(&text, &v));
    } 

    #[test]
    fn n_gram_test() {
        let text = "I am an NLPer";
        println!("{:?}", chap1::char_n_gram(&text, 2));
        println!("{:?}", chap1::word_n_gram(&text, 2));
    }

}
