/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from the UAPI x86 ptrace header.
// The original includes provide __user, ptrace ABI definitions, and processor flags.

// The original declarations are excluded for kernel and assembler builds.

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct pt_regs {
    pub ebx: ::core::ffi::c_long,
    pub ecx: ::core::ffi::c_long,
    pub edx: ::core::ffi::c_long,
    pub esi: ::core::ffi::c_long,
    pub edi: ::core::ffi::c_long,
    pub ebp: ::core::ffi::c_long,
    pub eax: ::core::ffi::c_long,
    pub xds: ::core::ffi::c_int,
    pub xes: ::core::ffi::c_int,
    pub xfs: ::core::ffi::c_int,
    pub xgs: ::core::ffi::c_int,
    pub orig_eax: ::core::ffi::c_long,
    pub eip: ::core::ffi::c_long,
    pub xcs: ::core::ffi::c_int,
    pub eflags: ::core::ffi::c_long,
    pub esp: ::core::ffi::c_long,
    pub xss: ::core::ffi::c_int,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct pt_regs {
    // C ABI says these regs are callee-preserved. They aren't saved on kernel
    // entry unless syscall needs a complete, fully filled "struct pt_regs".
    pub r15: ::core::ffi::c_ulong,
    pub r14: ::core::ffi::c_ulong,
    pub r13: ::core::ffi::c_ulong,
    pub r12: ::core::ffi::c_ulong,
    pub rbp: ::core::ffi::c_ulong,
    pub rbx: ::core::ffi::c_ulong,
    // These regs are callee-clobbered. Always saved on kernel entry.
    pub r11: ::core::ffi::c_ulong,
    pub r10: ::core::ffi::c_ulong,
    pub r9: ::core::ffi::c_ulong,
    pub r8: ::core::ffi::c_ulong,
    pub rax: ::core::ffi::c_ulong,
    pub rcx: ::core::ffi::c_ulong,
    pub rdx: ::core::ffi::c_ulong,
    pub rsi: ::core::ffi::c_ulong,
    pub rdi: ::core::ffi::c_ulong,
    // On syscall entry, this is syscall#. On CPU exception, this is error code.
    // On hw interrupt, it's IRQ number:
    pub orig_rax: ::core::ffi::c_ulong,
    // Return frame for iretq
    pub rip: ::core::ffi::c_ulong,
    pub cs: ::core::ffi::c_ulong,
    pub eflags: ::core::ffi::c_ulong,
    pub rsp: ::core::ffi::c_ulong,
    pub ss: ::core::ffi::c_ulong,
    // top of stack page
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
