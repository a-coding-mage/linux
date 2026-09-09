/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of arch/arm/include/asm/uaccess.h. */

/* External kernel types, constants, and helpers are supplied by other units. */

#[cfg(CONFIG_CPU_SW_DOMAIN_PAN)]
#[inline(always)]
pub unsafe fn uaccess_save_and_enable() -> u32 {
    let old_domain = get_domain();
    set_domain((old_domain & !domain_mask(DOMAIN_USER)) |
               domain_val(DOMAIN_USER, DOMAIN_CLIENT));
    old_domain
}

#[cfg(CONFIG_CPU_SW_DOMAIN_PAN)]
#[inline(always)]
pub unsafe fn uaccess_restore(flags: u32) { set_domain(flags); }

#[cfg(all(not(CONFIG_CPU_SW_DOMAIN_PAN), CONFIG_CPU_TTBR0_PAN))]
#[inline(always)]
pub unsafe fn uaccess_save_and_enable() -> u32 {
    let old_ttbcr = cpu_get_ttbcr();
    cpu_set_ttbcr(old_ttbcr & !(TTBCR_A1 | TTBCR_EPD0 | TTBCR_T0SZ_MASK));
    isb();
    old_ttbcr
}

#[cfg(all(not(CONFIG_CPU_SW_DOMAIN_PAN), CONFIG_CPU_TTBR0_PAN))]
#[inline]
pub unsafe fn uaccess_restore(flags: u32) { cpu_set_ttbcr(flags); isb(); }

#[cfg(all(not(CONFIG_CPU_SW_DOMAIN_PAN), not(CONFIG_CPU_TTBR0_PAN)))]
#[inline]
pub unsafe fn uaccess_save_and_enable() -> u32 { 0 }

#[cfg(all(not(CONFIG_CPU_SW_DOMAIN_PAN), not(CONFIG_CPU_TTBR0_PAN)))]
#[inline]
pub unsafe fn uaccess_restore(_flags: u32) {}

extern "C" {
    pub fn __get_user_bad() -> i32;
    pub fn __put_user_bad() -> i32;
    pub fn __get_user_1(p: *mut core::ffi::c_void) -> i32;
    pub fn __get_user_2(p: *mut core::ffi::c_void) -> i32;
    pub fn __get_user_4(p: *mut core::ffi::c_void) -> i32;
    pub fn __get_user_32t_8(p: *mut core::ffi::c_void) -> i32;
    pub fn __get_user_8(p: *mut core::ffi::c_void) -> i32;
    pub fn __get_user_64t_1(p: *mut core::ffi::c_void) -> i32;
    pub fn __get_user_64t_2(p: *mut core::ffi::c_void) -> i32;
    pub fn __get_user_64t_4(p: *mut core::ffi::c_void) -> i32;
    pub fn __put_user_1(p: *mut core::ffi::c_void, value: u32) -> i32;
    pub fn __put_user_2(p: *mut core::ffi::c_void, value: u32) -> i32;
    pub fn __put_user_4(p: *mut core::ffi::c_void, value: u32) -> i32;
    pub fn __put_user_8(p: *mut core::ffi::c_void, value: u64) -> i32;
}

/* C __typeof__/statement-expression forms are represented by exported macros.
 * ARM inline assembly and exception-table fixups remain external ABI operations. */
#[macro_export]
macro_rules! uaccess_mask_range_ptr { ($ptr:expr, $size:expr) => {{
    unsafe { $crate::__uaccess_mask_range_ptr($ptr as *const _, $size) as *mut _ }
}} }

pub unsafe fn __uaccess_mask_range_ptr<T>(ptr: *const T, size: usize) -> *mut T {
    let addr = ptr as usize;
    if addr.wrapping_add(size) > TASK_SIZE as usize { core::ptr::null_mut() } else { ptr as *mut T }
}

#[macro_export]
macro_rules! get_user { ($x:expr, $p:expr) => {{ unsafe { might_fault(); $crate::__get_user($x, $p) } }} }
#[macro_export]
macro_rules! put_user { ($x:expr, $p:expr) => {{ unsafe { $crate::__put_user($x, $p) } }} }
#[macro_export]
macro_rules! __get_user { ($x:expr, $p:expr) => {{ $crate::__get_user_err($x, $p) }} }

#[inline]
pub unsafe fn __get_user<T: Copy>(dst: &mut T, ptr: *const T) -> i32 {
    __get_user_err(dst, ptr)
}

#[inline]
pub unsafe fn __get_user_err<T: Copy>(dst: &mut T, ptr: *const T) -> i32 {
    chk_user_ptr(ptr);
    might_fault();
    let flags = uaccess_save_and_enable();
    let result = core::ptr::read_volatile(ptr);
    core::ptr::write(dst, result);
    uaccess_restore(flags);
    0
}

#[inline]
pub unsafe fn __put_user<T: Copy>(value: T, ptr: *mut T) -> i32 {
    might_fault();
    let flags = uaccess_save_and_enable();
    core::ptr::write_volatile(ptr, value);
    uaccess_restore(flags);
    0
}

extern "C" {
    pub fn arm_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: u64) -> u64;
    pub fn arm_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: u64) -> u64;
    pub fn __copy_to_user_std(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: u64) -> u64;
    pub fn arm_clear_user(addr: *mut core::ffi::c_void, n: u64) -> u64;
    pub fn strncpy_from_user(dest: *mut i8, src: *const i8, count: i64) -> i64;
    pub fn strnlen_user(s: *const i8, n: i64) -> i64;
}

#[inline]
pub unsafe fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: u64) -> u64 {
    let flags = uaccess_save_and_enable();
    let remaining = arm_copy_from_user(to, from, n);
    uaccess_restore(flags);
    remaining
}

#[inline]
pub unsafe fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: u64) -> u64 {
    #[cfg(CONFIG_UACCESS_WITH_MEMCPY)]
    { return arm_copy_to_user(to, from, n); }
    #[cfg(not(CONFIG_UACCESS_WITH_MEMCPY))]
    { let flags = uaccess_save_and_enable(); let remaining = arm_copy_to_user(to, from, n); uaccess_restore(flags); remaining }
}

#[inline]
pub unsafe fn __clear_user(addr: *mut core::ffi::c_void, n: u64) -> u64 {
    let flags = uaccess_save_and_enable();
    let remaining = arm_clear_user(addr, n);
    uaccess_restore(flags);
    remaining
}

pub const INLINE_COPY_USER: bool = true;

#[inline]
pub unsafe fn clear_user(to: *mut core::ffi::c_void, n: u64) -> u64 {
    if access_ok(to, n) { __clear_user(to, n) } else { n }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
