use std::fmt;

#[derive(Debug)]
struct MinMax(i64,i64);

#[derive(Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}


impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut count = 0;
        for n in self.0..self.1+1 {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", n)?;
            count = count + 1;
        }
        write!(f, "]")
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {:.2}, y: {:.2}, z: {:.2})", self.x, self.y, self.z)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main(){
    // 3D POINT
    let point_3d = Point3D{x: 1.26, y: 3.57, z: 10.04};
    println!("Point3D is {}", point_3d);

    // RANGE
    let range = MinMax(1,30);
    println!("Range is {}", range);

    // COMPLEX NUMBER
    let complex = Complex{real: 3.3, imag: 7.2};
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}