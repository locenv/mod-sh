use crate::TYPE_SESSION;
use locenv::api::LuaState;
use locenv::{MethodEntry, Object, UserData};
use std::os::raw::c_int;
use std::process::{Command, Stdio};

pub struct Session;

impl Session {
    pub fn new() -> Self {
        Self {}
    }

    fn run(&mut self, lua: *mut LuaState) -> c_int {
        let command = locenv::check_string(lua, 2);

        // Launch sh.
        let mut launcher = Command::new("sh");

        launcher.current_dir(locenv::get_working_directory());
        launcher.arg("-ec");
        launcher.arg(&command);
        launcher.stdin(Stdio::null());

        let status = match launcher.status() {
            Ok(r) => r,
            Err(e) => locenv::error!(lua, "failed to launch sh: {}", e),
        };

        // Check status.
        if !status.success() {
            match status.code() {
                Some(v) => locenv::error!(lua, "process exited with status {}", v),
                None => locenv::error!(lua, "process terminated by signal"),
            }
        }

        0
    }
}

impl Object for Session {
    fn methods() -> &'static [MethodEntry<Self>] {
        &[MethodEntry {
            name: "run",
            function: Self::run,
        }]
    }
}

impl UserData for Session {
    fn type_name() -> &'static str {
        TYPE_SESSION
    }
}
