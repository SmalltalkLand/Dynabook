#[allow(unreachable_code)]
extern crate nix;
use nix::{mount::*,unistd::{fork,ForkResult,Pid}};
use std::process::{Command,Child};
use shared_memory::*;
fn create_child_proc(c: Option<Child>) -> Result<Option<Pid>,()>{
    Ok(match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            Some(child)
        }
        Ok(ForkResult::Child) => {
            match c{
                None => None,
                Some(mut c2) => {
                    match c2.wait(){
                        Ok(_) => {},
                        Err(_) => {}
                    };
                    None
                }
            }
        },
        Err(_) => return Err(())})
}
fn main2()  -> Result<(),()>{
    let mut pid: u32 = 0;
    let mut c: Option<Child> = None;
    match Command::new("/__linuxwfs/phase2").spawn(){
        Ok(c2) => {pid = c2.id(); c = Some(c2);},
        Err(_) => return Err(())
    };
    let shmem = match ShmemConf::new().size(4096).flink("event_mapping").create() {
        Ok(m) => m,
        Err(ShmemError::LinkExists) => ShmemConf::new().flink("event_mapping").open()?,
        Err(e) => return Err(()),
    };
    match c{
        None => return Err(()),
        Some(c2) => {
            match c2.stdin{
                None => {return main2();},
                Some(in_) => {write!(in_,"{}",shmem.as_ptr() as u64)}
            }
        }
    };
    let child_proc = create_child_proc(c)?;
    Ok(())
}
fn main() -> Result<(),()>{
    println!("Loading...");
    match mount(Some("/dev/linuxwfs"),"/__linuxwfs",Some("ext4"),MsFlags::empty(),Some("")){
        Ok(()) => {},
        Err(_) => return Err(()),
    };
    return main2();
}