/*
 * Rust translation of the MIPS Linux uaccess header.
 * Original file is GPL-licensed; see the source header for copyright details.
 * C preprocessor include dependencies are supplied by the surrounding build.
 */

#[cfg(CONFIG_32BIT)]
pub const __UA_LIMIT: usize = 0x8000_0000;
#[cfg(CONFIG_32BIT)]
pub const TASK_SIZE_MAX: usize = KSEG0;
#[cfg(CONFIG_32BIT)]
pub const __UA_ADDR: &str = ".word";
#[cfg(CONFIG_32BIT)]
pub const __UA_LA: &str = "la";
#[cfg(CONFIG_32BIT)]
pub const __UA_ADDU: &str = "addu";
#[cfg(CONFIG_32BIT)]
pub const __UA_t0: &str = "$8";
#[cfg(CONFIG_32BIT)]
pub const __UA_t1: &str = "$9";

#[cfg(CONFIG_64BIT)]
extern "C" {
    pub static mut __ua_limit: u64;
}
#[cfg(CONFIG_64BIT)]
pub const TASK_SIZE_MAX: usize = XKSSEG;
#[cfg(CONFIG_64BIT)]
pub const __UA_ADDR: &str = ".dword";
#[cfg(CONFIG_64BIT)]
pub const __UA_LA: &str = "dla";
#[cfg(CONFIG_64BIT)]
pub const __UA_ADDU: &str = "daddu";
#[cfg(CONFIG_64BIT)]
pub const __UA_t0: &str = "$12";
#[cfg(CONFIG_64BIT)]
pub const __UA_t1: &str = "$13";

#[repr(C)]
pub struct __large_struct {
    pub buf: [::core::ffi::c_ulong; 100],
}

pub const INLINE_COPY_USER: bool = true;

extern "C" {
    pub fn __raw_copy_from_user(to: *mut ::core::ffi::c_void, from: *const ::core::ffi::c_void, n: usize) -> usize;
    pub fn __raw_copy_to_user(to: *mut ::core::ffi::c_void, from: *const ::core::ffi::c_void, n: usize) -> usize;
    pub fn __bzero(addr: *mut ::core::ffi::c_void, size: usize) -> usize;
    pub fn __strncpy_from_user_asm(to: *mut i8, from: *const i8, len: isize) -> isize;
    pub fn __strnlen_user_asm(s: *const i8, n: isize) -> isize;
}

/* The following macros preserve the source-level interface and require the
 * corresponding kernel-provided symbols and architecture helpers. */
#[macro_export]
macro_rules! __m { ($x:expr) => { &mut *((($x) as *mut $crate::__large_struct)) }; }

#[macro_export]
macro_rules! put_user {
    ($x:expr, $ptr:expr) => {{
        unsafe { might_fault(); }
        if unsafe { access_ok($ptr, ::core::mem::size_of_val(&*$ptr)) } {
            unsafe { __put_user!($x, $ptr) }
        } else { -EFAULT }
    }};
}

#[macro_export]
macro_rules! get_user {
    ($x:expr, $ptr:expr) => {{
        unsafe { might_fault(); }
        if unsafe { access_ok($ptr, ::core::mem::size_of_val(&*$ptr)) } {
            unsafe { __get_user!($x, $ptr) }
        } else { $x = 0; -EFAULT }
    }};
}

#[macro_export]
macro_rules! __put_user {
    ($x:expr, $ptr:expr) => {{
        unsafe { __chk_user_ptr($ptr); }
        let __pu_val = $x;
        let mut __pu_err: i32 = 0;
        match ::core::mem::size_of_val(&*$ptr) {
            1 => unsafe { __put_data_asm!(user_sb, $ptr, __pu_val, __pu_err) },
            2 => unsafe { __put_data_asm!(user_sh, $ptr, __pu_val, __pu_err) },
            4 => unsafe { __put_data_asm!(user_sw, $ptr, __pu_val, __pu_err) },
            8 => unsafe { __PUT_DW!(user_sd, $ptr, __pu_val, __pu_err) },
            _ => unsafe { BUILD_BUG!() },
        }
        __pu_err
    }};
}

#[macro_export]
macro_rules! __get_user {
    ($x:expr, $ptr:expr) => {{
        unsafe { __chk_user_ptr($ptr); }
        let mut __gu_err: i32 = 0;
        match ::core::mem::size_of_val(&*$ptr) {
            1 => unsafe { __get_data_asm!($x, user_lb, $ptr, __gu_err) },
            2 => unsafe { __get_data_asm!($x, user_lh, $ptr, __gu_err) },
            4 => unsafe { __get_data_asm!($x, user_lw, $ptr, __gu_err) },
            8 => unsafe { __GET_DW!($x, user_ld, $ptr, __gu_err) },
            _ => unsafe { BUILD_BUG!() },
        }
        __gu_err
    }};
}

#[macro_export]
macro_rules! __get_data_asm { ($val:expr, $insn:ident, $addr:expr, $err:expr) => {{
    /* MIPS exception-table assembly is architecture-specific and retained as
     * an explicit unsafe operation placeholder for the external asm helper. */
    unsafe { $val = $crate::uaccess_get_data($addr, &mut $err); }
}}; }

#[macro_export]
macro_rules! __get_data_asm_ll32 { ($val:expr, $insn:ident, $addr:expr) => {
    __get_data_asm!($val, $insn, $addr, __gu_err)
}; }

#[cfg(CONFIG_32BIT)]
#[macro_export]
macro_rules! __GET_DW { ($val:expr, $insn:ident, $ptr:expr, $err:expr) => {
    __get_data_asm_ll32!($val, $insn, $ptr)
}; }
#[cfg(CONFIG_64BIT)]
#[macro_export]
macro_rules! __GET_DW { ($val:expr, $insn:ident, $ptr:expr, $err:expr) => {
    __get_data_asm!($val, $insn, $ptr, $err)
}; }

#[macro_export]
macro_rules! __put_data_asm { ($insn:ident, $ptr:expr, $val:expr, $err:expr) => {
    unsafe { $err = $crate::uaccess_put_data($ptr, $val); }
}; }
#[macro_export]
macro_rules! __put_data_asm_ll32 { ($insn:ident, $ptr:expr, $val:expr, $err:expr) => {
    __put_data_asm!($insn, $ptr, $val, $err)
}; }
#[cfg(CONFIG_32BIT)]
#[macro_export]
macro_rules! __PUT_DW { ($insn:ident, $ptr:expr, $val:expr, $err:expr) => { __put_data_asm_ll32!($insn, $ptr, $val, $err) }; }
#[cfg(CONFIG_64BIT)]
#[macro_export]
macro_rules! __PUT_DW { ($insn:ident, $ptr:expr, $val:expr, $err:expr) => { __put_data_asm!($insn, $ptr, $val, $err) }; }

#[macro_export]
macro_rules! __get_kernel_nofault { ($dst:expr, $src:expr, $ty:ty, $err_label:lifetime) => {{
    let mut __gu_err: i32 = 0;
    match ::core::mem::size_of::<$ty>() {
        1 => __get_data_asm!(*($dst as *mut $ty), kernel_lb, $src, __gu_err),
        2 => __get_data_asm!(*($dst as *mut $ty), kernel_lh, $src, __gu_err),
        4 => __get_data_asm!(*($dst as *mut $ty), kernel_lw, $src, __gu_err),
        8 => __GET_DW!(*($dst as *mut $ty), kernel_ld, $src, __gu_err),
        _ => unsafe { BUILD_BUG!() },
    }
    if unlikely(__gu_err != 0) { break $err_label; }
}}; }

#[macro_export]
macro_rules! __put_kernel_nofault { ($dst:expr, $src:expr, $ty:ty, $err_label:lifetime) => {{
    let __pu_val: $ty = unsafe { *($src as *const $ty) };
    let mut __pu_err: i32 = 0;
    match ::core::mem::size_of::<$ty>() {
        1 => __put_data_asm!(kernel_sb, $dst, __pu_val, __pu_err),
        2 => __put_data_asm!(kernel_sh, $dst, __pu_val, __pu_err),
        4 => __put_data_asm!(kernel_sw, $dst, __pu_val, __pu_err),
        8 => __PUT_DW!(kernel_sd, $dst, __pu_val, __pu_err),
        _ => unsafe { BUILD_BUG!() },
    }
    if unlikely(__pu_err != 0) { break $err_label; }
}}; }

#[cfg(MODULE)]
#[macro_export]
macro_rules! __MODULE_JAL { ($destination:ident) => { concat!(".set\\tnoat\\n\\t", __UA_LA, "\\t$1, ", stringify!($destination), "\\n\\tjalr\\t$1\\n\\t.set\\tat\\n\\t") }; }
#[cfg(not(MODULE))]
#[macro_export]
macro_rules! __MODULE_JAL { ($destination:ident) => { concat!("jal\\t", stringify!($destination), "\\n\\t") }; }

#[cfg(any(CONFIG_CPU_DADDI_WORKAROUNDS, all(CONFIG_EVA, CONFIG_CPU_HAS_PREFETCH)))]
pub const DADDI_SCRATCH: &str = "$3";
#[cfg(not(any(CONFIG_CPU_DADDI_WORKAROUNDS, all(CONFIG_EVA, CONFIG_CPU_HAS_PREFETCH))))]
pub const DADDI_SCRATCH: &str = "$0";

#[inline]
pub unsafe fn raw_copy_from_user(to: *mut ::core::ffi::c_void, from: *const ::core::ffi::c_void, n: usize) -> usize {
    __raw_copy_from_user(to, from, n)
}

#[inline]
pub unsafe fn raw_copy_to_user(to: *mut ::core::ffi::c_void, from: *const ::core::ffi::c_void, n: usize) -> usize {
    __raw_copy_to_user(to, from, n)
}

#[inline]
pub unsafe fn __clear_user(addr: *mut ::core::ffi::c_void, size: usize) -> usize {
    might_fault();
    __bzero(addr, size)
}

#[macro_export]
macro_rules! clear_user { ($addr:expr, $n:expr) => {{
    let mut __cl_size = $n;
    if __cl_size != 0 && unsafe { access_ok($addr, __cl_size) } {
        __cl_size = unsafe { __clear_user($addr, __cl_size) };
    }
    __cl_size
}}; }

#[inline]
pub unsafe fn strncpy_from_user(to: *mut i8, from: *const i8, len: isize) -> isize {
    if !access_ok(from, len as usize) { return -EFAULT as isize; }
    might_fault();
    __strncpy_from_user_asm(to, from, len)
}

#[inline]
pub unsafe fn strnlen_user(s: *const i8, n: isize) -> isize {
    if !access_ok(s, 1) { return 0; }
    might_fault();
    __strnlen_user_asm(s, n)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
