/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from the UAPI x86 ptrace header.
// The original includes provide __user, ptrace ABI definitions, and processor flags.

// The original declarations are excluded for kernel and assembler builds.

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct pt_regs {
    pub ebx: ::kernel::ffi::c_long,
    pub ecx: ::kernel::ffi::c_long,
    pub edx: ::kernel::ffi::c_long,
    pub esi: ::kernel::ffi::c_long,
    pub edi: ::kernel::ffi::c_long,
    pub ebp: ::kernel::ffi::c_long,
    pub eax: ::kernel::ffi::c_long,
    pub xds: ::kernel::ffi::c_int,
    pub xes: ::kernel::ffi::c_int,
    pub xfs: ::kernel::ffi::c_int,
    pub xgs: ::kernel::ffi::c_int,
    pub orig_eax: ::kernel::ffi::c_long,
    pub eip: ::kernel::ffi::c_long,
    pub xcs: ::kernel::ffi::c_int,
    pub eflags: ::kernel::ffi::c_long,
    pub esp: ::kernel::ffi::c_long,
    pub xss: ::kernel::ffi::c_int,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct pt_regs {
    // C ABI says these regs are callee-preserved. They aren't saved on kernel
    // entry unless syscall needs a complete, fully filled "struct pt_regs".
    pub r15: ::kernel::ffi::c_ulong,
    pub r14: ::kernel::ffi::c_ulong,
    pub r13: ::kernel::ffi::c_ulong,
    pub r12: ::kernel::ffi::c_ulong,
    pub rbp: ::kernel::ffi::c_ulong,
    pub rbx: ::kernel::ffi::c_ulong,
    // These regs are callee-clobbered. Always saved on kernel entry.
    pub r11: ::kernel::ffi::c_ulong,
    pub r10: ::kernel::ffi::c_ulong,
    pub r9: ::kernel::ffi::c_ulong,
    pub r8: ::kernel::ffi::c_ulong,
    pub rax: ::kernel::ffi::c_ulong,
    pub rcx: ::kernel::ffi::c_ulong,
    pub rdx: ::kernel::ffi::c_ulong,
    pub rsi: ::kernel::ffi::c_ulong,
    pub rdi: ::kernel::ffi::c_ulong,
    // On syscall entry, this is syscall#. On CPU exception, this is error code.
    // On hw interrupt, it's IRQ number:
    pub orig_rax: ::kernel::ffi::c_ulong,
    // Return frame for iretq
    pub rip: ::kernel::ffi::c_ulong,
    pub cs: ::kernel::ffi::c_ulong,
    pub eflags: ::kernel::ffi::c_ulong,
    pub rsp: ::kernel::ffi::c_ulong,
    pub ss: ::kernel::ffi::c_ulong,
    // top of stack page
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
