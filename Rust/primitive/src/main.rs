fn main() {
    //let a = [1, 2, 3];

    //println!("a has {} elements", a.len());

    let names = ["Graydon", "Brian", "Niko"];
    println!("The second name is: {}", names[1]);

    let a = [0, 1, 2, 3, 4];
    let complete = &a[..];
    let middle = &a[1..4];
    println!("The middle {}", middle[0]);

    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("The x is {}", x);
    println!("The y is {}", y);
    println!("The z is {}", z);

    
}
