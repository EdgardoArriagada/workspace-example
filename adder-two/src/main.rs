use add_two;

fn main() {
    let num: i32 = 10;
    println!(
        "Hello, world2! {} plus two is {}!",
        num,
        add_two::add_two(num)
    );
}
