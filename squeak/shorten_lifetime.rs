pub struct R<'l,T>{
    pub val: T<'l>,
}
pub unsafe fn shorten_invariant_lifetime<'b, 'c, S>(r: &'b mut R<S,'static>)
                                             -> &'b mut R<S,'c> {
    core::mem::transmute::<&'b mut R<S,'static>, &'b mut R<S,'c>>(r)
}