/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: `__u32` and `__u64` are supplied by the Linux types bindings.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum pr_status {
    PR_STS_SUCCESS = 0x0,
    /*
     * The following error codes are based on SCSI, because the interface
     * was originally created for it and has existing users.
     */
    /* Generic device failure. */
    PR_STS_IOERR = 0x2,
    PR_STS_RESERVATION_CONFLICT = 0x18,
    /* Temporary path failure that can be retried. */
    PR_STS_RETRY_PATH_FAILURE = 0xe0000,
    /* The request was failed due to a fast failure timer. */
    PR_STS_PATH_FAST_FAILED = 0xf0000,
    /* The path cannot be reached and has been marked as failed. */
    PR_STS_PATH_FAILED = 0x10000,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum pr_type {
    PR_WRITE_EXCLUSIVE = 1,
    PR_EXCLUSIVE_ACCESS = 2,
    PR_WRITE_EXCLUSIVE_REG_ONLY = 3,
    PR_EXCLUSIVE_ACCESS_REG_ONLY = 4,
    PR_WRITE_EXCLUSIVE_ALL_REGS = 5,
    PR_EXCLUSIVE_ACCESS_ALL_REGS = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_reservation {
    pub key: __u64,
    pub type_: __u32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_registration {
    pub old_key: __u64,
    pub new_key: __u64,
    pub flags: __u32,
    pub __pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_preempt {
    pub old_key: __u64,
    pub new_key: __u64,
    pub type_: __u32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_clear {
    pub key: __u64,
    pub flags: __u32,
    pub __pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_read_keys {
    pub generation: __u32,
    pub num_keys: __u32,
    pub keys_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_read_reservation {
    pub key: __u64,
    pub generation: __u32,
    pub type_: __u32,
}

pub const PR_FL_IGNORE_KEY: u32 = 1 << 0; /* ignore existing key */

// These ioctl values depend on the platform-specific Linux `_IO*` definitions.
pub const IOC_PR_REGISTER: usize = _IOW!('p', 200, pr_registration);
pub const IOC_PR_RESERVE: usize = _IOW!('p', 201, pr_reservation);
pub const IOC_PR_RELEASE: usize = _IOW!('p', 202, pr_reservation);
pub const IOC_PR_PREEMPT: usize = _IOW!('p', 203, pr_preempt);
pub const IOC_PR_PREEMPT_ABORT: usize = _IOW!('p', 204, pr_preempt);
pub const IOC_PR_CLEAR: usize = _IOW!('p', 205, pr_clear);
pub const IOC_PR_READ_KEYS: usize = _IOWR!('p', 206, pr_read_keys);
pub const IOC_PR_READ_RESERVATION: usize = _IOR!('p', 207, pr_read_reservation);

pub const PR_KEYS_MAX: __u32 = 1u32 << 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
