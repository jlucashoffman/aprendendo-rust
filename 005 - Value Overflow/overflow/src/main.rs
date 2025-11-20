fn main() {
    let a:u8 = 250; // valores de 0 até 255

    // wrapping_*
    let b:u8 = a.wrapping_add(10);
    println!("variável em situação de overflow wrap: {b}");
    
    
    // checked_*
    match a.checked_add(10) {
        Some(valor) => println!("Sem overflow com a variável {a}, valor: {valor}"),
        None => println!("Houve overflow!"),
    }

    // overflowing_*
    let (c, overflow) = a.overflowing_add(10);
    println!("Valor: {c} | Overflow: {overflow}");

    // saturating_*
    let d = a.saturating_add(10);
    println!("variável em situação de overflow saturate: {d}");
}
