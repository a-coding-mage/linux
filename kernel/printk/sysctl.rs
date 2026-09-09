// SPDX-License-Identifier: GPL-2.0-only
/*
 * sysctl.c: General linux system control interface
 */

// Dependencies supplied by the surrounding kernel translation unit.

static ten_thousand: i32 = 10000;

unsafe fn proc_dointvec_minmax_sysadmin(
    table: *const ctl_table,
    write: i32,
    buffer: *mut core::ffi::c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> i32 {
    if write != 0 && capable(CAP_SYS_ADMIN) == 0 {
        return -EPERM;
    }

    proc_dointvec_minmax(table, write, buffer, lenp, ppos)
}

static printk_sysctls: [ctl_table; 7] = [
    ctl_table {
        procname: "printk\0".as_ptr() as *const i8,
        data: unsafe { &console_loglevel as *const _ as *mut core::ffi::c_void },
        maxlen: 4 * core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: core::ptr::null_mut(),
        extra2: core::ptr::null_mut(),
    },
    ctl_table {
        procname: "printk_ratelimit\0".as_ptr() as *const i8,
        data: unsafe { &printk_ratelimit_state.interval as *const _ as *mut core::ffi::c_void },
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_jiffies),
        extra1: core::ptr::null_mut(),
        extra2: core::ptr::null_mut(),
    },
    ctl_table {
        procname: "printk_ratelimit_burst\0".as_ptr() as *const i8,
        data: unsafe { &printk_ratelimit_state.burst as *const _ as *mut core::ffi::c_void },
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: core::ptr::null_mut(),
        extra2: core::ptr::null_mut(),
    },
    ctl_table {
        procname: "printk_delay\0".as_ptr() as *const i8,
        data: unsafe { &printk_delay_msec as *const _ as *mut core::ffi::c_void },
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: SYSCTL_ZERO,
        extra2: &ten_thousand as *const _ as *mut core::ffi::c_void,
    },
    ctl_table {
        procname: "printk_devkmsg\0".as_ptr() as *const i8,
        data: devkmsg_log_str as *mut core::ffi::c_void,
        maxlen: DEVKMSG_STR_MAX_SIZE,
        mode: 0o644,
        proc_handler: Some(devkmsg_sysctl_set_loglvl),
        extra1: core::ptr::null_mut(),
        extra2: core::ptr::null_mut(),
    },
    ctl_table {
        procname: "dmesg_restrict\0".as_ptr() as *const i8,
        data: unsafe { &dmesg_restrict as *const _ as *mut core::ffi::c_void },
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax_sysadmin),
        extra1: SYSCTL_ZERO,
        extra2: SYSCTL_ONE,
    },
    ctl_table {
        procname: "kptr_restrict\0".as_ptr() as *const i8,
        data: unsafe { &kptr_restrict as *const _ as *mut core::ffi::c_void },
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax_sysadmin),
        extra1: SYSCTL_ZERO,
        extra2: SYSCTL_TWO,
    },
];

unsafe fn printk_sysctl_init() {
    register_sysctl_init("kernel\0".as_ptr() as *const i8, printk_sysctls.as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
