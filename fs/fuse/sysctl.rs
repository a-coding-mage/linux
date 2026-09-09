// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/fuse/fuse_sysctl.c
 *
 * Sysctl interface to fuse parameters
 */

// C dependencies supplied by other translation units/headers:
// linux/sysctl.h, sysctl.h, and fuse_i.h.

extern "C" {
    fn register_sysctl(
        path: *const core::ffi::c_char,
        table: *const ctl_table,
    ) -> *mut ctl_table_header;
    fn unregister_sysctl_table(header: *mut ctl_table_header);
    fn proc_douintvec_minmax(
        table: *mut ctl_table,
        write: i32,
        buffer: *mut core::ffi::c_void,
        length: *mut usize,
        ppos: *mut i64,
    ) -> i32;
}

extern "C" {
    static mut fuse_max_pages_limit: ::core::ffi::c_uint;
    static mut fuse_default_req_timeout: ::core::ffi::c_uint;
    static mut fuse_max_req_timeout: ::core::ffi::c_uint;
}

#[allow(non_camel_case_types)]
type ctl_table_header = core::ffi::c_void;
#[allow(non_camel_case_types)]
#[repr(C)]
struct ctl_table {
    procname: *const core::ffi::c_char,
    data: *mut core::ffi::c_void,
    maxlen: usize,
    mode: u16,
    proc_handler: Option<unsafe extern "C" fn(
        *mut ctl_table,
        i32,
        *mut core::ffi::c_void,
        *mut usize,
        *mut i64,
    ) -> i32>,
    extra1: *mut core::ffi::c_void,
    extra2: *mut core::ffi::c_void,
}

extern "C" {
    static mut SYSCTL_ZERO: core::ffi::c_void;
    static mut SYSCTL_ONE: core::ffi::c_void;
}

static mut fuse_table_header: *mut ctl_table_header = core::ptr::null_mut();

/* Bound by fuse_init_out max_pages, which is a u16 */
static mut sysctl_fuse_max_pages_limit: ::core::ffi::c_uint = 65535;

/*
 * fuse_init_out request timeouts are u16.
 * This goes up to ~18 hours, which is plenty for a timeout.
 */
static mut sysctl_fuse_req_timeout_limit: ::core::ffi::c_uint = 65535;

static fuse_sysctl_table: [ctl_table; 3] = [
    ctl_table {
        procname: b"max_pages_limit\0".as_ptr() as *const core::ffi::c_char,
        data: unsafe { &mut fuse_max_pages_limit as *mut _ as *mut core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_uint>(),
        mode: 0o644,
        proc_handler: Some(proc_douintvec_minmax),
        extra1: unsafe { &mut SYSCTL_ONE as *mut _ },
        extra2: unsafe { &mut sysctl_fuse_max_pages_limit as *mut _ as *mut core::ffi::c_void },
    },
    ctl_table {
        procname: b"default_request_timeout\0".as_ptr() as *const core::ffi::c_char,
        data: unsafe { &mut fuse_default_req_timeout as *mut _ as *mut core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_uint>(),
        mode: 0o644,
        proc_handler: Some(proc_douintvec_minmax),
        extra1: unsafe { &mut SYSCTL_ZERO as *mut _ },
        extra2: unsafe { &mut sysctl_fuse_req_timeout_limit as *mut _ as *mut core::ffi::c_void },
    },
    ctl_table {
        procname: b"max_request_timeout\0".as_ptr() as *const core::ffi::c_char,
        data: unsafe { &mut fuse_max_req_timeout as *mut _ as *mut core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_uint>(),
        mode: 0o644,
        proc_handler: Some(proc_douintvec_minmax),
        extra1: unsafe { &mut SYSCTL_ZERO as *mut _ },
        extra2: unsafe { &mut sysctl_fuse_req_timeout_limit as *mut _ as *mut core::ffi::c_void },
    },
];

pub unsafe extern "C" fn fuse_sysctl_register() -> i32 {
    fuse_table_header = register_sysctl(
        b"fs/fuse\0".as_ptr() as *const core::ffi::c_char,
        fuse_sysctl_table.as_ptr(),
    );
    if fuse_table_header.is_null() {
        return -12; // -ENOMEM
    }
    0
}

pub unsafe extern "C" fn fuse_sysctl_unregister() {
    unregister_sysctl_table(fuse_table_header);
    fuse_table_header = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
