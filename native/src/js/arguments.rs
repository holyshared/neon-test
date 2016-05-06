use neon::vm:: { Call, JsResult };
use neon::js:: { JsInteger, JsUndefined };

pub fn arguments_check(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    let integer = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>());

    println!("{}", integer.value());

    Ok(JsUndefined::new())
}
