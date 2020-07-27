pub trait Morph<Submorphs>{
    fn get_origin(&self) -> (i64,i64);
    fn set_origin(&mut self,v: (i64,i64));
    fn get_owner(&self) -> dyn Morph<Self>;
    fn add_morph (&mut self, m: Submorphs);
    fn remove_morph(&mut self,m: Submorphs);
    fn get_submorphs(&self) -> Vec<Submorphs>;
}
pub trait BoundedMorph<Submorphs>: Morph<Submorphs>{
    fn get_extent(&self) -> (i64,i64);
    fn set_extent(&mut self,v: (i64,i64));
}
pub trait RectangleLikeMorph<Submorphs,Color>: BoundedMorph<Submorphs>{
    fn get_color(&self) -> Color;
    fn set_color(&mut self,v: Color);
}