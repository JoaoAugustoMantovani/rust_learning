
#[derive(Debug)]
enum MyError{
    Error1
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError>{
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let divide = divide(4, 2);
    //let res = divide.expect("we crashed");

    /*
    This is a pretty secure way to verify results using match.
    */
    match divide {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{:?}", v)
    }

    /*
    Another way is checking using if statement the divide function and
    if is ok then unwrap the value.
     */
    // if divide.is_ok(){
    //     println!("{}", divide.unwrap());
    // }

    /* 
    Or we can simply run the print, and if there is an error it will
    crash the program.
    */
    // println!("\n{}", divide.unwrap());

    /*
    We can use the unwrap_or function to return a default value if 
    the function fails.
     */
    //  println!("\n{:?}", divide.unwrap_or(100));

 
    //  println!("\n{:?}", res);
}