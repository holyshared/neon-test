#[macro_use]
extern crate neon;

mod js {
    pub mod objects;
    pub mod arrays;
    pub mod arguments;
}
use js::objects::*;
use js::arrays::*;
use js::arguments::*;

register_module!(m, {
    try!(m.export("return_js_object_with_integer", return_js_object_with_integer));
    try!(m.export("return_js_array", return_js_array));
    try!(m.export("arguments_check", arguments_check));
    Ok(())
});
