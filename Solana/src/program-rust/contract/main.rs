// mod print;
// mod vars;
// mod types;
// // mod string;
// mod question;

// fn main() {
//     // print::run();
//     // vars::run();
//     // types::run();
//     // string::run();
//     question::run();
// }
// }
use std::io;

fn main() {
    let mut input = String::new();
    println!("Would you like to register your wedding on Solana Blockchain? (y/n)");
    match io::stdin().read_line(&mut input) {
        Ok(y) => {
            println!("{} ", input);
            // println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
    let mut answer = String::new();
    println!("Enter your ID user1, Please");
    match io::stdin().read_line(&mut answer) {
        Ok(y) => {
            println!("user1:{} ", answer);
        }
        Err(error) => println!("error: {}", error),
    }
    let mut user = String::new();
    println!("Enter your ID user2, Please");
    match io::stdin().read_line(&mut user) {
        Ok(y) => {
            println!("user2:{} ", user);
        }
        Err(error) => println!("error: {}", error),
    }
}
