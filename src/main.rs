

use std::sync::Arc;
use std::cell::Cell;

fn main() {

    //let x = Arc::new(5);
    //let y = x.clone();

    // let x = RefCell::new(10);
    // let y = x.borrow_mut();
    // let z = x.borrow_mut();

    // println!("{:?}",y);

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut point = Point{x:10,y: 15};
    //point.y.set(7);

    println!("{:?}",point);

    #[derive(Debug)]
    struct PointRef<'a> {
        x: &'a mut i32,
        y: &'a mut i32,
    }

    {
        let point_ref = PointRef{x: &mut point.x , y: &mut point.y};
        *point_ref.y += 1;
    }
    println!("{:?}",point);

    fn coorinates() -> (i32,i32,i32){
        (10,11,12)
    }

{
    let (x,_,_) = coorinates();
    println!("{:?}",x);
}   

let tuple: (u32, String) = (5,String::from("five"));
let (x,_) = tuple;
println!("{:?}",tuple);

#[derive(Debug)]
struct Person {
    name: Option<String>,
}

let name = "Steve".to_string();
let x: Option<Person> = Some(Person {name: Some(name)});
match x {
    Some(Person {name: ref a@ Some(_),..}) => println!("{:?}",a),
    _ => {}
}


for x in 1..16 {
    match (x%3 , x%5) {
        (0 , 0) => println!("{:?}","FizzBuzz"),
        (0,_) => println!("{:?}","Fizz"),
        (_, 0) => println!("{:?}","Buzz"),
        (..) => println!("{:?}",x),
    }

}

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

impl Circle {
     fn area(&self) -> f64{
         std::f64::consts::PI * (self.r * self.r)
     }
}

let c = Circle{x:0.0,y:0.0,r:2.0};
println!("{:?}",c.area());

}