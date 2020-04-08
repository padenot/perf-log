use perf_log::{log_1, log_2, log_3, log_4};
use std::ffi::CStr;

fn main() {
    log_1("asdasd", |s: &CStr| { println!("{:?}", s.to_string_lossy().into_owned()) });
    log_2("asdasd", |s: &CStr| { println!("{:?}", s.to_string_lossy().into_owned()) });
    log_3("asdasd", |s: &CStr| { println!("{:?}", s.to_string_lossy().into_owned()) });
    log_4("asdasd", |s: &CStr| { println!("{:?}", s.to_string_lossy().into_owned()) });
}
