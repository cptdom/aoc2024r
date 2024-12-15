pub fn check_array_safe(line: String) -> bool {
    let split_data = line.split_whitespace();
    let mut last_digit: usize = 0;
    let mut is_decreasing = false;
    let cnt = split_data.clone().count();
    for (i, digit) in split_data.enumerate() {
        let digit: usize = digit.parse().unwrap();
        if i == 0 {
            last_digit = digit;
            continue
        }
        if digit == last_digit {
            return false
        }
        if i == 1 && digit < last_digit {
            is_decreasing = true;
        }
        if i > 1 && (digit < last_digit) != is_decreasing {
            return false
        }
        let diff = last_digit.abs_diff(digit);
        if diff >= 1 && diff <= 3 {
            last_digit = digit;
            if i == cnt-1 {
                return true
            }
            continue
        } else {
            return false
        }
    }
    return false
}