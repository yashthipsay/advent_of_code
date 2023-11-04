mod text_processing {
    mod letter {
        fn count_letters(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_alphabetic()).count()
        }
    }

    mod numbers {
        fn count_numbers(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_numeric()).count()
        }
    }
}

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters: usize;
    let number_of_numbers: usize;
    (number_of_letters, number_of_numbers);
}

fn main() {
    println!("Hello, world!");
}
