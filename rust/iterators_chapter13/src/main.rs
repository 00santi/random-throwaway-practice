fn main() {
    let vec: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4, 5], vec![]];
    let vec: Vec<i32> = vec.into_iter()
        .flatten().map(|x| x * 2).collect();
    let target = vec![2, 4, 6, 8, 10];
    println!("{}", vec == target);

}

fn count_pos(nums: &[i32]) -> usize {
    nums.iter()
        .filter(|&&x| x > 0)
        .count()
}

fn count_pos_fold(nums: &[i32]) -> usize {
    nums.iter().fold(0, |acc, &el| {
        if el > 0 {
            acc + 1
        } else {
            acc
        }
    })
}

fn sum_even(nums: &[i32]) -> i32 {
    nums.iter().copied()
        .filter(|n| n % 2 == 0)
        .sum()
}

fn _sum_even(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for n in nums {
        if n % 2 == 0 {
            sum += n;
        }
    }
    sum
}

fn _sum_of_squares_of_even(nums: &[i32]) -> i32 {
    let mut evens = vec![];
    for n in nums.iter().copied() {
        if n % 2 == 0 {
            evens.push(n);
        }
    }

    let mut squares = vec![];
    for n in evens {
        squares.push(n * n);
    }

    let mut sum = 0;
    for n in squares {
        sum += n
    }

    sum
}

fn sum_of_squares_of_even(nums: &[i32]) -> i32 {
    nums.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .sum()
}

fn sum_of_squares_of_even_fold(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &el| {
        if el % 2 == 0 {
            acc + el * el
        } else {
            acc
        }
    })
}

fn first_long_word(words: Vec<String>) -> Option<String> {
    words.into_iter().find(|w| w.len() > 5)
}

fn count_gt10_odd(nums: &[i32]) -> usize {
    nums.iter()
        .filter(|&&x| x > 10 && x % 2 != 0)
        .count()
}

fn indexed_sum(nums: &[i32]) -> i32 {
    // sum of (index * value)
    // [10, 20, 30] → 0*10 + 1*20 + 2*30 = 80
    nums.iter().copied()
        .enumerate()
        .map(|(i, e)| i as i32 * e)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sum_even_test() {
        let vec = Vec::from([1, 9, 3, 11, 39, 349]);
        assert_eq!(sum_even(&vec), _sum_even(&vec));
        assert_eq!(sum_even(&vec), 0);

        let vec = Vec::from([2, 192, 3, 11, 38, 4]);
        assert_eq!(sum_even(&vec), _sum_even(&vec));
        assert_eq!(sum_even(&vec), 2 + 192 + 38 + 4);
    }
}