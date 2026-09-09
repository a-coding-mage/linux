/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by other headers are intentionally referenced here.

#[cfg(feature = "CONFIG_X86_32")]
pub type sigframe_ia32 = sigframe;
#[cfg(feature = "CONFIG_X86_32")]
pub type rt_sigframe_ia32 = rt_sigframe;
#[cfg(feature = "CONFIG_X86_32")]
pub type ucontext_ia32 = ucontext;

#[cfg(all(not(feature = "CONFIG_X86_32"), feature = "CONFIG_IA32_EMULATION"))]
// C: #include <asm/ia32.h>

#[cfg(any(feature = "CONFIG_X86_32", feature = "CONFIG_IA32_EMULATION"))]
#[repr(C)]
pub struct sigframe_ia32 {
    pub pretcode: u32,
    pub sig: i32,
    pub sc: sigcontext_32,
    /*
     * fpstate is unused. fpstate is moved/allocated after
     * retcode[] below. This movement allows to have the FP state and the
     * future state extensions (xsave) stay together.
     * And at the same time retaining the unused fpstate, prevents changing
     * the offset of extramask[] in the sigframe and thus prevent any
     * legacy application accessing/modifying it.
     */
    pub fpstate_unused: _fpstate_32,
    pub extramask: [u32; 1],
    pub retcode: [core::ffi::c_char; 8],
    /* fp state follows here */
}

#[cfg(any(feature = "CONFIG_X86_32", feature = "CONFIG_IA32_EMULATION"))]
#[repr(C)]
pub struct rt_sigframe_ia32 {
    pub pretcode: u32,
    pub sig: i32,
    pub pinfo: u32,
    pub puc: u32,
    #[cfg(feature = "CONFIG_IA32_EMULATION")]
    pub info: compat_siginfo_t,
    #[cfg(not(feature = "CONFIG_IA32_EMULATION"))]
    pub info: siginfo,
    pub uc: ucontext_ia32,
    pub retcode: [core::ffi::c_char; 8],
    /* fp state follows here */
}

#[cfg(feature = "CONFIG_X86_64")]
#[repr(C)]
pub struct rt_sigframe {
    pub pretcode: *mut core::ffi::c_char,
    pub uc: ucontext,
    pub info: siginfo,
    /* fp state follows here */
}

#[cfg(all(feature = "CONFIG_X86_64", feature = "CONFIG_X86_X32_ABI"))]
#[repr(C)]
pub struct ucontext_x32 {
    pub uc_flags: u32,
    pub uc_link: u32,
    pub uc_stack: compat_stack_t,
    pub uc__pad0: u32, // needed for alignment
    pub uc_mcontext: sigcontext, // the 64-bit sigcontext type
    pub uc_sigmask: compat_sigset_t, // mask last for extensibility
}

#[cfg(all(feature = "CONFIG_X86_64", feature = "CONFIG_X86_X32_ABI"))]
#[repr(C)]
pub struct rt_sigframe_x32 {
    pub pretcode: u64,
    pub uc: ucontext_x32,
    pub info: compat_siginfo_t,
    /* fp state follows here */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
