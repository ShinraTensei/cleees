use std::io;
use std::io::*;
use std::fs;
use rand::Rng;

fn main() {


    let post_bare ="
________\n
|      |\n
|\n
|\n
|";

    let post_1 ="
________\n
|      |\n
|      O\n
|\n
|";

    let post_2 ="
________\n
|      |\n
|      O\n
|      |\n
|";

    let post_3 ="
________\n
|      |\n
|      O\n
|      |\n
|     /";

    let post_4 ="
________\n
|      |\n
|      O\n
|      |\n
|     / \\";

    let post_5 ="
________\n
|      |\n
|    __O\n
|      |\n
|     / \\";

    let post_6 ="
________\n
|      |\n
|    __O__\n
|      |\n
|     / \\";
    
    // Load word file
    let wordlist = fs::read_to_string("res/wordlist.txt").expect("Something went wrong reading the file");
    // Pick  a word
    let splitwords: Vec<&str> = wordlist.split("\n").collect();
    let word = splitwords[rand::thread_rng().gen_range(0..splitwords.len()-1)];
    
    // Let the gays begin
    // Take start loop
    let mut start = true;
    let mut input = String::new();
    let mut stage = 0;
    let mut hit = false;
    
    println!("The word has {} letters", word.len());
    let word_vec: Vec<char> = word.chars().collect();
    let word_size = word_vec.len();
    let mut solve: Vec<char> = vec!['-'; word_size];
    
    while start {
        match stage {
            1 => println!("{}", post_1),
            2 => println!("{}", post_2),
            3 => println!("{}", post_3),
            4 => println!("{}", post_4),
            5 => println!("{}", post_5),
            _ => println!("{}", post_bare),
        }
        println!("The word: {:?}", solve);
        print!("Input a letter: ");
        io::stdout().flush().expect("Error flushing stream!");
        io::stdin().read_line(&mut input).expect("Failed to read from stdin!");
        let ch = input.chars().next().unwrap();
        
        //checkuu
        for n in 0..word_size {
            if ch == word_vec[n] {
                solve[n] = word_vec[n];
                hit = true;
            }
        }
        if !hit {
                stage += 1;
                if stage >= 6 {
                    println!("{}\nGAME OVER!", post_6);
                    io::stdout().flush().expect("Error flushing stream!");
                    start = false;
                }
        }
        if solve == word_vec {
            println!("YOU IZ WINRAR!!!!");
            start = false;
        }
        hit = false;
        input.clear()
    }
}