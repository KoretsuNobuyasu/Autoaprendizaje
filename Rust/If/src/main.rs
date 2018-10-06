fn main() {
    let x = 6;

    if x == 5{
        println!("x は ５ です！");
    } else if x == 6 {
        println!("x は ６ です！");
    } else {
        println!("x は ５ でも ６ でもありません");
    }

    let y = if x == 5 {10} else {15};
    println!("yは{}です",y);
}
