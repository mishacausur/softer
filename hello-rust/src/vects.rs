#[derive(Debug)]
struct S {
   value: i32
}

fn main() {
    let r: &i32;
    {
        let s = S { value: 92 };
        let rs: &S = &s;
        r = f(rs);
    }
}

fn f<'a>(s: &'a S) -> &'a i32 {
    &s.value
}