// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <zstd.h>

#[repr(C)]
pub struct ZSTD_CStream {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn ZSTD_createCStream() -> *mut ZSTD_CStream;
    pub fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> usize;
}

fn main() {
    let cstream: *mut ZSTD_CStream;

    unsafe {
        cstream = ZSTD_createCStream();
        ZSTD_freeCStream(cstream);
    }
}
