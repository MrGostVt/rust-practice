
fn gcd(a: u32, b: u32) -> u32 {
    let base: u32 = if(a > b) {b} else{a};
    let mut divisor = 0;

    for i in 1..base+1 {
        if(a % i == 0 && b % i == 0 ){
            divisor = i;
        }
    }

    divisor
}

#[test]
fn test() {
    let data:[((u32, u32), u32); 11] =
        [
            ((24,  60), 12),
            ((15,   9),  3),
            ((15,   6),  3),
            ((140, 40), 20),
            ((24,  16),  8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37,  11),  1),
            ((120, 90), 30),
        ];


    for ((a, b), exp) in data.iter() {
        println!("gcd({}, {}) = {}", a, b, gcd(*a, *b));
        assert_eq!(*exp, gcd(*a, *b));
    }
}
