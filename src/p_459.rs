impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        s.repeat(2)[1..s.len()*2-1].contains(&s)
    }
}
