/* * * * * * * * /
* rfetch
* a fetch script written in rust
* because im bored lol
* made by James Doherty
* RIPBETTY
* * * * * */
use std::time::SystemTime;
use std::env;

fn boxed(input: &str) {
    // ┃┏ ┓┛━┓ ┗┃┛┗ ┏━
    print!("┏");
    for x in input.chars() {
        print!("━");
    }
    println!("┓");
    print!("┃");
    print!("{}", input);
    println!("┃");
    print!("┗");
    for x in input.chars() {
        print!("━");
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

    println!("Welcome Back, {}", name);
    println!("It has been {} seconds since the unix epoch", seconds_since_unix_epoch);
    println!("It has been {} minutes since the unix epoch", minutes_since_unix_epoch);
    println!("It has been {} hours since the unix epoch", hours_since_unix_epoch);
    println!("It has been {} days since the unix epoch", days_since_unix_epoch);
    println!("It has been {} years since the unix epoch", years_since_unix_epoch);

}
