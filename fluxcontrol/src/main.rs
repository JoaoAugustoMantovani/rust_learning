fn main() {
    let a: i32 = 55;
    let b: i32 = 45;

    return_bigger(a,b);
}

fn return_bigger (a: i32, b: i32){

    if a > b {
        println!("a is bigger than b");
    } else {
        println!("b is bigger than a");
    }

}