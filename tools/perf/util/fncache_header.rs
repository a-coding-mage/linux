// Header guard _FCACHE_H omitted in Rust.

extern "C" {
    pub fn file_available(name: *const ::std::os::raw::c_char) -> bool;
}
