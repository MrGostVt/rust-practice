
fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let max = candles.iter().max().unwrap();
    let mut count = 0;

    candles.iter().for_each(|candle| {if candle == max{count += 1}});

    count
}
#[test]
fn test(){
    birthdayCakeCandles(&[4,4,3,1]);
    println!("{}", birthdayCakeCandles(&[4,4,3,1]));
}