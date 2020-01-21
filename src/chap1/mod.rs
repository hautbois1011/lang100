use std::collections::HashMap;
use std::fmt::Display;

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

/// 文字列を単語に分け、vに対応した文字数だけの頭文字のハッシュを返します。
pub fn hash_of_elements(s: &str, v: &Vec<usize>)
    -> HashMap<String, usize> {

    let mut ret = HashMap::new();
    let it = s.split_whitespace().zip(v.iter());
    it.enumerate().for_each(|(index, (word, &i))| {
        ret.insert(word[..i].to_string(), index + 1);
    });

    ret
}

/// 文字のn-gramを作ります。
pub fn char_n_gram(s: &str, n: usize) -> Vec<String> {
    let l = s.len() - n + 1;
    let mut ret = Vec::new();
    for i in 0..l {
        ret.push(s[i..(i + n)].to_string());
    }

    ret
}

/// 単語のn-gramを作ります。
pub fn word_n_gram(s: &str, n: usize)
    -> Vec<Vec<String>> {

    let it = s.split_whitespace().collect::<Vec<_>>();
    let l = it.len() - n + 1;
    let mut ret = Vec::new();
    for i in 0..l {
        let mut n_gram = Vec::new();
        for j in 0..n {
            n_gram.push(it[i + j].to_string());
        }
        ret.push(n_gram);
    }

    ret
}

/// テンプレート文を生成します。
pub fn template<X, Y, Z>(x: X, y: Y, z:Z) -> String
    where X: Display, Y: Display, Z: Display {

    format!("{}時の{}は{}", x, y, z)
}

pub fn encryption(s: &str) -> String {
    s.chars().map(|c|
        if c.is_ascii_lowercase() {
            (219 - c as u8) as char
        } else {
            c
        }
    ).collect()
}
