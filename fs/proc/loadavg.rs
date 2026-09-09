// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to the C includes are supplied by other files.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pid_namespace {
    pub idr: idr,
}

#[repr(C)]
pub struct idr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    fn get_avenrun(avnrun: *mut c_ulong, offset: c_ulong, shift: c_ulong);
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn nr_running() -> c_uint;
    static mut nr_threads: c_int;
    fn current() -> *mut task_struct;
    fn task_active_pid_ns(task: *mut task_struct) -> *mut pid_namespace;
    fn idr_get_cursor(idr: *const idr) -> c_int;
    fn proc_create_single(
        name: *const c_char,
        mode: c_uint,
        parent: *mut proc_dir_entry,
        show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int,
    ) -> *mut proc_dir_entry;
    fn pde_make_permanent(pde: *mut proc_dir_entry);
}

const FSHIFT: c_ulong = 11;
const FIXED_1: c_ulong = 1 << FSHIFT;

macro_rules! LOAD_INT {
    ($x:expr) => {
        ($x >> FSHIFT)
    };
}

macro_rules! LOAD_FRAC {
    ($x:expr) => {
        LOAD_INT!((($x & (FIXED_1 - 1)) * 100))
    };
}

unsafe extern "C" fn loadavg_proc_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let mut avnrun: [c_ulong; 3] = [0; 3];

    get_avenrun(avnrun.as_mut_ptr(), FIXED_1 / 200, 0);

    let fmt = b"%lu.%02lu %lu.%02lu %lu.%02lu %u/%d %d\n\0";
    seq_printf(
        m,
        fmt.as_ptr() as *const c_char,
        LOAD_INT!(avnrun[0]),
        LOAD_FRAC!(avnrun[0]),
        LOAD_INT!(avnrun[1]),
        LOAD_FRAC!(avnrun[1]),
        LOAD_INT!(avnrun[2]),
        LOAD_FRAC!(avnrun[2]),
        nr_running(),
        nr_threads,
        idr_get_cursor(&(*task_active_pid_ns(current())).idr) - 1,
    );
    0
}

unsafe extern "C" fn proc_loadavg_init() -> c_int {
    let pde: *mut proc_dir_entry;

    pde = proc_create_single(b"loadavg\0".as_ptr() as *const c_char, 0, core::ptr::null_mut(), loadavg_proc_show);
    pde_make_permanent(pde);
    0
}

// fs_initcall(proc_loadavg_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
