use std::io;

fn is_valid(username: String, password: String) -> bool {
    // Write your code below
    if ((username == "user" && password == "qweasd") || username == "admin") {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut input_user = String::new();
    let mut input_pass = String::new();
    io::stdin().read_line(&mut input_user).unwrap();
    io::stdin().read_line(&mut input_pass).unwrap();

    let user: String = input_user.trim().parse().unwrap();
    let pass: String = input_pass.trim().parse().unwrap();
    let res = is_valid(user, pass);
    println!("{}", res);
}