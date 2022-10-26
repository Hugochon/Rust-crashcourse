trait Area {
    fn area(&self) -> f64;
}

trait Volume {
    fn volume(&self) -> f64;
}

struct Square{
    length: f64,
}

impl Square {
    fn new<T>(l:T) -> Self 
    where
        T: TryInto<f64>,
    {
        Self {length: l.try_into().unwrap_or(0.0)}
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        &self.length * &self.length
    }
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    fn new<T>(a:T, b:T, c:T) -> Self 
    where
        T: TryInto<f64>,
    {
        Self {
            a: a.try_into().unwrap_or(0.0),
            b: b.try_into().unwrap_or(0.0),
            c: c.try_into().unwrap_or(0.0),
        }
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        let (a, b, c) = (&self.a, &self.b, &self.c);
        let perimeter = a + b + c;
        let s = perimeter / 2.0;
        (s*(s-a)*(s-b)*(s-c)).sqrt()
    }
}

struct Pyramid<T: Area> {
    base: T,
    height: f64,
}

impl<T: Area> Pyramid<T> {
    fn new<U>(b:T, h:U) -> Self 
    where
        U: TryInto<f64>
    {
        Self { base: b, height: h.try_into().unwrap_or(0.0) }
    }
}

impl<T: Area> Volume for Pyramid<T> {
    fn volume(&self) -> f64 {
        let area = self.base.area();
        area * self.height
    }
}

fn main() {
    let square = Square::new(5);
    let square_float = Square::new(5.4);
    let square_string = Square::new("6".parse::<f64>().unwrap());

    println!("square area is {:.2}", square.area());
    println!("square_float area is {:.2}", square_float.area());
    println!("square_string area is {:.2}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1, 9.7);
    let pyramid_square = Pyramid::new(square, 24.3);
    let pyramid_triangle = Pyramid::new(triangle, 24.3);

    println!("pyramid_square volume is {:.2}", pyramid_square.volume());
    println!("pyramid_triangle volume is {:.2}", pyramid_triangle.volume());
}
