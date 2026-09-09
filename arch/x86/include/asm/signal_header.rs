/* SPDX-License-Identifier: GPL-2.0 */

/* Translation of the non-assembler portion of asm/signal.h. */

pub const _NSIG: usize = 64;

/* The original selects this at build time for i386 versus other x86 targets. */
#[cfg(target_arch = "x86")]
pub const _NSIG_BPW: usize = 32;
#[cfg(not(target_arch = "x86"))]
pub const _NSIG_BPW: usize = 64;

pub const _NSIG_WORDS: usize = _NSIG / _NSIG_BPW;

pub type old_sigset_t = core::ffi::c_ulong;

#[repr(C)]
pub struct sigset_t {
    pub sig: [core::ffi::c_ulong; _NSIG_WORDS],
}

/* Non-uapi in-kernel SA_FLAGS indicating the ABI for a signal frame. */
pub const SA_IA32_ABI: u32 = 0x02000000u32;
pub const SA_X32_ABI: u32 = 0x01000000u32;

pub const __ARCH_HAS_SA_RESTORER: bool = true;

#[cfg(target_arch = "x86")]
pub const __HAVE_ARCH_SIG_BITOPS: bool = true;

#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn __gen_sigaddset(set: *mut sigset_t, _sig: i32) {
    core::arch::asm!(
        "btsl {bit}, [{set}]",
        bit = in(reg) (_sig - 1),
        set = in(reg) set,
        options(nostack)
    );
}

#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn __const_sigaddset(set: *mut sigset_t, _sig: i32) {
    let sig = (_sig - 1) as usize;
    (*set).sig[sig / _NSIG_BPW] |= 1 as core::ffi::c_ulong << (sig % _NSIG_BPW);
}

#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn __gen_sigdelset(set: *mut sigset_t, _sig: i32) {
    core::arch::asm!(
        "btrl {bit}, [{set}]",
        bit = in(reg) (_sig - 1),
        set = in(reg) set,
        options(nostack)
    );
}

#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn __const_sigdelset(set: *mut sigset_t, _sig: i32) {
    let sig = (_sig - 1) as usize;
    (*set).sig[sig / _NSIG_BPW] &= !(1 as core::ffi::c_ulong << (sig % _NSIG_BPW));
}

#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn __const_sigismember(set: *mut sigset_t, _sig: i32) -> i32 {
    let sig = (_sig - 1) as usize;
    (1 & ((*set).sig[sig / _NSIG_BPW] >> (sig % _NSIG_BPW))) as i32
}

#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn __gen_sigismember(set: *mut sigset_t, _sig: i32) -> bool {
    let ret: u8;
    core::arch::asm!(
        "btl {bit}, [{set}]",
        "setc {ret}",
        bit = in(reg) (_sig - 1),
        set = in(reg) set,
        ret = lateout(reg_byte) ret,
        options(nostack)
    );
    ret != 0
}

#[cfg(target_arch = "x86")]
pub struct pt_regs;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
