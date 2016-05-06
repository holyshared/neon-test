use neon::vm:: { Call, JsResult };
use neon::mem::Handle;
use neon::js:: { JsInteger, JsArray, Object };

pub fn return_js_array(call: Call) -> JsResult<JsArray> {
    let scope = call.scope;
    let array: Handle<JsArray> = JsArray::new(scope, 3);

    try!(array.set(0, JsInteger::new(scope, 9000)));
    try!(array.set(1, JsInteger::new(scope, 9000)));
    try!(array.set(2, JsInteger::new(scope, 9000)));
    Ok(array)
}
