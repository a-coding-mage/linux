/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are referenced here.

pub struct coredump_params;

#[repr(C)]
pub struct elf_siginfo {
    pub si_signo: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
}

/*
 * Definitions to generate Intel SVR4-like core files.
 * These mostly have the same names as the SVR4 types with "elf_"
 * tacked on the front to prevent clashes with linux definitions,
 * and the typedef forms have been avoided.  This is mostly like the
 * SVR4 structure, but more Linuxy, with things that Linux does
 * not support and which gdb doesn't really use excluded.
 */
#[repr(C)]
pub struct elf_prstatus_common {
    pub pr_info: elf_siginfo,
    pub pr_cursig: ::core::ffi::c_short,
    pub pr_sigpend: ::core::ffi::c_ulong,
    pub pr_sighold: ::core::ffi::c_ulong,
    pub pr_pid: pid_t,
    pub pr_ppid: pid_t,
    pub pr_pgrp: pid_t,
    pub pr_sid: pid_t,
    pub pr_utime: __kernel_old_timeval,
    pub pr_stime: __kernel_old_timeval,
    pub pr_cutime: __kernel_old_timeval,
    pub pr_cstime: __kernel_old_timeval,
}

#[repr(C)]
pub struct elf_prstatus {
    pub common: elf_prstatus_common,
    pub pr_reg: elf_gregset_t,
    pub pr_fpvalid: ::core::ffi::c_int,
}

pub const ELF_PRARGSZ: usize = 80;

#[repr(C)]
pub struct elf_prpsinfo {
    pub pr_state: ::core::ffi::c_char,
    pub pr_sname: ::core::ffi::c_char,
    pub pr_zomb: ::core::ffi::c_char,
    pub pr_nice: ::core::ffi::c_char,
    pub pr_flag: ::core::ffi::c_ulong,
    pub pr_uid: __kernel_uid_t,
    pub pr_gid: __kernel_gid_t,
    pub pr_pid: pid_t,
    pub pr_ppid: pid_t,
    pub pr_pgrp: pid_t,
    pub pr_sid: pid_t,
    /* Lots missing */
    /*
     * The hard-coded 16 is derived from TASK_COMM_LEN, but it can't be
     * changed as it is exposed to userspace. We'd better make it hard-coded
     * here.
     */
    pub pr_fname: [::core::ffi::c_char; 16],
    pub pr_psargs: [::core::ffi::c_char; ELF_PRARGSZ],
}

pub unsafe fn elf_core_copy_regs(elfregs: *mut elf_gregset_t, regs: *mut pt_regs) {
    // ELF_CORE_COPY_REGS is a target-specific build-time macro.
    #[cfg(feature = "ELF_CORE_COPY_REGS")]
    {
        // TODO: invoke the target-specific ELF_CORE_COPY_REGS operation.
    }
    #[cfg(not(feature = "ELF_CORE_COPY_REGS"))]
    {
        BUG_ON(core::mem::size_of::<elf_gregset_t>() != core::mem::size_of::<pt_regs>());
        *(elfregs as *mut pt_regs) = *regs;
    }
}

pub unsafe fn elf_core_copy_task_regs(t: *mut task_struct, elfregs: *mut elf_gregset_t) -> ::core::ffi::c_int {
    // ELF_CORE_COPY_TASK_REGS is a target-specific build-time macro.
    #[cfg(feature = "ELF_CORE_COPY_TASK_REGS")]
    {
        // TODO: invoke the target-specific ELF_CORE_COPY_TASK_REGS operation.
        return 0;
    }
    #[cfg(not(feature = "ELF_CORE_COPY_TASK_REGS"))]
    {
        elf_core_copy_regs(elfregs, task_pt_regs(t));
    }
    0
}

unsafe extern "C" {
    pub fn elf_core_copy_task_fpregs(t: *mut task_struct, fpu: *mut elf_fpregset_t) -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_ARCH_BINFMT_ELF_EXTRA_PHDRS")]
unsafe extern "C" {
    pub fn elf_core_extra_phdrs(cprm: *mut coredump_params) -> Elf_Half;
    pub fn elf_core_write_extra_phdrs(cprm: *mut coredump_params, offset: loff_t) -> ::core::ffi::c_int;
    pub fn elf_core_write_extra_data(cprm: *mut coredump_params) -> ::core::ffi::c_int;
    pub fn elf_core_extra_data_size(cprm: *mut coredump_params) -> usize;
}

#[cfg(not(feature = "CONFIG_ARCH_BINFMT_ELF_EXTRA_PHDRS"))]
pub unsafe fn elf_core_extra_phdrs(_cprm: *mut coredump_params) -> Elf_Half { 0 }

#[cfg(not(feature = "CONFIG_ARCH_BINFMT_ELF_EXTRA_PHDRS"))]
pub unsafe fn elf_core_write_extra_phdrs(_cprm: *mut coredump_params, _offset: loff_t) -> ::core::ffi::c_int { 1 }

#[cfg(not(feature = "CONFIG_ARCH_BINFMT_ELF_EXTRA_PHDRS"))]
pub unsafe fn elf_core_write_extra_data(_cprm: *mut coredump_params) -> ::core::ffi::c_int { 1 }

#[cfg(not(feature = "CONFIG_ARCH_BINFMT_ELF_EXTRA_PHDRS"))]
pub unsafe fn elf_core_extra_data_size(_cprm: *mut coredump_params) -> usize { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
