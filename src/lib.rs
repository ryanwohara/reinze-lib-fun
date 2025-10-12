mod beaver;
mod cameo;
mod chinchompa;
mod eightball;
mod flashbang;
mod golem;
mod heron;
mod horoscope;
mod liar;
mod noob;

extern crate common;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn exported(
    cmd: *const c_char,
    raw_query: *const c_char,
    raw_author: *const c_char,
) -> *mut c_char {
    let command = to_str_or_default(cmd);
    let query = to_str_or_default(raw_query);
    let author = to_str_or_default(raw_author);

    let result = match command.as_str() {
        "8ball" => eightball::shake(),
        "beaver" => beaver::beaver(),
        "chinchompa" => chinchompa::chinchompa(),
        "dra9" => cameo::dra9(),
        "flashbang" => flashbang::blind(),
        "golem" => golem::golem(),
        "heron" => heron::heron(),
        "horo" | "horoscope" => horoscope::lookup(query),
        "liar" => liar::liar(query, author),
        "noob" => noob::noob(query, author),
        "shrimp" => cameo::shrimp(),
        "zac" => cameo::zac(),
        "help" => Ok(vec![
            "8ball",
            "beaver",
            "chinchompa",
            "dra9",
            "flashbang",
            "golem",
            "heron",
            "horoscope",
            "liar",
            "noob",
            "shrimp",
            "zac",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()),
        "" => Ok("8ball
beaver
chinchompa
dra9
flashbang
golem
heron
horo(scope)?
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
