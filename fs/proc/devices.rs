// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to the Linux kernel includes and "internal.h"
// are supplied by other translation units.

use core::ffi::c_void;

// C types and symbols supplied by the included kernel headers.
// These declarations intentionally refer to those external definitions.
extern "C" {
    fn seq_puts(f: *mut seq_file, s: *const core::ffi::c_char);
    fn chrdev_show(f: *mut seq_file, i: i32);
    #[cfg(feature = "CONFIG_BLOCK")]
    fn blkdev_show(f: *mut seq_file, i: i32);
    fn proc_create_seq(
        name: *const core::ffi::c_char,
        mode: u16,
        parent: *mut proc_dir_entry,
        ops: *const seq_operations,
    ) -> *mut proc_dir_entry;
    fn pde_make_permanent(pde: *mut proc_dir_entry);
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut i64) -> *mut c_void>,
    pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut i64) -> *mut c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> i32>,
}

// CHRDEV_MAJOR_MAX and BLKDEV_MAJOR_MAX are supplied by the kernel headers.
extern "C" {
    static CHRDEV_MAJOR_MAX: i32;
    static BLKDEV_MAJOR_MAX: i32;
}

unsafe extern "C" fn devinfo_show(f: *mut seq_file, v: *mut c_void) -> i32 {
    let mut i = *(v as *mut i64) as i32;

    if i < CHRDEV_MAJOR_MAX {
        if i == 0 {
            seq_puts(f, c"Character devices:\n".as_ptr());
        }
        chrdev_show(f, i);
    }
    // CONFIG_BLOCK is a build-time condition from the C source.
    #[cfg(feature = "CONFIG_BLOCK")]
    {
        if i >= CHRDEV_MAJOR_MAX {
            i -= CHRDEV_MAJOR_MAX;
            if i == 0 {
                seq_puts(f, c"\nBlock devices:\n".as_ptr());
            }
            blkdev_show(f, i);
        }
    }
    0
}

unsafe extern "C" fn devinfo_start(_f: *mut seq_file, pos: *mut i64) -> *mut c_void {
    if *pos < (BLKDEV_MAJOR_MAX as i64 + CHRDEV_MAJOR_MAX as i64) {
        pos as *mut c_void
    } else {
        core::ptr::null_mut()
    }
}

unsafe extern "C" fn devinfo_next(
    _f: *mut seq_file,
    _v: *mut c_void,
    pos: *mut i64,
) -> *mut c_void {
    *pos += 1;
    if *pos >= (BLKDEV_MAJOR_MAX as i64 + CHRDEV_MAJOR_MAX as i64) {
        core::ptr::null_mut()
    } else {
        pos as *mut c_void
    }
}

unsafe extern "C" fn devinfo_stop(_f: *mut seq_file, _v: *mut c_void) {
    // Nothing to do
}

static DEVINFO_OPS: seq_operations = seq_operations {
    start: Some(devinfo_start),
    next: Some(devinfo_next),
    stop: Some(devinfo_stop),
    show: Some(devinfo_show),
};

unsafe extern "C" fn proc_devices_init() -> i32 {
    let pde: *mut proc_dir_entry = proc_create_seq(
        c"devices".as_ptr(),
        0,
        core::ptr::null_mut(),
        &DEVINFO_OPS,
    );
    pde_make_permanent(pde);
    0
}

// Corresponds to the C fs_initcall(proc_devices_init) registration macro.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
