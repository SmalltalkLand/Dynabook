pub fn pfinally(f: &dyn Fn (),f2: &dyn Fn ()) -> dyn Fn(){
    move ||{
        use core::panic;
        let r = panic::catch_unwind(f);
        f2();
        if let Err(err) = result {
            panic::resume_unwind(err);
        }
    }
}