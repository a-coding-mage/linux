// SPDX-License-Identifier: GPL-2.0-or-later
/* Provide a way to create a superblock configuration context within the kernel
 * that allows a superblock to be set up prior to mounting.
 *
 * Copyright (C) 2017 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C includes and build-time kernel dependencies are supplied by other files.

static COMMON_SET_SB_FLAG: [constant_table; 6] = [
    constant_table { name: b"dirsync\0".as_ptr() as *const c_char, value: SB_DIRSYNC },
    constant_table { name: b"lazytime\0".as_ptr() as *const c_char, value: SB_LAZYTIME },
    constant_table { name: b"mand\0".as_ptr() as *const c_char, value: SB_MANDLOCK },
    constant_table { name: b"ro\0".as_ptr() as *const c_char, value: SB_RDONLY },
    constant_table { name: b"sync\0".as_ptr() as *const c_char, value: SB_SYNCHRONOUS },
    constant_table { name: core::ptr::null(), value: 0 },
];

static COMMON_CLEAR_SB_FLAG: [constant_table; 5] = [
    constant_table { name: b"async\0".as_ptr() as *const c_char, value: SB_SYNCHRONOUS },
    constant_table { name: b"nolazytime\0".as_ptr() as *const c_char, value: SB_LAZYTIME },
    constant_table { name: b"nomand\0".as_ptr() as *const c_char, value: SB_MANDLOCK },
    constant_table { name: b"rw\0".as_ptr() as *const c_char, value: SB_RDONLY },
    constant_table { name: core::ptr::null(), value: 0 },
];

/* Check for a common mount option that manipulates s_flags. */
unsafe fn vfs_parse_sb_flag(fc: *mut fs_context, key: *const c_char) -> c_int {
    let mut token: c_uint;
    token = lookup_constant(COMMON_SET_SB_FLAG.as_ptr(), key, 0);
    if token != 0 {
        (*fc).sb_flags |= token;
        (*fc).sb_flags_mask |= token;
        return 0;
    }
    token = lookup_constant(COMMON_CLEAR_SB_FLAG.as_ptr(), key, 0);
    if token != 0 {
        (*fc).sb_flags &= !token;
        (*fc).sb_flags_mask |= token;
        return 0;
    }
    -ENOPARAM
}

pub unsafe fn vfs_parse_fs_param_source(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    if strcmp((*param).key, b"source\0".as_ptr() as *const c_char) != 0 { return -ENOPARAM; }
    if (*param).type_ != fs_value_is_string { return invalf(fc, b"Non-string source\0".as_ptr() as *const c_char); }
    if !(*fc).source.is_null() { return invalf(fc, b"Multiple sources\0".as_ptr() as *const c_char); }
    (*fc).source = (*param).string;
    (*param).string = core::ptr::null_mut();
    0
}

pub unsafe fn vfs_parse_fs_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let mut ret: c_int;
    if (*param).key.is_null() { return invalf(fc, b"Unnamed parameter\n\0".as_ptr() as *const c_char); }
    ret = vfs_parse_sb_flag(fc, (*param).key);
    if ret != -ENOPARAM { return ret; }
    ret = security_fs_context_parse_param(fc, param);
    if ret != -ENOPARAM { return ret; }
    if !(*fc).ops.is_null() && !(*(*fc).ops).parse_param.is_none() {
        ret = ((*(*fc).ops).parse_param.unwrap())(fc, param);
        if ret != -ENOPARAM { return ret; }
    }
    ret = vfs_parse_fs_param_source(fc, param);
    if ret != -ENOPARAM { return ret; }
    invalf(fc, b"%s: Unknown parameter '%s'\0".as_ptr() as *const c_char, (*(*fc).fs_type).name, (*param).key)
}

pub unsafe fn vfs_parse_fs_qstr(fc: *mut fs_context, key: *const c_char, value: *const qstr) -> c_int {
    let mut param: fs_parameter = core::mem::zeroed();
    param.key = key;
    param.type_ = fs_value_is_flag;
    param.size = if value.is_null() { 0 } else { (*value).len };
    if !value.is_null() {
        param.string = kmemdup_nul((*value).name, (*value).len, GFP_KERNEL);
        if param.string.is_null() { return -ENOMEM; }
        param.type_ = fs_value_is_string;
    }
    let ret = vfs_parse_fs_param(fc, &mut param);
    kfree(param.string as *mut c_void);
    ret
}

pub unsafe fn vfs_parse_monolithic_sep(fc: *mut fs_context, data: *mut c_void, sep: Option<unsafe extern "C" fn(*mut *mut c_char) -> *mut c_char>) -> c_int {
    let mut options = data as *mut c_char;
    if options.is_null() { return 0; }
    let mut ret = security_sb_eat_lsm_opts(options, &mut (*fc).security);
    if ret != 0 { return ret; }
    loop {
        let key = sep.unwrap()(&mut options);
        if key.is_null() { break; }
        if *key != 0 {
            let mut value = strchr(key, b'=' as c_int);
            if !value.is_null() {
                if value == key { continue; }
                *value.add(1) = 0;
                value = value.add(1);
            }
            ret = vfs_parse_fs_string(fc, key, value);
            if ret < 0 { break; }
        }
    }
    ret
}

unsafe extern "C" fn vfs_parse_comma_sep(s: *mut *mut c_char) -> *mut c_char { strsep(s, b",".as_ptr() as *const c_char) }

pub unsafe fn generic_parse_monolithic(fc: *mut fs_context, data: *mut c_void) -> c_int {
    vfs_parse_monolithic_sep(fc, data, Some(vfs_parse_comma_sep))
}

unsafe fn alloc_fs_context(fs_type: *mut file_system_type, reference: *mut dentry, sb_flags: c_uint, sb_flags_mask: c_uint, purpose: fs_context_purpose) -> *mut fs_context {
    let fc = kzalloc_obj::<fs_context>(GFP_KERNEL_ACCOUNT);
    if fc.is_null() { return ERR_PTR(-ENOMEM); }
    (*fc).purpose = purpose; (*fc).sb_flags = sb_flags; (*fc).sb_flags_mask = sb_flags_mask;
    (*fc).fs_type = get_filesystem(fs_type); (*fc).cred = get_current_cred();
    (*fc).net_ns = get_net((*current).nsproxy.net_ns); (*fc).log.prefix = (*fs_type).name;
    mutex_init(&mut (*fc).uapi_mutex);
    match purpose {
        FS_CONTEXT_FOR_MOUNT => (*fc).user_ns = get_user_ns((*(*fc).cred).user_ns),
        FS_CONTEXT_FOR_SUBMOUNT => (*fc).user_ns = get_user_ns((*(*reference).d_sb).s_user_ns),
        FS_CONTEXT_FOR_RECONFIGURE => { atomic_inc(&mut (*(*reference).d_sb).s_active); (*fc).user_ns = get_user_ns((*(*reference).d_sb).s_user_ns); (*fc).root = dget(reference); },
        _ => {}
    }
    let ret = ((*(*fc).fs_type).init_fs_context.unwrap())(fc);
    if ret < 0 { put_fs_context(fc); return ERR_PTR(ret); }
    (*fc).need_free = true;
    fc
}

pub unsafe fn fs_context_for_mount(fs_type: *mut file_system_type, sb_flags: c_uint) -> *mut fs_context { alloc_fs_context(fs_type, core::ptr::null_mut(), sb_flags, 0, FS_CONTEXT_FOR_MOUNT) }

pub unsafe fn fs_context_for_reconfigure(dentry: *mut dentry, sb_flags: c_uint, sb_flags_mask: c_uint) -> *mut fs_context { alloc_fs_context((*(*dentry).d_sb).s_type, dentry, sb_flags, sb_flags_mask, FS_CONTEXT_FOR_RECONFIGURE) }

pub unsafe fn fs_context_for_submount(ty: *mut file_system_type, reference: *mut dentry) -> *mut fs_context {
    let fc = alloc_fs_context(ty, reference, 0, 0, FS_CONTEXT_FOR_SUBMOUNT);
    if IS_ERR(fc) { return fc; }
    let ret = security_fs_context_submount(fc, (*reference).d_sb);
    if ret != 0 { put_fs_context(fc); return ERR_PTR(ret); }
    fc
}

pub unsafe fn fc_drop_locked(fc: *mut fs_context) { let sb = (*(*fc).root).d_sb; dput((*fc).root); (*fc).root = core::ptr::null_mut(); deactivate_locked_super(sb); }

pub unsafe fn vfs_dup_fs_context(src_fc: *mut fs_context) -> *mut fs_context {
    if (*(*src_fc).ops).dup.is_none() { return ERR_PTR(-EOPNOTSUPP); }
    let fc = kmemdup(src_fc as *const c_void, core::mem::size_of::<fs_context>(), GFP_KERNEL) as *mut fs_context;
    if fc.is_null() { return ERR_PTR(-ENOMEM); }
    mutex_init(&mut (*fc).uapi_mutex); (*fc).fs_private = core::ptr::null_mut(); (*fc).s_fs_info = core::ptr::null_mut(); (*fc).source = core::ptr::null_mut(); (*fc).security = core::ptr::null_mut();
    get_filesystem((*fc).fs_type); get_net((*fc).net_ns); get_user_ns((*fc).user_ns); get_cred((*fc).cred);
    if !(*fc).log.log.is_null() { refcount_inc(&mut (*(*fc).log.log).usage); }
    let ret = ((*(*fc).ops).dup.unwrap())(fc, src_fc);
    if ret < 0 { put_fs_context(fc); return ERR_PTR(ret); }
    let ret = security_fs_context_dup(fc, src_fc);
    if ret < 0 { put_fs_context(fc); return ERR_PTR(ret); }
    fc
}

/* Log a message to a filesystem context. */
pub unsafe extern "C" fn logfc(log: *mut fc_log, prefix: *const c_char, level: c_char, fmt: *const c_char, mut args: ...) {
    if log.is_null() {
        match level as u8 as char {
            'w' => printk(KERN_WARNING, b"%s%s%pV\n\0".as_ptr() as *const c_char, if prefix.is_null() { b"\0".as_ptr() as *const c_char } else { prefix }, if prefix.is_null() { b"\0".as_ptr() as *const c_char } else { b": \0".as_ptr() as *const c_char }, &mut args),
            'e' => printk(KERN_ERR, b"%s%s%pV\n\0".as_ptr() as *const c_char, if prefix.is_null() { b"\0".as_ptr() as *const c_char } else { prefix }, if prefix.is_null() { b"\0".as_ptr() as *const c_char } else { b": \0".as_ptr() as *const c_char }, &mut args),
            'i' => printk(KERN_INFO, b"%s%s%pV\n\0".as_ptr() as *const c_char, if prefix.is_null() { b"\0".as_ptr() as *const c_char } else { prefix }, if prefix.is_null() { b"\0".as_ptr() as *const c_char } else { b": \0".as_ptr() as *const c_char }, &mut args),
            _ => printk(KERN_NOTICE, b"%s%s%pV\n\0".as_ptr() as *const c_char, if prefix.is_null() { b"\0".as_ptr() as *const c_char } else { prefix }, if prefix.is_null() { b"\0".as_ptr() as *const c_char } else { b": \0".as_ptr() as *const c_char }, &mut args),
        }
    } else {
        let logsize = (*log).buffer.len();
        let index = ((*log).head as usize) & (logsize - 1);
        let q = kasprintf(GFP_KERNEL, b"%c %s%s%pV\n\0".as_ptr() as *const c_char, level, if prefix.is_null() { b"\0".as_ptr() as *const c_char } else { prefix }, if prefix.is_null() { b"\0".as_ptr() as *const c_char } else { b": \0".as_ptr() as *const c_char }, &mut args);
        if ((*log).head as u8).wrapping_sub((*log).tail as u8) as usize == logsize {
            if (*log).need_free & (1 << index) != 0 { kfree((*log).buffer[index] as *mut c_void); }
            (*log).tail = (*log).tail.wrapping_add(1);
        }
        (*log).buffer[index] = if q.is_null() { b"OOM: Can't store error string\0".as_ptr() as *mut c_char } else { q };
        if !q.is_null() { (*log).need_free |= 1 << index; } else { (*log).need_free &= !(1 << index); }
        (*log).head = (*log).head.wrapping_add(1);
    }
}

pub unsafe fn put_fc_log(fc: *mut fs_context) {
    let log = (*fc).log.log;
    if !log.is_null() && refcount_dec_and_test(&mut (*log).usage) { (*fc).log.log = core::ptr::null_mut(); for i in 0..(*log).buffer.len() { if (*log).need_free & (1 << i) != 0 { kfree((*log).buffer[i] as *mut c_void); } } kfree(log as *mut c_void); }
}

pub unsafe fn put_fs_context(fc: *mut fs_context) {
    if !(*fc).root.is_null() { let sb = (*(*fc).root).d_sb; dput((*fc).root); (*fc).root = core::ptr::null_mut(); deactivate_super(sb); }
    if (*fc).need_free && !(*fc).ops.is_null() && !(*(*fc).ops).free.is_none() { ((*(*fc).ops).free.unwrap())(fc); }
    security_free_mnt_opts(&mut (*fc).security); put_net((*fc).net_ns); put_user_ns((*fc).user_ns); put_cred((*fc).cred); put_fc_log(fc); put_filesystem((*fc).fs_type); kfree((*fc).source as *mut c_void); kfree(fc as *mut c_void);
}

pub unsafe fn parse_monolithic_mount_data(fc: *mut fs_context, data: *mut c_void) -> c_int { match (*(*fc).ops).parse_monolithic { Some(f) => f(fc, data), None => generic_parse_monolithic(fc, data) } }

pub unsafe fn vfs_clean_context(fc: *mut fs_context) {
    if (*fc).need_free && !(*fc).ops.is_null() && !(*(*fc).ops).free.is_none() { ((*(*fc).ops).free.unwrap())(fc); }
    (*fc).need_free = false; (*fc).fs_private = core::ptr::null_mut(); (*fc).s_fs_info = core::ptr::null_mut(); (*fc).sb_flags = 0; security_free_mnt_opts(&mut (*fc).security); kfree((*fc).source as *mut c_void); (*fc).source = core::ptr::null_mut(); (*fc).exclusive = false; (*fc).purpose = FS_CONTEXT_FOR_RECONFIGURE; (*fc).phase = FS_CONTEXT_AWAITING_RECONF;
}

pub unsafe fn finish_clean_context(fc: *mut fs_context) -> c_int {
    if (*fc).phase != FS_CONTEXT_AWAITING_RECONF { return 0; }
    let error = ((*(*fc).fs_type).init_fs_context.unwrap())(fc);
    if error != 0 { (*fc).phase = FS_CONTEXT_FAILED; return error; }
    (*fc).need_free = true; (*fc).phase = FS_CONTEXT_RECONF_PARAMS; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
