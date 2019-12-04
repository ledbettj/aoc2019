const INPUT : std::ops::Range<usize> = 265275..781584;

pub fn is_valid_password(v: usize) -> bool {
    let digits = vec![
        v / 100_000,
        v / 10_000 % 10,
        v / 1_000  % 10,
        v / 100    % 10,
        v / 10     % 10,
        v / 1      % 10
    ];

    let is_increasing = digits
        .iter()
        .enumerate()
        .skip(1)
        .all(|(index, &digit)| digit >= digits[index - 1]);

    let has_one_dupe = digits
        .iter()
        .enumerate()
        .skip(1)
        .any(|(index, &digit)| digit == digits[index - 1]);

    is_increasing && has_one_dupe
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

}
