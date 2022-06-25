
// DON'T let `println!` works
fn main() {

    
    never_return();
    println!("Failed!");

    
}

fn never_return() -> ! {
   
   panic!("") // Implement this function, don't modify the fn signatures
      
}
