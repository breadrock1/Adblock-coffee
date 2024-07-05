use jni::JNIEnv;
use jni::objects::{JObjectArray, JString};
use jni::sys::{jboolean, jlong};

use crate::adblock::AdvtBlocker;
use crate::errors::RustException;

pub(crate) fn init_object_wrapped(
    env: &mut JNIEnv,
    rules: &JObjectArray
) -> Result<AdvtBlocker, RustException> {
    let conv_rules = extract_list_str(env, rules)?;
    Ok(AdvtBlocker::new(conv_rules))
}

pub(crate) fn check_net_urls_wrapped(
    env: &mut JNIEnv,
    ptr: jlong,
    url: &JString,
    src_url: &JString,
    req_type: &JString,
) -> Result<jboolean, RustException> {
    unsafe {
        let advt_blocker = &mut *(ptr as *mut AdvtBlocker);

        let url_str = extract_str(env, url)?;

        let src_url_str = extract_str(env, src_url)?;

        let req_type_str = extract_str(env, req_type)?;

        let check_result = advt_blocker
            .check_network_urls(
                url_str.as_str(),
                src_url_str.as_str(),
                req_type_str.as_str(),
            )?;

        Ok(check_result as jboolean)
    }
}

fn extract_str<'a>(env: &'a mut JNIEnv, j_obj: &'a JString) -> Result<String, RustException> {
    let j_str = env
        .get_string(&j_obj)
        .map_err(|err| RustException::ExtractParameter(err.to_string()))?;

    let str_obj = j_str
        .to_str()
        .map_err(|err| RustException::ExtractParameter(err.to_string()))?;

    Ok(str_obj.to_string())
}

fn extract_list_str<'a>(env: &'a mut JNIEnv, j_obj_arr: &'a JObjectArray) -> Result<Vec<String>, RustException> {
    let j_list = env
        .get_list(&j_obj_arr)
        .map_err(|err| RustException::ExtractParameter(err.to_string()))?;

    let j_list_size = j_list
        .size(env)
        .map_err(|err| RustException::ExtractParameter(err.to_string()))?;

    let mut list_data = Vec::with_capacity(j_list_size as usize);
    for index in 0..j_list_size {
        let j_obj = j_list
            .get(env, index)
            .map_err(|err| RustException::ExtractParameter(err.to_string()))?
            .unwrap();

        let j_str = JString::from(j_obj);
        let str_data = extract_str(env, &j_str)?;
        list_data.push(str_data);
    }

    Ok(list_data)
}
