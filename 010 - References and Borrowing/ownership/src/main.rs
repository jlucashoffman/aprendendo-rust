fn main() {
    let mut a = String::from("peixe");
    let b = &a;

    let c = String::from("peixe");
    let d = c;

    println!("{a}");
    println!("{b}, {a}");

    //  println!("{c}"); -> este código não funciona, pois c o valor que existia em c foi repassado para a variável d, logo c está inválido
    println!("{d}");

    let e = &mut a;
//  let f = &mut a; -> não pode ocorrer

    // a mutabilidade é aplicada ao valor, assim a String se torna mutável, não a variável

    e.push_str(" pequeno");
    println!("{e}");
    println!("{a}");
    //  println!("{e}, {a}"); -> não pode, dá erro!
}
