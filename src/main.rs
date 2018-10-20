// standard input/ouput lib
use std::io;

// main
fn main() {
  /* IO */
  println!("Guess a number");
  println!("Please input your guess.");

  // create a mutable variable called guess
  // String::new() is new instance of string
  let mut guess = String::new();

  // using the sdn input, read the line
  // and store it in the mutable var guess
  // & = reference, so it changes the value
  // of guess.
  // .expect() for error handling
  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read the line.");

  // print result
  println!("You guessed: {}", guess);

  /* PRIMITIVES */
  // mutable variable
  let mut x = 5;
  println!("x: {}", x);
  x = 7;
  println!("x: {}", x);
}
