
// Fix all errors without adding newline
fn main() {
    let mut s =  String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

/*
 Hint:  read the difference between push and push_str
*/




