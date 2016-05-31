#[allow(dead_code)]
use std::net::{TcpListener, TcpStream,Shutdown,IpAddr,Ipv4Addr};
use std::thread;
use std::time::Duration;
//由于tcpStream是继承了io里的file，所以需要包含
use std::io::*;
use std::mem;
use std::ptr;
extern crate comm;
use comm::pkg_desc;
use std::collections::VecDeque;

//struct ConnectInfo<'a> {
//    buffer:[u8;pkg_desc::MAX_PKG_SIZE], //存收到的数据
//    pkg_size:usize,                     //head中的pkg_size
//    recved_size:usize,                  //这个包中一共收到的数据
//    //stream:&'a TcpStream,                  //链接
//    stream: TcpStream,
//    client_ip: IpAddr,
//}
//
//impl<'a> ConnectInfo<'a> {
//    pub fn new(tcp_stream:TcpStream) -> ConnectInfo<'a> {
//        ConnectInfo {
//            buffer : [0;pkg_desc::MAX_PKG_SIZE],
//            pkg_size : 0,
//            recved_size : 0,
//            stream: tcp_stream,
//            client_ip: IpAddr::V4(Ipv4Addr::new(0,0,0,0))
//        }
//    }
//}


struct ConnectInfo {
    buffer:[u8;pkg_desc::MAX_PKG_SIZE], //存收到的数据
    pkg_size:usize,                     //head中的pkg_size
    recved_size:usize,                  //这个包中一共收到的数据
    stream: TcpStream,
    client_ip: IpAddr,
}

impl ConnectInfo {
    pub fn new(tcp_stream:TcpStream) -> ConnectInfo {
        ConnectInfo {
            buffer : [0;pkg_desc::MAX_PKG_SIZE],
            pkg_size : 0,
            recved_size : 0,
            stream: tcp_stream,
            client_ip: IpAddr::V4(Ipv4Addr::new(0,0,0,0))
        }
    }
}

fn main() {
    let head_size = mem::size_of::<pkg_desc::pkg_head>();
    if head_size != pkg_desc::HEAD_SIZE {
        println!("head_size is error:real is :{},but defined:{}", head_size,pkg_desc::HEAD_SIZE);
        return;
    }

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("conn start run.");

    //设置为非阻塞IO
    listener.set_nonblocking(true).unwrap();
    let mut pool = VecDeque::new();
    loop {
        let ret = listener.accept();
        match ret {
            Ok(stream) => {
                let mut connect = ConnectInfo::new(stream.0);
                connect.client_ip = stream.1.ip();
                connect.stream.set_nonblocking(true);
                pool.push_back(connect);
            },
            Err(e) => {
                //println!("recv error:{}",e);
            },
        }
        let mut removed = vec![];
        let mut count = 0;
        for connect in pool.iter_mut() {
            let ret = handle_client(connect);
            if ret == 1 {
                removed.push(count);
            }
            count+=1;
        }

        for remove in removed {
            println!("remove:{}",remove);
            pool.swap_remove_back(remove);
        }
    }
}

#[allow(unused_variables)]
fn handle_client(connect:&mut ConnectInfo) ->i32 {
    let mut stream:&TcpStream = &connect.stream;
    let ip = stream.peer_addr().unwrap().ip();

    //一次读1k
    let mut read_buf:[u8;1024] = [0;1024];
    let ret = stream.read(&mut read_buf);
    match ret {
        Ok(n) => {
            if n == 0 {
                println!("client :{},disconnect",ip );
                stream.shutdown(Shutdown::Both);
                return 1;
            }
            let mut pkg = pkg_desc::pkg::new();
            let mut buffer = &mut connect.buffer;
            let mut pkg_size = &mut connect.pkg_size;
            let mut total_size = &mut connect.recved_size;
            let mut next_size = 0;
            let mut valid_size = n;
            if *total_size + n > pkg_desc::MAX_PKG_SIZE {
                //这个包已经超了，可能读取到了下一个包的东西
                valid_size = pkg_desc::MAX_PKG_SIZE - *total_size;
                next_size  = n - valid_size;
            }
            buffer[*total_size..*total_size+valid_size].copy_from_slice(&read_buf[0..valid_size]);
            //unsafe { ptr::copy(&mut buffer[total_size..total_size+n],&mut read_buf[0..n],n)};
            *total_size += valid_size;
            println!("read {} bytes from {},valid {} bytes,total {} bytes",n,ip,valid_size,*total_size);
            //for x in 0..29 {
            //    print!("{}",buffer[x]);
            //}
            //println!("");
            if *total_size < pkg_desc::HEAD_SIZE {
                println!("head not read,continue");
                return 0;
            }

            if *pkg_size == 0 {
                unsafe { pkg = mem::transmute(*buffer); }
                println!("head is readed,size:{},magic:{}",pkg.head.pkg_len,pkg.head.magic);
                if pkg.head.magic != pkg_desc::MAGIC_IN_HEAD {
                    println!("magic not match,disconnect");
                    stream.shutdown(Shutdown::Both);
                    return 1;
                }
                if pkg.head.pkg_len > pkg_desc::MAX_PKG_SIZE || pkg.head.pkg_len == 0{
                    println!("pkg_len is illeage:{}",pkg.head.pkg_len);
                    stream.shutdown(Shutdown::Both);
                    return 1;
                }
                *pkg_size = pkg.head.pkg_len;
            }

            if total_size < pkg_size {
                println!("pkg not read done,continue");
                return 0;
            }
            unsafe { pkg =  mem::transmute(*buffer); }
            println!("pkg is read done,data is:");
            for x in 0..*pkg_size-pkg_desc::HEAD_SIZE {
                print!("{:?}", pkg.body.message[x]);
            }

            *total_size = 0;
            *pkg_size = 0;
            if next_size > 0 {
                //下一个包的
                buffer.copy_from_slice(&read_buf[valid_size..n]);
                *total_size += next_size;
            }
            stream.shutdown(Shutdown::Both);
            return 1;
        },
        Err(_) => {},
    }
    return 0;
}
