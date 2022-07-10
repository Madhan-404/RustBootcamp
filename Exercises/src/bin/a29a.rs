

/*

write a  generic function  having three parameters of generic type  and     return the greatest number among them 
use this function for i32 and f32 types

*/


// Coding Exercise :-


// Link the rustling exercise for generic. 


fn main() {

let res1 = greater_no::<i32>(1, 20, 300);
let res2 = greater_no::<f32>(1.0, 20.0, 300.0);

println!("{} {}",res1,res2);

}

fn greater_no<T:PartialOrd>(a:T,b:T,c:T) ->T {
    
    if a>b && a>c {
        return a ;
    }else if b>a && b>c {
        return b ;
    }else {
        return c ;
    }
}