pub mod chap1;

#[cfg(test)]
mod chap1_tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    #[ignore]
    fn reverse_str_test() {
        assert_eq!("desserts".to_string(), chap1::reverse_str("stressed"));
    }

    #[test]
    #[ignore]
    fn step_str_test() {
        assert_eq!("パトカー".to_string(), chap1::step_str("パタトクカシーー", 2))
    }

    #[test]
    #[ignore]
    fn alternate_str_test() {
        assert_eq!("パタトクカシーー".to_string(), chap1::alternate_str("パトカー", "タクシー"))
    }

    #[test]
    #[ignore]
    fn list_of_len_of_words_test() {
        let text = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
        println!("{:?}", chap1::list_of_len_of_words(&text));
    }

    #[test]
    #[ignore]
    fn hash_of_elements_test() {
        let text = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
        let v = vec![1, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 2, 2, 1, 2];

        println!("{:?}", chap1::hash_of_elements(&text, &v));
    } 

    #[test]
    #[ignore]
    fn n_gram_test() {
        let text = "I am an NLPer";
        println!("{:?}", chap1::char_n_gram(&text, 2));
        println!("{:?}", chap1::word_n_gram(&text, 2));
    }

    #[test]
    #[ignore]
    fn set_test() {
        let a: HashSet<_> = chap1::char_n_gram("paraparaparadise", 2)
            .into_iter().collect::<HashSet<_>>();
        let b: HashSet<_> = chap1::char_n_gram("paragraph", 2)
            .into_iter().collect::<HashSet<_>>();

        println!("union: {:?}", &a | &b);
        println!("intersection: {:?}", &a & &b);
        println!("differrence: {:?}", &a - &b);
        println!("is 'se' in a: {:?}", a.contains(&"se".to_string()));
        println!("is 'se' in b: {:?}", b.contains(&"se".to_string()));
    }

}
