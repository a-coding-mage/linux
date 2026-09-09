// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the Linux kernel headers:
// linux/debugfs.h, linux/seq_file.h, and asm/ptdump.h.

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ptdump_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

extern "C" {
    static ptdump_fops: file_operations;

    fn ptdump_walk_pgd(m: *mut seq_file, info: *mut ptdump_info);

    fn debugfs_create_file(
        name: *const core::ffi::c_char,
        mode: u16,
        parent: *mut core::ffi::c_void,
        data: *mut core::ffi::c_void,
        fops: *const file_operations,
    ) -> *mut core::ffi::c_void;
}

unsafe fn ptdump_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    // struct ptdump_info *info = m->private;
    let info = *(m as *mut *mut ptdump_info);

    ptdump_walk_pgd(m, info);
    0
}

// DEFINE_SHOW_ATTRIBUTE(ptdump);
// The kernel macro provides the corresponding file_operations instance.

#[no_mangle]
pub unsafe extern "C" fn ptdump_debugfs_register(
    info: *mut ptdump_info,
    name: *const core::ffi::c_char,
) {
    debugfs_create_file(
        name,
        0o400,
        core::ptr::null_mut(),
        info as *mut core::ffi::c_void,
        &ptdump_fops,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
