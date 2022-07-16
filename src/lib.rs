use self::session::Session;
use locenv::api::LuaState;
use locenv::FunctionEntry;
use locenv_macros::loader;
use std::os::raw::c_int;

mod session;

const MODULE_FUNCTIONS: [FunctionEntry; 1] = [FunctionEntry {
    name: "newsession",
    function: Some(newsession),
}];

const TYPE_SESSION: &str = "session";

extern "C" fn newsession(lua: *mut LuaState) -> c_int {
    locenv::new_userdata(lua, Session::new());
    1
}

#[loader]
extern "C" fn loader(lua: *mut LuaState) -> c_int {
    locenv::create_table(lua, 0, MODULE_FUNCTIONS.len() as _);
    locenv::set_functions(lua, &MODULE_FUNCTIONS, 0);
    1
}
