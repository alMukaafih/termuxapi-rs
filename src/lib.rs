/*  */
//! # Installation
//! Download the [Termux:API](https://wiki.termux.com/wiki/Termux:API) add-on.
//!
//! You also need to install `termux-api` package. To install Run.
//! ```sh
//! pkg i termux-api
//! ```
//! Then install the crate
//! ```sh
//! cargo add termuxapi
//! ```
//! 
//! # Usage
//! Each command in `termux-api` is mapped to a function by replacing `-` with `::`.
//! ## Example
//! The function equivalent of `termux-clipboard-get` would be [termux::clipboard::get]
//! 
//! All functions return a [Result][std::io::Result].
//! 
//! # Implemented commands
//! Not all commands are implemented yet.
//! This is the list of all implemented commands.
//! - [termux::api::start]
//! - [termux::api::stop]
//! - [termux::audio::info]
//! - [termux::clipboard::get]
//! - [termux::clipboard::set]
//! 
//!
#[cfg(doc)]
use crate as termux;
use std::io;
use std::process::Command;

pub mod api;
pub mod audio;
pub mod clipboard;

fn run_api_cmd(cmd: &str) -> io::Result<String> {
    let output = Command::new("/data/data/com.termux/files/usr/libexec/termux-api")
        .arg(cmd)
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn run_cmd(cmd: &str, arg: &str) -> io::Result<()> {
    let _output = Command::new(cmd).arg(arg).output()?;
    Ok(())
}

fn run_cmd_with_args(cmd: &str, args: &[&str]) -> io::Result<()> {
    let _output = Command::new(cmd).args(args).output()?;
    Ok(())
}
