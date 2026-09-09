/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of trace/events/devlink.h.  The C tracepoint machinery and
 * the types supplied by <linux/device.h> and <net/devlink.h> are external. */

#[cfg(feature = "CONFIG_NET_DEVLINK")]
mod net_devlink_tracepoints {
    use core::ffi::{c_char, c_void};

    #[repr(C)]
    pub struct devlink {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct sk_buff {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct devlink_trap_metadata {
        _private: [u8; 0],
    }

    /* Tracepoint for devlink hardware message. */
    extern "C" {
        pub fn trace_devlink_hwmsg(
            devlink: *const devlink,
            incoming: bool,
            type_: usize,
            buf: *const u8,
            len: usize,
        );

        /* Tracepoint for devlink hardware error. */
        pub fn trace_devlink_hwerr(
            devlink: *const devlink,
            err: i32,
            msg: *const c_char,
        );

        /* Tracepoint for devlink health message. */
        pub fn trace_devlink_health_report(
            devlink: *const devlink,
            reporter_name: *const c_char,
            msg: *const c_char,
        );

        /* Tracepoint for devlink health recover aborted message. */
        pub fn trace_devlink_health_recover_aborted(
            devlink: *const devlink,
            reporter_name: *const c_char,
            health_state: bool,
            time_since_last_recover: u64,
        );

        /* Tracepoint for devlink health reporter state update. */
        pub fn trace_devlink_health_reporter_state_update(
            devlink: *const devlink,
            reporter_name: *const c_char,
            new_state: bool,
        );

        /* Tracepoint for devlink packet trap. */
        pub fn trace_devlink_trap_report(
            devlink: *const devlink,
            skb: *mut sk_buff,
            metadata: *const devlink_trap_metadata,
        );
    }

    /* The following entry layouts mirror TP_STRUCT__entry declarations. */
    #[repr(C)]
    pub struct devlink_hwmsg_entry {
        pub incoming: bool,
        pub type_: usize,
        pub buf: *mut u8,
        pub len: usize,
    }

    #[repr(C)]
    pub struct devlink_hwerr_entry {
        pub err: i32,
        pub msg: *mut c_char,
    }

    #[repr(C)]
    pub struct devlink_health_report_entry {
        pub reporter_name: *mut c_char,
        pub msg: *mut c_char,
    }

    #[repr(C)]
    pub struct devlink_health_recover_aborted_entry {
        pub health_state: bool,
        pub time_since_last_recover: u64,
        pub reporter_name: *mut c_char,
    }

    #[repr(C)]
    pub struct devlink_health_reporter_state_update_entry {
        pub reporter_name: *mut c_char,
        pub new_state: u8,
    }

    pub const DEVLINK_TRAP_INPUT_DEV_NAME_LEN: usize = 16;

    #[repr(C)]
    pub struct devlink_trap_report_entry {
        pub input_dev_name: [c_char; DEVLINK_TRAP_INPUT_DEV_NAME_LEN],
        pub trap_name: *mut c_char,
        pub trap_group_name: *mut c_char,
    }

    /* `c_void` is retained here to preserve the source's dependency on the
     * tracepoint entry's dynamically allocated string fields. */
    #[allow(dead_code)]
    type TraceEntryStorage = c_void;
}

/* CONFIG_NET_DEVLINK disabled: the C header supplies empty inline stubs. */
#[cfg(not(feature = "CONFIG_NET_DEVLINK"))]
pub mod no_net_devlink_tracepoints {
    use core::ffi::c_char;

    #[repr(C)]
    pub struct devlink {
        _private: [u8; 0],
    }

    #[inline]
    pub unsafe fn trace_devlink_hwmsg(
        _devlink: *const devlink,
        _incoming: bool,
        _type_: usize,
        _buf: *const u8,
        _len: usize,
    ) {
    }

    #[inline]
    pub unsafe fn trace_devlink_hwerr(
        _devlink: *const devlink,
        _err: i32,
        _msg: *const c_char,
    ) {
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
