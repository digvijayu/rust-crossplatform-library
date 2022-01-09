extern crate jni;

use std::os::raw::{c_char};
use std::ffi::{CString};

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};

use hello::hello;

pub type Callback = unsafe extern "C" fn(*const c_char) -> ();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn invokeCallbackViaJNA(callback: Callback) {
    let s = CString::new(hello::greetings_from_rust()).unwrap();
    unsafe { callback(s.as_ptr()); }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_rc_rustspike_myapplication_MainActivity_invokeCallbackViaJNI(
    env: JNIEnv,
    _class: JClass,
    callback: JObject
) {
    let s = String::from(hello::greetings_from_rust());
    let response = env.new_string(&s)
        .expect("Couldn't create java string!");
    env.call_method(callback, "callback", "(Ljava/lang/String;)V",
                    &[JValue::from(JObject::from(response))]).unwrap();
}
