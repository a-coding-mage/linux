// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Code for sysctl handling in NTFS Linux kernel driver.
 *
 * Copyright (C) 1997 Martin von Löwis, Régis Duchesne
 * Copyright (c) 2002-2005 Anton Altaparmakov
 */

// Translated from C under the DEBUG build condition.
// The CONFIG_SYSCTL condition is likewise preserved below.

#[cfg(all(feature = "DEBUG", feature = "CONFIG_SYSCTL"))]
mod sysctl_impl {
    use core::ffi::{c_char, c_int, c_void};

    // Declarations supplied by the kernel and other translation units.
    #[repr(C)]
    pub struct ctl_table {
        pub procname: *const c_char,
        pub data: *mut c_void,
        pub maxlen: c_int,
        pub mode: u16,
        pub proc_handler: Option<unsafe extern "C" fn()>,
    }

    #[repr(C)]
    pub struct ctl_table_header {
        _private: [u8; 0],
    }

    extern "C" {
        static mut debug_msgs: c_int;
        fn proc_dointvec();
        fn register_sysctl(
            path: *const c_char,
            table: *const ctl_table,
        ) -> *mut ctl_table_header;
        fn unregister_sysctl_table(table: *mut ctl_table_header);
    }

    /* Definition of the ntfs sysctl. */
    static mut ntfs_sysctls: [ctl_table; 1] = [ctl_table {
        procname: b"ntfs-debug\0".as_ptr() as *const c_char,
        data: core::ptr::addr_of_mut!(debug_msgs) as *mut c_void,
        maxlen: core::mem::size_of::<c_int>() as c_int,
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
    }];

    /* Storage for the sysctls header. */
    static mut sysctls_root_table: *mut ctl_table_header = core::ptr::null_mut();

    /*
     * ntfs_sysctl - add or remove the debug sysctl
     * @add: add (1) or remove (0) the sysctl
     *
     * Add or remove the debug sysctl. Return 0 on success or -errno on error.
     */
    pub unsafe extern "C" fn ntfs_sysctl(add: c_int) -> c_int {
        if add != 0 {
            sysctls_root_table = register_sysctl(
                b"fs/ntfs\0".as_ptr() as *const c_char,
                ntfs_sysctls.as_ptr(),
            );
            if sysctls_root_table.is_null() {
                return -12; // -ENOMEM
            }
        } else {
            unregister_sysctl_table(sysctls_root_table);
            sysctls_root_table = core::ptr::null_mut();
        }
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
