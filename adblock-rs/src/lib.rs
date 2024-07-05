mod adblock;
mod errors;
mod wrapper;

use jni::JNIEnv;
use jni::objects::{JObject, JObjectArray, JString};
use jni::sys::{jboolean, jlong};

use crate::wrapper::*;

#[no_mangle]
pub extern "system" fn Java_com_example_adblock_AdvtBlocker_initObject(
    mut env: JNIEnv,
    _class: JObject,
    rules: JObjectArray,
) -> jlong {
    match init_object_wrapped(&mut env, &rules) {
        Ok(instance) => Box::into_raw(Box::new(instance)) as jlong,
        Err(err) => {
            log::error!("{:?}", err);
            0_i64 as jlong
        }
    }
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
    check_net_urls_wrapped(&mut env, ptr, &url, &src_url, &req_type)
        .unwrap_or_else(|err| {
            log::error!("{:?}", err);
            false as jboolean
        })
}

