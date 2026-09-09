// SPDX-License-Identifier: GPL-2.0-only

// Kernel, module, netdevice, and notifier-error-inject declarations are
// supplied by the surrounding kernel bindings/build environment.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_err_inject_action {
    pub action: c_int,
}

#[repr(C)]
pub struct notifier_err_inject {
    pub actions: [notifier_err_inject_action; 10],
    pub nb: notifier_block,
}

extern "C" {
    static mut notifier_err_inject_dir: *mut dentry;

    fn notifier_err_inject_init(
        name: *const c_char,
        parent: *mut dentry,
        inject: *mut notifier_err_inject,
        priority: c_int,
    ) -> *mut dentry;
    fn register_netdevice_notifier(nb: *mut notifier_block) -> c_int;
    fn unregister_netdevice_notifier(nb: *mut notifier_block) -> c_int;
    fn debugfs_remove_recursive(dent: *mut dentry);
}

// The values are provided by linux/netdevice.h.
extern "C" {
    static NETDEV_REGISTER: c_int;
    static NETDEV_CHANGEMTU: c_int;
    static NETDEV_CHANGENAME: c_int;
    static NETDEV_PRE_UP: c_int;
    static NETDEV_PRE_TYPE_CHANGE: c_int;
    static NETDEV_POST_INIT: c_int;
    static NETDEV_PRECHANGEMTU: c_int;
    static NETDEV_PRECHANGEUPPER: c_int;
    static NETDEV_CHANGEUPPER: c_int;
}

static mut priority: c_int = 0;

static mut netdev_notifier_err_inject: notifier_err_inject = notifier_err_inject {
    actions: [
        notifier_err_inject_action { action: unsafe { NETDEV_REGISTER } },
        notifier_err_inject_action { action: unsafe { NETDEV_CHANGEMTU } },
        notifier_err_inject_action { action: unsafe { NETDEV_CHANGENAME } },
        notifier_err_inject_action { action: unsafe { NETDEV_PRE_UP } },
        notifier_err_inject_action { action: unsafe { NETDEV_PRE_TYPE_CHANGE } },
        notifier_err_inject_action { action: unsafe { NETDEV_POST_INIT } },
        notifier_err_inject_action { action: unsafe { NETDEV_PRECHANGEMTU } },
        notifier_err_inject_action { action: unsafe { NETDEV_PRECHANGEUPPER } },
        notifier_err_inject_action { action: unsafe { NETDEV_CHANGEUPPER } },
        notifier_err_inject_action { action: 0 },
    ],
    nb: notifier_block { _private: [] },
};

static mut dir: *mut dentry = core::ptr::null_mut();

unsafe fn netdev_err_inject_init() -> c_int {
    let mut err: c_int;

    dir = notifier_err_inject_init(
        b"netdev\0".as_ptr() as *const c_char,
        notifier_err_inject_dir,
        &mut netdev_notifier_err_inject,
        priority,
    );
    // IS_ERR/PTR_ERR are kernel helpers supplied by the surrounding bindings.
    if (dir as usize) >= (!4095usize) {
        return -(dir as isize) as c_int;
    }

    err = register_netdevice_notifier(&mut netdev_notifier_err_inject.nb);
    if err != 0 {
        debugfs_remove_recursive(dir);
    }

    err
}

unsafe fn netdev_err_inject_exit() {
    unregister_netdevice_notifier(&mut netdev_notifier_err_inject.nb);
    debugfs_remove_recursive(dir);
}

// module_param(priority, int, 0)
// MODULE_PARM_DESC(priority, "specify netdevice notifier priority")
// module_init(netdev_err_inject_init)
// module_exit(netdev_err_inject_exit)
// MODULE_DESCRIPTION("Netdevice notifier error injection module")
// MODULE_LICENSE("GPL")
// MODULE_AUTHOR("Nikolay Aleksandrov <razor@blackwall.org>")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
