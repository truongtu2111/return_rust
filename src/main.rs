fn main() {
    let mut x = 5;
    println!("The value of x is: {}",x);
    x=7;
    println!("The value of x is: {}",x);

    println!("Use shadow variable for same type");
    let y =23;
    println!("The value of y is: {}",y);
    let y =79;
    println!("The value of y is: {}",y);

    println!("Use shadow variable for difference type");
    let string_var = "Truong Quang Tu";
    println!("The value of string is: {}",string_var);
    let string_var = string_var.len();
    println!("The len of string is: {}",string_var);
}
