/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// #include <linux/uaccess.h>

/*
 * Runtime-constant references are recorded in linker sections by the inline
 * assembler, and are fixed up by runtime_const_init().
 */
#[macro_export]
macro_rules! runtime_const_ptr {
    ($sym:expr) => {{
        let mut __ret = $sym;
        unsafe {
            core::arch::asm!(
                "0: iihf {ret}, {c1}",
                "iilf {ret}, {c2}",
                ".pushsection runtime_ptr_{sym},\"a\"",
                ".long 0b - .",
                ".popsection",
                ret = lateout(reg) __ret,
                c1 = const 0x01234567_u64,
                c2 = const 0x89abcdef_u64,
                sym = sym $sym,
            );
        }
        __ret
    }};
}

#[macro_export]
macro_rules! runtime_const_shift_right_32 {
    ($val:expr, $sym:ident) => {{
        let mut __ret: u32 = $val;
        unsafe {
            core::arch::asm!(
                "0: srl {ret}, 12",
                ".pushsection runtime_shift_{} ,\"a\"",
                ".long 0b - .",
                ".popsection",
                ret = inout(reg) __ret,
                sym = sym $sym,
            );
        }
        __ret
    }};
}

#[macro_export]
macro_rules! runtime_const_mask_32 {
    ($val:expr, $sym:ident) => {{
        let mut __ret: u32 = $val;
        unsafe {
            core::arch::asm!(
                "0: nilf {ret}, 12",
                ".pushsection runtime_mask_{} ,\"a\"",
                ".long 0b - .",
                ".popsection",
                ret = inout(reg) __ret,
                sym = sym $sym,
                lateout("cc") _,
            );
        }
        __ret
    }};
}

#[macro_export]
macro_rules! runtime_const_init {
    ($type:ident, $sym:ident) => {{
        extern "C" {
            static __start_runtime_: i32;
            static __stop_runtime_: i32;
        }
        runtime_const_fixup(
            __runtime_fixup_$type,
            $sym as usize as u64,
            &__start_runtime_ as *const _ as *mut i32,
            &__stop_runtime_ as *const _ as *mut i32,
        );
    }};
}

unsafe fn __runtime_fixup_32(p: *mut u32, val: u32) {
    s390_kernel_write(p as *mut core::ffi::c_void, &val as *const u32 as *const core::ffi::c_void, core::mem::size_of::<u32>());
}

/* 32-bit immediate for iihf and iilf in bits in I2 field */
unsafe fn __runtime_fixup_ptr(where_: *mut core::ffi::c_void, val: u64) {
    __runtime_fixup_32(where_.add(2) as *mut u32, (val >> 32) as u32);
    __runtime_fixup_32(where_.add(8) as *mut u32, val as u32);
}

/* Immediate value is lower 12 bits of D2 field of srl */
unsafe fn __runtime_fixup_shift(where_: *mut core::ffi::c_void, val: u64) {
    let mut insn = core::ptr::read(where_ as *const u32);
    insn &= 0xfffff000;
    insn |= (val & 63) as u32;
    s390_kernel_write(where_, &insn as *const u32 as *const core::ffi::c_void, core::mem::size_of::<u32>());
}

/* 32-bit immediate for nilf in bits in I2 field */
unsafe fn __runtime_fixup_mask(where_: *mut core::ffi::c_void, val: u64) {
    __runtime_fixup_32(where_.add(2) as *mut u32, val as u32);
}

unsafe fn runtime_const_fixup(
    fn_: unsafe fn(*mut core::ffi::c_void, u64),
    val: u64,
    mut start: *mut i32,
    end: *mut i32,
) {
    while start < end {
        fn_(((*start) as isize as usize + start as usize) as *mut core::ffi::c_void, val);
        start = start.add(1);
    }
}

extern "C" {
    fn s390_kernel_write(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
