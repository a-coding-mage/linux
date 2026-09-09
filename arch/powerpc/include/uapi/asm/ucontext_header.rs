/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The C header includes architecture-specific definitions from asm/sigcontext.h,
// asm/elf.h, and asm/signal.h. Those supplied types are referenced below.

#[cfg(not(target_arch = "powerpc64"))]
#[repr(C, align(16))]
pub struct mcontext {
    pub mc_gregs: elf_gregset_t,
    pub mc_fregs: elf_fpregset_t,
    pub mc_pad: [c_ulong; 2],
    pub mc_vregs: elf_vrregset_t,
}

#[repr(C)]
pub struct ucontext {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    #[cfg(not(target_arch = "powerpc64"))]
    pub uc_pad: [c_int; 7],
    #[cfg(not(target_arch = "powerpc64"))]
    pub uc_regs: *mut mcontext,
    pub uc_sigmask: sigset_t,
    // glibc has 1024-bit signal masks, ours are 64-bit
    #[cfg(target_arch = "powerpc64")]
    pub __unused: [sigset_t; 15],
    #[cfg(target_arch = "powerpc64")]
    pub uc_mcontext: sigcontext,
    #[cfg(not(target_arch = "powerpc64"))]
    pub uc_maskext: [c_int; 30],
    #[cfg(not(target_arch = "powerpc64"))]
    pub uc_pad2: [c_int; 3],
    #[cfg(not(target_arch = "powerpc64"))]
    pub uc_mcontext: mcontext,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
