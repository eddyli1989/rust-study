use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

const FILE_PATH:&'static str = "../conf/conf.json";
#[derive(Debug)]
pub struct Config {
    file_content:String,
    json_obj: Json
}

lazy_static! {
    static ref CONFIG: Config = Config::new();
}

impl Config {

    fn new() -> Config {
        println!("new Config");
        let mut m = Config { file_content: String::new(),json_obj: Json::Null };
        let mut file = File::open(FILE_PATH).unwrap();
        file.read_to_string(&mut m.file_content).unwrap();
        m.json_obj = Json::from_str(&m.file_content).unwrap();
        println!("Config init done");
        m
    }

    pub fn init(&self)  {
        println!("start init Config");
    }

    pub fn instance<'a>() -> &'a Config{
        &CONFIG
    }

    pub fn get_inst_id(&self) -> i32 {
        self.json_obj.find("inst_id").unwrap().as_i64().unwrap() as i32
    }

    pub fn get_shm_addr(&self) -> &str {
        self.json_obj.find("shm_addr").unwrap().as_string().unwrap()
    }

    pub fn get_app_type(&self) -> &str {
        self.json_obj.find("app_type").unwrap().as_string().unwrap()
    }
}

