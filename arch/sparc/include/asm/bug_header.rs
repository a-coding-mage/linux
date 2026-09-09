/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _SPARC_BUG_H */

/* CONFIG_BUG is a build-time condition from the original header. */
#[cfg(CONFIG_BUG)]
extern "C" {
    #[cfg(CONFIG_DEBUG_BUGVERBOSE)]
    pub fn do_BUG(file: *const core::ffi::c_char, line: core::ffi::c_int);

    pub fn barrier_before_unreachable();
    pub fn __builtin_trap() -> !;
}

#[cfg(CONFIG_BUG)]
#[inline(always)]
pub unsafe fn BUG() -> ! {
    #[cfg(CONFIG_DEBUG_BUGVERBOSE)]
    {
        /* __FILE__ and __LINE__ are supplied by the C preprocessor. */
        do_BUG(core::ptr::null(), 0);
    }
    barrier_before_unreachable();
    __builtin_trap()
}

#[cfg(CONFIG_BUG)]
pub const HAVE_ARCH_BUG: bool = true;

/* asm-generic/bug.h is an external dependency of the original header. */

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn die_if_kernel(
        str_: *mut core::ffi::c_char,
        regs: *mut pt_regs,
    ) -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
