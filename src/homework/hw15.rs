

fn calculate_muxa_slon() -> Vec<Vec<u32>> {
    let mut right_answers: Vec<Vec<u32>> = vec![];

    for a in 1..=8 {
        for b in 1..=8 {
            if b == a { continue; }
            for c in 1..=8 {
                if c == a || c == b { continue; }
                for d in 1..=8 {
                    if d == a || d == b || d == c { continue; }

                    let numbers = vec![a, b, c, d];

                    let current_answer = 1000 * a + 100 * b + 10 * c + d;
                    let slon = current_answer * d;

                    if slon >= 10000 {
                        continue;
                    }

                    let slon_nums = vec![
                        slon / 1000,
                        (slon % 1000) / 100,
                        (slon % 100) / 10,
                        slon % 10,
                    ];

                    let mut is_unique = true;
                    for i in 0..slon_nums.len() {
                        for j in (i+1)..slon_nums.len() {
                            if slon_nums[i] == slon_nums[j] {
                                is_unique = false;
                                break;
                            }
                        }
                        if !is_unique { break; }
                    }

                    let mut is_ok = is_unique;
                    if is_ok {
                        for num in &slon_nums {
                            if numbers.contains(num) || *num == 0 || *num == 9 {
                                is_ok = false;
                                break;
                            }
                        }
                    }

                    if is_ok {
                        right_answers.push(numbers.clone());
                    }
                }
            }
        }
    }

    right_answers
}


#[test]
fn test(){
    // println!();
    for i in calculate_muxa_slon(){
        i.iter().for_each(|&x| print!("{x}"));
        println!();
    }
}