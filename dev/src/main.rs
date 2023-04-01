#![allow(unused_variables)] // 在构思过程中放宽编译器警告

#[derive(Debug)] // 允许使用println!宏（以及其他fmt!等宏）来打印File，std::fmt::Debug这个trait与此宏中的迷你语言{:?}结合在一起，就可以将File表示为可打印的字符串了。
struct File {
    name: String,
    data: Vec<u8>,
}

#[allow(dead_code)] // 放宽一个未使用函数的编译器警告, 函数未使用，默认是#[warn(dead_code)]，也就是未使用给警告
fn read() {}


fn main() {
    print!("Hi");
}