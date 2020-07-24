use alloc::boxed::*;
pub trait Q{
    fn sandbox(&mut self,instruction: u8,block: Box<dyn FnOnce()>);
    fn pop_sandbox(&mut self);
}
pub struct AsmQ{}
impl Q for AsmQ{
    fn sandbox(&mut self,ins: u8,block: Box<dyn FnOnce()>){
        unsafe{
            llvm_asm!("f3 $1" :: "1"(ins));
            block();
            llvm_asm!("f2");
        }
    }
    fn pop_sandbox(&mut self){
        unsafe{
            llvm_asm!("f1");
        }
    }
}