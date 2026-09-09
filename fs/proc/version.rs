// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux kernel and by internal.h are referenced
// here as external C interfaces.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct new_utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

unsafe extern "C" {
    static linux_proc_banner: *const c_char;

    fn utsname() -> *const new_utsname;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn proc_create_single(
        name: *const c_char,
        mode: u16,
        parent: *mut proc_dir_entry,
        show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int,
    ) -> *mut proc_dir_entry;
    fn pde_make_permanent(pde: *mut proc_dir_entry);
}

unsafe extern "C" fn version_proc_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let uts = utsname();
    seq_printf(
        m,
        linux_proc_banner,
        (*uts).sysname.as_ptr(),
        (*uts).release.as_ptr(),
        (*uts).version.as_ptr(),
    );
    0
}

unsafe extern "C" fn proc_version_init() -> c_int {
    let pde: *mut proc_dir_entry;

    pde = proc_create_single(
        b"version\0".as_ptr() as *const c_char,
        0,
        core::ptr::null_mut(),
        version_proc_show,
    );
    pde_make_permanent(pde);
    0
}

// Equivalent registration intent for the C fs_initcall(proc_version_init)
// macro; the surrounding kernel build supplies the initcall mechanism.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
