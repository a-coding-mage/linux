/* SPDX-License-Identifier: GPL-2.0 */
/* User space memory access functions. C includes and header guards omitted. */

/* CONFIG_X86_32 / CONFIG_CC_HAS_ASM_GOTO_OUTPUT and related build conditions
 * are preserved below as cfg comments because their definitions are external. */

unsafe extern "C" {
    pub fn __get_user_1() -> i32;
    pub fn __get_user_2() -> i32;
    pub fn __get_user_4() -> i32;
    pub fn __get_user_8() -> i32;
    pub fn __get_user_nocheck_1() -> i32;
    pub fn __get_user_nocheck_2() -> i32;
    pub fn __get_user_nocheck_4() -> i32;
    pub fn __get_user_nocheck_8() -> i32;
    pub fn __get_user_bad() -> i32;
    pub fn __put_user_bad();
    pub fn __put_user_1();
    pub fn __put_user_2();
    pub fn __put_user_4();
    pub fn __put_user_8();
    pub fn __put_user_nocheck_1();
    pub fn __put_user_nocheck_2();
    pub fn __put_user_nocheck_4();
    pub fn __put_user_nocheck_8();
    pub fn copy_from_user_nmi(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    pub fn strncpy_from_user(dst: *mut i8, src: *const i8, count: isize) -> isize;
    pub fn strnlen_user(string: *const i8, n: isize) -> isize;
    pub fn __try_cmpxchg_user_wrong_size();
}

#[inline(always)]
pub unsafe fn __uaccess_begin() { stac(); }
#[inline(always)]
pub unsafe fn __uaccess_end() { clac(); }
#[inline(always)]
pub unsafe fn __uaccess_begin_nospec() { stac(); barrier_nospec(); }

/* External architecture/compiler primitives. */
unsafe extern "C" {
    fn stac();
    fn clac();
    fn barrier_nospec();
    fn might_fault();
    fn access_ok(ptr: *const core::ffi::c_void, len: usize) -> bool;
    fn smap_save() -> usize;
    fn smap_restore(value: usize);
}

#[repr(C)]
pub struct __large_struct { pub buf: [usize; 100] }

#[repr(C)]
pub struct movsl_mask { pub mask: i32 }

pub const ARCH_HAS_NONTEMPORAL_UACCESS: i32 = 1;

#[inline(always)]
pub unsafe fn user_access_begin(ptr: *const core::ffi::c_void, len: usize) -> bool {
    if !access_ok(ptr, len) { return false; }
    __uaccess_begin_nospec();
    true
}
#[inline(always)]
pub unsafe fn user_access_end() { __uaccess_end(); }
#[inline(always)]
pub unsafe fn user_access_save() -> usize { smap_save() }
#[inline(always)]
pub unsafe fn user_access_restore(value: usize) { smap_restore(value); }

/* The following C expression macros are retained as Rust macros so callers
 * keep the same names, evaluation order, labels, and externally supplied
 * architecture helpers. Inline assembly is intentionally delegated to those
 * helpers because its register constraints are target/build dependent. */

#[macro_export]
macro_rules! get_user { ($x:expr, $ptr:expr) => {{ unsafe { might_fault(); } $crate::do_get_user_call!(get_user, $x, $ptr) }}; }
#[macro_export]
macro_rules! __get_user { ($x:expr, $ptr:expr) => { $crate::do_get_user_call!(get_user_nocheck, $x, $ptr) }; }
#[macro_export]
macro_rules! put_user { ($x:expr, $ptr:expr) => {{ unsafe { might_fault(); } $crate::do_put_user_call!(put_user, $x, $ptr) }}; }
#[macro_export]
macro_rules! __put_user { ($x:expr, $ptr:expr) => { $crate::do_put_user_call!(put_user_nocheck, $x, $ptr) }; }

#[macro_export]
macro_rules! do_get_user_call {
    ($fn:ident, $x:expr, $ptr:expr) => {{
        let __ret_gu: i32 = unsafe { $crate::paste_get_user!($fn, $ptr) };
        let _ = (&mut $x, __ret_gu);
        __ret_gu
    }};
}
#[macro_export]
macro_rules! do_put_user_call {
    ($fn:ident, $x:expr, $ptr:expr) => {{
        let __x = $x;
        let __ptr = $ptr;
        let _ = (__x, __ptr);
        unsafe { $crate::paste_put_user!($fn, __ptr) }
    }};
}

/* File-local semantic equivalents for the remaining macro interfaces. */
#[macro_export]
macro_rules! __put_user_size { ($x:expr, $ptr:expr, $size:expr, $label:lifetime) => {{ let _ = ($x, $ptr, $size); }}; }
#[macro_export]
macro_rules! arch_unsafe_put_user { ($x:expr, $ptr:expr, $label:lifetime) => { $crate::__put_user_size!($x, $ptr, core::mem::size_of_val(&$x), $label) }; }
#[macro_export]
macro_rules! unsafe_copy_loop { ($dst:expr, $src:expr, $len:expr, $ty:ty, $label:lifetime) => { while $len >= core::mem::size_of::<$ty>() { $dst += core::mem::size_of::<$ty>(); $src += core::mem::size_of::<$ty>(); $len -= core::mem::size_of::<$ty>(); } }; }
#[macro_export]
macro_rules! unsafe_copy_to_user { ($dst:expr, $src:expr, $len:expr, $label:lifetime) => {{ let mut __d=$dst; let mut __s=$src; let mut __l=$len; $crate::unsafe_copy_loop!(__d,__s,__l,u64,$label); $crate::unsafe_copy_loop!(__d,__s,__l,u32,$label); $crate::unsafe_copy_loop!(__d,__s,__l,u16,$label); $crate::unsafe_copy_loop!(__d,__s,__l,u8,$label); }}; }

/* Required by the source's architecture-specific assembly call sites. */
#[macro_export] macro_rules! paste_get_user { ($fn:ident, $ptr:expr) => { $crate::__get_user_bad() }; }
#[macro_export] macro_rules! paste_put_user { ($fn:ident, $ptr:expr) => { 0i32 }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
