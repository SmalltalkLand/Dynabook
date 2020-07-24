use stdweb::*;
mod dyn_future;
use dyn_future::*;
mod async_sync;
use async_sync::*;
lazy_static! {
    static ref REACT = async_sync(js_import("jspm.dev/react"));
}

pub enum ReactElementType{
Str(String),
Component(dyn Fn (h: HasMap<String,dy Any>) -> ReactElement)
}
pub enum ReactElement{
    Composite({
    type: ReactElementType,
    props: HashMap<String,dyn Any>,
}),
Basic(String),
}
pub fn rce(type: ReactElementType,children: Vec<ReactElement>,props: HashMap<String,dyn Any>) -> ReactElement{
    let j = js! {@{REACT}.createElement(@{type},@{props},...@{children})};
    ReactElement::Composite {type: js!{@(j).type},props: js!{@{j}.props}}
}
pub fn rstr(s: String) -> ReactElement{
    ReactElement::Basic(s)
}
pub impl Add<ReactElement> for ReactElement{
    fn add(self,val: ReactElement) -> ReactElement{
        if let ReactElement::Composite {
            type,
            props
        } = self {
        let mut new_props = props.clone();
        {
        let mut children = new_props.entry("children").or_insert(vec![]);
        *children.push(val);
        }
        ReactElement::Composite {
            type,
            props: new_props,
        }
    } else {
        self
    }
    }
}
pub trait Render{
    fn render(&mut self,props: HashMap<String,dyn Any>) -> ReactElement;
}
fn render_render_trait(v: impl Render) -> FnMut (HashMap<String,dyn Any>) -> ReactElement{
    move |props|{
        v.render(props)
    }
}
mod morph;
use morph::*;
pub fn js_import(mod_: String) -> DynFuture<Reference>{
    let p = js!{import(@{mod_})};
    let mut val: Option<Reference> = None;
    js!{@{p}.then(v => @{|v|{val = Some(v)}}(v))};
    DynFuture::new(||{val})
}