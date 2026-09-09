/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Kernel support for NT synchronization primitive emulation
 *
 * Copyright (C) 2021-2022 Elizabeth Figura <zfigura@codeweavers.com>
 */

// Dependency intent: the integer types and ioctl encoding macros are supplied
// by the corresponding Linux bindings.

#[repr(C)]
pub struct ntsync_sem_args {
    pub count: __u32,
    pub max: __u32,
}

#[repr(C)]
pub struct ntsync_mutex_args {
    pub owner: __u32,
    pub count: __u32,
}

#[repr(C)]
pub struct ntsync_event_args {
    pub manual: __u32,
    pub signaled: __u32,
}

pub const NTSYNC_WAIT_REALTIME: __u32 = 0x1;

#[repr(C)]
pub struct ntsync_wait_args {
    pub timeout: __u64,
    pub objs: __u64,
    pub count: __u32,
    pub index: __u32,
    pub flags: __u32,
    pub owner: __u32,
    pub alert: __u32,
    pub pad: __u32,
}

pub const NTSYNC_MAX_WAIT_COUNT: usize = 64;

pub const NTSYNC_IOC_CREATE_SEM: usize = _IOW!('N', 0x80, ntsync_sem_args);
pub const NTSYNC_IOC_WAIT_ANY: usize = _IOWR!('N', 0x82, ntsync_wait_args);
pub const NTSYNC_IOC_WAIT_ALL: usize = _IOWR!('N', 0x83, ntsync_wait_args);
pub const NTSYNC_IOC_CREATE_MUTEX: usize = _IOW!('N', 0x84, ntsync_mutex_args);
pub const NTSYNC_IOC_CREATE_EVENT: usize = _IOW!('N', 0x87, ntsync_event_args);

pub const NTSYNC_IOC_SEM_RELEASE: usize = _IOWR!('N', 0x81, __u32);
pub const NTSYNC_IOC_MUTEX_UNLOCK: usize = _IOWR!('N', 0x85, ntsync_mutex_args);
pub const NTSYNC_IOC_MUTEX_KILL: usize = _IOW!('N', 0x86, __u32);
pub const NTSYNC_IOC_EVENT_SET: usize = _IOR!('N', 0x88, __u32);
pub const NTSYNC_IOC_EVENT_RESET: usize = _IOR!('N', 0x89, __u32);
pub const NTSYNC_IOC_EVENT_PULSE: usize = _IOR!('N', 0x8a, __u32);
pub const NTSYNC_IOC_SEM_READ: usize = _IOR!('N', 0x8b, ntsync_sem_args);
pub const NTSYNC_IOC_MUTEX_READ: usize = _IOR!('N', 0x8c, ntsync_mutex_args);
pub const NTSYNC_IOC_EVENT_READ: usize = _IOR!('N', 0x8d, ntsync_event_args);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
