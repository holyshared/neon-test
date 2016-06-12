use neon::vm:: { Call, JsResult };
use neon::mem::Handle;
use neon::js:: { JsNull, JsNumber, JsFunction };

pub fn call_js_function(call: Call) -> JsResult<JsNumber> {
    let scope = call.scope;
    let f = try!(try!(call.arguments.require(scope, 0)).check::<JsFunction>());

    // fn(number) => number; 
    let args: Vec<Handle<JsNumber>> = vec![JsNumber::new(scope, 16.0)];
    try!(f.call(scope, JsNull::new(), args)).check::<JsNumber>()
}
