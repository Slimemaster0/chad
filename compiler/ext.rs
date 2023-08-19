pub fn string_is_numeric(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_numeric() { return false; }
    }
    return true;
}
