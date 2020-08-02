pub struct SecureStruct<T>{
    value: T
}
pub struct NonSecure<T>{
    value: T
}
pub trait Secure{

}
use std::{vec::*,cell::*,rc::*};
impl<T> Secure for SecureStruct<T>{

}
impl<T: Secure> Secure for &T{

}
impl<T: Secure> Secure for &mut T{

}
impl<T: Secure,U> Secure for Result<T,U>{

}
impl<T: Secure> Secure for Vec<T>{

}
impl<T: Secure> Secure for UnsafeCell<T>{

}
impl<T: Secure> Secure for RefCell<T>{

}
impl<T: Secure> Secure for Rc<T>{

}
impl<T: Secure,U: Secure> Secure for (T,U){

}
impl<T: Secure,U: Secure,V: Secure> Secure for (T,U,V){

}