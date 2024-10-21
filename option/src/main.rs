fn main() {
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    // Unwraping a 'Some' variant will extract the value wrapped.
    println!("\n{:?} unwraps to {}\n", divide1, divide1.unwrap());

    // Unwrapping a 'None' variant will 'panic!'
     println!("{:?} unwraps to {}", divide2, divide2.unwrap());
}

fn divide(dividend: i32, divisor: i32) -> Option<i32>{
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}