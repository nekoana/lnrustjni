use jni::objects::JClass;
use jni::sys::jboolean;
use jni::sys::jbyte;
use jni::sys::jchar;
use jni::sys::jdouble;
use jni::sys::jfloat;
use jni::sys::jlong;
use jni::sys::jshort;

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
pub extern "system" fn get_short(_env: JNIEnv, _: JClass) -> jshort {
    // Method implementation
    i16::MIN
}

#[no_mangle]
pub extern "system" fn get_char(_env: JNIEnv, _: JClass) -> jchar {
    // Method implementation
    'Q' as u16
}

#[no_mangle]
pub extern "system" fn get_int(_env: JNIEnv, _: JClass) -> jint {
    // Method implementation
    i32::MIN
}

#[no_mangle]
pub extern "system" fn get_long(_env: JNIEnv, _: JClass) -> jlong {
    // Method implementation
    i64::MIN
}

#[no_mangle]
pub extern "system" fn get_float(_env: JNIEnv, _: JClass) -> jfloat {
    // Method implementation
    f32::MIN
}

#[no_mangle]
pub extern "system" fn get_double(_env: JNIEnv, _: JClass) -> jdouble {
    // Method implementation
    f64::MIN
}

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    let mut env = vm.get_env().unwrap();
    let class = env.find_class("jjni/PrimitiveTypes").unwrap();

    let jni_methods = vec![
        NativeMethod {
            name: JNIString::from("getBoolean"),
            sig: JNIString::from("()Z"),
            fn_ptr: get_boolean as *mut c_void,
        },
        NativeMethod {
            name: JNIString::from("getByte"),
            sig: JNIString::from("()B"),
            fn_ptr: get_byte as *mut c_void,
        },
        NativeMethod {
            name: JNIString::from("getShort"),
            sig: JNIString::from("()S"),
            fn_ptr: get_short as *mut c_void,
        },
        NativeMethod {
            name: JNIString::from("getChar"),
            sig: JNIString::from("()C"),
            fn_ptr: get_char as *mut c_void,
        },
        NativeMethod {
            name: JNIString::from("getInt"),
            sig: JNIString::from("()I"),
            fn_ptr: get_int as *mut c_void,
        },
        NativeMethod {
            name: JNIString::from("getLong"),
            sig: JNIString::from("()J"),
            fn_ptr: get_long as *mut c_void,
        },
        NativeMethod {
            name: JNIString::from("getFloat"),
            sig: JNIString::from("()F"),
            fn_ptr: get_float as *mut c_void,
        },
        NativeMethod {
            name: JNIString::from("getDouble"),
            sig: JNIString::from("()D"),
            fn_ptr: get_double as *mut c_void,
        },
    ];

    let _ = env.register_native_methods(class, &jni_methods);

    JNIVersion::V8.into()
}
