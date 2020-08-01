use shredder::{
    number_of_active_handles, number_of_tracked_allocations, run_with_gc_cleanup, Gc, Scan
};
pub
struct WithLifetime<'lt,T> (std::marker::PhantomData<&'lt T>);
impl<'lt,T> WithLifetime<'lt,T> {
    pub
    fn produces (t: Box<dyn Fn () -> &'lt mut T>)
      -> &'lt mut T
    {
        t()
    }
    pub fn denies_dropping(source: &'lt mut T) -> &'static mut T{
        unsafe{std::mem::transmute(source)}
    }
    pub
    fn consumes (t: &'lt mut T)
    {
        std::mem::drop(t);
    }
}
impl <'lt,T: Scan + Send> WithLifetime<'lt,T>{
    pub fn garbage_collects (source: &'lt mut T) -> Gc<&'static mut T>{
        Gc::new(Self::denies_dropping(source))
    }
}