pub trait IEnclave{
    fn get_data(&self,key: u64) -> Vec<u64>;
    fn set_data(&mut self,key: u64,val: Vec<u64>);
}