fn main() {
    let x = convert_to_celsius(25.0);
    println!("{}", x);
    let x = convert_to_fahrenheit(x);
    println!("{}", x);
}

fn convert_to_celsius(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(x: f32) -> f32 {
    (x * 9.0 / 5.0) + 32.0
}
