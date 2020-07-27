pub static mut is_kernel: bool = false;
pub fn get_is_kernel() -> bool{unsafe{is_kernel}}