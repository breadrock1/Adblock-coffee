use jni::objects::{JObjectArray, JString};
use jni::sys::{jboolean, jlong};
use jni::JNIEnv;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::adblock::AdvtBlocker;
use crate::errors::RustException;

lazy_static! {
    static ref INSTANCE: Lazy<Mutex<AdvtBlocker>> = Lazy::new(|| {
        let init: Mutex<AdvtBlocker> = Mutex::new(AdvtBlocker::default());
        init
    });
}

pub(crate) fn init_object_wrapped(
    env: &mut JNIEnv,
    rules: &JObjectArray,
) -> Result<jlong, RustException> {
    let conv_rules = extract_list_str(env, rules)?;
    let mut instance_lock = INSTANCE.lock().map_err(RustException::from)?;

    instance_lock.recreate(conv_rules);

    let ptr = Box::into_raw(Box::new(&INSTANCE));
    Ok(ptr as jlong)
}

pub(crate) fn destroy_object_wrapped(
    _env: &mut JNIEnv,
    _ptr: jlong,
) -> Result<jboolean, RustException> {
    let instance_lock = INSTANCE.lock().map_err(RustException::from)?;

    drop(instance_lock);
    Ok(true as jboolean)
}

pub(crate) fn check_net_urls_wrapped(
    env: &mut JNIEnv,
    _ptr: jlong,
    url: &JString,
    src_url: &JString,
    req_type: &JString,
) -> Result<jboolean, RustException> {
    let advt_blocker = INSTANCE.lock().map_err(RustException::from)?;

    let url_str = extract_str(env, url)?;
    let src_url_str = extract_str(env, src_url)?;
    let req_type_str = extract_str(env, req_type)?;

    let check_result = advt_blocker.check_network_urls(
        url_str.as_str(),
        src_url_str.as_str(),
        req_type_str.as_str(),
    )?;

    Ok(check_result as jboolean)
}

fn extract_str<'a>(env: &'a mut JNIEnv, j_obj: &'a JString) -> Result<String, RustException> {
    let j_str = env.get_string(&j_obj).map_err(RustException::from)?;

    let str_obj = j_str
        .to_str()
        .map_err(|err| RustException::ExtractParameter(err.to_string()))?;

    Ok(str_obj.to_string())
}

fn extract_list_str<'a>(
    env: &'a mut JNIEnv,
    j_obj_arr: &'a JObjectArray,
) -> Result<Vec<String>, RustException> {
    let j_list = env.get_list(&j_obj_arr).map_err(RustException::from)?;

    let j_list_size = j_list.size(env).map_err(RustException::from)?;

    let mut list_data = Vec::with_capacity(j_list_size as usize);
    for index in 0..j_list_size {
        let j_obj_get_result = j_list.get(env, index);
        if j_obj_get_result.is_err() {
            let err = j_obj_get_result.err().unwrap();
            log::warn!("failed to parse rules: {:?}", err);
            continue;
        }

        let j_obj_opt = j_obj_get_result?;
        if j_obj_opt.is_none() {
            log::warn!("parsed rule is none. skipped...");
            continue;
        }

        let j_str = JString::from(j_obj_opt.unwrap());
        let str_data = extract_str(env, &j_str)?;
        list_data.push(str_data);
    }

    Ok(list_data)
}
