use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::process;
fn main() {
    
    fn open()->std::io::Result<()> {
        let args: Vec<_> = env::args().collect(); //类型推导能推导出来吗?
        if args.len() < 2 {
            println!("please input a file name");
            process::exit(1);
        }
        
        //arg[1]必须使用引用，不然编译不过
        //try!成功返回对象，失败直接在函数级别返回Result,所以open的返回值类型必须是Result
        //所以这句表达式没办法在main函数中直接写，因为main函数的返回值是（）
        let mut f = try!(File::open(&args[1]));

        //let buf_len = 10;
        let mut buffer = [0; 10];

        loop {
            let ret = f.read(&mut buffer[..]);
            match ret {
                Ok(n) => {
                    if n == 0 {
                        process::exit(0);
                    }
                    else {
                        let content = String::from_utf8_lossy(&buffer);
                        println!("{}",content );
                    }
                },
                Err(e) => panic!("error reading file: {:?}", e),
            }
        }
        //Ok(())
    }
    let ret = open();
    match ret {
        Err(e) => println!("open or read error:{}",e),
        Ok(_) => {}
    }

}
