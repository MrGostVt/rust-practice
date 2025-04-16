#[test]
fn draw(){
    const MAX_WIDTH: i32 = 14;
    const HEIGHT: i32 = 18;

    let diff: i32 = HEIGHT - MAX_WIDTH;

    let mut max_height: i32 = HEIGHT + -diff;

    for mut y in 0..max_height{
        if( diff < 0 && (diff.abs() < HEIGHT/2 + 2 && (y >= HEIGHT/2 + 2 - diff.abs()/2 && y <= HEIGHT/2 + 2 + diff.abs()/2))
            || (diff.abs() > HEIGHT/2 + 2 && (y <= diff.abs()/2 || y >= max_height-diff.abs()/2))){

            continue;
        }
        let mut curr_str: String = "".to_string();

        for x in 0..MAX_WIDTH {
            let mut formed_y: i32 = y;
            if(y > MAX_WIDTH/2){
                formed_y = MAX_WIDTH - formed_y;
            }
            let is_filling: bool = (x >= MAX_WIDTH/2 - formed_y && x <= MAX_WIDTH/2 + formed_y);

            if is_filling {curr_str.push_str("*")} else {curr_str.push_str(" ")}
        }

        println!("{}",curr_str);

        if(diff > 0 && (y >= max_height/2 - diff.abs()/2 && y <= max_height/2 + diff.abs()/2)){

            y -= 1;
            println!("{}", curr_str);
        }

    }
}