/* SPDX-License-Identifier: GPL-2.0 */

// Cannot use runtime-const infrastructure from modules.

// __ASSEMBLER__ counterpart:
// .macro RUNTIME_CONST_PTR sym reg
//     movq $0x0123456789abcdef, %\reg
// 1:
//     .pushsection runtime_ptr_\sym, "a"
//     .long 1b - 8 - .
//     .popsection
// .endm

#[macro_export]
macro_rules! runtime_const_ptr {
    ($sym:ident) => {{
        let mut __ret: u64;
        unsafe {
            core::arch::asm!(
                "mov {ret}, {value}\n1:\n",
                ".pushsection runtime_ptr_{$sym},\"a\"\n",
                ".long 1b - 8 - .\n",
                ".popsection",
                ret = out(reg) __ret,
                value = const 0x0123456789abcdefu64,
            );
        }
        __ret
    }};
}

// The `typeof` will create at least a 32-bit type, but will happily also take
// a bigger type and the `shrl` will clear the upper bits.
#[macro_export]
macro_rules! runtime_const_shift_right_32 {
    ($val:expr, $sym:ident) => {{
        let mut __ret = $val;
        unsafe {
            core::arch::asm!(
                "shrl $12, {ret}\n1:\n",
                ".pushsection runtime_shift_{$sym},\"a\"\n",
                ".long 1b - 1 - .\n",
                ".popsection",
                ret = inout(reg) __ret,
            );
        }
        __ret
    }};
}

#[macro_export]
macro_rules! runtime_const_mask_32 {
    ($val:expr, $sym:ident) => {{
        let mut __ret = $val;
        unsafe {
            core::arch::asm!(
                "and $0x12345678, {ret}\n1:\n",
                ".pushsection runtime_mask_{$sym},\"a\"\n",
                ".long 1b - 4 - .\n",
                ".popsection",
                ret = inout(reg) __ret,
            );
        }
        __ret
    }};
}

#[macro_export]
macro_rules! runtime_const_init {
    ($type:ident, $sym:expr) => {{
        unsafe {
            extern "C" {
                static __start_runtime_$type: i32;
                static __stop_runtime_$type: i32;
            }
            runtime_const_fixup(
                __runtime_fixup_$type,
                $sym as ::core::ffi::c_ulong,
                &__start_runtime_$type as *const i32 as *mut i32,
                &__stop_runtime_$type as *const i32 as *mut i32,
            );
        }
    }};
}

/*
 * The text patching is trivial - you can only do this at init time,
 * when the text section hasn't been marked RO, and before the text
 * has ever been executed.
 */
#[inline]
pub unsafe fn __runtime_fixup_ptr(where_: *mut core::ffi::c_void, val: usize) {
    *(where_ as *mut usize) = val;
}

#[inline]
pub unsafe fn __runtime_fixup_shift(where_: *mut core::ffi::c_void, val: usize) {
    *(where_ as *mut u8) = val as u8;
}

#[inline]
pub unsafe fn __runtime_fixup_mask(where_: *mut core::ffi::c_void, val: usize) {
    *(where_ as *mut u32) = val as u32;
}

#[inline]
pub unsafe fn runtime_const_fixup(
    fn_: unsafe fn(*mut core::ffi::c_void, usize),
    val: usize,
    mut start: *mut i32,
    end: *mut i32,
) {
    while start < end {
        fn_(
            (*start as isize as usize).wrapping_add(start as usize)
                as *mut core::ffi::c_void,
            val,
        );
        start = start.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
