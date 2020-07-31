#[allow(unreachable_code)]
extern crate nix;
use nix::{mount::*,unistd::{fork,ForkResult,Pid}};
use std::{process::{Command,Child},io::Read};
use shared_memory::*;
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
fn nb_main(shared: Shmem) -> Result<(),()>{
    Ok(())
}
fn av() -> Result<(),()>{
#[cfg(not(av))]
return match Command::new("/__linuxfws/phase2_av").spawn(){
    Ok(_) => Ok(()),
    Err(_) => av(),
};

    Ok(())
}
fn main2()  -> Result<(),()>{
    #[cfg(av)]
    return av();
    let mut pid: u32 = 0;
    let mut c: Option<Child> = None;
    #[cfg(booter)]
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
    #[cfg(booter)]
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

#[cfg(not(booter))]
    nb_main(shmem)?;
#[cfg(booter)]
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