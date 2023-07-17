use std::fmt::format;

use jni::objects::{JClass, JObject, JString, JValue};
use jni::signature::Primitive;
use jni::sys::{jboolean, JNI_TRUE};
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
pub extern "system" fn Java_jjni_JniCall_callIncCountFromJni(mut env: JNIEnv, obj: JObject) {
    env.call_method(obj, "incCount", "()V", &[])
        .expect("Can't call method");
}

#[no_mangle]
pub extern "system" fn Java_jjni_JniCall_callSayHiFromJni(mut env: JNIEnv, class: JClass) {
    env.call_static_method(class, "sayHi", "()V", &[])
        .expect("Can't call method");
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

#[no_mangle]
pub extern "system" fn Java_jjni_JniCall_readUserFromJni(
    mut env: JNIEnv,
    _obj: JObject,
    user: JObject,
) {
    let field = env
        .get_field(user, "name", "Ljava/lang/String;")
        .expect("Can't get field");
    let val = field.l().expect("Can't convert to string");

    let val = JString::from(val);
    let val = env.get_string(&val).expect("Can't convert to string");
    let val = val.to_str().expect("Can't convert to str");
    println!("name: {}", val);
}

#[no_mangle]
pub extern "system" fn Java_jjni_JniCall_setUserFromJni(mut env: JNIEnv, obj: JObject) -> jboolean {
    //获取class
    let class = env.get_object_class(&obj).expect("Can't get class");
    //创建Java字符串
    let name = env.new_string("Alice").expect("Can't create java string");
    //调用Java Static方法创建User对象
    let user = env
        .call_static_method(
            class,
            "createUser",
            "(Ljava/lang/String;)Ljjni/User;",
            &[JValue::Object(&name)],
        )
        .expect("Can't call method");
    //转换为User对象
    let user = user.l().expect("Can't convert to object");
    //获取User对象name字段
    let name = env
        .call_method(&user, "getName", "()Ljava/lang/String;", &[])
        .expect("Can't call method");
    let name = name.l().expect("Can't convert to string");
    //转换为String
    let name = JString::from(name);
    let name = env.get_string(&name).expect("Can't convert to string");
    let name = name.to_str().expect("Can't convert to str");
    println!("name: {}", name);
    //添加后缀From JNI !!
    let name = format!("{} From JNI !!", name);
    //转换为Java String
    let name = env.new_string(name).expect("Can't create java string");
    //调用User对象的setName方法
    env.call_method(
        &user,
        "setName",
        "(Ljava/lang/String;)V",
        &[JValue::Object(&name)],
    )
    .expect("Can't call method");

    env.call_method(&obj, "setUser", "(Ljjni/User;)V", &[JValue::Object(&user)])
        .expect("Can't call method");

    JNI_TRUE
}
