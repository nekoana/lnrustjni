use jni::objects::JIntArray;
use jni::objects::JObject;
use jni::objects::JObjectArray;
use jni::objects::JString;
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

#[no_mangle]
pub extern "system" fn Java_jjni_JniArray_mergeStringArray<'a>(
    mut env: JNIEnv<'a>,
    _obj: JObject<'a>,
    arr1: JObjectArray<'a>,
    arr2: JObjectArray<'a>,
) -> JObjectArray<'a> {
    let len1 = env.get_array_length(&arr1).expect("Can't get array length");
    let len2 = env.get_array_length(&arr2).expect("Can't get array length");
    if len1 != len2 {
        panic!("Array length not equal");
    }

    let mut result = env
        .new_object_array(len1, "java/lang/String", JObject::null())
        .expect("Can't create array");

    for i in 0..len1 {
        let s1 = env
            .get_object_array_element(&arr1, i)
            .expect("Can't get array element");
        let s2 = env
            .get_object_array_element(&arr2, i)
            .expect("Can't get array element");
        let s1 = JString::from(s1);
        let s2 = JString::from(s2);

        let s1: String = env.get_string(&s1).expect("Can't get string").into();
        let s2: String = env.get_string(&s2).expect("Can't get string").into();

        let s = env
            .new_string(format!("{} -> {}", s1, s2))
            .expect("Can't create string");

        env.set_object_array_element(&mut result, i, s)
            .expect("Can't set array element");
    }

    JObjectArray::from(result)
}
