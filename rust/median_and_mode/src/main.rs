use std::collections::HashMap;

fn main() {
    println!("Median and Mode");
    let vec = vec![1, 2, 8, 93, 91, -1, 2, -11, -111, 93, -65, -594, -2874, -483, 283, 988, 999];
    println!("Our vector: {:?}", vec);
    let median = find_median(&vec);
    let mode = find_mode(&vec);
    println!("Median of our vector: {}", median);
    println!("Mode of our vector: {}", mode);
}

fn find_median(nums: &Vec<i32>) -> f64 {
    let n = nums.len();
    if n == 0 {
        return 0.0;
    }
    if n == 1 {
        return nums[0] as f64;
    }

    let mut nums = nums.clone();
    nums.sort();
    let mid = n / 2;
    if n % 2 != 0 {
        nums[mid] as f64
    } else {
        let (a, b) = (nums[mid - 1], nums[mid]);
        (a as f64 + b as f64) / 2f64
    }
}

fn find_mode(nums: &Vec<i32>) -> i32 {
    let mut frequencies: HashMap<i32, i32> = HashMap::new();
    for n in nums {
        let count = match frequencies.get(n).copied() {
            Some(i) => i + 1,
            None => 1,
        };
        frequencies.insert(*n, count);
    }
    let mut max_num = -1;
    let mut max_freq = -1;
    for (num, freq) in frequencies {
        if freq > max_freq {
            max_freq = freq;
            max_num = num;
        }
    }
    max_num
}

// Extra practice with chapter 8 concepts

fn find_mode2(nums: &[i32]) -> i32 {
    let mut frequencies: HashMap<i32, i32> = HashMap::new();
    for n in nums.iter().copied() {
        *(frequencies.entry(n).or_insert(0)) += 1;
    }
    let mut max_num = -1;
    let mut max_freq = -1;
    for (num, freq) in frequencies {
        if freq > max_freq {
            max_freq = freq;
            max_num = num;
        }
    }
    max_num
}

fn word_freq(input_words: &str) -> HashMap<String, u32> {
    let mut freq = HashMap::new();
    for word in input_words.split_whitespace() {
        *freq.entry(word.to_string()).or_insert(0) += 1;
    }
    freq
}

#[derive(PartialEq, Eq, Hash)]
enum Parity {
    Even,
    Odd
}

fn group_even_odd(nums: &[i32]) -> HashMap<Parity, Vec<i32>> {
    let mut parities = HashMap::new();
    for n in nums.iter().copied() {
        let key = if n % 2 == 0 { Parity::Even } else { Parity::Odd };
        parities.entry(key).or_insert(Vec::new()).push(n);
    }
    parities
}