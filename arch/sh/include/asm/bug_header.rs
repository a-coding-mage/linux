/* SPDX-License-Identifier: GPL-2.0 */

/* The C header includes linux/linkage.h and asm-generic/bug.h.  Their
 * declarations and build-time configuration are supplied by other files. */

pub const TRAPA_BUG_OPCODE: u16 = 0xc33e; /* trapa #0x3e */
pub const BUGFLAG_UNWINDER: i32 = 1 << 1;

/* CONFIG_GENERIC_BUG / CONFIG_DEBUG_BUGVERBOSE are build-time conditions
 * from the original header. */
#[cfg(feature = "CONFIG_GENERIC_BUG")]
pub const HAVE_ARCH_BUG: bool = true;
#[cfg(feature = "CONFIG_GENERIC_BUG")]
pub const HAVE_ARCH_WARN_ON: bool = true;

#[cfg(feature = "CONFIG_GENERIC_BUG")]
#[inline(always)]
pub unsafe fn bug() -> ! {
    /* The original emits a .short TRAPA_BUG_OPCODE and a __bug_table entry.
     * Target-specific assembly and struct bug_entry are external dependencies. */
    core::hint::unreachable_unchecked()
}

#[cfg(feature = "CONFIG_GENERIC_BUG")]
#[inline(always)]
pub unsafe fn __warn_flags(_cond_str: *const core::ffi::c_char, _flags: i32) {
    /* The original emits the trap and __bug_table entry; see bug(). */
}

#[cfg(feature = "CONFIG_GENERIC_BUG")]
#[inline(always)]
pub fn warn_on(x: i32) -> i32 {
    let ret_warn_on: i32 = if x != 0 { 1 } else { 0 };
    if ret_warn_on != 0 {
        unsafe { __warn_flags(core::ptr::null(), 0) };
    }
    ret_warn_on
}

#[cfg(feature = "CONFIG_GENERIC_BUG")]
#[inline(always)]
pub unsafe fn unwinder_bug() {
    /* The original emits TRAPA_BUG_OPCODE with BUGFLAG_UNWINDER. */
    __warn_flags(core::ptr::null(), BUGFLAG_UNWINDER);
}

#[cfg(feature = "CONFIG_GENERIC_BUG")]
#[inline(always)]
pub fn unwinder_bug_on(x: i32) -> i32 {
    let ret_unwinder_on: i32 = if x != 0 { 1 } else { 0 };
    if ret_unwinder_on != 0 {
        unsafe { unwinder_bug() };
    }
    ret_unwinder_on
}

#[cfg(not(feature = "CONFIG_GENERIC_BUG"))]
pub use bug as unwinder_bug;
#[cfg(not(feature = "CONFIG_GENERIC_BUG"))]
pub use bug_on as unwinder_bug_on;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn die(
        str_: *const core::ffi::c_char,
        regs: *mut pt_regs,
        err: core::ffi::c_long,
    ) -> !;
    pub fn die_if_kernel(
        str_: *const core::ffi::c_char,
        regs: *mut pt_regs,
        err: core::ffi::c_long,
    );
    pub fn die_if_no_fixup(
        str_: *const core::ffi::c_char,
        regs: *mut pt_regs,
        err: core::ffi::c_long,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
