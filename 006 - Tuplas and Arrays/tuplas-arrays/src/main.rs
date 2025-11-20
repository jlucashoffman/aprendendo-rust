fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    {
        let a = x.0;
        let b = x.1;
        let c = x.2;
    }

    {
        let (a, b, c) = tup;
    }

    // Ambos os trechos de código funcionam iguais

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array2 = [3;5]; // [3, 3, 3, 3, 3] (este array foi tipado por inferência do compilador)

    println!("Primeiro elemento do \"array\": {array[0]}");
}
