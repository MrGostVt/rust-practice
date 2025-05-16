
fn count_permutation(mut shipments: Vec<u32>) -> usize{
    let average = shipments.iter().sum::<u32>() / shipments.len() as u32;
    let mut capacity:u32 = 0;
    let mut steps = 0;

    let mut iters = 0;
    while true {
        for shipment in shipments.iter_mut() {
            if(iters > 2 && capacity != 0){
                *shipment += 1;
                capacity -= 1;
                steps += 1;
            }
            if *shipment > average {
                capacity += *shipment - average;
                *shipment = average;

                steps += 1;
            }
            else if *shipment < average{
                let add_val = average - *shipment;
                if(capacity >= add_val) {
                    capacity -= add_val;
                    *shipment += add_val;

                    steps += 1;
                }
            }
        }
        if capacity == 0 || iters > 10{
            break;
        }
        iters += 1;
    }

    println!("updated shipments");
    print!("[");
    shipments.iter().for_each(|x| print!(" {},", x));
    print!("]");
    println!();
    steps as usize
}
#[test]
fn test(){
    let shipments = vec![9, 3, 7, 2, 9];
    let result = count_permutation(shipments);
    println!("result: {} steps", result);
}