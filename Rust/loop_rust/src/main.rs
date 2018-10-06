fn main() {
    println!("------------while------------");
    let mut x = 5;
    let mut done = false;

    while !done{
        x += x - 3;
        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
    println!("------------for------------");

    for x in 0 .. 10 {
        println!("{}", x);
    }

    println!("------------enumerate------------");

    for (i, j) in (5..10).enumerate(){
        println!("i = {} and j = {}", i, j);
    }

    println!("------------loop break------------");
    let mut x = 5;

    loop{
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {break;}
    }

    println!("------------ odd num ------------");

    for x in 0..10 {
        if x % 2 == 0{continue;}

        println!("{}", x);
    }

    println!("------------ loop rabel ------------");
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x: {}, y: {}", x, y);
        }
    }
    
}
