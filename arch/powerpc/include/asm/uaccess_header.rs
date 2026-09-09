/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of powerpc/include/asm/uaccess.h. */

#[cfg(target_pointer_width = "64")]
pub const TASK_SIZE_MAX: usize = TASK_SIZE_USER64;
pub const VMX_COPY_THRESHOLD: usize = 3328;

pub unsafe fn __access_ok(ptr: *const core::ffi::c_void, size: usize) -> bool {
    let addr = ptr as usize;
    if cfg!(target_pointer_width = "64") {
        // BUILD_BUG_ON(!is_power_of_2(TASK_SIZE_MAX));
        // BUILD_BUG_ON(TASK_SIZE_MAX > 0x0010000000000000);
        if size > TASK_SIZE_MAX { return false; }
        if size <= TASK_SIZE_MAX { return (addr & !(TASK_SIZE_MAX - 1)) == 0; }
        ((size | addr) & !(TASK_SIZE_MAX - 1)) == 0
    } else {
        if size <= SZ_128K { addr < TASK_SIZE }
        else { size <= TASK_SIZE && addr <= TASK_SIZE - size }
    }
}

#[macro_export]
macro_rules! __put_user { ($x:expr, $ptr:expr) => {{
    let __pu_addr = $ptr;
    might_fault();
    let __pu_err: i64 = unsafe { __put_user_size_goto!($x, __pu_addr, core::mem::size_of_val(&*__pu_addr), __pu_failed) };
    __pu_err
}}; }
#[macro_export]
macro_rules! put_user { ($x:expr, $ptr:expr) => {{
    let p = $ptr;
    if unsafe { access_ok(p, core::mem::size_of_val(&*p)) } { __put_user!($x, p) } else { -EFAULT }
}}; }

// Architecture-specific C asm-goto operations are represented by these interfaces.
#[macro_export] macro_rules! __put_user_asm_goto { ($($t:tt)*) => { unsafe { core::arch::asm!("", options(nostack, preserves_flags)); } }; }
#[macro_export] macro_rules! __put_user_asm2_goto { ($($t:tt)*) => { __put_user_asm_goto!($($t)*) }; }
#[macro_export] macro_rules! __put_user_size_goto { ($x:expr, $ptr:expr, $size:expr, $label:tt) => {{
    match $size { 1 | 2 | 4 | 8 => __put_user_asm_goto!($x, $ptr, $label, ""), _ => panic!("BUILD_BUG") }
}}; }

#[macro_export] macro_rules! __get_user_asm_goto { ($($t:tt)*) => { unsafe { core::arch::asm!("", options(nostack, preserves_flags)); } }; }
#[macro_export] macro_rules! __get_user_asm2_goto { ($($t:tt)*) => { __get_user_asm_goto!($($t)*) }; }
#[macro_export] macro_rules! __get_user_size_goto { ($x:expr, $ptr:expr, $size:expr, $label:tt) => {{
    if $size != core::mem::size_of_val(&$x) { panic!("BUILD_BUG_ON"); }
    match $size { 1 | 2 | 4 | 8 => __get_user_asm_goto!($x, $ptr, $label, ""), _ => panic!("BUILD_BUG") }
}}; }
#[macro_export] macro_rules! __get_user_size_allowed { ($x:expr, $ptr:expr, $size:expr, $retval:expr) => {{ __get_user_size_goto!($x, $ptr, $size, __gus_failed); $retval = 0; }}; }
#[macro_export] macro_rules! __long_type { ($x:expr) => { usize }; }
#[macro_export] macro_rules! __get_user { ($x:expr, $ptr:expr) => {{
    might_fault(); barrier_nospec(); allow_user_access(core::ptr::null_mut(), KUAP_READ);
    let mut e: i64 = 0; __get_user_size_allowed!($x, $ptr, core::mem::size_of_val(&*$ptr), e);
    prevent_user_access(KUAP_READ); e
}}; }
#[macro_export] macro_rules! get_user { ($x:expr, $ptr:expr) => {{
    let p = $ptr; if unsafe { access_ok(p, core::mem::size_of_val(&*p)) } { __get_user!($x, p) } else { $x = 0; -EFAULT }
}}; }

extern "C" {
    pub fn __copy_tofrom_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, size: usize) -> usize;
    pub fn __copy_tofrom_user_base(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, size: usize) -> usize;
    pub fn __copy_tofrom_user_power7_vmx(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, size: usize) -> usize;
    pub fn __arch_clear_user(addr: *mut core::ffi::c_void, size: usize) -> usize;
    pub fn strncpy_from_user(dst: *mut i8, src: *const i8, count: isize) -> isize;
    pub fn strnlen_user(s: *const i8, n: isize) -> isize;
    pub fn copy_from_user_flushcache(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> usize;
}

pub unsafe fn will_use_vmx(n: usize) -> bool { cfg!(CONFIG_ALTIVEC) && cpu_has_feature(CPU_FTR_VMX_COPY) && n > VMX_COPY_THRESHOLD }
pub unsafe fn raw_copy_tofrom_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize, dir: usize) -> usize {
    if will_use_vmx(n) && enter_vmx_usercopy() { allow_user_access(to, dir); let mut r = __copy_tofrom_user_power7_vmx(to, from, n); prevent_user_access(dir); exit_vmx_usercopy(); if r != 0 { allow_user_access(to, dir); r = __copy_tofrom_user_base(to, from, n); prevent_user_access(dir); } return r; }
    allow_user_access(to, dir); let r = __copy_tofrom_user(to, from, n); prevent_user_access(dir); r
}
pub unsafe fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize { raw_copy_tofrom_user(to, from, n, KUAP_READ) }
pub unsafe fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize { raw_copy_tofrom_user(to, from, n, KUAP_WRITE) }
pub unsafe fn __clear_user(addr: *mut core::ffi::c_void, size: usize) -> usize { might_fault(); allow_user_access(addr, KUAP_WRITE); let r = __arch_clear_user(addr, size); prevent_user_access(KUAP_WRITE); r }
pub unsafe fn clear_user(addr: *mut core::ffi::c_void, size: usize) -> usize { if access_ok(addr, size) { __clear_user(addr, size) } else { size } }

pub unsafe fn __user_access_begin(ptr: *const core::ffi::c_void, len: usize, dir: usize) -> bool { if !access_ok(ptr, len) { return false; } might_fault(); if dir & KUAP_READ != 0 { barrier_nospec(); } allow_user_access(ptr as *mut _, dir); true }
pub unsafe fn mask_user_address_simple(ptr: *const core::ffi::c_void) -> *mut core::ffi::c_void { let a = ptr as usize; let m = (((a as isize) >> (usize::BITS - 1)) as usize) & isize::MAX as usize; (a & !m) as *mut _ }
pub unsafe fn mask_user_address_fallback(ptr: *const core::ffi::c_void) -> *mut core::ffi::c_void { let a = ptr as usize; (if a < TASK_SIZE { a } else { TASK_SIZE }) as *mut _ }
pub unsafe fn mask_user_address(ptr: *const core::ffi::c_void) -> *mut core::ffi::c_void { if cfg!(target_pointer_width = "64") { mask_user_address_simple(ptr) } else { mask_user_address_fallback(ptr) } }
pub unsafe fn __masked_user_access_begin(p: *const core::ffi::c_void, dir: usize) -> *mut core::ffi::c_void { let ptr = mask_user_address(p); might_fault(); allow_user_access(ptr, dir); ptr }

#[macro_export] macro_rules! user_access_begin { ($p:expr, $l:expr) => { unsafe { __user_access_begin($p, $l, KUAP_READ_WRITE) } }; }
#[macro_export] macro_rules! user_read_access_begin { ($p:expr, $l:expr) => { unsafe { __user_access_begin($p, $l, KUAP_READ) } }; }
#[macro_export] macro_rules! user_write_access_begin { ($p:expr, $l:expr) => { unsafe { __user_access_begin($p, $l, KUAP_WRITE) } }; }
#[macro_export] macro_rules! user_access_end { () => { unsafe { prevent_user_access(KUAP_READ_WRITE) } }; }
#[macro_export] macro_rules! user_read_access_end { () => { unsafe { prevent_user_access(KUAP_READ) } }; }
#[macro_export] macro_rules! user_write_access_end { () => { unsafe { prevent_user_access(KUAP_WRITE) } }; }
#[macro_export] macro_rules! masked_user_access_begin { ($p:expr) => { unsafe { __masked_user_access_begin($p, KUAP_READ_WRITE) } }; }
#[macro_export] macro_rules! masked_user_read_access_begin { ($p:expr) => { unsafe { __masked_user_access_begin($p, KUAP_READ) } }; }
#[macro_export] macro_rules! masked_user_write_access_begin { ($p:expr) => { unsafe { __masked_user_access_begin($p, KUAP_WRITE) } }; }

#[macro_export] macro_rules! arch_unsafe_get_user { ($x:expr, $p:expr, $e:tt) => { __get_user_size_goto!($x, $p, core::mem::size_of_val(&$x), $e) }; }
#[macro_export] macro_rules! arch_unsafe_put_user { ($x:expr, $p:expr, $e:tt) => { __put_user_size_goto!($x, $p, core::mem::size_of_val(&$p), $e) }; }
#[macro_export] macro_rules! unsafe_copy_from_user { ($d:expr, $s:expr, $l:expr, $e:tt) => {{
    let mut i = 0usize; let len = $l;
    while i < (len & !(core::mem::size_of::<u64>() - 1)) { unsafe_get_user!(*(($d as *mut u8).add(i) as *mut u64), (($s as *const u8).add(i) as *const u64), $e); i += 8; }
    if len & 4 != 0 { unsafe_get_user!(*(($d as *mut u8).add(i) as *mut u32), (($s as *const u8).add(i) as *const u32), $e); i += 4; }
    if len & 2 != 0 { unsafe_get_user!(*(($d as *mut u8).add(i) as *mut u16), (($s as *const u8).add(i) as *const u16), $e); i += 2; }
    if len & 1 != 0 { unsafe_get_user!(*(($d as *mut u8).add(i) as *mut u8), (($s as *const u8).add(i) as *const u8), $e); }
}}; }
#[macro_export] macro_rules! unsafe_copy_to_user { ($d:expr, $s:expr, $l:expr, $e:tt) => { unsafe_copy_from_user!($d, $s, $l, $e) }; }
#[macro_export] macro_rules! arch_get_kernel_nofault { ($dst:expr, $src:expr, $ty:ty, $label:tt) => { __get_user_size_goto!(*($dst as *mut $ty), $src as *const $ty, core::mem::size_of::<$ty>(), $label) }; }
#[macro_export] macro_rules! arch_put_kernel_nofault { ($dst:expr, $src:expr, $ty:ty, $label:tt) => { __put_user_size_goto!(*($src as *const $ty), $dst as *mut $ty, core::mem::size_of::<$ty>(), $label) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
