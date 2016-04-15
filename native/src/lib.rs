#[macro_use]
extern crate neon;

use neon::vm::{ Call, JsResult };
use neon::js::{ JsString, Variant };

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node").unwrap())
}

fn hello2(call: Call) -> JsResult<JsString> {
    let mut scope = call.scope;
    let arguments = call.arguments;
    let int_value = try!(arguments.require(scope, 0));

    match int_value.variant() {
        Variant::Number(v) => { println!("number: {:?}", v.value()) },
        Variant::Integer(v) => { println!("integer: {:?}", v.value()); },
        _ => { println!("oops!"); }
    }
    Ok(JsString::new(scope, "hello node").unwrap())
}

register_module!(m, {
//    m.export("hello", hello),
    m.export("hello2", hello2)
});
