#[test]
fn main(){
    const width: u32 = 25;
    const height: u32 = 25;

    for y in 0..height {
        for x in 0..width{
            if((y == 0 || y == height-1)
                || (x == 0 || x == width-1)
                || (x == y || x == height-y)){
                print!("*");
            }
            else{
                print!(" ");
            }
        }
        println!()
    }
}
#[test]
fn clear_code_competition(){
    const WIDTH: u32 = 25;
    const HEIGHT: u32 = 25;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let is_horizontal: bool = (y == 0 || y == HEIGHT-1);
            let is_vertical: bool = (x == 0 || x == WIDTH-1);
            let is_diagonal: bool = (x == y || x == HEIGHT-y);

            let is_print: bool = is_horizontal || is_vertical || is_diagonal;
            if is_print {print!("*")} else {print!(" ")}
        }
        println!();
    }
}