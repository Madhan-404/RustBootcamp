
// Fix errors without removing any line
fn main() {
    // let s1 = String::from("hello,");
    // let s2  ="world!".into();
    // let s3 = s1 + s2; 
    // assert_eq!(s3,"hello,world!");
    // println!("{}",s1);



/*
Hint : only String and &str can be concated and the order is important as only string is growable and &str is not growable.


Case 1:- str+str concatanation //ownership
Case 2:- &str+&str  //&str is immutable
Case 3:- &str+str    //&str is immutable
//only parameter in left or initial parameter is conidered.


*/

// let x  = 4;
// let y  = x;  // shallow copy 

// println!("{}",y);

// here both x and y have same value but the addresses are different(in stacks) but the case is not the same with strings(in heaps)

let s = string::from("");
let s1= s;  //shallow copy but value is deleted . This is called as move in Rust

// using .clone() like 
// let s1 = s.clone(); // It is called as deep copy

// Generally it is not recommended

//println!("{}",s1);

// here both s and s1 points to the same data but it is against the ownership rules as a data can have only one owner at a time
//so the new variable takes the ownership and old one gets deleted as the rules are contradicted above

//here in this case, S is deleted and s1 gains ownership
}