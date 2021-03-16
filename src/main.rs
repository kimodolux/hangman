use std::fs;
use rand::Rng;
use std::io;

fn draw_hangman(count: u32) {
    match count {
        // Match a single value
        1 => println!("
     __________
      |        |
               |
               |
               |
               |
               |
               |
    ============
        "),
        2 => println!("
     __________
      |        |
      0        |
               |
               |
               |
               |
               |
    ============
        "),
        3 => println!("
     __________
      |        |
      0        |
      |        |
               |
               |
               |
               |
    ============
        "),
        4 => println!("
     __________
      |        |
      0        |
     /|        |
               |
               |
               |
               |
    ============
        "),
        5 => println!("
     __________
      |        |
      0        |
     /|\\       |
               |
               |
               |
               |
    ============
        "),
        6 => println!("
     __________
      |        |
      0        |
     /|\\       |
     /         |
               |
               |
               |
    ============
        "),
        7 => println!("
      __________
      |        |
      0        |
     /|\\       |
     / \\       |
               |
               |
               |
    ============
        "),
        _ => println!("Ain't special"),
    }
}

fn main() {
    println!("Welcome to hangman!");
    let wordlist = fs::read_to_string("src/wordlist.txt")
        .expect("something went wrong reading the wordlist");
    let mut rng = rand::thread_rng();
    let words: Vec<_> = wordlist.lines().collect();
    let length: usize = words.len();
    let random: usize = rng.gen_range(0..length);
    let word = wordlist.lines().nth(random).unwrap();
    let mut incomplete_word = String::from("");
    for _n in 0 .. word.len(){
        incomplete_word.push('_');
    }
    let mut wrong_count: u32 = 1;

    println!("guess is: {}", incomplete_word);

    println!("\n\n\nGuess a single letter or the whole word!");

    loop {

        draw_hangman(wrong_count);

        if !incomplete_word.contains('_') {
            println!("Yay you win, the word was {}", word);
            break;
        }
        if wrong_count >= 7 {
            println!("Game over :(");
            println!("The word was {}", word);
            break;
        }

        println!("guess is: {}", incomplete_word);
        println!("What's your guess?\n");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let mut output = String::with_capacity(word.len());
        let mut correct_guess = false;
        for (n, real_word_letter) in word.chars().enumerate(){
            let incomplete_word_letter = incomplete_word.chars().nth(n).expect("empty string");
            let guessed_letter = guess.chars().next().expect("empty string");

            match real_word_letter == guessed_letter {
                true => {
                    correct_guess = true;
                    output.push(guessed_letter);
                },
                false => {
                    output.push(incomplete_word_letter);
                }
            }
        }
        if correct_guess == false {
            wrong_count += 1;
        }
        incomplete_word = output; 

    }
    

}
