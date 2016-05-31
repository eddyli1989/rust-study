
use std::net::TcpStream;
use std::io::prelude::*;
use std::time::Duration;

#[warn(unused_imports)]
use std::thread::sleep;
use std::mem;
extern crate comm;
use comm::pkg_desc;

fn main() {
    let one_seconds = Duration::new(1, 0);
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("connect success!");
    let time_out = Some(one_seconds);
    //stream.set_write_timeout(None).unwrap();
    let mut pkg = pkg_desc::pkg::new();
    pkg.head.magic = pkg_desc::MAGIC_IN_HEAD;

    let head_size = mem::size_of::<pkg_desc::pkg_head>();
    println!("pkg_head size is {}",head_size);
    pkg.body.message = vec![1,2,3,4,5];
    let pkg_size = head_size + 5;
    pkg.head.pkg_len = pkg_size;

    let test_size = mem::size_of_val(&pkg);
    let mut arr:[u8; test_size] = [0; test_size];
    let arr : Vec<u8>= unsafe {mem::transmute_copy(&pkg)};


    let write_size = stream.write(&arr).unwrap();
    println!("write done:{}",write_size);
    //let _ = stream.flush().unwrap();
    let _ = stream.set_read_timeout(time_out).unwrap();

    loop {
        let mut content = String::new();
        let ret = stream.read_to_string(&mut content);
        match ret {
            Ok(n) => {
                println!("recv {} bytes from svr,said:{}",n,content);
                if n == 0 {
                    println!("disconnect");
                    break;
                }
            },
            Err(_) => {}
        }
    }
}