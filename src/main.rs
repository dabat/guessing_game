use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let min = 1;
    let max = 25;
    let secret_number = rand::thread_rng().gen_range(min, max + 1);
    let mut guesses: Vec<String> = vec!["-".to_string(); max];

    clear_screen();

    loop {
        println!("Pick a number from {}-{}:", min, max);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number, please try again", guess);
                continue;
            }
        };

        if guess > max {
            println!("{} is greater than the maximum number {}!", guess, max);
            println!("please try again");
            continue;
        } else if guess < min {
            println!("{} is less than the minimum number {}!", guess, min);
            println!("please try again");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                clear_screen();
                guesses_print(&mut guesses, guess, None);
                println!("{} is a bit too small", guess)
            }
            Ordering::Greater => {
                clear_screen();
                guesses_print(&mut guesses, guess, None);
                println!("{} is a bit too big", guess)
            }
            Ordering::Equal => {
                clear_screen();
                guesses_print(&mut guesses, guess, Some(true));
                println!("{} is the secret number, you guessed it!", guess);
                break;
            }
        }
    }
}

fn guesses_print(guess_list: &mut Vec<String>, guess: usize, print_number: Option<bool>) {
    match print_number {
        Some(true) => {
            if let Some(placeholder) = guess_list.get_mut(guess - 1) {
                *placeholder = guess.to_string();
            }
        }
        Some(false) | None => {
            if let Some(placeholder) = guess_list.get_mut(guess - 1) {
                *placeholder = "X".to_string();
            }
        }
    }
    println!("{}", &guess_list.join(" "));
    println!("");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    println!("Guess the secret number!");
}
