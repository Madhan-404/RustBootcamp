// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted  entry into a movie theatre
// * Restricted entry require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted entry
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a entry



// struct Customer {
//     age:i32
// }

fn entry(Age: i32) -> Result<bool,String>{
    if Age >=21 {
        Ok(true)
    }else {
         return Err("Your Age is less than 21 :(".to_string()) ;
    }
}

fn main() {

    
    let Entry = entry(28);

    match Entry {
        Ok(d) => println!("{}",d),
        Err(e) => println!("{}",e)
    }

}

