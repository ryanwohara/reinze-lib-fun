mod beaver;
mod cameo;
mod chinchompa;
mod flashbang;
mod golem;
mod heron;
mod liar;
mod noob;

extern crate common;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn exported(
    cmd: *const c_char,
    raw_query: *const c_char,
    raw_author: *const c_char,
) -> *mut c_char {
    let command = to_str_or_default(cmd);
    let query = to_str_or_default(raw_query);
    let author = to_str_or_default(raw_author);

    let result = match command.as_str() {
        "beaver" => beaver::beaver(),
        "chinchompa" => chinchompa::chinchompa(),
        "dra9" => cameo::dra9(),
        "flashbang" => flashbang::blind(),
        "golem" => golem::golem(),
        "heron" => heron::heron(),
        "liar" => liar::liar(query, author),
        "noob" => noob::noob(query, author),
        "shrimp" => cameo::shrimp(),
        "zac" => cameo::zac(),
        "help" => Ok(vec![
            "beaver".to_string(),
            "chinchompa".to_string(),
            "dra9".to_string(),
            "flashbang".to_string(),
            "golem".to_string(),
            "liar".to_string(),
            "noob".to_string(),
            "shrimp".to_string(),
            "zac".to_string(),
        ]),
        "" => Ok("beaver
dra9
chinchompa
flashbang
golem
heron
liar
noob
shrimp
zac"
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()),
        _ => Ok(vec![]),
    };

    let output = result.unwrap_or_default();

    CString::new(output.join("\n")).unwrap().into_raw()
}

fn to_str_or_default(ptr: *const c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_str().unwrap_or_default().to_owned()
}
