fn main() {
    let x = soma(10, 20);
    println!("{}", x);
}

fn soma(a:i32, b:i32) -> i32 {
    return a + b;
//  return a + b (sem ponto e vírgula) funciona
}

fn sub(a:i32, b:i32) -> i32 {
    a - b
//  a - b; não funciona
}
