/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by linux/elf.h, linux/elfcore.h, and linux/compat.h

/*
 * Make sure these layouts match the linux/elfcore.h native definitions.
 */

#[repr(C)]
pub struct compat_elf_siginfo {
    pub si_signo: compat_int_t,
    pub si_code: compat_int_t,
    pub si_errno: compat_int_t,
}

#[repr(C)]
pub struct compat_elf_prstatus_common {
    pub pr_info: compat_elf_siginfo,
    pub pr_cursig: i16,
    pub pr_sigpend: compat_ulong_t,
    pub pr_sighold: compat_ulong_t,
    pub pr_pid: compat_pid_t,
    pub pr_ppid: compat_pid_t,
    pub pr_pgrp: compat_pid_t,
    pub pr_sid: compat_pid_t,
    pub pr_utime: old_timeval32,
    pub pr_stime: old_timeval32,
    pub pr_cutime: old_timeval32,
    pub pr_cstime: old_timeval32,
}

#[repr(C)]
pub struct compat_elf_prpsinfo {
    pub pr_state: i8,
    pub pr_sname: i8,
    pub pr_zomb: i8,
    pub pr_nice: i8,
    pub pr_flag: compat_ulong_t,
    pub pr_uid: __compat_uid_t,
    pub pr_gid: __compat_gid_t,
    pub pr_pid: compat_pid_t,
    pub pr_ppid: compat_pid_t,
    pub pr_pgrp: compat_pid_t,
    pub pr_sid: compat_pid_t,
    /*
     * The hard-coded 16 is derived from TASK_COMM_LEN, but it can't be
     * changed as it is exposed to userspace. We'd better make it hard-coded
     * here.
     */
    pub pr_fname: [i8; 16],
    pub pr_psargs: [i8; ELF_PRARGSZ],
}

// CONFIG_ARCH_HAS_ELFCORE_COMPAT: include asm/elfcore-compat.h when enabled.

#[repr(C)]
pub struct compat_elf_prstatus {
    pub common: compat_elf_prstatus_common,
    pub pr_reg: compat_elf_gregset_t,
    pub pr_fpvalid: compat_int_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
