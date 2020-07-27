pub fn async_sync<T>(v: Future<T>) -> T{
    while let Poll::Pending = v.poll(){

    };
    match v.poll(){
        Poll::Pending => async_sync(v),
        Poll::Ready(v) => v,
    }
}