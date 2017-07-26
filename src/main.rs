

fn main() {
    another_function(true);

}

fn another_function(x: bool) -> i32{
    let number = if x {
        5
    } else {
        6
    };
    return number;
}
