fn miniMaxSum(arr: &[i32]) {
    let mut max: i64 = 0;
    let mut min: i64 = i64::MAX;

    for i in 0..arr.len() {
        let mut sum: i64 = 0;
        arr.iter().enumerate().for_each(|(j, &x)| {
            if j != i {
                sum += x as i64;
            }
        });

        if sum > max { max = sum; }
        if sum < min { min = sum; }
    }

    println!("{min} {max}");
}

#[test]
fn test(){
    miniMaxSum(&[256741038, 623958417, 467905213, 714532089, 938071625])
}