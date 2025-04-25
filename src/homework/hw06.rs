

fn draw_tree(triangle_count: u32){
    let first_triangle = [
        [" ", "*", " "],
        [" ", "*", " "],
        ["*", "*", "*"],
    ];

    let mut j:u32 = 0;
    let mut current_triangle = 0;
    let mut current_height = 3;
    let mut start_point = 0;
    let mut current_width: u32 = 5;
    let width = current_width + 2 * triangle_count ;
    println!("width: {}", width);
    let mut border: u32 = 0;

    while true {
        if(current_height + start_point <= j){
            if(current_triangle >= 1){
                current_height += 1;
            }

            current_triangle += 1;
            start_point = j;
            current_width += 2;

            if(current_triangle >= triangle_count){
                break;
            }
        }
        let count: u32 = j - start_point + j - start_point + 1;
        border =  ((width - current_width) / 2) + ((current_width - count) / 2) ;

        let mut local_i:u32 = 0;

        for i in 0..width {
            let mut is_star_draws = i > border && i <= border + count;
            if current_triangle == 0 {
                let in_border = if (i <= width / 2 + 2 && width / 2 <= i){true} else {false};
                is_star_draws = if (in_border && first_triangle[j as usize][local_i as usize] == "*"){true} else {false};

                if in_border && local_i < 2{
                    local_i += 1;
                }
            }

            if is_star_draws {print!("*")} else {print!(" ")};
        }
        println!();
        j += 1;
    }
}

#[test]
fn test(){
    draw_tree(7);
}