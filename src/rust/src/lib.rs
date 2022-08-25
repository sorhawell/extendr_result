use extendr_api::prelude::{extendr, extendr_module, list, GetSexp, IntoRobj}; // or just use extendr_api::prelude::*;
use extendr_api::NULL;

//convert rust Result into either list(ok=ok_value,err=NULL) or list(ok=NULL,err=err_string)
//use custom unwrap-function on R side or any custom code to read results and/or throw errors.
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

#[extendr]
fn rust_hello_result(is_ok: bool) -> list::List {
    let result = if is_ok {
        Ok(42i32)
    } else {
        Err("rust world error: is_ok must be set to true")
    };
    r_result_list(result)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloresult;
    fn rust_hello_result;
}
