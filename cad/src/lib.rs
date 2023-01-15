//! ## CAD geometry engine
//!
//! http://cliffle.com/blog/bare-metal-wasm/

#![cfg(target_family = "wasm")]
#![allow(unused)]

// #![feature(lang_items)]
// #![feature(no_core)]
// #![no_std]
// #![no_core]

// /// requires sized lang item
// #[lang = "sized"]
// trait Sized {}

use wasm_bindgen::prelude::*;

// #[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub static ABOUT: &str = "CAD";

// #[no_mangle]
// fn about() -> &'static str {
//     ABOUT
// }

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(module = "/cad.js")]
extern "C" {
    fn show_status(s: String);
}

#[wasm_bindgen]
pub fn hello() {
    alert("Hello");
}

// #[no_mangle]
#[wasm_bindgen]
pub fn none() {}

#[derive(Debug)]
enum Mode {
    Select,
    Point,
    Line,
    Del,
}

#[wasm_bindgen]
#[derive(Debug)]
/// https://rustwasm.github.io/wasm-bindgen/contributing/design/exporting-rust-struct.html
pub struct State {
    mode: Mode,
}

pub static mut STATE: State = State { mode: Mode::Select };

#[wasm_bindgen]
pub fn start() {
    show_status("start".to_string());
}

#[wasm_bindgen]
pub unsafe fn on_select() {
    STATE.mode = Mode::Select;
    show_status(format!("{:?}", STATE));
}

#[wasm_bindgen]
pub unsafe fn on_point() {
    STATE.mode = Mode::Point;
    show_status(format!("{:?}", STATE));
}

#[wasm_bindgen]
pub unsafe fn on_line() {
    STATE.mode = Mode::Line;
    show_status(format!("{:?}", STATE));
}

#[wasm_bindgen]
pub unsafe fn on_del() {
    STATE.mode = Mode::Del;
    show_status(format!("{:?}", STATE));
}

#[derive(Debug)]
pub struct Point {
    x: i16,
    y: i16,
}

fn touch(client: Point, offset: Point, screen: Point) {
    console_log!("client: {:?}", client);
}

#[wasm_bindgen]
pub unsafe fn on_touch(
    client_x: i16,
    client_y: i16,
    offset_x: i16,
    offset_y: i16,
    screen_x: i16,
    screen_y: i16,
    ctrl_key: bool,
    alt_key: bool,
    shift_key: bool,
) {
    console_log!("{:?}:{:?}", client_x, client_y);

    touch(
        Point {
            x: client_x,
            y: client_y,
        },
        Point {
            x: offset_x,
            y: offset_y,
        },
        Point {
            x: screen_x,
            y: screen_y,
        },
    );
}

// #![allow(unused)]

// pub struct Object {}

// #[derive(Debug)]
// pub struct Point {
//     x: i16,
//     y: i16,
// }

// impl Point {
//     pub fn new(x: i16, y: i16) -> Self {
//         Point { x: x, y: y }
//     }
// }

// struct Rect {}
