/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct xor_block_template {
    pub next: *mut xor_block_template,
    pub name: *const c_char,
    pub speed: i32,
    pub xor_gen: Option<unsafe extern "C" fn(*mut c_void, *mut *mut c_void, u32, u32)>,
}

#[macro_export]
macro_rules! __DO_XOR_BLOCKS {
    ($name:ident, $handle1:ident, $handle2:ident, $handle3:ident, $handle4:ident) => {
        /* Pass the complete generated function name (xor_gen_<name>) here;
         * Rust macro_rules! has no stable identifier concatenation facility. */
        pub unsafe extern "C" fn $name(
            dest: *mut core::ffi::c_void,
            srcs: *mut *mut core::ffi::c_void,
            mut src_cnt: u32,
            bytes: u32,
        ) {
            let mut src_off: usize = 0;
            while src_cnt > 0 {
                let this_cnt = core::cmp::min(src_cnt, 4);
                if this_cnt == 1 {
                    $handle1(bytes, dest, *srcs.add(src_off));
                } else if this_cnt == 2 {
                    $handle2(bytes, dest, *srcs.add(src_off), *srcs.add(src_off + 1));
                } else if this_cnt == 3 {
                    $handle3(bytes, dest, *srcs.add(src_off), *srcs.add(src_off + 1), *srcs.add(src_off + 2));
                } else {
                    $handle4(bytes, dest, *srcs.add(src_off), *srcs.add(src_off + 1), *srcs.add(src_off + 2), *srcs.add(src_off + 3));
                }
                src_cnt -= this_cnt;
                src_off += this_cnt as usize;
            }
        }
    };
}

#[macro_export]
macro_rules! DO_XOR_BLOCKS {
    ($name:ident, $handle1:ident, $handle2:ident, $handle3:ident, $handle4:ident) => {
        $crate::__DO_XOR_BLOCKS!($name, $handle1, $handle2, $handle3, $handle4);
    };
}

/* generic implementations */
extern "C" {
    pub static mut xor_block_8regs: xor_block_template;
    pub static mut xor_block_32regs: xor_block_template;
    pub static mut xor_block_8regs_p: xor_block_template;
    pub static mut xor_block_32regs_p: xor_block_template;
    pub fn xor_register(tmpl: *mut xor_block_template);
    pub fn xor_force(tmpl: *mut xor_block_template);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
