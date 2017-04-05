#[macro_use]
extern crate neon;

use neon::js::{JsFunction, JsString, JsInteger, Object, JsObject};
use neon::js::class::{Class, JsClass};
use neon::mem::Handle;
use neon::vm::Lock;

pub struct User {
  id: i32,
  first_name: String,
  last_name: String,
}

declare_types! {
  pub class JsUser for User {
    init(call) {
      let scope = call.scope;
      let user = try!(try!(call.arguments.require(scope, 0)).check::<JsObject>());
      let id: Handle<JsInteger> = try!(try!(user.get(scope, "id")).check::<JsInteger>());
      let first_name: Handle<JsString> = try!(try!(user.get(scope, "first_name")).check::<JsString>());
      let last_name: Handle<JsString> = try!(try!(user.get(scope, "last_name")).check::<JsString>());

      Ok(User {
        id: id.value() as i32,
        first_name: first_name.value(),
        last_name: last_name.value(),
      })
    }

    method get_full_name(call) {
      let scope = call.scope;
      let first_name = call.arguments.this(scope).grab(|user| { user.first_name.clone() });
      let last_name = call.arguments.this(scope).grab(|user| { user.last_name.clone() });
      Ok(try!(JsString::new_or_throw(scope, &(first_name + &last_name))).upcast())
    }

    method get_id(call) {
      let scope = call.scope;
      let id = call.arguments.this(scope).grab(|user| { user.id.clone() });
      Ok(JsInteger::new(scope, id as i32).upcast())
    }
  }
}

register_module!(m, {
  let class: Handle<JsClass<JsUser>> = try!(JsUser::class(m.scope));
  let constructor: Handle<JsFunction<JsUser>> = try!(class.constructor(m.scope));
  try!(m.exports.set("User", constructor));
  Ok(())
});
