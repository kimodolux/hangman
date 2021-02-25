use std::fs;
use rand::Rng;
use std::io;

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

    println!("guess is: {}", incomplete_word);
    println!("Word is: {}", word);

    println!("\n\n\nGuess a single letter or the whole word!");

    loop {
        if !incomplete_word.contains('_') {
            println!("Yay you win, the word was {}", word);
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
        for (n, real_word_letter) in word.chars().enumerate(){
            let incomplete_word_letter = incomplete_word.chars().nth(n).expect("empty string");
            let guessed_letter = guess.chars().next().expect("empty string");

            match real_word_letter == guessed_letter {
                true => {
                    output.push(guessed_letter);
                },
                false => {
                    output.push(incomplete_word_letter);
                }
            }
        }

        incomplete_word = output; 

    }
    

}
