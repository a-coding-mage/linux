/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/sys_info.h.  The declarations below depend on the
 * corresponding kernel types supplied by the surrounding translation. */

/*
 * SYS_INFO_PANIC_CONSOLE_REPLAY is for panic case only, as it needs special
 * handling which only fits panic case.
 */
pub const SYS_INFO_TASKS: kernel::ffi::c_ulong = 0x00000001;
pub const SYS_INFO_MEM: kernel::ffi::c_ulong = 0x00000002;
pub const SYS_INFO_TIMERS: kernel::ffi::c_ulong = 0x00000004;
pub const SYS_INFO_LOCKS: kernel::ffi::c_ulong = 0x00000008;
pub const SYS_INFO_FTRACE: kernel::ffi::c_ulong = 0x00000010;
pub const SYS_INFO_PANIC_CONSOLE_REPLAY: kernel::ffi::c_ulong = 0x00000020;
pub const SYS_INFO_ALL_BT: kernel::ffi::c_ulong = 0x00000040;
pub const SYS_INFO_BLOCKED_TASKS: kernel::ffi::c_ulong = 0x00000080;

extern "C" {
    pub fn sys_info(si_mask: kernel::ffi::c_ulong);
    pub fn sys_info_parse_param(str_: *mut kernel::ffi::c_char) -> kernel::ffi::c_ulong;

    /* Preserved from the CONFIG_SYSCTL conditional declaration. */
    #[cfg(CONFIG_SYSCTL)]
    pub fn sysctl_sys_info_handler(
        ro_table: *const crate::ctl_table,
        write: kernel::ffi::c_int,
        buffer: *mut kernel::ffi::c_void,
        lenp: *mut usize,
        ppos: *mut i64,
    ) -> kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
