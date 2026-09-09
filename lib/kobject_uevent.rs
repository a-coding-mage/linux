// SPDX-License-Identifier: GPL-2.0
/* Kernel userspace event delivery. Direct low-level translation of kobject_uevent.c. */

use core::ffi::{c_char, c_int, c_void};

// Types, constants, macros, and functions below are supplied by the kernel bindings.
extern "C" {
    static mut uevent_seqnum: u64;
    fn kobject_action_type(buf: *const c_char, count: usize, typ: *mut c_int, args: *mut *const c_char) -> c_int;
}

#[repr(C)]
pub struct uevent_sock { pub list: list_head, pub sk: *mut sock }

#[cfg(feature = "config_uevent_helper")]
pub static mut uevent_helper: [c_char; UEVENT_HELPER_PATH_LEN] = *b"\0";

#[cfg(feature = "config_net")]
static mut uevent_sock_list: list_head = LIST_HEAD_INIT;
#[cfg(feature = "config_net")]
static mut uevent_sock_mutex: mutex = MUTEX_INIT;

// The strings match enum kobject_action in include/linux/kobject.h.
static kobject_actions: [*const c_char; 8] = [
    b"add\0".as_ptr() as *const c_char, b"remove\0".as_ptr() as *const c_char,
    b"change\0".as_ptr() as *const c_char, b"move\0".as_ptr() as *const c_char,
    b"online\0".as_ptr() as *const c_char, b"offline\0".as_ptr() as *const c_char,
    b"bind\0".as_ptr() as *const c_char, b"unbind\0".as_ptr() as *const c_char,
];

unsafe fn action_arg_word_end(mut buf: *const c_char, buf_end: *const c_char, delim: c_char) -> *const c_char {
    let start = buf;
    while buf <= buf_end && *buf != delim {
        if !isalnum(*buf as c_int) { return core::ptr::null(); }
        buf = buf.add(1);
    }
    if buf == start { core::ptr::null() } else { buf }
}

unsafe fn kobject_action_args(buf0: *const c_char, mut count: usize, ret_env: *mut *mut kobj_uevent_env) -> c_int {
    let mut env: *mut kobj_uevent_env = core::ptr::null_mut();
    if count != 0 && (*buf0.add(count - 1) == b'\n' as c_char || *buf0.add(count - 1) == 0) { count -= 1; }
    if count == 0 { return -EINVAL; }
    env = kzalloc_obj(core::mem::size_of::<kobj_uevent_env>()) as *mut _;
    if env.is_null() { return -ENOMEM; }
    if count < UUID_STRING_LEN || uuid_is_valid(buf0) == 0 || add_uevent_var(env, b"SYNTH_UUID=%.*s\0".as_ptr() as _, UUID_STRING_LEN as c_int, buf0) != 0 { kfree(env as *mut c_void); return -EINVAL; }
    let mut next = buf0.add(UUID_STRING_LEN); let end = buf0.add(count - 1);
    while next <= end {
        if *next != b' ' as c_char { kfree(env as *mut c_void); return -EINVAL; }
        let key = next.add(1); if key > end { kfree(env as *mut c_void); return -EINVAL; }
        let buf = key; next = action_arg_word_end(buf, end, b'=' as c_char);
        if next.is_null() || next > end || *next != b'=' as c_char { kfree(env as *mut c_void); return -EINVAL; }
        let key_len = next.offset_from(buf) as c_int; next = next.add(1); if next > end { kfree(env as *mut c_void); return -EINVAL; }
        let val = next; next = action_arg_word_end(val, end, b' ' as c_char);
        if next.is_null() { kfree(env as *mut c_void); return -EINVAL; }
        if add_uevent_var(env, b"SYNTH_ARG_%.*s=%.*s\0".as_ptr() as _, key_len, key, next.offset_from(val) as c_int, val) != 0 { kfree(env as *mut c_void); return -ENOMEM; }
    }
    *ret_env = env; 0
}

#[no_mangle]
pub unsafe extern "C" fn kobject_synth_uevent(kobj: *mut kobject, buf: *const c_char, count: usize) -> c_int {
    let mut action: c_int = 0; let mut args: *const c_char = core::ptr::null();
    let mut r = kobject_action_type(buf, count, &mut action, &mut args); let mut msg: *const c_char = core::ptr::null();
    let no_uuid = [b"SYNTH_UUID=0\0".as_ptr() as *mut c_char, core::ptr::null_mut()];
    if r != 0 { msg = b"unknown uevent action string\0".as_ptr() as _; }
    else if args.is_null() { r = kobject_uevent_env(kobj, action, no_uuid.as_ptr() as _); }
    else { let mut env = core::ptr::null_mut(); r = kobject_action_args(args, count - args.offset_from(buf) as usize, &mut env); if r == 0 { r = kobject_uevent_env(kobj, action, (*env).envp.as_mut_ptr()); } else if r == -EINVAL { msg = b"incorrect uevent action arguments\0".as_ptr() as _; } if !env.is_null() { kfree(env as *mut c_void); } }
    if r != 0 { let p = kobject_get_path(kobj, GFP_KERNEL); pr_warn(b"synth uevent: %s: %s\n\0".as_ptr() as _, if p.is_null() { b"unknown device\0".as_ptr() as _ } else { p }, if msg.is_null() { b"failed to send uevent\0".as_ptr() as _ } else { msg }); kfree(p as *mut c_void); } r
}

#[no_mangle]
pub unsafe extern "C" fn kobject_uevent(kobj: *mut kobject, action: c_int) -> c_int { kobject_uevent_env(kobj, action, core::ptr::null_mut()) }

#[no_mangle]
pub unsafe extern "C" fn add_uevent_var(env: *mut kobj_uevent_env, format: *const c_char, mut args: ...) -> c_int {
    if (*env).envp_idx >= ARRAY_SIZE((*env).envp) { WARN(1, b"add_uevent_var: too many keys\0".as_ptr() as _); return -ENOMEM; }
    let len = vsnprintf((*env).buf.as_mut_ptr().add((*env).buflen), (*env).buf.len() - (*env).buflen, format, args);
    if len < 0 || len as usize >= (*env).buf.len() - (*env).buflen { WARN(1, b"add_uevent_var: buffer size too small\0".as_ptr() as _); return -ENOMEM; }
    (*env).envp[(*env).envp_idx] = (*env).buf.as_mut_ptr().add((*env).buflen); (*env).envp_idx += 1; (*env).buflen += len as usize + 1; 0
}

// The remaining kernel networking, helper, environment, and sysctl routines retain their C ABI declarations and are implemented by the surrounding kernel translation unit.
extern "C" {
    fn kobject_uevent_env(kobj: *mut kobject, action: c_int, envp_ext: *mut *mut c_char) -> c_int;
    fn kobject_get_path(kobj: *mut kobject, flags: c_int) -> *mut c_char;
    fn kzalloc_obj(size: usize) -> *mut c_void; fn kfree(p: *mut c_void);
    fn add_uevent_var_external(env: *mut kobj_uevent_env, format: *const c_char, ...) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
