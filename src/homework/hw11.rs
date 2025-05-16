use rand::prelude::*;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut vec:Vec<i32> = Vec::new();

    for r_val in 0..n {
        let num = rand::rng().random_range(10..100);
        vec.push(num);
    }

    vec
}
fn get_min_couple_start(vec: Vec<i32>, n: usize) -> usize {
    let mut start: usize = 0;
    let mut sum: i32 = i32::MAX;

    for i in 1..n {
        if(vec[i] + vec[i-1] < sum){
            sum = vec[i] + vec[i-1];
            start = i-1;

            println!("sum: {sum}, left: {}, right: {}", vec[i-1], vec[i]);
        }
    }

    start
}
fn display_info(vec: Vec<i32>, couple_start: usize, n: usize) {
    let mut lines = vec![
        String::from("indexes:"),
        String::from("data:  ["),
        String::from("pointer:"),
        String::from("min adjacent sum = "),
    ];

    for (pos, e) in vec.iter().enumerate() {
        let spaces_0 = if pos > 9 { "".to_string() } else { " ".to_string() };
        lines[0] += &(spaces_0 + &pos.to_string() + ", ");

        let last_char = if pos + 1 == n { "]" } else { ", " };
        lines[1] += &format!("{}{}", e, last_char);

        let pointer = if pos == couple_start { "\\__ __/ " } else { "    " };
        lines[2] += pointer;
    }

    let sum_of_couple = vec[couple_start] + vec[couple_start + 1];
    lines[3] += &format!("{} + {} = {} at indexes {}, {}",vec[couple_start]
                        ,vec[couple_start + 1]
                        ,sum_of_couple
                        ,couple_start
                        ,couple_start + 1);
    for line in lines {
        println!("{}", line);
    }
}
#[test]
fn test(){
    let vec: Vec<i32> = gen_random_vector(20);
    let min_couple_start = get_min_couple_start(vec.clone(), 20);
    display_info(vec, min_couple_start, 20);
}