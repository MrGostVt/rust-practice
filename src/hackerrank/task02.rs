fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut results = vec![0,0];

    for i in 0..3 {
        if a[i] > b[i] {results[0] += 1;}
        else if b[i] > a[i] {results[1] += 1;}
        else{continue;}
    }

    results
}

#[test]
fn test(){

}