use self::session::Session;
use locenv::api::LuaState;
use locenv::{create_table, new_userdata, push_value, set_functions, upvalue_index, FunctionEntry};
use locenv_macros::loader;
use std::os::raw::c_int;

mod session;

const MODULE_FUNCTIONS: [FunctionEntry; 1] = [FunctionEntry {
    name: "newsession",
    function: Some(newsession),
}];

const TYPE_SESSION: &str = "session";

extern "C" fn newsession(lua: *mut LuaState) -> c_int {
    new_userdata(lua, upvalue_index(1), Session::new());
    1
}

#[loader]
extern "C" fn loader(lua: *mut LuaState) -> c_int {
    create_table(lua, 0, MODULE_FUNCTIONS.len() as _);
    push_value(lua, 2); // Push Context.
    set_functions(lua, &MODULE_FUNCTIONS, 1);
    1
}
