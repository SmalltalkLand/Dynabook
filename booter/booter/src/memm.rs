pub unsafe fn mem_at(adress: u64) -> u64{
    *(adress as *const u64)
}
pub unsafe fn mem_put(adress: u64,val: u64) -> Result<(),()>{
    *(adress as *mut u64) = val;
    Ok(())
}