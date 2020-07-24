macro_rules! pop{() => {
    let v: u64;
    asm!("pop $1":"1"(v));
    v
}}
macro_rules! reg{($reg:ident) => {
    let v: u64;
    asm!("mov $reg, $1":"1"(v));
    v
}}
macro_rules! push{($val:expr) => {
    asm!("push $1":"1"(val));
}}