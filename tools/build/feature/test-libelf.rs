// SPDX-License-Identifier: GPL-2.0
// C dependency: <libelf.h>

#[repr(C)]
pub struct Elf {
    _private: [u8; 0],
}

pub const ELF_C_READ: i32 = 1;

unsafe extern "C" {
    pub fn elf_begin(fildes: i32, cmd: i32, ref_: *mut Elf) -> *mut Elf;
}

pub fn main() -> i32 {
    let elf: *mut Elf = unsafe { elf_begin(0, ELF_C_READ, core::ptr::null_mut()) };

    (elf != core::ptr::null_mut()) as i32
}
