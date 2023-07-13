use jni::objects::{JClass, JObject, JValue};
use jni::signature::Primitive;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_jjni_JniCall_incCountFromJni(mut env: JNIEnv, _: JClass) {
    // 获取字段的值
    let class = env.find_class("jjni/JniCall").expect("Can't find class");

    let count_id = env
        .get_static_field_id(&class, "count", "I")
        .expect("Can't get field id");

    let count = env
        .get_static_field_unchecked(
            &class,
            count_id,
            jni::signature::JavaType::Primitive(Primitive::Int),
        )
        .expect("Can't get count");
    let mut count = count.i().expect("Can't  convert to i32");

    count += 1;

    env.set_static_field(&class, count_id, JValue::Int(count))
        .expect("Can't set count");
}

#[no_mangle]
pub extern "system" fn Java_jjni_JniCall_callIncCountFromJni(
    mut env: JNIEnv,
    obj: JObject,
) {
     env.call_method(obj, "incCount", "()V", &[]).expect("Can't call method");
}

#[no_mangle]
pub extern "system" fn Java_jjni_JniCall_getUserFromJni<'a>(
    mut env: JNIEnv<'a>,
    _: JClass<'a>,
) -> jni::objects::JObject<'a> {
    let class = env.find_class("jjni/User").expect("Can't find class");

    let alice = env
        .new_string("Alice")
        .expect("Couldn't create java string");
    let user = env
        .new_object(class, "(Ljava/lang/String;)V", &[JValue::Object(&alice)])
        .expect("Can't create object");

    user
}
