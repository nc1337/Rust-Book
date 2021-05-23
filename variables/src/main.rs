fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    another_function(43);
}

fn another_function(x: i32) {
    println!("Another function with x: {}", x);
}
