use std::{cmp::min, collections::HashMap};

fn get_char_counts(s: String) -> HashMap<char, i32> {
    let mut char_counts = HashMap::new();

    for c in s.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    char_counts
}

fn solution(s1: String, s2: String) -> i32 {
    let count1 = get_char_counts(s1);
    let count2 = get_char_counts(s2);
    let mut sum = 0;

    for (c, count) in count1.iter() {
        sum += min(count, count2.get(c).unwrap_or(&0));
    }

    sum
}

#[cfg(test)]
mod challenges {
    #[test]
    fn common_character_count() {
        assert_eq!(crate::solution("aabcc".to_string(), "adcaa".to_string()), 3);
    }
}
