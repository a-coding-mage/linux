/* SPDX-License-Identifier: GPL-2.0 */

/*
 * User space memory access functions
 *
 * Dependencies supplied by the surrounding kernel translation:
 * asm/page.h, asm/cache.h, asm/extable.h, linux/bug.h,
 * linux/string.h, asm/pgtable.h, and asm-generic/access_ok.h.
 */

pub const TASK_SIZE_MAX: usize = DEFAULT_TASK_SIZE;

pub use __put_user as put_user;
pub use __get_user as get_user;

/* CONFIG_64BIT selects the corresponding assembly implementation. */
#[cfg(not(CONFIG_64BIT))]
macro_rules! LDD_USER { ($sr:expr, $val:expr, $ptr:expr) => { __get_user_asm64!($sr, $val, $ptr) }; }
#[cfg(not(CONFIG_64BIT))]
macro_rules! STD_USER { ($sr:expr, $x:expr, $ptr:expr) => { __put_user_asm64!($sr, $x, $ptr) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! LDD_USER { ($sr:expr, $val:expr, $ptr:expr) => { __get_user_asm!($sr, $val, "ldd", $ptr) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! STD_USER { ($sr:expr, $x:expr, $ptr:expr) => { __put_user_asm!($sr, "std", $x, $ptr) }; }

macro_rules! __get_user_internal {
    ($sr:expr, $val:expr, $ptr:expr) => {{
        let mut __gu_err: libc::c_long = 0;
        match core::mem::size_of_val(unsafe { &*$ptr }) {
            1 => __get_user_asm!($sr, $val, "ldb", $ptr),
            2 => __get_user_asm!($sr, $val, "ldh", $ptr),
            4 => __get_user_asm!($sr, $val, "ldw", $ptr),
            8 => LDD_USER!($sr, $val, $ptr),
            _ => BUILD_BUG!(),
        }
        __gu_err
    }};
}

macro_rules! __probe_user_internal {
    ($sr:expr, $error:expr, $ptr:expr) => {{
        core::arch::asm!(
            "proberi (%%sr{sr},{ptr}),{privilege},{error}",
            "cmpiclr,= 1,{error},{error}",
            "ldi {fault},{error}",
            sr = const $sr, ptr = in(reg) $ptr, privilege = const PRIV_USER,
            error = inout(reg) $error, fault = const -EFAULT,
        );
    }};
}

macro_rules! __get_user {
    ($val:expr, $ptr:expr) => {{
        let mut __gu_err: libc::c_long = __get_user_internal!(SR_USER, $val, $ptr);
        if likely(__gu_err == 0) {
            __probe_user_internal!(SR_USER, __gu_err, $ptr);
        }
        __gu_err
    }};
}

macro_rules! __get_user_asm {
    ($sr:expr, $val:expr, $ldx:expr, $ptr:expr) => {{
        let mut __gu_val: libc::c_long;
        core::arch::asm!(
            "1: {ldx} 0(%%sr{sr},{ptr})",
            "9:",
            ldx = const $ldx, sr = const $sr, ptr = in(reg) $ptr,
            gu_val = lateout(reg) __gu_val,
            gu_err = inout(reg) __gu_err,
        );
        $val = __gu_val as _;
    }};
}

macro_rules! __get_kernel_nofault {
    ($dst:expr, $src:expr, $type:ty, $err_label:ident) => {{
        let __z: $type = __get_user_internal!(SR_KERNEL, __z, ($src as *const $type));
        let __err = __get_user_internal!(SR_KERNEL, __z, ($src as *const $type));
        if unlikely(__err != 0) { goto!($err_label); } else { *($dst as *mut $type) = __z; }
    }};
}

macro_rules! __put_user_internal {
    ($sr:expr, $x:expr, $ptr:expr) => {{
        let mut __pu_err: libc::c_long = 0;
        match core::mem::size_of_val(unsafe { &*$ptr }) {
            1 => __put_user_asm!($sr, "stb", $x, $ptr),
            2 => __put_user_asm!($sr, "sth", $x, $ptr),
            4 => __put_user_asm!($sr, "stw", $x, $ptr),
            8 => STD_USER!($sr, $x, $ptr),
            _ => BUILD_BUG!(),
        }
        __pu_err
    }};
}

macro_rules! __put_user {
    ($x:expr, $ptr:expr) => {{
        let __ptr = $ptr;
        let __x = $x;
        __put_user_internal!(SR_USER, __x, __ptr)
    }};
}

macro_rules! __put_kernel_nofault {
    ($dst:expr, $src:expr, $type:ty, $err_label:ident) => {{
        let __z: $type = *($src as *const $type);
        let __err = __put_user_internal!(SR_KERNEL, __z, ($dst as *mut $type));
        if unlikely(__err != 0) { goto!($err_label); }
    }};
}

/* The original macros use inline assembly and exception-table fixups. */
macro_rules! __put_user_asm {
    ($sr:expr, $stx:expr, $x:expr, $ptr:expr) => {{
        core::arch::asm!("1: {stx} {x},0(%%sr{sr},{ptr})", "9:",
            stx = const $stx, x = in(reg) $x, sr = const $sr,
            ptr = in(reg) $ptr, pu_err = inout(reg) __pu_err);
    }};
}

/* Complex access routines -- external declarations */
extern "C" {
    pub fn strncpy_from_user(dst: *mut libc::c_char, src: *const libc::c_char, count: libc::c_long) -> libc::c_long;
    pub fn lclear_user(dst: *mut core::ffi::c_void, count: libc::c_ulong) -> libc::c_uint;
    pub fn strnlen_user(src: *const libc::c_char, n: libc::c_long) -> libc::c_long;
    pub fn raw_copy_to_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: libc::c_ulong) -> libc::c_ulong;
    pub fn raw_copy_from_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: libc::c_ulong) -> libc::c_ulong;
}

pub use lclear_user as clear_user;
pub use lclear_user as __clear_user;

/* INLINE_COPY_USER */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
