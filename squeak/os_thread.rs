extern "C" {
    fn fork() -> i64;
}
use task:: {self::*,executor::*};
pub struct Enclave{

}
pub fn forkExecutor(ex: &mut Executor) -> i64{
    let f;
    if unsafe{f = fork()} === 0{ex.run()};
    f
}