/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <uapi/asm/signal.h>
// C dependency: <asm/sigcontext.h>

pub const _NSIG: usize = 64;
pub const _NSIG_BPW: usize = 32;
pub const _NSIG_WORDS: usize = _NSIG / _NSIG_BPW;

pub type old_sigset_t = kernel::ffi::c_ulong;

#[repr(C)]
pub struct sigset_t {
    pub sig: [kernel::ffi::c_ulong; _NSIG_WORDS],
}

pub const __ARCH_HAS_SA_RESTORER: bool = true;

// C condition: !CONFIG_CPU_HAS_NO_BITFIELDS
pub const __HAVE_ARCH_SIG_BITOPS: bool = true;

#[inline]
pub unsafe fn sigaddset(set: *mut sigset_t, _sig: kernel::ffi::c_int) {
    let bit: kernel::ffi::c_int = (_sig - 1) ^ 31;
    core::arch::asm!(
        "bfset [{set}]{{{bit}, #1}}",
        set = in(reg) set,
        bit = in(reg) bit,
        options(nostack, preserves_flags),
    );
}

#[inline]
pub unsafe fn sigdelset(set: *mut sigset_t, _sig: kernel::ffi::c_int) {
    let bit: kernel::ffi::c_int = (_sig - 1) ^ 31;
    core::arch::asm!(
        "bfclr [{set}]{{{bit}, #1}}",
        set = in(reg) set,
        bit = in(reg) bit,
        options(nostack, preserves_flags),
    );
}

#[inline]
pub unsafe fn __const_sigismember(set: *mut sigset_t, _sig: kernel::ffi::c_int) -> kernel::ffi::c_int {
    let sig: kernel::ffi::c_ulong = (_sig - 1) as kernel::ffi::c_ulong;
    1 & ((*set).sig[(sig / _NSIG_BPW as kernel::ffi::c_ulong) as usize]
        >> (sig % _NSIG_BPW as kernel::ffi::c_ulong)) as kernel::ffi::c_int
}

#[inline]
pub unsafe fn __gen_sigismember(set: *mut sigset_t, _sig: kernel::ffi::c_int) -> kernel::ffi::c_int {
    let bit: kernel::ffi::c_int = (_sig - 1) ^ 31;
    let mut ret: kernel::ffi::c_int;
    core::arch::asm!(
        "bfextu [{set}]{{{bit}, #1}}, {ret}",
        set = in(reg) set,
        bit = in(reg) bit,
        ret = lateout(reg) ret,
        options(nostack),
    );
    ret
}

#[macro_export]
macro_rules! sigismember {
    ($set:expr, $sig:expr) => {{
        // C uses __builtin_constant_p($sig) to select the constant path.
        unsafe { $crate::__gen_sigismember($set, $sig) }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
