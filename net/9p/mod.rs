// SPDX-License-Identifier: GPL-2.0-only
/*
 *  9P entry point
 *
 *  Copyright (C) 2007 by Latchesar Ionkov <lucho@ionkov.net>
 *  Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
 *  Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Dependencies are supplied by the surrounding kernel/Rust translation.

#[cfg(CONFIG_NET_9P_DEBUG)]
pub static mut p9_debug_level: u32 = 0;

#[cfg(CONFIG_NET_9P_DEBUG)]
pub unsafe fn _p9_debug(
    level: p9_debug_flags,
    func: *const core::ffi::c_char,
    fmt: *const core::ffi::c_char,
    mut args: ...,
) {
    // va_list/va_format and task_pid_nr(current) are kernel facilities.
    if (p9_debug_level & level as u32) != level as u32 {
        return;
    }

    // The original constructs a va_format and emits either:
    // pr_notice("(%8.8d) %pV", task_pid_nr(current), &vaf)
    // or pr_notice("-- %s (%d): %pV", func, task_pid_nr(current), &vaf).
    let _ = (func, fmt, &mut args);
    if level == p9_debug_flags::P9_DEBUG_9P {
        pr_notice_debug(format_args!("({:8.8}) %pV", task_pid_nr_current()));
    } else {
        pr_notice_debug(format_args!("-- %s (%d): %pV", func, task_pid_nr_current()));
    }
}

// Dynamic Transport Registration Routines

static mut v9fs_trans_lock: spinlock_t = spinlock_t::new();
static mut v9fs_trans_list: list_head = list_head::new();

pub unsafe fn v9fs_register_trans(m: *mut p9_trans_module) {
    spin_lock(&raw mut v9fs_trans_lock);
    list_add_tail(&mut (*m).list, &raw mut v9fs_trans_list);
    spin_unlock(&raw mut v9fs_trans_lock);
}

pub unsafe fn v9fs_unregister_trans(m: *mut p9_trans_module) {
    spin_lock(&raw mut v9fs_trans_lock);
    list_del_init(&mut (*m).list);
    spin_unlock(&raw mut v9fs_trans_lock);
}

unsafe fn _p9_get_trans_by_name(s: *const core::ffi::c_char) -> *mut p9_trans_module {
    let mut found: *mut p9_trans_module = core::ptr::null_mut();

    spin_lock(&raw mut v9fs_trans_lock);
    list_for_each_entry!(t, v9fs_trans_list, list, {
        if strcmp((*t).name, s) == 0 && try_module_get((*t).owner) {
            found = t;
            break;
        }
    });
    spin_unlock(&raw mut v9fs_trans_lock);
    found
}

pub unsafe fn v9fs_get_trans_by_name(s: *const core::ffi::c_char) -> *mut p9_trans_module {
    let mut found = _p9_get_trans_by_name(s);
    // CONFIG_MODULES preserves the original optional request_module path.
    #[cfg(CONFIG_MODULES)]
    if found.is_null() {
        request_module(c"9p-%s".as_ptr(), s);
        found = _p9_get_trans_by_name(s);
    }
    found
}

static v9fs_default_transports: [&[u8]; 6] = [
    b"virtio\0", b"tcp\0", b"fd\0", b"unix\0", b"xen\0", b"rdma\0",
];

pub unsafe fn v9fs_get_default_trans() -> *mut p9_trans_module {
    let mut found: *mut p9_trans_module = core::ptr::null_mut();
    spin_lock(&raw mut v9fs_trans_lock);
    list_for_each_entry!(t, v9fs_trans_list, list, {
        if (*t).def && try_module_get((*t).owner) { found = t; break; }
    });
    if found.is_null() {
        list_for_each_entry!(t, v9fs_trans_list, list, {
            if try_module_get((*t).owner) { found = t; break; }
        });
    }
    spin_unlock(&raw mut v9fs_trans_lock);
    let mut i = 0;
    while found.is_null() && i < v9fs_default_transports.len() {
        found = v9fs_get_trans_by_name(v9fs_default_transports[i].as_ptr() as *const _);
        i += 1;
    }
    found
}

pub unsafe fn v9fs_put_trans(m: *mut p9_trans_module) {
    if !m.is_null() { module_put((*m).owner); }
}

unsafe fn init_p9() -> i32 {
    let ret = p9_client_init();
    if ret != 0 { return ret; }
    p9_error_init();
    pr_info!("Installing 9P2000 support\n");
    ret
}

unsafe fn exit_p9() {
    pr_info!("Unloading 9P2000 support\n");
    p9_client_exit();
}

// module_init(init_p9)
// module_exit(exit_p9)
// MODULE_AUTHOR("Latchesar Ionkov <lucho@ionkov.net>");
// MODULE_AUTHOR("Eric Van Hensbergen <ericvh@gmail.com>");
// MODULE_AUTHOR("Ron Minnich <rminnich@lanl.gov>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Plan 9 Resource Sharing Support (9P2000)");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
