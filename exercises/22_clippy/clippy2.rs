fn main() {
    let mut res = 42;
    let option = Some(12);

    // Fixed: use `if let` instead of iterating over Option
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}