/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Comparison type */
#[repr(i32)]
pub enum kcmp_type {
    KCMP_FILE,
    KCMP_VM,
    KCMP_FILES,
    KCMP_FS,
    KCMP_SIGHAND,
    KCMP_IO,
    KCMP_SYSVSEM,
    KCMP_EPOLL_TFD,
    KCMP_TYPES,
}

/* Slot for KCMP_EPOLL_TFD */
#[repr(C)]
pub struct kcmp_epoll_slot {
    pub efd: u32,  /* epoll file descriptor */
    pub tfd: u32,  /* target file number */
    pub toff: u32, /* target offset within same numbered sequence */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
