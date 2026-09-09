/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependencies are supplied by the surrounding kernel translation. */

pub const KUAP_READ: i32 = 1;
pub const KUAP_WRITE: i32 = 2;
pub const KUAP_READ_WRITE: i32 = KUAP_READ | KUAP_WRITE;

#[cfg(not(feature = "assembler"))]
extern "C" {
    pub static mut disable_kuep: bool;
    pub static mut disable_kuap: bool;

    pub fn setup_kup();
    pub fn setup_kuep(disabled: bool);
}

#[cfg(all(not(feature = "assembler"), feature = "ppc_kuap"))]
extern "C" {
    pub fn setup_kuap(disabled: bool);
}

#[cfg(all(not(feature = "assembler"), not(feature = "ppc_kuap")))]
#[inline(always)]
pub fn setup_kuap(_disabled: bool) {}

#[cfg(all(not(feature = "assembler"), feature = "ppc_kuap"))]
#[inline(always)]
pub fn kuap_is_disabled() -> bool {
    // Equivalent to !mmu_has_feature(MMU_FTR_KUAP).
    !unsafe { mmu_has_feature(MMU_FTR_KUAP) }
}

#[cfg(all(not(feature = "assembler"), not(feature = "ppc_kuap")))]
#[inline(always)]
pub const fn kuap_is_disabled() -> bool { true }

#[cfg(all(not(feature = "assembler"), not(feature = "ppc_kuap")))]
#[inline(always)]
pub unsafe fn __bad_kuap_fault(
    _regs: *mut pt_regs,
    _address: c_ulong,
    _is_write: bool,
) -> bool { false }

#[cfg(all(not(feature = "assembler"), not(feature = "ppc_kuap")))]
#[inline(always)]
pub unsafe fn kuap_user_restore(_regs: *mut pt_regs) {}

#[cfg(all(not(feature = "assembler"), not(feature = "ppc_kuap")))]
#[inline(always)]
pub unsafe fn __kuap_kernel_restore(_regs: *mut pt_regs, _amr: c_ulong) {}

/* book3s/64/kup-radix.h supplies these on the !KUAP case for cache flushing. */
#[cfg(all(not(feature = "assembler"), not(feature = "ppc_kuap"), not(feature = "ppc_book3s_64")))]
#[inline(always)]
pub unsafe fn allow_user_access(_to: *mut core::ffi::c_void, _dir: c_ulong) {}

#[cfg(all(not(feature = "assembler"), not(feature = "ppc_kuap"), not(feature = "ppc_book3s_64")))]
#[inline(always)]
pub unsafe fn prevent_user_access(_dir: c_ulong) {}

#[cfg(all(not(feature = "assembler"), not(feature = "ppc_kuap"), not(feature = "ppc_book3s_64")))]
#[inline(always)]
pub unsafe fn prevent_user_access_return() -> c_ulong { 0 }

#[cfg(all(not(feature = "assembler"), not(feature = "ppc_kuap"), not(feature = "ppc_book3s_64")))]
#[inline(always)]
pub unsafe fn restore_user_access(_flags: c_ulong) {}

#[cfg(not(feature = "assembler"))]
#[inline(always)]
pub unsafe fn bad_kuap_fault(regs: *mut pt_regs, address: c_ulong, is_write: bool) -> bool {
    if kuap_is_disabled() { return false; }
    __bad_kuap_fault(regs, address, is_write)
}

#[cfg(not(feature = "assembler"))]
#[inline(always)]
pub unsafe fn kuap_lock() {
    // The C implementation is conditional on the platform-provided __kuap_lock macro.
    #[cfg(feature = "kuap_lock")]
    if !kuap_is_disabled() { __kuap_lock(); }
}

#[cfg(not(feature = "assembler"))]
#[inline(always)]
pub unsafe fn kuap_save_and_lock(regs: *mut pt_regs) {
    #[cfg(feature = "kuap_save_and_lock")]
    if !kuap_is_disabled() { __kuap_save_and_lock(regs); }
}

#[cfg(not(feature = "assembler"))]
#[inline(always)]
pub unsafe fn kuap_kernel_restore(regs: *mut pt_regs, amr: c_ulong) {
    if kuap_is_disabled() { return; }
    __kuap_kernel_restore(regs, amr);
}

#[cfg(not(feature = "assembler"))]
#[inline(always)]
pub unsafe fn kuap_get_and_assert_locked() -> c_ulong {
    #[cfg(feature = "kuap_get_and_assert_locked")]
    if !kuap_is_disabled() { return __kuap_get_and_assert_locked(); }
    0
}

#[cfg(not(feature = "assembler"))]
#[inline(always)]
pub unsafe fn kuap_assert_locked() {
    if cfg!(feature = "ppc_kuap_debug") { kuap_get_and_assert_locked(); }
}

/* External kernel types and symbols intentionally remain unresolved here. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
