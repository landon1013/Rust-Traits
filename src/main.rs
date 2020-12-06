use std::ops::{Add};

trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name:&'static str) -> Human {
        Human{name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    fn create(name:&'static str) -> Cat {
        Cat{name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self { result += *x }
        return result;
    }
}

fn traits() {
    // let h = Human{name:"Landon"};
    let h:Human = Animal::create("John");
    h.talk();

    let c = Cat{name:"Bear"};
    c.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
}

// Operator Overloading
#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

impl Add for Complex<i32> {
    type Output = Complex<i32>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

fn operator_overloading() {
    let mut a = Complex::new(1, 2);
    let mut b = Complex:: new(3, 4);
    
    println!("{:?}", a + b);
}

fn main() {
    //traits();
    operator_overloading();
}
