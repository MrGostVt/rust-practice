fn staircase(n: i32) {
    for j in 0..n{
        for i in 0..n{
            let border = n - (j + 1);

            if(border <= i){print!("#")}
            else{print!(" ")}
        }
        println!("");
    }
}

#[test]
fn test(){
    staircase(6);
}