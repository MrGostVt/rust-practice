fn is_prime(number: i32) -> bool{
    let mut is_prime = true;

    if(number > 1){
        for i in 2..number/2 + 1 {
            if(number % i == 0 ){
                is_prime = false;
                break;
            }
        }
    }
    else { is_prime = false; }
    println!("{number} is_prime: {}", is_prime);
    is_prime
}

#[test]
fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];


    test_data
        .iter()
        .for_each(|(n, prime)|
            assert_eq!(is_prime(*n), *prime)
        )
}
