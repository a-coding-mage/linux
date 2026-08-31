// SPDX-License-Identifier: GPL-2.0
// C source included <zlib.h>; z_stream and inflateInit are external zlib API.

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_stream {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn inflateInit(strm: *mut z_stream) -> ::std::os::raw::c_int;
}

fn main() -> ::std::os::raw::c_int {
    let mut zs: z_stream = unsafe { ::std::mem::zeroed() };

    unsafe {
        inflateInit(&mut zs);
    }
    return 0;
}
