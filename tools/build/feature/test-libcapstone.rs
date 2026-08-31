// SPDX-License-Identifier: GPL-2.0

// C source dependency: #include <capstone/capstone.h>
use core::mem::MaybeUninit;

extern "C" {
    fn cs_open(arch: u32, mode: u32, handle: *mut csh) -> i32;
}

type csh = usize;

const CS_ARCH_X86: u32 = 3;
const CS_MODE_64: u32 = 1 << 3;

fn main() {
    let mut handle = MaybeUninit::<csh>::uninit();

    unsafe {
        cs_open(CS_ARCH_X86, CS_MODE_64, handle.as_mut_ptr());
    }
}
