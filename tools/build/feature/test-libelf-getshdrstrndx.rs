// SPDX-License-Identifier: GPL-2.0
// Depends on libelf.h for elf_getshdrstrndx.

use core::ffi::c_int;

type size_t = usize;

unsafe extern "C" {
    fn elf_getshdrstrndx(elf: *mut core::ffi::c_void, dst: *mut size_t) -> c_int;
}

fn main() -> c_int {
    let mut dst: size_t;

    unsafe { elf_getshdrstrndx(core::ptr::null_mut(), &mut dst) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
