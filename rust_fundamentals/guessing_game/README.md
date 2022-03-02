# Build a Guess Game Application using RUST Cargo
#### by Jahidul Arafat
## Step-0: Setting Up a New Project
```shell
> cargo new guessing_game
> cd guessing_game

# Looking at the generated Cargo.toml file
> cat Cargo.toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

# Checkout src/main.rs
> cat src/main.rs
fn main() {
    println!("Hello, world!");
}

# Compile, Build and Run for the first time
> cargo check 
> cargo run 
```

## Step-1: Processing a Guess
```rust
fn main() {

    // Ask for user input, process the input and check that the input is in the expected form

    // 1.1 import the `io` library as-> use std::io
    println!("Guess the number");
    println!("Please input your guess.");

    // 1.2 Create a mutable variable named `guess` as type `string`, which will act as a placeholder for user input next
    let mut guess = String::new();  // create a new mutable variable called 'guess' of type <String>
    // in RUST, variables are mutable by default; that's why we used the <mut>

    // 1.3 Receive the user input
    /* Explanations
    * <io::stdin>
    * is::stdin().readline(&).expect(msg:)
    * -----------------------------------------------------------
    * .readline() calls readline method
    *`&` indicates that this argument is a reference.
    * this let multiple parts of your code access one piece of data without needing to copy the data into <memory> multiple times
    * references are immutable by default like variables
    * that's why we used `&mut guess` instead of `& guess` to make it immutable
    * for handing potential failure we used `.expect(msg:)`
    * XXXXX ~~~ If you dont call .expect(), then > cargo build will compile with a <WARNING!!!!> ~~~ XXXXX
    *
    * <io::Result>
    * -----------------------------------------------------------
    * read_line puts whatever the user enters into the string we pass to it,
    * but it also returns a value <io::Result> type: enumerations <enums>
    * Result's variants are <Ok> or <Err>
    * Enums are often used with match, for conditional matching and execution based on <variants>
    */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
}
```
```shell
# Compile, Build and Run: 2nd time
> cargo check 
> cargo build 
> cargo run 
```

## Step-2: Generating a Secret Number
### 2.1 Add a Crate named 'rand' in Cargo.toml
```shell
# Add a Crate named 'rand' in Cargo.toml under [dependencies] as external dependencies
> cat Cargo.toml
...
[dependencies]
rand = "0.8.5" # the external caret
               # means, any version at least 0.8.5 but below 0.9.0

# Build the project without changing anything in src/main.rs
> cargo build

# look for the Cargo.lock; this will ensure reproducible build with the Cargo.lock file

# Updating a Crate to Get a New Version
# Means, updating rand Crate to a new version. if you try to update the version of dependencies to the version they have released, but not to 0.9.0
> cargo update
```

### 2.2 Generating a Random Number
```rust
use std::io;   // library to obtain user input and then to print result as output
use rand::Rng; // Generating a Random Number

fn main() {
    println!("Generating a Secret Number between [1 to 100]...");
    let secret_number = rand::thread_rng().gen_range(1..101); // or .gen_range(1..=100)
    println!("The Secret Number is: {}",secret_number);
    
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

```shell
# Compile, Build and Run: 3rd time
> cargo check 
> cargo run 
```

## Step-3: Comparing the Guess to the Secret Number
### 3.1 Comparing: Ends with an Error
```rust
// --snip--
println!("You guessed: {}", guess);

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

```shell
# Compile, Build and Run: 4th time
> cargo check 
> cargo build # This will produce an error, as secret_number is a string when <guess> is an Integer
```

### 3.2 Comparing: Error Resolved | Convert the String the program reads as input into a real number type
```rust
// --snip--

let mut guess = String::new();

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

// Convert the String number to an unsigned 32bit integer number
// Throw an error if the user input anything other than <number string>
// recreating variable <guess> ?? - instead we are shadowing
// Shadowing lets us reuse the <guess> variable name rather than forcing us to create two unique variables such as <guess_str> and <guess>
// this shadowing is often used when you want to convert a value from one type to another type
let guess: u32 = guess.trim().parse().expect("Please type a number!"); # 

println!("You guessed: {}", guess);

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```
```shell
# Compile, Build and Run: 5th time
> cargo check 
> cargo run 
```

## Step-4: Allowing Multiple Guesses with Looping 
### 4.1 Problem is Infinite Loop, even if the guess match. How to solve it?
```rust
// --snip--

println!("The secret number is: {}", secret_number);

loop {
    println!("Please input your guess.");

    // --snip--

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

```

```shell
# Compile, Build and Run: 6th time
> cargo check 
> cargo run 
```

### 4.2 Quitting After a Correct Guess. Problem is Program raise expception when user inputs non-numeric strings i.e. tysd
```rust
 // --snip--

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;                  // break the look when user guesses the secret number correctly.
    }
}
```

### 4.3 Handling Invalid Input
```rust
// --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Remember that parse returns a Result type and Result is an enum that has the variants Ok or Err. 
        // We’re using a match expression here, as we did with the Ordering result of the cmp method.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,        // 1st arm; 
            Err(_) => continue,   // 2nd ard; _ means anything, catch all and continue to next loop if invalid input
        };

        println!("You guessed: {}", guess);

        // --snip--
```

```shell
# Compile, Build and Run: 7th time
> cargo check 
> cargo run 
```

## Step-5: Final Code src/main.rs
```rust
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Generating a Secret Number between [1 to 100]...");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The Secret Number is: {}",secret_number);

    println!("Guess the number [1 to 100]");
    loop {
        println!("Please input your guess [1 to 100].");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You Guessed: {}",guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
```
## Step-6: Generate the dependencies documentation
```shell
> cargo doc --open
# will build documentation provided by all of your dependencies locally and open it in your browser.
```
