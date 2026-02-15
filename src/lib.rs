mod beaver;
mod cameo;
mod chinchompa;
mod eightball;
mod flashbang;
mod golem;
mod heron;
mod horoscope;
mod joke;
mod liar;
mod noob;
mod toucan;

extern crate common;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use common::PluginContext;

#[unsafe(no_mangle)]
pub extern "C" fn exported(context: *const PluginContext) -> *mut c_char {
    unsafe {
        let command = to_str_or_default((*context).cmd);
        let query = to_str_or_default((*context).param);
        let author = to_str_or_default((*context).author);
        let _color = (*context).color;

        let result = match command.as_str() {
            "8ball" => eightball::shake(),
            "beaver" => beaver::beaver(),
            "chinchompa" => chinchompa::chinchompa(),
            "dra9" => cameo::dra9(),
            "flashbang" => flashbang::blind(),
            "golem" => golem::chat(),
            "heron" => heron::chat(),
            "horo" | "horoscope" => horoscope::lookup(query),
            "joke" => joke::lookup(),
            "liar" => liar::liar(query, author),
            "noob" => noob::noob(query, author),
            "shrimp" => cameo::shrimp(),
            "toucan" => toucan::examine(),
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
                "joke",
                "liar",
                "noob",
                "shrimp",
                "toucan",
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
joke
horo(scope)?
liar
noob
shrimp
toucan
zac"
                .split("\n")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()),
            _ => Ok(vec![]),
        };

        let output = result.unwrap_or_default();

        CString::new(output.join("\n")).unwrap().into_raw()
    }
}

fn to_str_or_default(ptr: *const c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_str().unwrap_or_default().to_owned()
}
