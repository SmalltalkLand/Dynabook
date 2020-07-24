static mut rand;
pub static mut ads: HashMap<u64,dyn Fn(dyn Any)> = HashMap::new();
#[cfg(feature = "web")]
use crate::web::*;
pub fn rce_hook(elem: ReactElement) -> ReactElement{
let b = [0u8; 2];
rand.fill_bytes(b);
if b[0] == 0 & b[1] == 0{
    rce("div",HashMap::new(),vec![elem])
}else{
    elem
}
}