pub fn is_number(chars: String) -> bool {
    for c in chars.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}