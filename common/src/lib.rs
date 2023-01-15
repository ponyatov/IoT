//! ## common code for all IoT components

/// shared
pub fn common(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = common(2, 2);
        assert_eq!(result, 4);
    }
}
