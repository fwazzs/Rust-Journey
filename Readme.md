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

## Day 5 (Structs & Method Syntax | 27 Oct 2026)

1. Struct
- Custom type that groups related values under named fields, declared with `struct` (CamelCase name)
- Create an instance with `StructName { field: value }`, read or change a field with `.`
- Only the whole instance can be `mut`, you cannot make a single field mutable
- Field init shorthand: if a variable has the same name as a field, write it once (`email,`)
- Struct update syntax `..other` fills the remaining fields from another instance

        let mut user1 = User {
            active: true,
            username: String::from("Fawaz"),
            email: String::from("Fawaz123@gmail.com"),
            sign_in_count: 1,
        };
        user1.email = String::from("Pewes123@gmail.com");

        let user2 = User {
            username: String::from("user2"),
            ..user1
        };

2. Tuple Struct & Unit-Like Struct
- Tuple struct: fields have no names, accessed by index (`.0`), and each struct is its own type even with the same field types
- Unit-like struct: no fields at all, useful when you need a type but no data (e.g. implementing a trait later)

        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        let black = Color(0, 0, 0);

        struct AlwaysEqual;
        let subject = AlwaysEqual;

3. Printing a Struct
- `#[derive(Debug)]` above the struct lets it print with `{:?}` or pretty `{:#?}`
- `dbg!(&rec1)` prints the file, line and value to stderr; pass a reference so ownership is not taken

        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        println!("Rec1 is : {rec1:#?}");
        dbg!(&rec1);

4. Passing a Struct to a Function
- Borrow it with `&Rectangle` so the function can read it without taking ownership
- A plain tuple `(u32, u32)` also works, but the names of the values are lost; a struct is clearer

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

        let rec1 = Rectangle { width: 20, height: 10 };
        let result = area(&rec1); //200

5. Method Syntax
- `impl` block holds functions that belong to a struct
- `&self` is the first parameter of a method, call it with dot syntax: `rec1.area()`
- Methods can take more parameters, e.g. `other: &Rectangle`

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        println!("{}", rec1.can_hold(&rec2));

## Day 6 (Enums & Pattern Matching | 29 Oct 2026)

1. Enum
- Defines a type that can be one of several variants, declared with `enum` (CamelCase name)
- Variants can hold no data, a tuple of values, a named struct-like set of fields, or a single `String`
- Each variant is accessed with `Enum::Variant`

        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        enum Message {
            Quit,
            Move { x: u32, y: i32 },
            Write(String),
            Change_color(i32, i32, i32),
        }

        let home = IpAddr::V4(127, 0, 0, 1);
        let mut msg = Message::Move { x: 32, y: 12 };

2. Option Enum
- `Option<T>` is in the standard prelude and means a value is either `Some(T)` or `None`
- Rust has no `null`, so `Option<T>` must be handled before it can be used as a `T`
- `None` needs a type annotation when the compiler cannot infer it, e.g. `let z: Option<u32> = None;`

        let x = Some(5);
        let z: Option<u32> = None;

3. Match Expression
- `match` compares a value against patterns, and the first matching arm runs
- Arms must cover every possible case (exhaustive), otherwise the code does not compile
- An arm can bind a value from the variant, e.g. `Coin::Quarter(state)` gives access to `state`
- Arms with a block `{}` can run several statements; the arm's last expression is the returned value

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State Quarter from {state:?}!");
                    25
                },
            }
        }

4. Matching Option
- Use `match` to handle both `Some` and `None`, and `Some(i)` binds the inner value
- Matching `None` must be written too, the compiler forces the case to be handled

        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let a = Some(5);
        let b = plus_one(a); //Some(6)

5. If Let
- `if let PATTERN = value { ... }` is a shorter `match` when you only care about one pattern and ignore the rest
- Add `else` to handle every other case, e.g. `if let Coin::Quarter(state) = coin { ... } else { ... }`

        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum of config {max}")
        }

        let coin = Coin::Quarter(UsState::Alaska);
        if let Coin::Quarter(state) = coin {
            println!("State of Quarter is {state:?}");
        } else {
            count += 1;
        }
