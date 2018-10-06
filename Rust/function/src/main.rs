fn main() {
    print_number(5);
    print_sum(5, 6);
    let y: i32 = plus_one(4);
    println!("{}", y);
}

fn print_number(x: i32){
    println!("I call print_number--> x is : {}", x);
}

fn print_sum(x: i32, y: i32){
    println!("I call print_sum--> sum is {}", x + y);
}

fn plus_one(i: i32) -> i32 {
    i + 1
}