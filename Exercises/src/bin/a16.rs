// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>


struct Lockers {
    name:String,
    locker:Option<i32>
}

fn main() {

    let l1 = Lockers{
        name:"Madhan".to_string(),
        locker:Some(1)
    };

    let l2 = Lockers{
        name:"Raj".to_string(),
        locker:None
    };

    match l1.locker {
        Some(data) => println!("NAME:{:?} LOCKER:{:?}", l1.name,data),
        None => println!("LOCKER NOT ASSIGNED")

    }

    match l2.locker {
        Some(data) => println!("NAME:{:?} LOCKER:{:?}", l2.name,data),
        None => println!("LOCKER NOT ASSIGNED")

    }
}
