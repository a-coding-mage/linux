/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard `_LINUX_PID_TYPES_H` has no executable Rust equivalent.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pid_type {
    PIDTYPE_PID,
    PIDTYPE_TGID,
    PIDTYPE_PGID,
    PIDTYPE_SID,
    PIDTYPE_MAX,
}

#[repr(C)]
pub struct pid_namespace {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut init_pid_ns: pid_namespace;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
