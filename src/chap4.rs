fn main() {
    println!("Hello, world!");

    let mut v = vec![1,2,3,4,5];
    let mut v2 = v;
    //println!("{:?}",v);
    println!("{:?}",v2);
    //v2 = take(v2);
    func1(&v2);
    println!("{:?}",v2);
    println!("{:?}" , sum_vec(&v2));

    let mut x  = 5;
    println!("{:?}", x);
{
    let  y = &mut x;
    *y += 1;
}
    println!("{:?}", x);

    for i in &mut v2 {
        println!("{:?}",i);
        //v2.push(34);
    }
let x =5;
    let y: &i32;
    
    y = &x;
    print!("{:?}",y);

    let line = "lang:en=Hello World!";
    let lang = "en";
    let v;
    {
        let p = format!("lang:{}",lang);
        v = skip_prefix(&line,p.as_str());
    }
    println!("{:?}",v);
    let z = &5;
    let f = Foo{x: y};
    println!("{:?}",f.x);

println!("{:?}",f.x());


{
   
    let yy = &5;
    let ff = Foo{x: yy};
    let xx;
    xx = &ff.x;
    println!("{:?}", xx);
}

fn frob<'a>() -> &'a str{
   "test"
}

}

#[derive(Debug)]
struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
     fn x(&self) -> &'a i32 {
         self.x
     }
}



fn skip_prefix<'a,'b>(line: &'a str, prefix: &'b str) -> &'a str {
    return "rest";
}

fn sum_vec(v: &Vec<i32>) -> i32{
    return v.iter().fold(0,|a,&b| a+b);
}


fn take(v: Vec<i32>)-> Vec<i32>{
 return v;
}

fn func1(v: &Vec<i32>){

}

