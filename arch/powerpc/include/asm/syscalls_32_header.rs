/* SPDX-License-Identifier: GPL-2.0-or-later */

// Data types and macros for providing 32b PowerPC support.
// These are here to support 32-bit syscalls on a 64-bit kernel.
//
// The following types are supplied by the corresponding kernel headers:
// compat_uptr_t, compat_stack_t, compat_sigset_t, elf_gregset_t32,
// elf_fpregset_t, elf_vrregset_t32, and elf_vsrreghalf_t32.

#[repr(C)]
pub struct pt_regs32 {
    pub gpr: [u32; 32],
    pub nip: u32,
    pub msr: u32,
    pub orig_gpr3: u32, // Used for restarting system calls
    pub ctr: u32,
    pub link: u32,
    pub xer: u32,
    pub ccr: u32,
    pub mq: u32, // 601 only (not used at present)
    pub trap: u32, // Reason for being here
    pub dar: u32, // Fault registers
    pub dsisr: u32,
    pub result: u32, // Result of a system call
}

#[repr(C)]
pub struct sigcontext32 {
    pub _unused: [u32; 4],
    pub signal: i32,
    pub handler: compat_uptr_t,
    pub oldmask: u32,
    pub regs: compat_uptr_t, // 4 byte pointer to the pt_regs32 structure.
}

#[repr(C)]
pub struct mcontext32 {
    pub mc_gregs: elf_gregset_t32,
    pub mc_fregs: elf_fpregset_t,
    pub mc_pad: [u32; 2],
    // __attribute__((__aligned__(16)))
    pub mc_vregs: elf_vrregset_t32,
    // __attribute__((__aligned__(16)))
    pub mc_vsregs: elf_vsrreghalf_t32,
}

#[repr(C)]
pub struct ucontext32 {
    pub uc_flags: u32,
    pub uc_link: u32,
    pub uc_stack: compat_stack_t,
    pub uc_pad: [i32; 7],
    pub uc_regs: compat_uptr_t, // points to uc_mcontext field
    pub uc_sigmask: compat_sigset_t, // mask last for extensibility
    // glibc has 1024-bit signal masks, ours are 64-bit
    pub uc_maskext: [i32; 30],
    pub uc_pad2: [i32; 3],
    pub uc_mcontext: mcontext32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
