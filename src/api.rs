use crate::*;

pub fn start() -> io::Result<()> {
    run_cmd_with_args(
        "am",
        &["startservice", "-n", "com.termux.api/.KeepAliveService"],
    )
}

pub fn stop() -> io::Result<()> {
    run_cmd_with_args(
        "am",
        &["stopservice", "-n", "com.termux.api/.KeepAliveService"],
    )
}
