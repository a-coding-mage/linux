// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 1997-1998 Transmeta Corporation -- All Rights Reserved
 */

// Dependencies supplied by the Linux kernel and autofs_i.h are intentionally
// left as external Rust items.

extern "C" {
    static THIS_MODULE: *mut core::ffi::c_void;
    static autofs_param_specs: *const core::ffi::c_void;

    fn autofs_init_fs_context();
    fn autofs_kill_sb();
    fn autofs_dev_ioctl_init();
    fn autofs_dev_ioctl_exit();
    fn register_filesystem(fs: *mut file_system_type) -> core::ffi::c_int;
    fn unregister_filesystem(fs: *mut file_system_type);
}

#[repr(C)]
pub struct file_system_type {
    pub owner: *mut core::ffi::c_void,
    pub name: *const core::ffi::c_char,
    pub init_fs_context: Option<unsafe extern "C" fn()>,
    pub parameters: *const core::ffi::c_void,
    pub kill_sb: Option<unsafe extern "C" fn()>,
}

#[no_mangle]
pub static mut autofs_fs_type: file_system_type = file_system_type {
    owner: unsafe { THIS_MODULE },
    name: b"autofs\0".as_ptr() as *const core::ffi::c_char,
    init_fs_context: Some(autofs_init_fs_context),
    parameters: unsafe { autofs_param_specs },
    kill_sb: Some(autofs_kill_sb),
};

// MODULE_ALIAS_FS("autofs");
// MODULE_ALIAS("autofs");

#[allow(non_snake_case)]
unsafe fn init_autofs_fs() -> core::ffi::c_int {
    let mut err: core::ffi::c_int;

    autofs_dev_ioctl_init();

    err = register_filesystem(&mut autofs_fs_type);
    if err != 0 {
        autofs_dev_ioctl_exit();
    }

    err
}

#[allow(non_snake_case)]
unsafe fn exit_autofs_fs() {
    autofs_dev_ioctl_exit();
    unregister_filesystem(&mut autofs_fs_type);
}

// module_init(init_autofs_fs)
// module_exit(exit_autofs_fs)
// MODULE_DESCRIPTION("Kernel automounter support");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
