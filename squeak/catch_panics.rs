#[cfg(feature = "boot")]
use boot;
use core::panic::PanicInfo;
use alloc::string::*;
use crate::k::*;
use alloc::boxed::*;
use alloc::vec::*;
use crate::shorten_lifetime::*;
pub static mut alert: Option<fn (String)> = None;
pub enum Error{
    unrecoverable_panic(!),
    group_error(Vec<Error>)
}
pub static mut hooks: Vec<Box<dyn Fn (&&PanicInfo) -> Result<(),Error>>> = Vec::new();
pub fn on_panic(info: &PanicInfo,quit: Box<dyn Fn () -> Result<!,!>>)-> Result<(),Error> {
if get_is_kernel(){match quit(){Ok(quit) => return Err(Error::unrecoverable_panic(quit)},Err(quit) => return Err(Error::unrecoverable_panic(quit))}};
unsafe{is_kernel = true;};
let h: Vec<Result<(),Error>> = (unsafe{&mut hooks}).iter().map(|h|h(&info)).collect();
let herr: Vec<&Error> = h.iter().filter(|v|match v{Ok(_) => false,Err(_) => true}).map(|v|match v{Ok(_) => None,Err(v) => Some(v)}).map(|v|shorten_invariant_lifetime(&mut R {val: v}).val.unwrap()).collect();
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        if let Some(alert_) = unsafe{alert}{
        //alert_(format!("panic occurred: {:?}", s));
        alert_(String::from("panic occurred"));
        }
    } else {
        if let Some(alert_) = unsafe{alert}{
        alert_(String::from("panic occurred"));
    }
    };
    #[cfg(feature = "boot")]
    {
        let q = unsafe{boot::qs};
        if let Some(q) = q{

        }else{
            unsafe{is_kernel = false;};
            match quit(){Ok(quit) => return Err(Error::unrecoverable_panic(quit)},Err(quit) => return Err(Error::unrecoverable_panic(quit))};
            unsafe{is_kernel = true;};
        }
    };
unsafe{is_kernel = false;};
for v in herr.iter(){return Err(Error::group_error(herr))};
    return Ok(());
}