fn main() {
    // Fix: make pi the same type as radius (f32)
    let pi: f32 = 3.1;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}