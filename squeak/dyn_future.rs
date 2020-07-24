pub struct DynFuture<T,Result_> where T: FnMut () -> Option<Result_>{
poll_func: T,
}
pub impl Future for DynFuture<T,Result_>{
    type Output = Result_;
    fn poll(Pin<&mut self>,ctxt: &mut Context) -> Poll<Result_>{
        match self.poll_func{
            Some(v) => Poll::Ready(v),
            None => Poll::Pending,
        }
    }
}
pub impl DynFuture<T,Result_>{
    fn new(func: T) -> Self{
        Self{
            poll_func: func
        }
    }
}