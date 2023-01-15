fn main() {
    let x = 1;
    let r: &i32;
    {
        let y = 2;
        r = f(&x, &y);
    }
    println!("{}", *r);
}

fn f<'a, 'b>(_x: &'a i32, _y: &'b i32) -> &'a i32 {
    _x
}