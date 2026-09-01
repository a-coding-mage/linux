/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Depends on <linux/types.h> for __u32. */

/* Comparison type */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kcmp_type {
    KCMP_FILE = 0,
    KCMP_VM = 1,
    KCMP_FILES = 2,
    KCMP_FS = 3,
    KCMP_SIGHAND = 4,
    KCMP_IO = 5,
    KCMP_SYSVSEM = 6,
    KCMP_EPOLL_TFD = 7,

    KCMP_TYPES = 8,
}

/* Slot for KCMP_EPOLL_TFD */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcmp_epoll_slot {
    pub efd: __u32,  /* epoll file descriptor */
    pub tfd: __u32,  /* target file number */
    pub toff: __u32, /* target offset within same numbered sequence */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
