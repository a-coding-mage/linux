// SPDX-License-Identifier: GPL-2.0
// C dependency: #include <lzma.h>

use core::ffi::c_int;

extern "C" {
    static LZMA_STREAM_INIT: lzma_stream;
    static LZMA_CONCATENATED: u32;

    fn lzma_stream_decoder(
        strm: *mut lzma_stream,
        memlimit: u64,
        flags: u32,
    ) -> lzma_ret;
}

#[repr(C)]
pub struct lzma_stream {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
type lzma_ret = c_int;

const UINT64_MAX: u64 = u64::MAX;

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    let mut strm: lzma_stream = LZMA_STREAM_INIT;
    let ret: lzma_ret;

    ret = lzma_stream_decoder(&mut strm, UINT64_MAX, LZMA_CONCATENATED);
    if ret != 0 { -1 } else { 0 }
}
