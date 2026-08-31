// SPDX-License-Identifier: GPL-2.0
// C dependencies: <stddef.h>, <libelf.h>

#[repr(C)]
pub struct Elf {
    _private: [u8; 0],
}

extern "C" {
    static ELFCOMPRESS_ZSTD: ::std::os::raw::c_int;

    fn elf_compress(
        elf: *mut Elf,
        typ: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}

fn main() {
    unsafe {
        elf_compress(::std::ptr::null_mut(), ELFCOMPRESS_ZSTD, 0);
    }
}
