#![feature(proc_macro_hygiene, decl_macro)]
// use crate::Content::Html;
extern crate clipboard;
use std::thread;

use std::process::Command;
use web_view::*;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

static WIDTH: i32 = 1280;
static HEIGHT: i32 = 768;

fn check_clipboard() -> bool {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let clippy = ctx.get_contents().unwrap();
    if clippy.contains("Plug") {
        let name = String::from(clippy);
        return true;
    };
    return false;
}

fn open_vimawesome() {
    web_view::builder()
        .title("Heimat")
        .content(Content::Url("https://vimawesome.com"))
        .size(WIDTH, HEIGHT)
        .resizable(false)
        .debug(true)
        .user_data(vec!["i feel good"])
        .invoke_handler(|_, _| Ok(()))
        .run()
        .unwrap();
}

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(format!(""));
    let plugin_name_url = "";
    thread::spawn(|| loop {
        if check_clipboard() {
            std::process::exit(0);
        };
        let two_ms = std::time::Duration::new(0, 2);
        thread::sleep(two_ms);
        // TODO: rename
    });
    open_vimawesome();
}
