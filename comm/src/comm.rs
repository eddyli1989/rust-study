pub struct Err {
    pub code:i32,
    pub desc:String
}

pub type Ret<T> = Result<T,Err>;