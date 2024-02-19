mod build;

#[no_mangle]
pub extern "C" fn compute_square(a: i32, b: i32) -> i32 {
    let square = (a * a) + (b * b);
    square
}

#[no_mangle]
pub extern "C" fn get_hello() -> *const u8 {
    let s = "Hello from Rust!";
    s.as_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_square() {
        assert_eq!(compute_square(2, 3), 13);
        assert_ne!(compute_square(5, 10), 126);
    }

    #[test]
    fn test_get_hello() {
        let s = "Hello from Rust!";
        let ptr = get_hello();
        let len = s.len();
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        let s2 = std::str::from_utf8(slice).unwrap();
        assert_eq!(s, s2);
    }
}
