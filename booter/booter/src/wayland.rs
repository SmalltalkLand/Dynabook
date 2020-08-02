use wayland_server::*;
static mut display: Option<Display> = None;
pub fn init(){
    unsafe{display = Some(Display::new())}
}
pub fn is_inited() -> bool{
    match unsafe{display}{
        Some(_) => true,
        None => false
    }
}
