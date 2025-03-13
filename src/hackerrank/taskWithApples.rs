// https://www.hackerrank.com/challenges/apple-and-orange/problem?isFullScreen=true

fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut sumA: u32 = 0;
    let mut sumB: u32 = 0;

    for apple in apples{
        if(a + apple >= s && a + apple <= t){
            sumA += 1;
        }
    }

    for orange in oranges{
        if(orange + b <= t && orange + b >= s){
            sumB += 1;
        }
    }

    println!("{}", sumA);
    println!("{}", sumB);
}
