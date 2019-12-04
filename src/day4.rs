const INPUT : std::ops::Range<usize> = 265275..781584;

pub fn digits_are_increasing(digits: &Vec<usize>) -> bool {
    digits
        .iter()
        .enumerate()
        .skip(1)
        .all(|(index, &digit)| digit >= digits[index - 1])
}

pub fn digits_contain_repeated_value(digits: &Vec<usize>) -> bool {
    digits
        .iter()
        .enumerate()
        .skip(1)
        .any(|(index, &digit)| digit == digits[index - 1])
}

pub fn digits_contain_repeated_value_exactly_once(digits: &Vec<usize>) -> bool {
    digits
        .iter()
        .fold(vec![0; 10], |mut acc, &d|{
            acc[d] += 1;
            acc
        })
        .iter()
        .any(|&count| count == 2)
}

pub fn digits_for(v: usize) -> Vec<usize> {
    vec![
        v / 100_000,
        v / 10_000 % 10,
        v / 1_000  % 10,
        v / 100    % 10,
        v / 10     % 10,
        v / 1      % 10
    ]
}

pub fn is_valid_password(v: usize) -> bool {
    let digits = digits_for(v);
    digits_are_increasing(&digits) && digits_contain_repeated_value(&digits)
}

pub fn is_valid_password_p2(v: usize) -> bool {
    let digits = digits_for(v);

    digits_are_increasing(&digits) &&
        digits_contain_repeated_value_exactly_once(&digits)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_password_works() {
        assert_eq!(is_valid_password(111111), true);
        assert_eq!(is_valid_password(223450), false);
        assert_eq!(is_valid_password(123789), false);
    }

    #[test]
    fn p1_solution() {
        let c = INPUT
            .filter(|&v| is_valid_password(v))
            .count();

        assert_eq!(c, 960);
    }

    #[test]
    fn p2_solution() {
        let c = INPUT
            .filter(|&v| is_valid_password_p2(v))
            .count();

        assert_eq!(c, 626);
    }
}
