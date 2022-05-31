impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n).map(|i| 
            if i % 15 == 0 {
                "FizzBuzz".into()
            } else if i % 3 == 0 {
                "Fizz".into()
            } else if i % 5 == 0 {
                "Buzz".into()
            } else {
                i.to_string()
            }
        ).collect()
    }
}
