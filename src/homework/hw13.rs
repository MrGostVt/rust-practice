struct Point {
    x: i32,
    y: i32,
}
struct Rectangle {
    a: Point,
    b: Point,
}
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied_space = 0;
    let mut common_cells = 0;

    for i in 0..xs.len() {
        let len_i = (xs[i].b.x - xs[i].a.x);
        let height_i = (xs[i].a.y - xs[i].b.y);
        occupied_space +=  len_i * height_i;

        for j in i+1..xs.len(){
            let height_j = (xs[j].a.y - xs[j].b.y);

            if(xs[i].a.x < xs[j].b.x && xs[i].b.x > xs[j].a.x){
                if(xs[i].a.y > xs[j].b.y && xs[i].b.y < xs[i].a.y){
                    let mut common_x = if(xs[j].a.x > xs[i].a.x){xs[i].b.x - xs[j].a.x} else{xs[j].b.x - xs[i].a.x};
                    common_x = if(common_x > len_i){len_i}else{common_x};
                    let mut common_y;
                    if(height_j < height_i){
                        common_y = if(xs[j].a.y > xs[i].a.y){xs[i].a.y - xs[j].b.y} else{xs[j].a.y - xs[i].b.y};
                    }
                    else{
                        common_y = height_j - if(xs[i].a.y > xs[j].a.y){xs[j].a.y - xs[i].b.y} else{xs[i].a.y - xs[j].b.y};
                    }
                    common_y = if(common_y > height_j){height_j}else{common_y};

                    common_cells += common_x * common_y;
                }
            }
        }
    }

    occupied_space - common_cells
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}
