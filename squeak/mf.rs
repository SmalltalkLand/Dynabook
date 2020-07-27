pub fn merge_futures<T,U>(fa: impl Future<T>,fb: impl Future<U>) -> DynFuture<(T,U)>{
    let mut a: Option<T>, b: Option<U>;
    DynFuture::new(||{
        match a {
            Some(v) => match b{
                Some(v2) => Some((v,v2)),
                None => {match fb.poll(){
                    Poll::Ready(bb) => b = Some(bb),
                    Poll::Pending => {}}; None}
                },
            None => {
                match fa.poll(){
                    Poll::Ready(aa) => a = Some(a),
                    Poll::Pending => {}
                };
                None
            }
            }
    })
}