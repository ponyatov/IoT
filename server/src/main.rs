#![allow(unused)]

// https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
// https://doc.rust-lang.ru/book/ch20-00-final-project-a-web-server.html

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let argv = std::env::args();
    let argc = argv.len();
    dbg!(argc, argv);

    // tcp
    println!("{}", config::server::HTTP);
    let listener = TcpListener::bind(config::server::ADDR).unwrap();
    for stream in listener.incoming() {
        handle(stream.unwrap());
    }
}

// https://doc.rust-lang.ru/book/ch20-01-single-threaded.html#Возвращение-реального-html

/// HTTP Request
#[derive(Debug)]
struct Request {
    reqline: String,
    method: String,
    uri: String,
    version: String,
    field: HashMap<String, String>,
    request: Vec<String>,
}

impl Request {
    /// create Request structure
    ///
    /// * `request` bare HTTP request lines vector
    fn new(request: Vec<String>) -> Self {
        let [method, uri, version] = reqline(&request);
        //
        let field = HashMap::new();
        for i in request.iter().skip(1) {
            let a = i.split(": ");
            // dbg!(a);
        }
        //
        Self {
            reqline: request[0].clone(),
            method: method.to_string(),
            uri: uri.to_string(),
            version: version.to_string(),
            field: field,
            request: request.clone(),
        }
    }
}

/// HTTP request line decode
fn reqline(request: &Vec<String>) -> [&str; 3] {
    let reqline = request[0].split_whitespace().collect::<Vec<&str>>();
    let method = reqline[0];
    let uri = reqline[1];
    let version = reqline[2];
    [method, uri, version]
}

/// handle incoming HTTP request
fn handle(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let request = Request::new(request);
    dbg!(&request.uri); // make bench
    route(request, stream);
}

const INDEX_HEAD: &[u8] = include_bytes!("../../static/bootstrap.head.html");
const INDEX_FOOT: &[u8] = include_bytes!("../../static/bootstrap.foot.html");
const INDEX_PRE: &[u8] = "<pre class=\"row font-monospace\">".as_bytes();
lazy_static! {
    static ref INDEX_RX: Regex = Regex::new(r"/(|index.html?|.+)$").unwrap();
}
fn index(request: &Request, mut stream: TcpStream) {
    let status = "HTTP/1.1 200 OK";
    let header = mime::TEXT_HTML;
    let response = format!("{status}\r\n{header}\r\n\r\n");
    stream.write_all(response.as_bytes());
    stream.write_all(INDEX_HEAD);
    stream.write_all(INDEX_PRE);
    stream.write_all(format!("\n{request:#?}").as_bytes());
    stream.write_all(INDEX_FOOT);
}

fn alert(status: &str) -> String {
    format!("\n<pre class=\"alert alert-danger\">{status}</pre>\n")
}

fn not_found_404(request: &Request, mut stream: TcpStream) {
    let status = "HTTP/1.1 404 Not Found";
    let header = mime::TEXT_HTML;
    let response = format!("{status}\r\n{header}\r\n");
    stream.write_all(response.as_bytes());
    stream.write_all(INDEX_HEAD);
    stream.write_all(alert(status).as_bytes());
    stream.write_all(INDEX_PRE);
    stream.write_all(format!("\n{request:#?}").as_bytes());
}

/// precompiled logo
const LOGO: &[u8] = include_bytes!("../../doc/logo.png");
const LOGO256: &[u8] = include_bytes!("../../static/logo_256.png");
//
const LOGO48: &[u8] = include_bytes!("../../static/logo_48.png");
const LOGO72: &[u8] = include_bytes!("../../static/logo_72.png");
const LOGO128: &[u8] = include_bytes!("../../static/logo_128.png");
const LOGO192: &[u8] = include_bytes!("../../static/logo_192.png");

lazy_static! {
    static ref LOGO_RX: Regex = Regex::new("/(logo.*.png|favicon.ico)").unwrap();
}

/// serve .png
fn png(request: &Request, mut stream: TcpStream, file: String) {
    let status = "HTTP/1.1 200 OK";
    let header = mime::IMAGE_PNG;
    let response = format!("{status}\r\n{header}\r\n\r\n");
    stream.write_all(response.as_bytes());
    match &file[..] {
        "/logo.png" => stream.write_all(LOGO),
        "/logo_48.png" => stream.write_all(LOGO48),
        "/logo_72.png" => stream.write_all(LOGO72),
        "/logo_128.png" => stream.write_all(LOGO128),
        "/favicon.ico" | "/logo_192.png" => stream.write_all(LOGO192),
        "/logo_256.png" => stream.write_all(LOGO256),
        _ => Ok(not_found_404(request, stream)),
    };
}

fn logo(request: &Request, mut stream: TcpStream, file: String) {
    png(request, stream, file);
}

/// precompiled JQuery
const JQUERY: &[u8] = include_bytes!("../../static/cdn/jquery.min.js");
const POPPER: &[u8] = include_bytes!("../../static/cdn/popper.min.js");
const BOOTSTRAP: &[u8] = include_bytes!("../../static/cdn/bootstrap.min.js");

const DARKLY: &[u8] = include_bytes!("../../static/cdn/darkly.min.css");
//
lazy_static! {
    static ref CSS_RX: Regex = Regex::new(r"/.+.css$").unwrap();
}
fn css(request: &Request, mut stream: TcpStream, file: String) {
    let status = "HTTP/1.1 200 OK";
    let header = format!("Content-Type: {}", mime::TEXT_CSS);
    let response = format!("{status}\r\n{header}\r\n\r\n");
    stream.write_all(response.as_bytes());
    match &file[..] {
        "/darkly.min.css" => stream.write_all(DARKLY),
        _ => Ok(not_found_404(request, stream)),
    };
}

const LATO: &[u8] = include_bytes!("../../static/cdn/Lato.css");
const LATO_ITALIC_400: &[u8] = include_bytes!("../../static/cdn/Lato_italic_400.ttf");
const LATO_NORMAL_400: &[u8] = include_bytes!("../../static/cdn/Lato_normal_400.ttf");
const LATO_NORMAL_700: &[u8] = include_bytes!("../../static/cdn/Lato_normal_700.ttf");
lazy_static! {
    static ref LATO_RX: Regex = Regex::new("/Lato.+").unwrap();
}
//
fn lato(request: &Request, mut stream: TcpStream, file: String) {
    let status = "HTTP/1.1 200 OK";
    let header = format!("Content-Type: {}", mime::TEXT_CSS);
    let response = format!("{status}\r\n{header}\r\n\r\n");
    stream.write_all(response.as_bytes());
    match &file[..] {
        "/Lato.css" => stream.write_all(LATO),
        "/Lato_italic_400.ttf" => stream.write_all(LATO_ITALIC_400),
        "/Lato_normal_400.ttf" => stream.write_all(LATO_NORMAL_400),
        "/Lato_normal_700.ttf" => stream.write_all(LATO_NORMAL_700),
        _ => Ok(not_found_404(request, stream)),
    };
}

/// stub sends empty .map
lazy_static! {
    static ref MAP_RX: Regex = Regex::new(r"/.+.map$").unwrap();
}
fn map(request: &Request, mut stream: TcpStream) {
    let status = "HTTP/1.1 200 OK";
    let header = format!("Content-Type: {}", mime::APPLICATION_JSON);
    let response = format!("{status}\r\n{header}\r\n\r\n");
    stream.write_all(response.as_bytes());
    stream.write_all(b"{\"version\": 3, \"sources\": []}");
}

const MANIFEST: &[u8] = include_bytes!("../../static/manifest.json");
fn manifest(request: &Request, mut stream: TcpStream) {
    let status = "HTTP/1.1 200 OK";
    let header = "Content-Type: application/manifest+json";
    let response = format!("{status}\r\n{header}\r\n\r\n");
    stream.write_all(response.as_bytes());
    stream.write_all(MANIFEST);
}

// const WASM: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/debug/cad.wasm");
const CAD_WASM: &[u8] = include_bytes!("../../cad/pkg/cad_bg.wasm");
const CAD_PKG: &[u8] = include_bytes!("../../cad/pkg/cad.js");
const CAD_JS: &[u8] = include_bytes!("../../cad/cad.js");
lazy_static! {
    static ref CAD_JS_RX: Regex = Regex::new("/snippets/.*/cad.js").unwrap();
    static ref WASM_RX: Regex = Regex::new(r"/.+\.wasm$").unwrap();
}
fn wasm(request: &Request, mut stream: TcpStream, file: String) {
    let status = "HTTP/1.1 200 OK";
    let header = "Content-Type: application/wasm";
    let response = format!("{status}\r\n{header}\r\n\r\n");
    stream.write_all(response.as_bytes());
    match &file[..] {
        "/cad_bg.wasm" => stream.write_all(CAD_WASM),
        _ => Ok(not_found_404(request, stream)),
    };
}

const JS: &[u8] = include_bytes!("../../static/js.js");
lazy_static! {
    static ref JS_RX: Regex = Regex::new("/.+.js$").unwrap();
}
fn js(request: &Request, mut stream: TcpStream, file: String) {
    let status = "HTTP/1.1 200 OK";
    let header = format!("Content-Type: {}", mime::APPLICATION_JAVASCRIPT);
    let response = format!("{status}\r\n{header}\r\n\r\n");
    // stream.write_all(response.as_bytes());
    match &file[..] {
        "/jquery.min.js" => {
            stream.write_all(response.as_bytes());
            stream.write_all(JQUERY)
        }
        "/popper.min.js" => {
            stream.write_all(response.as_bytes());
            stream.write_all(POPPER)
        }
        "/bootstrap.min.js" => {
            stream.write_all(response.as_bytes());
            stream.write_all(BOOTSTRAP)
        }
        //
        "/js.js" => {
            stream.write_all(response.as_bytes());
            stream.write_all(JS)
        }
        "/cad.js" => {
            stream.write_all(response.as_bytes());
            stream.write_all(CAD_PKG)
        }
        js if CAD_JS_RX.is_match(js) => {
            stream.write_all(response.as_bytes());
            stream.write_all(CAD_JS)
        }
        // stream.write_all(JS);
        _ => Ok(not_found_404(request, stream)),
    };
}

/// route Request to its handler
fn route(request: Request, mut stream: TcpStream) {
    // let cadjs = Regex::new("/snippets/.*/cad.js").unwrap();
    match &request.uri[..] {
        uri if LOGO_RX.is_match(uri) => logo(&request, stream, uri.to_string()),
        "/manifest.json" => manifest(&request, stream),
        uri if JS_RX.is_match(uri) => js(&request, stream, uri.to_string()),
        uri if MAP_RX.is_match(uri) => map(&request, stream),
        uri if CSS_RX.is_match(uri) => css(&request, stream, uri.to_string()),
        uri if LATO_RX.is_match(uri) => lato(&request, stream, uri.to_string()),
        uri if WASM_RX.is_match(uri) => wasm(&request, stream, uri.to_string()),
        uri if INDEX_RX.is_match(uri) => index(&request, stream),
        _ => not_found_404(&request, stream),
    };
}

// https://doc.rust-lang.ru/book/ch20-02-multithreaded.html#Порождение-потока-для-каждого-запроса

// #[bench]
pub fn bench_localhost() {
    //b: &mut test::Bencher) {
    // https://klau.si/blog/benchmarking-a-rust-web-application/
    dbg!("bench_localhost()");
}
