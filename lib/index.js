const addon = require('../native');

console.log(addon.return_js_object_with_integer());

const values = addon.return_js_array();
values.forEach((v, i) => console.log(`${i} - ${v}`));

addon.arguments_check(1);
addon.arguments_check(2);
