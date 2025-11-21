fn main() {
    println!("{}", celsius_to_faren(20.0))
}

fn celsius_to_faren(c:f32) -> f32 {
    return (c * 1.8) + 32.0;
}
