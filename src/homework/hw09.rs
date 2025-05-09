
fn rotate(s: String, mut n: i32) -> String {
    let len = s.chars().count() as i32;
    if n == 0 || n % len == 0 {
        return s;
    }
    n = ((n % len) + len) % len;

    let chars: Vec<char> = s.chars().collect();

    let rotated: String = chars[(len - n) as usize..]
        .iter()
        .chain(&chars[0..(len - n) as usize])
        .collect();

    rotated
}


#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];


    // shifts
    //     .iter()
    //     .for_each(|(n, exp)|
    //         assert_eq!(
    //             rotate(s.to_string(), *n),
    //             exp.to_string()
    //         )
    //     );
    rotate("Lol kek".to_string(), 3 );
}
