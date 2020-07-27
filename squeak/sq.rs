use alloc::string::*;
pub struct SqueakRunning{
    kill: fn (),
    add_hook: fn (dyn Fn(),dyn Fn()),
    set_args: fn ([String])
}