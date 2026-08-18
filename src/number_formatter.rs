const MAX_DIGITS: usize = 7; // increase to 8 if 10M entries are used

pub fn format_n(n: usize) -> String {
    let s = n.to_string();
    let len = s.len();
    let mut formatted = String::with_capacity(len + (len.saturating_sub(1) / 3));
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            formatted.push('_');
        }
        formatted.push(ch);
    }
    let underscores = if MAX_DIGITS > 0 {
        (MAX_DIGITS - 1) / 3
    } else {
        0
    };
    let target_width = MAX_DIGITS + underscores;
    format!("{:>width$}", formatted, width = target_width)
}
