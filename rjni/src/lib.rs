use jni::{objects::JClass, JNIEnv};

pub mod array;
pub mod obj_call;
pub mod primitive_types;

#[no_mangle]
pub extern "system" fn Java_jjni_App_helloJni(_env: JNIEnv, _clazz: JClass) {
    println!("Hello JNI!");
}
