// SPDX-License-Identifier: GPL-2.0-only
// Kernel module and notifier-error-inject.h declarations are supplied externally.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
    pub next: *mut notifier_block,
    pub priority: c_int,
}

#[repr(C)]
pub struct notifier_err_inject_action {
    pub name: *const c_char,
    pub val: c_ulong,
    pub error: c_int,
}

#[repr(C)]
pub struct notifier_err_inject {
    pub nb: notifier_block,
    pub actions: *mut notifier_err_inject_action,
}

extern "C" {
    fn debugfs_create_file(
        name: *const c_char,
        mode: u16,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const c_void,
    ) -> *mut dentry;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn pr_info(fmt: *const c_char, ...);
}

const MAX_ERRNO: i32 = 4095;
const S_IFREG: u16 = 0o100000;
const S_IRUSR: u16 = 0o400;
const S_IWUSR: u16 = 0o200;

static mut notifier_err_inject_dir: *mut dentry = core::ptr::null_mut();

unsafe extern "C" fn debugfs_errno_set(data: *mut c_void, val: u64) -> c_int {
    *(data as *mut c_int) = (val as i32).clamp(-MAX_ERRNO, 0);
    0
}

unsafe extern "C" fn debugfs_errno_get(data: *mut c_void, val: *mut u64) -> c_int {
    *val = *(data as *mut c_int) as u64;
    0
}

// DEFINE_SIMPLE_ATTRIBUTE_SIGNED(fops_errno, debugfs_errno_get, debugfs_errno_set, "%lld\n");
static fops_errno: [u8; 0] = [];

unsafe fn debugfs_create_errno(
    name: *const c_char,
    mode: u16,
    parent: *mut dentry,
    value: *mut c_int,
) -> *mut dentry {
    debugfs_create_file(name, mode, parent, value as *mut c_void, &fops_errno as *const _ as *const c_void)
}

unsafe extern "C" fn notifier_err_inject_callback(
    nb: *mut notifier_block,
    val: c_ulong,
    p: *mut c_void,
) -> c_int {
    let mut err: c_int = 0;
    let err_inject = (nb as *mut u8).sub(core::mem::offset_of!(notifier_err_inject, nb))
        as *mut notifier_err_inject;
    let mut action = (*err_inject).actions;

    while !(*action).name.is_null() {
        if (*action).val == val {
            err = (*action).error;
            break;
        }
        action = action.add(1);
    }
    if err != 0 {
        pr_info(b"Injecting error (%d) to %s\n\0".as_ptr() as *const c_char, err, (*action).name);
    }

    notifier_from_errno(err)
}

unsafe extern "C" {
    fn notifier_from_errno(err: c_int) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn notifier_err_inject_init(
    name: *const c_char,
    parent: *mut dentry,
    err_inject: *mut notifier_err_inject,
    priority: c_int,
) -> *mut dentry {
    let mut action: *mut notifier_err_inject_action;
    let mode: u16 = S_IFREG | S_IRUSR | S_IWUSR;
    let dir: *mut dentry;
    let actions_dir: *mut dentry;

    (*err_inject).nb.notifier_call = Some(notifier_err_inject_callback);
    (*err_inject).nb.priority = priority;

    dir = debugfs_create_dir(name, parent);
    actions_dir = debugfs_create_dir(b"actions\0".as_ptr() as *const c_char, dir);

    action = (*err_inject).actions;
    while !(*action).name.is_null() {
        let action_dir = debugfs_create_dir((*action).name, actions_dir);
        debugfs_create_errno(
            b"error\0".as_ptr() as *const c_char,
            mode,
            action_dir,
            &mut (*action).error,
        );
        action = action.add(1);
    }
    dir
}

unsafe extern "C" fn err_inject_init() -> c_int {
    notifier_err_inject_dir = debugfs_create_dir(
        b"notifier-error-inject\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
    );
    0
}

unsafe extern "C" fn err_inject_exit() {
    debugfs_remove_recursive(notifier_err_inject_dir);
}

// module_init(err_inject_init);
// module_exit(err_inject_exit);
// MODULE_DESCRIPTION("Notifier error injection module");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Akinobu Mita <akinobu.mita@gmail.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
