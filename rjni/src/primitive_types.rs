use jni::objects::JClass;
use jni::sys::jboolean;
use jni::sys::jbyte;

use jni::sys::jint;
use jni::JNIEnv;
use jni::JavaVM;
use jni::NativeMethod;
use std::ffi::c_void;

use jni::strings::JNIString;
use jni::JNIVersion;

use jni::sys::JNI_TRUE;
#[no_mangle]
pub extern "system" fn get_boolean(_env: JNIEnv, _: JClass) -> jboolean {
    // Method implementation
    JNI_TRUE
}

#[no_mangle]
pub extern "system" fn get_byte(_env: JNIEnv, _: JClass) -> jbyte {
    // Method implementation
    i8::MIN
}

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    println!("Call JNI_OnLoad");
    let mut env = vm.get_env().unwrap();
    let class = env.find_class("jjni/PrimitiveTypes").unwrap();

    let JNI_METHODS: [NativeMethod; 1] = [NativeMethod {
        name: JNIString::from("getBoolean"),
        sig: JNIString::from("()Z"),
        fn_ptr: get_boolean as *mut c_void,
    }];

    let _ = env.register_native_methods(class, &JNI_METHODS);

    JNIVersion::V8.into()
}
