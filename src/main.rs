fn main() {
   reverse();
}

fn reverse() {
    let str = "stressed";
    println!("{}", str);
    
    println!("{}", str.chars().rev().collect::<String>());
    
    for chr in str.chars().rev() {
        print!("{}", chr);
    }
}