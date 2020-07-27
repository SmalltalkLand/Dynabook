#![feature(llvm_asm)]
#![no_std]
#![feature(wake_trait)]
#![feature(never_type)]
extern crate alloc;
#[macro_use]
extern crate lazy_static;
pub mod q;
pub mod task;
pub mod sq;
pub mod catch_panics;
pub mod k;
pub mod shorten_lifetime;
pub mod blobbi;
#[cfg(feature = "boot")]
pub mod boot;
#[cfg(feature = "web")]
pub mod web;
pub fn top_bar(props: HashMap<String,dyn Any>){
    use web::*;
    return rce("div", vec![], HashMap::new());
}
#[cfg(test)]
mod tests {

}