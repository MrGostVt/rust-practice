fn plusMinus(arr: &[i32]) {
    let mut zero = 0;
    let mut positive = 0;
    let mut negative = 0;

    let mut ratios: Vec<f64> = vec![0.0, 0.0, 0.0];

    arr.iter().for_each(|&x|{if x > 0 {positive+=1}else if x < 0 {negative+=1} else {zero+=1}});

    ratios[0] = positive as f64/arr.len() as f64;
    ratios[1] = negative as f64/arr.len() as f64;
    ratios[2] = zero as f64/arr.len() as f64;

    ratios.iter().for_each(|&x|{println!("{x}")});
}
#[test]
fn test(){
    plusMinus(&[-4,3,-9,0,4,1]);
}