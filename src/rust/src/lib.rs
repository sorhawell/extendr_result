use extendr_api::prelude::{
    eval_string, extendr, extendr_module, list, GetSexp, IntoRobj, Robj, R,
}; // or just use extendr_api::prelude::*;
use extendr_api::{rprintln, NULL};

//convert any extendr exportalbe Result into either list(ok=ok_value,err=NULL) or list(ok=NULL,err=err_string)
//... where error implements Display. This error traitbound could be resolved on R side instead.
pub fn r_result_list<T, E>(x: std::result::Result<T, E>) -> list::List
where
    T: std::fmt::Debug + IntoRobj,
    E: std::fmt::Debug + std::fmt::Display,
{
    if x.is_ok() {
        let v = vec![1, 2, 3];
        let ret = list!(ok = x.unwrap().into_robj(), err = NULL, vec = &v);
        //println!("{:?}", v);
        ret
    } else {
        list!(ok = NULL, err = x.unwrap_err().to_string())
    }
}

pub fn r_result_condition<T, E>(x: std::result::Result<T, E>) -> Robj
where
    T: std::fmt::Debug + IntoRobj,
    E: std::fmt::Debug + std::fmt::Display,
{
    if x.is_ok() {
        x.unwrap().into_robj()
    } else {
        let err_call_string = format!("simpleError(message = '{}')", x.unwrap_err().to_string());
        let robj = extendr_api::eval_string(&err_call_string).unwrap();
        robj
    }
}

//some function using the r_result_list generic function to export
///@export
#[extendr]
fn rust_helloresult(is_ok: bool) -> list::List {
    //some silly logic to emulate Result
    let result = if is_ok {
        Ok(42i32)
    } else {
        Err("rust world error: is_ok must be set to true")
    };

    //instead of returning Result<T,E> then wrap like this and return list::List
    r_result_list(result)
}

///@export
#[extendr]
fn rust_hellocondition(is_ok: bool) -> Robj {
    //some silly logic to emulate Result
    let result = if is_ok {
        Ok(42i32)
    } else {
        Err("rust world error: is_ok must be set to true")
    };

    //instead of returning Result<T,E> then wrap like this and return list::List
    r_result_condition(result)
}

extendr_module! {
    mod helloresult;
    fn rust_helloresult;
    fn rust_hellocondition;
}
