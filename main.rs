#[derive(Debug)]
struct TestStruct {
    x: i32,
    y: i32,
}

fn main() {
    let mut x = 5;

    let y:&mut i32 = &mut x;
    println!("{:?}", *y+1);
    *y+=1;

    println!("{:?}", x);

  // let mut st = TestStruct {x:0,y:0};
  // test(st);
  // println!("{:?}", st.x);
}



fn test(x:TestStruct) {
    let mut y = x;
    y.x+=1;
    println!("{:?}", y.x);
}
