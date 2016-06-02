
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
    let mut pkg = *Box::new(pkg_desc::Pkg::new());
    pkg.head.magic = pkg_desc::MAGIC_IN_HEAD;

    let head_size = mem::size_of::<pkg_desc::PkgHead>();
    println!("pkg_head size is {}",head_size);
    pkg.body.message[0] = 1;
    pkg.body.message[1] = 2;
    pkg.body.message[2] = 3;
    pkg.body.message[3] = 4;
    pkg.body.message[4] = 5;
    let pkg_size = pkg_desc::MAX_PKG_SIZE;
    pkg.head.pkg_len = 40;

    //分配在堆上
    let mut arr:[u8; pkg_desc::MAX_PKG_SIZE] = *(Box::new([0;pkg_desc::MAX_PKG_SIZE]));
    println!("start trans");
    arr = unsafe {mem::transmute(pkg)};
    println!("trans done");

    let write_size = stream.write(&arr[0..20]).unwrap();
    println!("write done:{}",write_size);

    sleep(one_seconds);
    let write_size = stream.write(&arr[20..40]).unwrap();
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