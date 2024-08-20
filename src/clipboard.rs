use crate::*;

/// Get the system clipboard text.
pub fn get() -> io::Result<String> {
    run_api_cmd("Clipboard")
}

/// Set the system clipboard text.
pub fn set(text: &str) -> io::Result<()> {
    run_cmd("termux-clipboard-set", text)
}

#[test]
fn test_set_and_get() {
    let _ = set("Some text...\nSome more text...");
    assert_eq!(&get().unwrap(), "Some text...\nSome more text...")
}
