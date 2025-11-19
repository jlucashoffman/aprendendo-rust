fn main() {
    let x = 5;      // imutável
    let mut y = 10; // mutável

    println!("primeira instância: {x}, {y}"); // 5, 10

    {
        let x = 6;
        y = 12;

        println!("segunda instância: {x}, {y}"); // 6, 12
    }
    
    println!("terceira instância: {x}, {y}"); // 5, 12
}
