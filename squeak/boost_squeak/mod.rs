use gc;
#[cfg(feature = "web")]
use crate::web::*;
pub fn isPointerS(v: u64) -> bool{
    v % 2 == 0
}
pub fn getLenS(v: u64) -> u64{
    v / 4 % 64
}
pub struct SVal{
    pointerVal: *const u64,
    len: *const u64,
}
pub impl SVal{
fn track(self,mark: dyn Fn(dyn Any)){
    for i in (0..*self.len){
        if isPointerS(*self.pointerVal + i){
            mark(SVal {
                pointerVal: core::mem::transmute<u64,*const u64>((*self.pointerVal + i) + 1),
                len: getLenS(*core::mem::transmute<u64,*const u64>((*self.pointerVal + i)))
            })
        }
    }
}
}
pub impl gc::Trace for SVal{
    custom_trace!(sef,{
        sef.track(mark);
    })
}
pub impl gc::Finalize for SVal{
    fn finalize(&mut self){
        unsafe{
        let p = core::mem::transmute<u64,*const u64>(*self.pointerVal - 1);
        core::alloc::dealloc(p,Layout::from_size_align_unchecked(*self.len,4));
        }
    }
}
#[cfg(feature = "web")]
pub impl Render for SVal{
    fn render(&mut self) -> ReactElement{
        let mut l = vec![];
        self.track(|v|{l.push(v.render());});
        return rce('div',HashMap::new(),l)
    }
}