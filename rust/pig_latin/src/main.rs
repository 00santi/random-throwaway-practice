fn main() {
    let word1 = "somewordinlatin";
    let word2 = "aaaasomewordinlatin";
    println!("{} -> {}", word1, to_pig_latin(word1));
    println!("{} -> {}", word2, to_pig_latin(word2));
}

fn to_pig_latin(word: &str) -> String {
    if word.is_empty() { return "".to_string(); }

    let first = word.chars().next().unwrap();
    if is_vowel(first) {
        return format!("{}-way", word);
    }

    let mut result = String::new();
    // let mut result: String = word.chars().skip(1).collect()
    for c in word.chars().skip(1) {
        result.push(c);
    }
    result.push('-');
    result.push(first);
    result.push_str("ay");
    result
}

fn sentence_to_pig_latin(sentence: &str) -> String {
    let mut result = String::new();

    for word in sentence.split_whitespace() {
        result.push_str(&to_pig_latin(word));
        result.push(' ');
    }

    result
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}