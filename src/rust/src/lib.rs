use extendr_api::prelude::{extendr, extendr_module, list, GetSexp, IntoRobj}; // or just use extendr_api::prelude::*;
use extendr_api::NULL;


//convert any extendr exportalbe Result into either list(ok=ok_value,err=NULL) or list(ok=NULL,err=err_string)
//... where error implements Display. This error traitbound could be resolved on R side instead.
pub fn r_result_list<T, E>(x: std::result::Result<T, E>) -> list::List
where
    T: std::fmt::Debug + IntoRobj,
    E: std::fmt::Debug + std::fmt::Display,
{
    if x.is_ok() {
        list!(ok = x.unwrap().into_robj(), err = NULL)
    } else {
        list!(ok = NULL, err = x.unwrap_err().to_string())
    }
}

//some function using the r_result_list generic function to export
#[extendr]
fn rust_hello_result(is_ok: bool) -> list::List {
    //some silly logic to emulate Result
    let result = if is_ok {
        Ok(42i32)
    } else {
        Err("rust world error: is_ok must be set to true")
    };

    //instead of returning Result<T,E> then wrap like this and return list::List
    r_result_list(result)
}

extendr_module! {
    mod helloresult;
    fn rust_hello_result;
}
