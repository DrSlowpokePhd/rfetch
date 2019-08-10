/* * * * * * * * /
* rfetch
* a fetch script written in rust
* because im bored lol
* made by James Doherty
* RIPBETTY
* * * * * */
use std::time::SystemTime;
use std::env;


#[cfg(test)]
// tests for the helper functions
mod test {
    #[test]
    #[ignore]
    fn test_name() {
        use crate::boxed;
        boxed("hello, world!", 1);
        // test boxed with newline
        boxed("hello,\n world!", 1);
    }
}

// helper functions for the boxed function
fn print_times(content: &str, times: u8) {
    // prints a string a set amount of times
    for _ in 0..times {
        print!("{}", content)
    }
}


fn print_block(width: u8, times: u8) {
    // prints a vertical block with a width of
    // spaces between bars
    let mut block = String::new();
    block.push_str("┃");
    for _ in 0..width {
        block.push_str(" ");
    }
    block.push_str("┃");
    println!("{}", block);
}

fn boxed(input: &str, margin: u8) {
    // added a margin parameter to allow for
    // adjustment of margin of the box
    // ┃┏ ┓┛━┓ ┗┃┛┗ ┏━
    print!("┏");

    print_times("━", margin);

    for x in input.chars() {
        print!("━");
    }
    print_times("━", margin);
    println!("┓");
    print_block(2*margin + input.len() as u8, margin);
    print!("┃");
    print_times(" ", margin);
    print!("{}", input);
    print_times(" ", margin);
    println!("┃");
    print_block(2*margin + input.len() as u8, margin);
    print!("┗");
    for _ in 0..margin {
        print!("━")
    }
    for x in input.chars() {
        print!("━");
    }for _ in 0..margin {
        print!("━")
    }
    println!("┛");
}

fn main() {
    let seconds_since_unix_epoch = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("System Time before unix epoch"),
    };
    let minutes_since_unix_epoch = seconds_since_unix_epoch/60;
    let hours_since_unix_epoch = minutes_since_unix_epoch/60;
    let days_since_unix_epoch = hours_since_unix_epoch/24;
    let years_since_unix_epoch = days_since_unix_epoch/360;
    let mut name;
    // get current username of user logged in
    match env::var("USER") {
        Ok(val) => name = val,
        Err(_) => panic!("Name doesnt exist")
    }

    for args in env::args() {
        if args == "-s" {
            println!("It has been {} seconds since the unix epoch", seconds_since_unix_epoch);
        }
    }

    let mut greeting: String = String::from("Welcome Back, ");
    for x in String::from(name).chars() {
        greeting.push(x);
    }
    boxed(greeting.as_str(), 2u8);

    println!("It has been {} minutes since the unix epoch", minutes_since_unix_epoch);
    println!("It has been {} hours since the unix epoch", hours_since_unix_epoch);
    println!("It has been {} days since the unix epoch", days_since_unix_epoch);
    println!("It has been {} years since the unix epoch", years_since_unix_epoch);

}
