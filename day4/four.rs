
fn main() {

    let mut valid_passwords: u64 = 0;
    for password in 171309..=643603 {
        let digits = number_into_digits(password);

        if two_adjacent_same(digits) && is_monotone(digits) {
            valid_passwords += 1;
        }
    }

    println!("Valid passwords in the range: {}", valid_passwords);
}

fn number_into_digits(number: u64) -> [u8; 6] {
    let digits: [u8; 6] = [
        ((number / 100000) % 10) as u8,
        ((number / 10000) % 10) as u8,
        ((number / 1000) % 10) as u8,
        ((number / 100) % 10) as u8,
        ((number / 10) % 10) as u8,
        (number % 10) as u8,
    ];

    return digits;
}

fn two_adjacent_same(password: [u8; 6]) -> bool {
    for number in 0..=9 {
        if two_adjacent_same_number(password, number) {
            return true;
        }
    }

    return false;
}

fn two_adjacent_same_number(password: [u8; 6], num: u8) -> bool {
    return (password[0] == num && password[1] == num && password[2] != num) ||
           (password[0] != num && password[1] == num && password[2] == num && password[3] != num) ||
           (password[1] != num && password[2] == num && password[3] == num && password[4] != num) ||
           (password[2] != num && password[3] == num && password[4] == num && password[5] != num) ||
           (password[3] != num && password[4] == num && password[5] == num);
}

fn is_monotone(password: [u8; 6]) -> bool {
    return password[0] <= password[1] &&
           password[1] <= password[2] &&
           password[2] <= password[3] &&
           password[3] <= password[4] &&
           password[4] <= password[5];
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_number_into_digits() {
        assert_eq!(number_into_digits(123456), [1, 2, 3, 4, 5, 6]);
        assert_eq!(number_into_digits(532355), [5, 3, 2, 3, 5, 5]);
        assert_eq!(number_into_digits(12), [0, 0, 0, 0, 1, 2]);
    }

    #[test]
    fn test_adjacent_same() {

        assert_eq!(two_adjacent_same([1, 2, 3, 3, 4, 5]), true);
        assert_eq!(two_adjacent_same([1, 1, 2, 2, 3, 3]), true);
        assert_eq!(two_adjacent_same([1, 2, 3, 6, 4, 5]), false);
        assert_eq!(two_adjacent_same([1, 2, 3, 4, 4, 4]), false);
        assert_eq!(two_adjacent_same([1, 1, 1, 1, 2, 2]), true);
        assert_eq!(two_adjacent_same([3, 3, 3, 3, 3, 3]), false);
        assert_eq!(two_adjacent_same([9, 9, 9, 1, 1, 9]), true);
        assert_eq!(two_adjacent_same([3, 3, 3, 3, 1, 3]), false);
        assert_eq!(two_adjacent_same([0, 3, 0, 3, 3, 3]), false);
    }
}