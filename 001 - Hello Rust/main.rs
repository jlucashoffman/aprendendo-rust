fn main() {
    println!("Hello, World!");
}

/*
------------------------------
- EXECUTANDO O CÓDIGO
------------------------------

>> rustc main.rs
>> ./main

------------------------------
- RESULTADO ESPERADO
------------------------------
<< Hello, World!

------------------------------
- DESTRINCHANDO O CÓDIGO
------------------------------

fn main() define uma função chamada "main", ela é uma função especial, pois é o primeiro código que roda em qualquer programa Rust executável.

fn main(argumentos ficam aqui) {
    O corpo da função fica aqui
}

println! chama uma macro Rust, se fosse uma função, não teria o "!". Macros são uma forma de escrever código que gera código para estender a sintaxe Rust. Por agora basta apenas saber que "!" é utilizado para chamar macros e não funções.

Após chamar o "rustc main.rs" o programa é compilado, resultando um binário executável.
* main.rs é o código fonte
* main.exe é o executável binário (windows, em outras plataformas é apenas main)
* main.pdb informações de debug (windows)

Diferentemente de outras linguagens, os passos de compilar e executar em Rust são separados, isso permite que se possa enviar o arquivo executável para outra pessoa e ela não precisa ter o Rust instalado.

Apenas compilar com rustc é ok para programas simples, mas conforme o projeto cresce, é preterido que as opções de compilação sejam gerenciáveis, para isso temos o Cargo
*/