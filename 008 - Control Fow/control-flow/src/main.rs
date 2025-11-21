fn main() {

    let mut round: i32 = 5;

    'contando: loop {

        println!("Round {round}! Fight!");

        let mut count: i32 = 3;

        while count > 0 {
            println!("{count}")    ;

            if round == 1 {
                println!("O adversário pediu pinico, luta finalizada!");
                break 'contando;
            }

            count -= 1;
        }

        println!("Round {round}, finished!");

        round -= 1;
    }

    let arr: [i32; 3] = [3, 2, 1];

    for element in arr {
        println!("{element}");
    }

    for number in (1..4).rev() {
        println!("{number}!")
    }
}
