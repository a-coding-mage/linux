// SPDX-License-Identifier: GPL-2.0
// C dependency: <libelf.h>

type size_t = usize;

extern "C" {
    fn elf_getphdrnum(elf: *mut core::ffi::c_void, dst: *mut size_t) -> core::ffi::c_int;
}

fn main() -> core::ffi::c_int {
    let mut dst: size_t = 0;

    unsafe { elf_getphdrnum(core::ptr::null_mut(), &mut dst) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
