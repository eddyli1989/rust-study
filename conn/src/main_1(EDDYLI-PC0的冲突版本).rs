#[allow(dead_code)]
use std::net::{TcpListener, TcpStream,Shutdown};
use std::thread;
use std::time::Duration;
//由于tcpStream是继承了io里的file，所以需要包含
use std::io::*;
use std::mem;
use std::ptr;
extern crate comm;
use comm::pkg_desc;
//TODO:改为单进程单线程
//TODO:设置为非阻塞
#[allow(unused_variables)]
fn handle_client(mut stream:TcpStream) {
    let ip = stream.peer_addr().unwrap().ip();
    println!("recv a request,from {}",ip);
    let one_seconds = Duration::new(1, 0); //10ms
    let _ = stream.set_read_timeout(Some(one_seconds));

    let mut buffer:[u8;pkg_desc::MAX_PKG_SIZE] = *(Box::new([0;pkg_desc::MAX_PKG_SIZE]));
    let mut head_read = false;
    let mut pkg = *Box::new(pkg_desc::pkg::new());
    let mut pkg_size = 0;
    let mut total_size = 0;
    loop {
        //一次读1k
        let mut read_buf:[u8;1024]=[0;1024];
        let ret = stream.read(&mut read_buf);
        match ret {
            Ok(n) => {
                if n == 0 {
                    println!("client :{},disconnect",ip );
                    break;
                } else {
                    let mut next_size = 0;
                    let mut valid_size = n;
                    if total_size + n > pkg_desc::MAX_PKG_SIZE {
                        //这个包已经超了，可能读取到了下一个包的东西
                        //how?
                        valid_size = pkg_desc::MAX_PKG_SIZE - total_size;
                        next_size  = n - valid_size;
                    }
                    buffer[total_size..total_size+valid_size].copy_from_slice(&read_buf[0..valid_size]);
                    //unsafe { ptr::copy(&mut buffer[total_size..total_size+n],&mut read_buf[0..n],n)};
                    total_size += valid_size;
                    println!("read {} bytes", valid_size);
                    if !head_read {
                        if total_size < head_size {
                            println!("head not read continue read");
                            continue;
                        } else {
                            //head is readed
                            head_read = true;

                            unsafe { pkg.head = mem::transmute_copy(&buffer); }
                            println!("head is readed,size:{},magic:{}",pkg.head.pkg_len,pkg.head.magic);
                            if pkg.head.magic != pkg_desc::MAGIC_IN_HEAD {
                                println!("magic not match,disconnect");
                                stream.shutdown(Shutdown::Both);
                                break;
                            }
                            if pkg.head.pkg_len > pkg_desc::MAX_PKG_SIZE || pkg.head.pkg_len == 0{
                                println!("pkg_len is illeage:{}",pkg.head.pkg_len);
                                stream.shutdown(Shutdown::Both);
                                break;
                            }
                            pkg_size =  pkg.head.pkg_len;
                        }
                    }
                    if !head_read {continue;}

                    if total_size < pkg_size {
                        println!("pkg not read done,continue");
                        continue;
                    } else {
                        //pkg is readed
                        head_read = false;
                        total_size = 0;
                        pkg_size = 0;
                        unsafe { pkg =  mem::transmute_copy(&buffer); }
                        buffer = [0; pkg_desc::MAX_PKG_SIZE];
                        if next_size > 0 {
                            //下一个包的
                            buffer.copy_from_slice(&read_buf[valid_size..n]);
                            total_size += next_size;
                        }
                        println!("pkg is read done,data is:");
                        for x in 0..pkg_size-pkg_desc::HEAD_SIZE {
                            print!("{:?}", pkg.body.message[x]);
                        }
                        stream.shutdown(Shutdown::Both);
                    }


                }
            },
            Err(_) => {},
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
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn( move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { println!("connect error:{}",e)/* connection failed */ }
        }
    }
    // close the socket server
    drop(listener);
}
