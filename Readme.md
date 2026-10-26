## WELCOME TO THE RUST LEARNING LOG
It would be contain all of my Rust Programming Language learning progress. It started on 19 Oct 2026.

### First Week
## Day 1 (Getting Started & Programming a Guessing Game | 19 Oct 2026)

1. Learn about Cargo (cargo build, cargo run, cargo build --release, etc)
2. Varibale (var = immutatable & var mut = mutable variable)
3. std::io (Input and Output library)

        io::stdin()
            .read_line(&mut guess) //take input value of mut guess and store it in guess variable
            .expect("Failed to read line");

4. std::cmp::Ordering; (Comparation library)

        match guess.cmp(&secret_number) { //compare guess with secret number
            Ordering::Less => println!("Too small!"), //if guess is less than secret number
            Ordering::Greater => println!("Too big!"), //if guess is greater than secret number
            Ordering::Equal => {
                println!("You win!");
                break;
            } //if guess is equal to secret number then break the Loop
        }

5. use rand::prelude::*; (Random Generating Number)

        rand::rng().random_range(1..=100); //generate random number between 1 and 100

## Day 2 (Common Programming Concepts | 20 Oct 2026)

1. Variable (let, let mut, const, shadowing)
- let : Immutable, cannot be changed after it is set
- let mut : Mutable, can be reassigned with a new value of the same type
- const : Constant, always immutable, must have a type annotation, and can be set in any scope
- shadowing : Declaring a new variable with the same name using `let`, it can also change the data type

        let mut x = 5;
        x = 6;

        const SECOND_IN_HOUR: u32 = 60 * 60;

        let spaces = "   ";
        let spaces = spaces.len(); //shadowing changes the type from &str to usize

2. Data Type
- Integer : Whole number, signed (i8, i16, i32, i64, i128, isize) can be negative, unsigned (u8, u16, u32, u64, u128, usize) only positive
- Float : Decimal number, f64 (default) and f32
- Character : Single Unicode character in single quotes, e.g. 'y', 'z' (`char`)
- Boolean : `true` or `false`, type `bool`
- Tuple : Group of values with different types, fixed length, accessed by index (`tup.0`) or destructuring (`let (x, y, z) = tup;`)
- Array : Group of values with the same type, fixed length, accessed by index (`a[0]`)

        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
        let five_hundred = tup.0;

        let b = [1, 2, 3, 4, 5];
        let element = b[0];

3. Arithmetic Operators
- `+` addition, `-` subtraction, `*` multiplication, `/` division, `%` remainder
- Integer division truncates toward zero: `-5 / 3` gives `-1`
- Integer and float operations cannot be mixed, `5 + 2.0` does not compile

        let sum = 5 + 10;
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; //-1
        let remainder = 43 % 5; //3

4. Array Index Input
- Read a line from the user with `io::stdin().read_line`, then `trim` and `parse` it to `usize`
- Index out of range makes the program panic at runtime

## Day 3 (Functions, Control Flow & Loops | 21 Oct 2026)

1. Function
- Declared with `fn`, names use snake_case
- `main` is the entry point; other functions can be defined before or after it
- Every parameter must have a type annotation
- Return type is written after `->`; the last expression (without `;`) is returned, or use `return`
- Statement (ends with `;`, returns nothing) vs Expression (evaluates to a value)

        fn add_one(x: i32) -> i32 {
            x + 1
        }

        let y = {
            let x = 3;
            x + 3
        }; //y = 6

2. Comment
- `//` for single line comment

3. If Expression
- Condition must be a `bool`, numbers are not converted automatically
- `else if` checks more conditions, only the first true branch runs
- `if` can be used inside `let`, but both branches must return the same type

        let numb = if condition {5} else {6};

4. Loop
- `loop` runs forever until `break`, `break` can also return a value
- Loop label (`'counting_up:`) to break an outer loop: `break 'counting_up;`
- `while` runs while the condition is true
- `for` iterates over a collection, `1..4` gives 1, 2, 3 (end is not included)

        let result = loop {
            x += 1;

            if x == 10 {
                break x;
            }
        };

## Day 4 (Ownership, Borrowing & Slices | 22 Oct 2026)

1. Ownership
- Each value in Rust has one owner, and when the owner goes out of scope the value is dropped
- Assigning or passing a heap value (like `String`) moves ownership, the old variable can no longer be used
- Simple stack values (like `i32`) are copied instead, so the original stays valid

        let x = String::from("Hello World");
        take_ownership(x); //x is moved here, using x after this line is a compile error

        let y = 5;
        make_copy(y); //y is copied, still usable after this line

2. Variable Scope
- A variable is valid from its declaration until the end of the enclosing block `}`

        fn main() {
            let s = "Hello There!"; //s valid from this line
        } //s no longer valid (out of scope)

3. String Type
- String literal (`&str`) is fixed and immutable, e.g. `"YOO"`
- `String` is growable and mutable, created with `String::from` and extended with `push_str`

        let x = "YOO BRO WHAT'S SUPPP!!!"; //&str
        let mut y = String::from("Hello Fella!"); //String
        y.push_str(", My big boy!");

4. Borrowing
- `&value` borrows a value without taking ownership (read only)
- `&mut value` borrows a value and allows changing it, the variable must be declared `mut`

        let len = calculate_length(&s1); //borrow s1 to read its length
        change(&mut s); //borrow s to change it

5. String Slice
- A slice (`&s[start..end]`) is a reference to part of a string, the end index is not included
- `&s[..]` slices the whole string
- `first_word` returns the index of the first space as `usize`; it does not return the slice itself yet

        let s = String::from("hello");
        let slice = &s[3..s.len()]; //"lo"
        let whole = &s[..]; //"hello"
