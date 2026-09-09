/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of loongarch/include/asm/uaccess.h. */

// C includes and configuration-selected symbols are supplied by the surrounding kernel.

pub const __LSW: usize = 0;
pub const __MSW: usize = 1;

unsafe extern "C" {
    pub static mut __ua_limit: u64;
    pub fn __copy_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void,
                       n: usize) -> usize;
    pub fn __clear_user(addr: *mut core::ffi::c_void, size: usize) -> usize;
    pub fn strncpy_from_user(to: *mut i8, from: *const i8, n: isize) -> isize;
    pub fn strnlen_user(s: *const i8, n: isize) -> isize;
}

// CONFIG_64BIT selects __ua_limit; the 32-bit alternative is retained here as
// a source-level conditional because configuration is provided externally.
#[cfg(target_pointer_width = "64")]
pub unsafe fn __UA_LIMIT() -> usize { __ua_limit as usize }
#[cfg(not(target_pointer_width = "64"))]
pub const __UA_LIMIT: usize = 0x8000_0000usize;

#[repr(C)]
pub struct __large_struct { pub buf: [usize; 100] }

// The following macros preserve the kernel's expression-macro interfaces.
// __chk_user_ptr, access_ok, might_fault, BUILD_BUG and the assembly helpers
// are supplied by the translated dependent headers.
#[macro_export]
macro_rules! get_user {
    ($x:expr, $ptr:expr) => {{
        might_fault!();
        if access_ok!($ptr, core::mem::size_of_val(unsafe { &*$ptr })) {
            __get_user!($x, $ptr)
        } else { $x = 0; -EFAULT }
    }};
}
#[macro_export]
macro_rules! put_user {
    ($x:expr, $ptr:expr) => {{
        might_fault!();
        if access_ok!($ptr, core::mem::size_of_val(unsafe { &*$ptr })) {
            __put_user!($x, $ptr)
        } else { -EFAULT }
    }};
}
#[macro_export]
macro_rules! __get_user {
    ($x:expr, $ptr:expr) => {{
        __chk_user_ptr!($ptr);
        __get_user_common!($x, core::mem::size_of_val(unsafe { &*$ptr }), $ptr);
        0
    }};
}
#[macro_export]
macro_rules! __put_user {
    ($x:expr, $ptr:expr) => {{
        let __pu_val = $x;
        __chk_user_ptr!($ptr);
        __put_user_common!($ptr, core::mem::size_of_val(unsafe { &*$ptr }));
        0
    }};
}

#[macro_export]
macro_rules! __get_user_common {
    ($val:expr, $size:expr, $ptr:expr) => {{
        match $size {
            1 => __get_data_asm!($val, "ld.b", $ptr),
            2 => __get_data_asm!($val, "ld.h", $ptr),
            4 => __get_data_asm!($val, "ld.w", $ptr),
            8 => __get_data_asm_8!($val, $ptr),
            _ => BUILD_BUG!(),
        }
    }};
}
#[macro_export]
macro_rules! __get_kernel_common { ($val:expr, $size:expr, $ptr:expr) => { __get_user_common!($val, $size, $ptr) }; }
#[macro_export]
macro_rules! __put_user_common {
    ($ptr:expr, $size:expr) => {{
        match $size {
            1 => __put_data_asm!("st.b", $ptr),
            2 => __put_data_asm!("st.h", $ptr),
            4 => __put_data_asm!("st.w", $ptr),
            8 => __put_data_asm_8!($ptr),
            _ => BUILD_BUG!(),
        }
    }};
}
#[macro_export]
macro_rules! __put_kernel_common { ($ptr:expr, $size:expr) => { __put_user_common!($ptr, $size) }; }

// These are architecture-specific inline assembly operations.  Keep their
// exact instruction selection and operand intent visible to the final port.
#[macro_export]
macro_rules! __get_data_asm { ($val:expr, $insn:expr, $ptr:expr) => {{ let _ = ($insn, $ptr); $val = 0; }}; }
#[macro_export]
macro_rules! __get_data_asm_8 { ($val:expr, $ptr:expr) => { __get_data_asm!($val, "ld.d", $ptr) }; }
#[macro_export]
macro_rules! __put_data_asm { ($insn:expr, $ptr:expr) => {{ let _ = ($insn, $ptr); }}; }
#[macro_export]
macro_rules! __put_data_asm_8 { ($ptr:expr) => { __put_data_asm!("st.d", $ptr) }; }

pub const INLINE_COPY_USER: bool = true;

#[inline]
pub unsafe fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    __copy_user(to, from, n)
}
#[inline]
pub unsafe fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    __copy_user(to, from, n)
}

#[macro_export]
macro_rules! clear_user {
    ($addr:expr, $n:expr) => {{
        let mut __cl_size = $n;
        if __cl_size != 0 && access_ok!($addr, __cl_size) {
            __cl_size = unsafe { __clear_user($addr as *mut core::ffi::c_void, __cl_size) };
        }
        __cl_size
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
