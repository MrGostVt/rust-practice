fn array_sum(array: Vec<i32>) -> i32{
    let sum = array.iter().sum::<i32>();
    sum
}

#[test]
fn test1() {

    println!("{}",array_sum(vec![2,6,7,8,1]))
}
