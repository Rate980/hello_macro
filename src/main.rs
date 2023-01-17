use hello_macro_derive::types;
use ts_rs::TS;
type Ret = u8;

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(TS)]
#[ts(export)]
#[allow(dead_code)]
struct User<T, L> {
    user_id: i32,
    first_name: String,
    last_name: String,
    x: (T, L),
    y: Ret,
}

#[types]
#[allow(dead_code, unused_variables)]
fn foo(x: u8, aaa: (u8, Ret)) -> Vec<u8> {
    todo!()
}

fn main() {}
