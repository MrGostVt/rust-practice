
fn is_palindrome(x: u32) -> bool {
    let s_number = x.to_string();
    let s_number_reversed = s_number.chars().rev().collect::<String>();

    s_number == s_number_reversed
}


#[test]
fn test() {
    let data =
        [
            (123, false),
            (121, true),
            (1221, true),
        ];


    data
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
}
