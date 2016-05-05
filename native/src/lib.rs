#[macro_use]
extern crate neon;

mod js {
    pub mod objects;
}
use js::objects::*;

register_module!(m, {
    try!(m.export("return_js_object_with_integer", return_js_object_with_integer));
    Ok(())
});
