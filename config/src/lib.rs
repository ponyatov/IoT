#![allow(unused)]

/// server configuration
pub mod server {

    /// bind IP address
    pub const IP: &str = "127.0.0.1";

    /// bind port 1024..65535
    pub const PORT: u16 = 12345;

    /// precompiled `bind(addr)`
    pub const ADDR: &str = const_format::formatcp!("{IP}:{PORT}");

    /// precompiled `http://` link
    pub const HTTP: &str = const_format::formatcp!("http://{ADDR}");
}

fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
