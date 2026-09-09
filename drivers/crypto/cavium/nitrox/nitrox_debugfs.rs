// SPDX-License-Identifier: GPL-2.0
//
// C includes and declarations supplied by the Linux kernel and the Nitrox
// headers are intentionally represented as external Rust dependencies.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

#[repr(C)]
pub struct seq_file {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic64_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nitrox_hw {
    pub fw_name: [*const c_char; 2],
    pub partname: *const c_char,
    pub freq: c_int,
    pub device_id: c_uint,
    pub revision_id: c_uint,
    pub ae_cores: c_uint,
    pub se_cores: c_uint,
    pub zip_cores: c_uint,
}

#[repr(C)]
pub struct nitrox_stats {
    pub posted: atomic64_t,
    pub completed: atomic64_t,
    pub dropped: atomic64_t,
}

#[repr(C)]
pub struct nitrox_device {
    pub hw: nitrox_hw,
    pub idx: c_int,
    pub stats: nitrox_stats,
    pub debugfs_dir: *mut dentry,
}

extern "C" {
    fn seq_printf(s: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn atomic64_read(v: *const atomic64_t) -> c_ulonglong;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(
        name: *const c_char,
        mode: c_ulong,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);

    // DEFINE_SHOW_ATTRIBUTE(firmware), DEFINE_SHOW_ATTRIBUTE(device), and
    // DEFINE_SHOW_ATTRIBUTE(stats) provide these file-operation objects.
    static firmware_fops: file_operations;
    static device_fops: file_operations;
    static stats_fops: file_operations;
}

unsafe extern "C" fn firmware_show(s: *mut seq_file, _v: *mut c_void) -> c_int {
    let ndev = (*s).private as *mut nitrox_device;

    seq_printf(s, b"Version: %s\n\0".as_ptr() as *const c_char, (*ndev).hw.fw_name[0]);
    seq_printf(s, b"Version: %s\n\0".as_ptr() as *const c_char, (*ndev).hw.fw_name[1]);
    0
}

unsafe extern "C" fn device_show(s: *mut seq_file, _v: *mut c_void) -> c_int {
    let ndev = (*s).private as *mut nitrox_device;

    seq_printf(s, b"NITROX [%d]\n\0".as_ptr() as *const c_char, (*ndev).idx);
    seq_printf(s, b"  Part Name: %s\n\0".as_ptr() as *const c_char, (*ndev).hw.partname);
    seq_printf(s, b"  Frequency: %d MHz\n\0".as_ptr() as *const c_char, (*ndev).hw.freq);
    seq_printf(s, b"  Device ID: 0x%0x\n\0".as_ptr() as *const c_char, (*ndev).hw.device_id);
    seq_printf(s, b"  Revision ID: 0x%0x\n\0".as_ptr() as *const c_char, (*ndev).hw.revision_id);
    seq_printf(
        s,
        b"  Cores: [AE=%u  SE=%u  ZIP=%u]\n\0".as_ptr() as *const c_char,
        (*ndev).hw.ae_cores,
        (*ndev).hw.se_cores,
        (*ndev).hw.zip_cores,
    );
    0
}

unsafe extern "C" fn stats_show(s: *mut seq_file, _v: *mut c_void) -> c_int {
    let ndev = (*s).private as *mut nitrox_device;

    seq_printf(s, b"NITROX [%d] Request Statistics\n\0".as_ptr() as *const c_char, (*ndev).idx);
    seq_printf(s, b"  Posted: %llu\n\0".as_ptr() as *const c_char, atomic64_read(&(*ndev).stats.posted));
    seq_printf(s, b"  Completed: %llu\n\0".as_ptr() as *const c_char, atomic64_read(&(*ndev).stats.completed));
    seq_printf(s, b"  Dropped: %llu\n\0".as_ptr() as *const c_char, atomic64_read(&(*ndev).stats.dropped));
    0
}

#[no_mangle]
pub unsafe extern "C" fn nitrox_debugfs_exit(ndev: *mut nitrox_device) {
    debugfs_remove_recursive((*ndev).debugfs_dir);
    (*ndev).debugfs_dir = core::ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn nitrox_debugfs_init(ndev: *mut nitrox_device) {
    let dir = debugfs_create_dir(b"KBUILD_MODNAME\0".as_ptr() as *const c_char, core::ptr::null_mut());

    (*ndev).debugfs_dir = dir;
    debugfs_create_file(b"firmware\0".as_ptr() as *const c_char, 0o400, dir, ndev as *mut c_void, &firmware_fops);
    debugfs_create_file(b"device\0".as_ptr() as *const c_char, 0o400, dir, ndev as *mut c_void, &device_fops);
    debugfs_create_file(b"stats\0".as_ptr() as *const c_char, 0o400, dir, ndev as *mut c_void, &stats_fops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
