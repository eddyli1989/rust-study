#[allow(dead_code)]
pub const HEAD_SIZE:usize = 24;
pub const MAX_BODY_SIZE: usize = 64*1024;
pub const MAGIC_IN_HEAD: i32 = 9527;
pub const MAX_PKG_SIZE: usize = MAX_BODY_SIZE + HEAD_SIZE;

pub struct PkgHead {
    pub magic: i32,
    pub pkg_len: usize,
    pub client_id:i64,
}

impl PkgHead {
    pub fn new() -> PkgHead {
        PkgHead {
            magic : 0,
            pkg_len : 0,
            client_id : 0,
        }
    }
}

pub struct PkgBody {
    pub message:[u8;MAX_BODY_SIZE]
}

impl PkgBody {
    pub fn new() -> PkgBody {
        PkgBody {
            message : [0; MAX_BODY_SIZE]
        }
    }
}

pub struct Pkg {
    pub head: PkgHead,
    pub body: PkgBody,
}

impl Pkg {
    pub fn new() -> Pkg {
        Pkg {
            head : PkgHead::new(),
            body : PkgBody::new(),
        }
    }
}