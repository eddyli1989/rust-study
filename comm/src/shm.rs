use config::Config;
use comm::Ret;
use comm::Err;
use pkg_desc::MAX_PKG_SIZE;

use redis;
use redis::Commands;

pub const CONN:&'static str = "CONN";
pub const ZONE:&'static str = "ZONE";
const BUS_PRE:&'static str = "BUS";
const DOT:&'static str = "->";
const SEP:&'static str = ":";

#[derive(Debug)]
pub struct ShmMgr {
    client: redis::Client, //redis进程

}

lazy_static! {
    static ref SHMMGR: ShmMgr = ShmMgr::new();
}

impl ShmMgr {

    fn new() -> ShmMgr {
        println!("start init ShmMgr");
        let mut m = ShmMgr {
            client: redis::Client::open(Config::instance().get_shm_addr()).unwrap()
        };
        println!("shm addr:{}",Config::instance().get_shm_addr());
        println!("ShmMgr init done");
        m
    }

    pub fn init(&self)  {
        //empty
    }

    fn get_dst_addr(&self,src:&str) -> String {
        match src {
            CONN => {
                let inst_id = Config::instance().get_inst_id();
                format!("{}:{}{}:->{}{}",BUS_PRE,CONN,inst_id,ZONE,inst_id)
            },
            _ => {
                String::new()
            }
        }
    }

    pub fn instance<'a>() -> &'a ShmMgr {
        &SHMMGR
    }

    pub fn send(&self,buf:Vec<u8>) -> Ret<()> {
        let mut connection = try!(self.client.get_connection().map_err(|redis_err| {
            Err { code:redis_err.kind() as i32, desc:redis_err.category().to_string() }
        }));
        let key = self.get_dst_addr(Config::instance().get_app_type());
        connection.lpush(key,buf).map_err(|redis_err| {
            Err {code:redis_err.kind() as i32, desc:redis_err.category().to_string()}
        })
    }
}