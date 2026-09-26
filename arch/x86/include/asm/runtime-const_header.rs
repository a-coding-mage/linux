/* SPDX-License-Identifier: GPL-2.0 */

// MODULE: "Cannot use runtime-const infrastructure from modules".
#[cfg(MODULE)]
compile_error!("Cannot use runtime-const infrastructure from modules");

// __ASSEMBLER__ counterpart:
// .macro RUNTIME_CONST_PTR sym reg
//     movq $0x0123456789abcdef, %\reg
// 1:
//     .pushsection runtime_ptr_\sym, "a"
//     .long 1b - 8 - .
//     .popsection
// .endm

/// `runtime_const_ptr(sym)`: a `movabs` whose immediate is patched with the
/// value of `sym` at boot; evaluates to the caller's pointer-sized type.
#[macro_export]
macro_rules! runtime_const_ptr {
    ($sym:ident) => {{
        let __ret: u64;
        // SAFETY: loads an immediate; the boot-time patch only rewrites it.
        unsafe {
            core::arch::asm!(
                "movabs ${value}, {ret}\n1:",
                concat!(".pushsection runtime_ptr_", stringify!($sym), ",\"a\""),
                ".long 1b - {size} - .",
                ".popsection",
                ret = out(reg) __ret,
                value = const 0x0123456789abcdefu64,
                size = const core::mem::size_of::<core::ffi::c_long>(),
                options(att_syntax, pure, nomem, nostack, preserves_flags),
            );
        }
        __ret as _
    }};
}

// The 'typeof' will create at _least_ a 32-bit type, but
// will happily also take a bigger type and the 'shrl' will
// clear the upper bits
#[macro_export]
macro_rules! runtime_const_shift_right_32 {
    ($val:expr, $sym:ident) => {{
        let mut __ret: u64 = ($val) as u64;
        // SAFETY: register-only shift; the boot-time patch rewrites the count.
        unsafe {
            core::arch::asm!(
                "shrl $12, {ret:e}\n1:",
                concat!(".pushsection runtime_shift_", stringify!($sym), ",\"a\""),
                ".long 1b - 1 - .",
                ".popsection",
                ret = inout(reg) __ret,
                options(att_syntax, pure, nomem, nostack),
            );
        }
        __ret as _
    }};
}

#[macro_export]
macro_rules! runtime_const_mask_32 {
    ($val:expr, $sym:ident) => {{
        let mut __ret: u64 = ($val) as u64;
        // SAFETY: register-only mask; the boot-time patch rewrites the mask.
        unsafe {
            core::arch::asm!(
                "and $0x12345678, {ret:e}\n1:",
                concat!(".pushsection runtime_mask_", stringify!($sym), ",\"a\""),
                ".long 1b - 4 - .",
                ".popsection",
                ret = inout(reg) __ret,
                options(att_syntax, pure, nomem, nostack),
            );
        }
        __ret as _
    }};
}

#[macro_export]
macro_rules! runtime_const_init {
    ($type:ident, $sym:ident) => {
        ::kernel::macros::paste! {{
            extern "C" {
                static [<__start_runtime_ $type _ $sym>]: [i32; 0];
                static [<__stop_runtime_ $type _ $sym>]: [i32; 0];
            }
            runtime_const_fixup(
                [<__runtime_fixup_ $type>],
                $sym as core::ffi::c_ulong,
                [<__start_runtime_ $type _ $sym>].as_ptr().cast_mut(),
                [<__stop_runtime_ $type _ $sym>].as_ptr().cast_mut(),
            );
        }}
    };
}

/*
 * The text patching is trivial - you can only do this at init time,
 * when the text section hasn't been marked RO, and before the text
 * has ever been executed.
 */
#[inline]
pub unsafe fn __runtime_fixup_ptr(where_: *mut core::ffi::c_void, val: core::ffi::c_ulong) {
    where_.cast::<core::ffi::c_ulong>().write_unaligned(val);
}

#[inline]
pub unsafe fn __runtime_fixup_shift(where_: *mut core::ffi::c_void, val: core::ffi::c_ulong) {
    where_.cast::<u8>().write_unaligned(val as u8);
}

#[inline]
pub unsafe fn __runtime_fixup_mask(where_: *mut core::ffi::c_void, val: core::ffi::c_ulong) {
    where_.cast::<u32>().write_unaligned(val as u32);
}

#[inline]
pub unsafe fn runtime_const_fixup(
    fn_: unsafe fn(*mut core::ffi::c_void, core::ffi::c_ulong),
    val: core::ffi::c_ulong,
    mut start: *mut i32,
    end: *mut i32,
) {
    while start < end {
        fn_(start.cast::<u8>().offset(*start as isize).cast(), val);
        start = start.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
