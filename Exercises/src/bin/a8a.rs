// Topic: Getting Used to with struct
//
//Requirements:
/*
 1-Define  struct Shape having  square:Square and rectangle:Rectangle 
 2-Square has one field as side:i32
 3-Rectangle has two length:i32 and width:i32
 4-Create a function which returns a new Rectangle
 5-Create a function which returns a new Square
 6-Create a function which returns a new Shape 
 7-Create a function which takes Shape and prints the dimension of all shapes 
*/


struct Shape {
    square: Square,
    rectangle: Rectangle,
    }


#[derive(Debug)]
    struct Square {
    side: i32,
    }

#[derive(Debug)]
    struct Rectangle {
    length: i32,
    width: i32,
    }


// 4-Create a function which returns a new Rectangle

fn new_rect(length:i32, width:i32 ) -> Rectangle {

   return Rectangle { length, width } ;

}



// 5-Create a function which returns a new Square

fn new_square(side:i32) -> Square {
    Square { side }
}



// 6-Create a function which returns a new Shape 



fn new_shape(square:Square,rectangle:Rectangle) -> Shape {

    Shape { square:square, rectangle:rectangle }
}

// 7-Create a function which takes Shape and prints the dimension of all shapes 


fn print_shape(shape:Shape) {

    println!("{:?} {:?} ", shape.rectangle,shape.square);

}



fn main() {

  let rect_insatnce = new_rect(4,6);
  let square_instace = new_square(8);

  let shape = Shape{
    square:square_instace,
    rectangle:rect_insatnce
  };


  print_shape(shape);

}
