//模块要引用外部的包，必须在这个文件里声明，不能在子模块里声明
extern crate rustc_serialize;

#[macro_use]
extern crate lazy_static;
extern crate redis;


mod comm;
pub mod pkg_desc;
pub mod config;
pub mod shm;