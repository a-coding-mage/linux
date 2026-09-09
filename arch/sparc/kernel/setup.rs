// SPDX-License-Identifier: GPL-2.0

// C dependencies: <asm/setup.h>, <linux/sysctl.h>

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut reboot_command: *mut c_char;
    static mut stop_a_enabled: c_int;
    static mut scons_pwroff: c_int;

    fn proc_dostring(
        table: *mut ctl_table,
        write: c_int,
        buffer: *mut c_void,
        length: *mut usize,
        ppos: *mut u64,
    ) -> c_int;
    fn proc_dointvec(
        table: *mut ctl_table,
        write: c_int,
        buffer: *mut c_void,
        length: *mut usize,
        ppos: *mut u64,
    ) -> c_int;
    fn register_sysctl_init(
        path: *const c_char,
        table: *const ctl_table,
    ) -> *mut c_void;
}

#[cfg(CONFIG_SPARC64)]
extern "C" {
    static mut sysctl_tsb_ratio: c_int;
}

#[repr(C)]
struct ctl_table {
    procname: *const c_char,
    data: *mut c_void,
    maxlen: usize,
    mode: u16,
    proc_handler: Option<unsafe extern "C" fn(
        table: *mut ctl_table,
        write: c_int,
        buffer: *mut c_void,
        length: *mut usize,
        ppos: *mut u64,
    ) -> c_int>,
}

static mut SPARC_SYSCTL_TABLE: &[ctl_table] = &[
    ctl_table {
        procname: b"reboot-cmd\0".as_ptr() as *const c_char,
        data: unsafe { reboot_command as *mut c_void },
        maxlen: 256,
        mode: 0o644,
        proc_handler: Some(proc_dostring),
    },
    ctl_table {
        procname: b"stop-a\0".as_ptr() as *const c_char,
        data: unsafe { core::ptr::addr_of_mut!(stop_a_enabled) as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
    },
    ctl_table {
        procname: b"scons-poweroff\0".as_ptr() as *const c_char,
        data: unsafe { core::ptr::addr_of_mut!(scons_pwroff) as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
    },
    #[cfg(CONFIG_SPARC64)]
    ctl_table {
        procname: b"tsb-ratio\0".as_ptr() as *const c_char,
        data: unsafe { core::ptr::addr_of_mut!(sysctl_tsb_ratio) as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
    },
];

#[inline]
unsafe fn init_sparc_sysctls() -> c_int {
    register_sysctl_init(
        b"kernel\0".as_ptr() as *const c_char,
        SPARC_SYSCTL_TABLE.as_ptr(),
    );
    0
}

// arch_initcall(init_sparc_sysctls);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
