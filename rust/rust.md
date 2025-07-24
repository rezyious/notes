# Rust
**Interesting fact about shadowing:** <br/>
This example of shadowing is interesting. Shadowing doesn't destroy a value but **blocks** it.
```
    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8;
    println!("{country_ref} {country}");
```
output:
```
    Austria 8
```

**Interesting fact about move**: <br />
The variable is not mutable but it does not matter since it **moves** to
the function but the parameter of the function must be `mut`:
```
    fn main() {
        let country = String::from("Austria");
        adds_hungary(country);
    }

    fn adds_hungary(mut string_to_add_hungary_to: String) {
        string_to_add_hungary_to.push_str("-Hungary");
        println!("{}", string_to_add_hungary_to);
    }
```
the variable `country` can be mut but it is not necessary and compiler gives a warning of a unused mut, 
because the `country` moves in to the function and function takes ownership

**Complex printing**:
```
    {variable:padding alignment minimum.maximum}
    alignment (left/middle/right) are like this < ^ >
```
Also Debug and pretty debug come at the end, so use `?` at the end .

Some examples:
```
    let letter = "a";
    println!("{:0^11}", letter);
    // output : 00000a00000

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);  // note that space used as padding
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
    // output : 
    ---------TODAY'S NEWS---------
    |                            |
    SEOUL--------------------TOKYO
```

A useless example to demonstrate the concept and also it looks cool:
```
use core::time;
use rand::Rng;
use std::thread;

fn main() {
    for _ in 1..1_000_000 {
        let random_number = rand::thread_rng().gen_range(1..=1_000_000);
        println!(
            "{:0>9} {:0>9} {:0>9} {:0>9} {:0>9} {:0>9} {:0>9} {:0>9} {:0>9} {:0>9}",
            random_number,
            random_number,
            random_number,
            random_number,
            random_number,
            random_number,
            random_number,
            random_number,
            random_number,
            random_number,
        );
        thread::sleep(time::Duration::from_millis(100));
    }
}
```

**Neat trick to find a variable type:** <br/>
Here is a good tip for arrays as well as other types: to find the type of a variable, you
can “ask” the compiler by giving it bad instructions, such as trying to call a method
that doesn’t exist. Take this code for example:
```
    fn main() {
        let seasons = ["Spring", "Summer", "Autumn", "Winter"];
        let seasons2 = ["Spring", "Summer", "Fall", "Autumn", "Winter"];
        seasons.ddd();
        seasons2.thd();
    }
```
And the compiler in the error message point out the type.

**About match** : <br/>
You can even put `if` inside of match. This is called a *match guard*:
```
    fn main() {
        let children = 5;
        let married = true;
        match (children, married) {
            (children, married) if married == false => println!("Not married with {children} kids"),
            (children, married) if children == 0 && married == true => {
                println!("Married but no children")
            }
            _ => println!("Married? {married}. Number of children: {children}."),
        }
    }
```

You also don’t need to write `== true` or `== false` when checking a bool. Instead, you can write the name of the 
variable by itself (to check if true) or the name of the variable with an exclamation mark in front (to check if false). 
Here’s the same code as before using this shortcut:
```
    fn main() {
        let children = 5;
        let married = true;
        match (children, married) {
            (children, married) if !married => println!("Not married with {children} kids"),
            (children, married) if children == 0 && married => println!("Married but no children"),
            _ => println!("Married? {married}. Number of children: {children}."),
        }
    }
```
Note that in simple `if` statements this is wrong and it is not like javascript and in rust you should always check
for example like this `condition == true` and not just `condition`. <br/>

You can use _ as many times as you want in a match. In this match on colors, we have three to match on, but only 
check one at a time:
```
    fn match_colors(rgb: (i32, i32, i32)) {
        match rgb {
            (r, _, _) if r < 10 => println!("Not much red"),
            (_, g, _) if g < 10 => println!("Not much green"),
            (_, _, b) if b < 10 => println!("Not much blue"),
            _ => println!("Each color has at least 10"),
        }
    }
    fn main() {
        let first = (200, 0, 0);
        let second = (50, 50, 50);
        let third = (200, 50, 0);
        match_colors(first);
        match_colors(second);
        match_colors(third);
    }
```

This example also shows how match statements work because in the first example, it only prints "Not much blue". But 
first also has “not much green.” **A match statement always stops when it finds a match and doesn’t check the rest.** 
This is a good example of code that compiles well but is probably not the code you want.<br/>

You can also use `@` to give a name to the value of a match expression, and then you can
use it. In this example, we match an i32 input in a function. If it’s 4 or 13, we want to
use that number in a println! statement. Otherwise, we don’t need to use it:
```
    fn match_number(input: i32) {
        match input {
            number @ 4 => println!("{number} is unlucky in China !"),
            number @ 13 => println!("{number} is lucky in Italy! In bocca al lupo!"),
            number @ 14..=19 => println!("Some other number that ends with -teen: {number}"),
            _ => println!("Some other number, I guess"),
        }
    }
    fn main() {
        match_number(50);
        match_number(13);
        match_number(16);
        match_number(4);
    }
```
Also note the use of range `14..=19` in the match arm.

**Fun and Stupid fact about methods:** <br/>
You can call all methods using `::` if you want, but methods that take self use `.` for convenience.
```
    fn main() {
        let person = Person {
            name: String::from("reza"),
            age: 38,
        };
        Person::dumb();
        person.introduce();
        Person::introduce(&person);
    }

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    impl Person {
        fn dumb() {
            println!("Dumb method");
        }

        fn introduce(&self) {
            println!("{} is {} years old", self.name, self.age);
        }
    }
```

**Destructuring structs :** <br/>
```
    struct Person {
        name: String,
        real_name: String,
        height: u8,
        happiness: bool,
    }

    // Destructuring inside the signature of a function
    fn check_if_happy_destructured(
        Person {
            name, happiness, ..
        }: &Person,
    ) {
        println!("Is {name} happy? {happiness}");
    }

    fn main() {
        let papa_doc = Person {
            name: "Papa Doc".to_string(),
            real_name: "Clarence".to_string(),
            height: 170,
            happiness: false,
        };

        check_if_happy_destructured(&papa_doc);

        // Destructuring a struct
        let Person {
            name : fake_name,  // we can aslo rename
            real_name,
            ..  // This is to say we only need name and real_name and ignore
                // height, happiness,
        } = papa_doc;

        println!("They call him {fake_name} but his real name is {real_name}. ");
    }
```

**This example from the `Book` on `if...let` and `let...else`:**
```
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self {
                UsState::Alabama => year >= 1819,
                UsState::Alaska => year >= 1959,
            }
        }
    }

    // This is the long way with `if...let`
    fn describe_state_quarter(coin: Coin) -> Option<String> {
        if let Coin::Quarter(state) = coin {
            if state.existed_in(1900) {
                Some(format!("{state:?} is pretty old, for America!"))
            } else {
                Some(format!("{state:?} is relatively new."))
            }
        } else {
            None
        }
    }

    // here we use the fact that expressions can return a value
    fn describe_state_quarter(coin: Coin) -> Option<String> {
        let state = if let Coin::Quarter(state) = coin {
            state
        } else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }

    // this is with `let...else`
    fn describe_state_quarter(coin: Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }
```
