use bootloader::BootInfo;
use crate::q::*;
mod asm::*;
use alloc::collections::HashMap;
mod task::*;
use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll},
};
use crate::sq::*;
mod squeak-flags::*;
mod k::*;
static mut ffun: Option<(fn(*mut ()),*mut ())> = None;
pub static mut hcall_map: HashMap<u64,dyn Fn()> =  HashMap::new();
pub static mut qs: Option<&dyn Q> = None;
pub static mut is_kernel = false;
pub fn get_is_kernel() -> bool{unsafe{is_kernel}}
pub fn hcall_init(q: &mut &'static dyn Q){
    q.sandbox(0xF7,||{unsafe{
        let p = pop!();
    let hp = hcall_map.get(p);
    match hp{
        None => {push!(0);push!(p); return;},
        Some(v) => {
            push!(1);
            match core::panic::catch_unwind(||v()){
                Ok(_) => {push!(0)},
                Err(error) => {push!(error)},
            }
        }
    }
    }});
}
pub fn q_init(b: &'static BootInfo,q: &'static dyn Q){
let mut q = mutt(q);
q.sandbox(0xF5,||{unsafe{
let o = is_kernel;
is_kernel = true;
match ffun{Some(v){
let (f,v) = v;
q.sandbox(0xF6,||{unsafe{
    let oo = is_kernel;
    is_kernel = (pop!()) == 1;
    (pop!() as fn(u64))(pop!());
    is_kernel = oo;
}});
f(v);
q.pop_sandbox();
}};
is_kernel = o;
}});
hcall_init(&mut q);
return q;
}
async fn nul(){

}
pub async fn fetch_squeak() -> &dyn Future<Option<fn () -> (SqueakRunning)>>{return None}
pub fn sqs(mq1: &mut dyn Q) -> &dyn Fn(&mut dyn Q){
    return | mq2 |{

    }
}
pub async fn boot_init_phase2(b: &'static BootInfo,executor: &mut task::executor::Executor,q: Option<&dyn Q>,start: &fn ()) -> &dyn Future<()>{
await nul();
let mut srun: Option<SqueakRuning> = None;
let squeak = await fetch_squeak();
match squeak{
    Some(s) => {
        srun = Some(s());
    },
    None => {

    }
};
match q{
    Some(q) => {
        match srun{
            Some(s) => {
                let mut sqsv: Box<Option<dyn Fn ()>> = Box::new(None);
                s.add_hook(||{
                    let r = sqsv.deref_mut();
                    r = Some(sqs(&q));
                },||{
                    let d = sqsv.deref();
                    match d{Some(dd) => dd(&q),None => {}};
                    let r = sqsv.deref_mut();
                    r = None;
                });
            },
            None => {}
        }
    },
    None => {
        match srun{
            Some(s) => {
                let v: Vec<String,'static> = vec!["-no-q","--addr",format!("addr:{}",start as *mut () as u64),"--bts",format!("bts:{}",&show_boot_taskbar as *const bool as u64)];
                s.set_args(v.into_boxed_slice().deref());
            },
            None => {

            }
        }
    }
}
}
pub fn boot_init(b: &'static BootInfo,start: &fn ()){
//boot function for Q to run apps
let mut q_done = false;
let mut q: Option<&dyn Q> = None;
if unsafe{*(b.physical_memory_offset + 23)} == 1{q = Some(q_init(b,&AsmQ{})); q_done = true};
if unsafe{*(b.physical_memory_offset + 23) == 2 && *(b.physical_memory_offset + 24) != 0}{q = Some(q_init(b,unsafe{core::mem::transmute<*mut (),&dyn Q>((b.physical_memory_offset + 24))})); q_done = true};
    let mut ex = task::executor::Executor::new();
    ex.spawn(task::Task::new(boot_init_phase2(b,unsafe{mutt(&ex)},q,start)));
    unsafe{qs = q;};
    ex.run();
}