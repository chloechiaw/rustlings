// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}

// two solutions to this: 
    // 1. Add a `return` ahead of `num * num;`
    // 2. remove `;`, make it to be `num * num`
// remove the semicolon = allows fn to return a value. 
