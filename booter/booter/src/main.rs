#[allow(unreachable_code)]
extern crate nix;
#[cfg(feature = "web")]
extern crate stdweb;
#[cfg(feature = "web")]
use stdweb::*;

use nix::{mount::*,unistd::{fork,ForkResult,Pid}};
use std::{process::{Command,Child},io::Read,rc::*};

use shredder::{
    number_of_active_handles, number_of_tracked_allocations, run_with_gc_cleanup, Gc, Scan,
};
use shared_memory::*;
mod memm;
mod lf;
fn create_child_proc(c: Option<Child>) -> Result<Option<Pid>,()>{
    Ok(match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            Some(child)
        },
        Ok(ForkResult::Child) => {
            match c{
                None => None,
                Some(mut c2) => {
                    None
                }
            }
        },
        Err(_) => return Err(()),
        })
}
fn nb_main(shared: Option<Shmem>) -> Result<(),()>{
    Ok(())
}
macro_rules! trap_ {
    ($tag: ident,$trap: ident) => {
        {
        let t = "$tag";
        let t2 = $trap;
        match t{"$tag" => {},_ => {return Err(())}};
        let mut t3 = t2;
        t3
        }
    };
}
struct Virus<'a>{
    name: &'a str,
    solver: Box<&'a mut dyn FnMut () -> VirusResolver<'a>>
}
struct VirusResolver<'a>{
target_mem_adresses: Vec<*mut u64>,
target_mem_vals: Vec<Option<(*mut u64,u64)>>,
virus_ref: Rc<Virus<'a>>,
}
fn init_resolver(res: &mut VirusResolver){
    res.target_mem_vals = Vec::new();
    for adress in &res.target_mem_adresses{
        (&mut res.target_mem_vals).push(Some((*adress,unsafe{**adress})))
    }
}
fn av() -> Result<(),()>{
    let trap: &'static [u8] = &[0u8];
    let trap = trap_!(test,trap);
    let trap = trap_!(test2,trap);
    let displacer = 1;
    let trap = trap_!(test3,trap);
    let trap = trap_!(ᒻ긢⳯課𖧭,trap);
#[cfg(not(feature = "av"))]
return match Command::new("/__linuxfws/phase2_av").spawn(){
    Ok(_) => Ok(()),
    Err(_) => av(),
};
let shmem = match (match ShmemConf::new().size(4096).flink("stuff").create() {
    Ok(m) => Ok(m),
    Err(ShmemError::LinkExists) => ShmemConf::new().flink("stuff").open(),
    Err(e) => return Err(()),
}){
    Ok(v) => v,
    Err(_) => return Err(())
};
    Ok(())
}
fn main2()  -> Result<(),()>{
    #[cfg(feature = "av")]
    return av();
    #[cfg(feature = "web")]
    {
        let loader = js!{await import("/bootloader.js")};
        let jscall = |obj: Reference,args: Vec<Reference>| -> Reference{js!{return @{obj}(...@{args})}};
        let jsval = |st| {
            js!{return eval(@{st})}
        };
        
        return nb_main(None);
    };
    let mut pid: u32 = 0;
    let mut c: Option<Child> = None;
    #[cfg(feature = "booter")]
    match Command::new("/__linuxwfs/phase2").spawn(){
        Ok(c2) => {pid = c2.id(); c = Some(c2);},
        Err(_) => return Err(())
    };
    let shmem = match (match ShmemConf::new().size(4096).flink("stuff").create() {
        Ok(m) => Ok(m),
        Err(ShmemError::LinkExists) => ShmemConf::new().flink("stuff").open(),
        Err(e) => return Err(()),
    }){
        Ok(v) => v,
        Err(_) => return Err(())
    };
    #[cfg(feature = "booter")]
    match c{
        None => return Err(()),
        Some(c2) => {
            match c2.stdin{
                None => {return main2();},
                Some(in_) => {write!(in_,"{}",shmem.as_ptr() as u64)}
            };
            match c2.stdout{
                None => None,
                Some(mut v) => {
                    let mut str_ = String::new();
                    v.read_to_string(&mut str_);
                    if str_.contains("Fe2O3"){av()?; return main2();};
                    None
                }
        }
    }
}

#[cfg(not(feature = "booter"))]
    nb_main(Some(shmem))?;
#[cfg(feature = "booter")]
    let child_proc = create_child_proc(c)?;
    Ok(())
}
fn main() -> Result<(),()>{
    println!("Loading...");
    #[cfg(feature = "booter")]
    match mount(Some("/dev/linuxwfs"),"/__linuxwfs",Some("ext4"),MsFlags::empty(),Some("")){
        Ok(()) => {},
        Err(_) => return Err(()),
    };
    return main2();
}