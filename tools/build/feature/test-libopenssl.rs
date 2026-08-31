use std::os::raw::c_int;

// C dependencies:
// #include <openssl/ssl.h>
// #include <openssl/opensslv.h>
extern "C" {
    fn SSL_library_init() -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    SSL_library_init()
}
