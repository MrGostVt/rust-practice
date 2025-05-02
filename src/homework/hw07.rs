
fn invert_the_case(string: String) -> String {
    let mut inverted_string = "".to_string();
    for ch in string.chars() {
        if(ch as u32 >= 65 && ch as u32 <= 96 || ch as u32 >= 1040 && (ch as u32) < 1071) {
            inverted_string.push(char::from_u32((ch as u32) + 32).unwrap());
        }
        else if (ch as u32 > 96 && ch as u32 <= 127 || ch as u32 >= 1071 && ch as u32 <= 1103){
            inverted_string.push(char::from_u32((ch as u32) - 32).unwrap());
        }
    }
    println!("inverted_string: {}", inverted_string);

    inverted_string
}

#[test]
fn test() {
    let data =
        [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];


    data
        .iter()
        .for_each(|(a, b)| {
            assert_eq!(
                invert_the_case(a.to_string()),
                b.to_string()
            );
            assert_eq!(
                invert_the_case(b.to_string()),
                a.to_string()
            );
        });
}
