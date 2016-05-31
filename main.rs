fn main() {
  let mut x = 5;
  {
    let y:&mut i32 = &mut x;
    println!("{:?}", *y+1);
    *y+=1;
  }
  println!("{:?}", x);
}
