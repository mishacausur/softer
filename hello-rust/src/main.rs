#[derive(Debug)]
struct S {
   value: i32
}

struct Wrapper {
    value: Box<i32>,
}

fn main() {
    timer();
    wrapped()
}

fn wrapped() {
    let w = Wrapper { value: Box::new(192)} ;
    let r: &i32 = &*w.value;
    if w.value > Box::new(100) { println!("enough"); }
    println!("{}", *r);
}

fn timer() {
    let r: &i32;
    {
        let s = S { value: 92 };
        let rs: &S = &s;
        r = f(rs);
        println!("{}", *r);
    }
}

fn f<'a>(s: &'a S) -> &'a i32 {
    &s.value
}