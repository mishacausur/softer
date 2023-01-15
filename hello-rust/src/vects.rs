 print_vec(xs: &Vec<i32>) {
     for x in xs {
         println!("{}", x);
     }
 }

 fn main() {
     let xs = vec![1,2,3];
     print_vec(&xs);
     print_vec(&xs);