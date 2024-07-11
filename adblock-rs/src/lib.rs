mod adblock;
mod errors;
mod logger;
mod wrapper;

use jni::objects::{JObject, JObjectArray, JString};
use jni::sys::{jboolean, jlong};
use jni::JNIEnv;

use crate::wrapper::*;

#[no_mangle]
pub extern "system" fn Java_com_example_adblock_AdvtBlocker_initObject(
    mut env: JNIEnv,
    _class: JObject,
    rules: JObjectArray,
) -> jlong {
    init_object_wrapped(&mut env, &rules).unwrap_or_else(|err| {
        log::error!("{:?}", err);
        -1_i64 as jlong
    })
}

#[no_mangle]
pub extern "system" fn Java_com_example_adblock_AdvtBlocker_destroyObject(
    mut env: JNIEnv,
    _class: JObject,
    ptr: jlong,
) -> jboolean {
    destroy_object_wrapped(&mut env, ptr).unwrap_or_else(|err| {
        log::error!("{:?}", err);
        false as jboolean
    })
}

#[no_mangle]
pub extern "system" fn Java_com_example_adblock_AdvtBlocker_checkNetworkUrls(
    mut env: JNIEnv,
    _class: JObject,
    ptr: jlong,
    url: JString,
    src_url: JString,
    req_type: JString,
) -> jboolean {
    check_net_urls_wrapped(&mut env, ptr, &url, &src_url, &req_type).unwrap_or_else(|err| {
        log::error!("{:?}", err);
        false as jboolean
    })
}
