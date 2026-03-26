// Define the macro first
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // Now you can call it
    my_macro!();
}