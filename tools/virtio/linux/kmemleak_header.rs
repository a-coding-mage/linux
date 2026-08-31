pub unsafe fn kmemleak_ignore(ptr: *const core::ffi::c_void) {
    let _ = ptr;
}
