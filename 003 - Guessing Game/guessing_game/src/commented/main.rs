// a biblioteca "io" vem da biblioteca padrão "std", por mpadrão o Rust tem vários itens definidos em sua biblioteca padrão que entra no escopo de cada programa, este é chamado de prelúdio (prelude). Caso queira utilizar algo que não está no prelúgio, você precisa declarar explicitamente com o "use".

// a biblioteca std::io permite a capacidade de aceitar entrada de usuário.

use std::cmp::Ordering; // é um enum de comparação
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    /*
    rand::thread_rng() é a função que possibilita o gerator de número aleatórios. gen_range é o método da função, este é definido pela trait "rng" que trouxemos para o escopo com o "use rand::Rng". Este método é uma expressão que tem como argumento uma expressão de intervalo do tipo start..=end (incluindo os extremos)
    */
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // let mut define uma variável mutável
        let mut guess = String::new();

        // lida com o padrão de entrada para o terminal
        io::stdin()
            .read_line(&mut guess) // o uso de & é uma referência de memória, desta forma não há necessidade de copiar as informações
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // parse retorna um Result então é possível de passar por um match
            Ok(num) => num,
            Err(_) => continue, // o _ serve como um valor geral, estamos dizendo que queremos todos os valores de erro, independente da informação que está nele. O continue pula a instancia do loop para a próxima
        };

        // readl_line além de retornar o valor inserido pelo usuário também retorna um valor Result, este é uma enumeração (enumeration - enum), um tipo que pode se um de múltiplos estados, cada estado é chamado de variante (variant). As variantes de um Result são Ok e Err, representando sucesso e falha respectivamente. Em casos de falha informações da operação também são passadas. Uma instância Result possui um método expect, onde se a variante  de read_line for Err, expect retornará o resultado do erro que veio do sistema operacional, caso a variante seja Ok, então expect retonra o valor retornado.

        // se não colocar o expect o programa compila, mas gera um aviso
        
        println!("You guessed: {guess}");

        // este {guess} é um placeholder

        match guess.cmp(&secret_number) {
        // o primeiro resultado que satisfazer termina o match
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // sai do loop
            }
        }
    }
}