struct Square<T> {
    side : T,
}
impl Square<u32> {
    fn new(x : u32) -> Self {
        Square { 
        side : x,
        }
    }
    fn area(&self) -> u32 {
        return self.side * self.side
    }
}
impl Square<f64> {
    fn new(x : f64) -> Self {
        Square { 
        side : x,
        }
    }
    fn area(&self) -> f64 {
        return self.side * self.side
    }
}
impl Square<String> {
    fn new(x : &str) -> Self {
        Square { 
        side : x.to_owned(),
        }
    }
    fn area(&self) -> f64 {
        return self.side.parse::<f64>().unwrap() * self.side.parse::<f64>().unwrap()
    }
}
struct Triangle<T>{
    base : T,
    height : T,
}
impl Triangle<f64> {
    fn new(x : f64, y : f64) -> Self {
        Triangle {
        base : x,
        height : y,
        }
    }
    fn area(&self) -> f64 {
         (self.base*self.height)/2.0
    }
}
struct Pyramid<T,U> {
    base : T,
    height : U,
}
impl Pyramid<Square<u32>,f64> {
    fn new(square : Square<u32>, h : f64) -> Self {
        Pyramid {
            base : square,
            height : h,
        }
    }
    fn volume(&self) -> f64{
        self.base.area() as f64 * (self.height/3.0) 
    }
}
impl Pyramid<Triangle<f64>,f64> {
    fn new(triangle : Triangle<f64>, h : f64) -> Self {
        Pyramid {
            base : triangle,
            height : h,
        }
    }
    fn volume(&self) -> f64{
        self.base.area() as f64 * (self.height/3.0) 
    }
}

fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");
    
    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());
  
    let triangle = Triangle::new(14.9, 20.1);
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}
