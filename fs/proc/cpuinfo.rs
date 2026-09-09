// SPDX-License-Identifier: GPL-2.0
//
// C dependencies:
// linux/cpufreq.h, linux/fs.h, linux/init.h, linux/proc_fs.h,
// linux/seq_file.h

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_operations {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}

pub type ProcOpen = unsafe extern "C" fn(*mut inode, *mut file) -> c_int;
pub type ProcReadIter = unsafe extern "C" fn(*mut file, *mut iov_iter) -> isize;
pub type ProcLseek = unsafe extern "C" fn(*mut file, i64, c_int) -> i64;
pub type ProcRelease = unsafe extern "C" fn(*mut inode, *mut file) -> c_int;

#[repr(C)]
pub struct proc_ops {
    pub proc_flags: c_uint,
    pub proc_open: Option<ProcOpen>,
    pub proc_read_iter: Option<ProcReadIter>,
    pub proc_lseek: Option<ProcLseek>,
    pub proc_release: Option<ProcRelease>,
}

// External declarations supplied by the Linux kernel and other translation units.
extern "C" {
    pub static cpuinfo_op: seq_operations;

    fn seq_open(file: *mut file, ops: *const seq_operations) -> c_int;
    fn seq_read_iter(file: *mut file, iter: *mut iov_iter) -> isize;
    fn seq_lseek(file: *mut file, offset: i64, whence: c_int) -> i64;
    fn seq_release(inode: *mut inode, file: *mut file) -> c_int;
    fn proc_create(
        name: *const c_char,
        mode: c_uint,
        parent: *mut c_void,
        proc_ops: *const proc_ops,
    ) -> *mut c_void;
}

// PROC_ENTRY_PERMANENT from linux/proc_fs.h.
const PROC_ENTRY_PERMANENT: c_uint = 1 << 0;

unsafe extern "C" fn cpuinfo_open(inode: *mut inode, file: *mut file) -> c_int {
    seq_open(file, &cpuinfo_op)
}

static CPUINFO_PROC_OPS: proc_ops = proc_ops {
    proc_flags: PROC_ENTRY_PERMANENT,
    proc_open: Some(cpuinfo_open),
    proc_read_iter: Some(seq_read_iter),
    proc_lseek: Some(seq_lseek),
    proc_release: Some(seq_release),
};

unsafe extern "C" fn proc_cpuinfo_init() -> c_int {
    proc_create(
        b"cpuinfo\0".as_ptr() as *const c_char,
        0,
        core::ptr::null_mut(),
        &CPUINFO_PROC_OPS,
    );
    0
}

// fs_initcall(proc_cpuinfo_init);
// Build-system registration equivalent for the Linux kernel initcall macro.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
