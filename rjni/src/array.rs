use jni::objects::JIntArray;
use jni::objects::JObject;
use jni::objects::ReleaseMode;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_jjni_JniArray_doubleIntArray<'a>(
    mut env: JNIEnv<'a>,
    _obj: JObject<'a>,
    array: JIntArray<'a>,
) -> JIntArray<'a> {
    let mut array = unsafe {
        env.get_array_elements(&array, ReleaseMode::NoCopyBack)
            .expect("Can't get array elements")
    };
    for i in 0..array.len() {
        array[i] *= 2;
    }
    let result = env.new_int_array(array.len() as i32).unwrap();
    env.set_int_array_region(&result, 0, &array).unwrap();

    JIntArray::from(result)
}
