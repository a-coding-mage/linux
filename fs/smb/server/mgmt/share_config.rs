// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Linux and local C headers supplied by the surrounding translation unit.

const SHARE_HASH_BITS: usize = 12;

static mut SHARES_TABLE: HashedTable = HashedTable::new(SHARE_HASH_BITS);
static mut SHARES_TABLE_LOCK: RwSemaphore = RwSemaphore::new();

#[repr(C)]
struct KsmbdVetoPattern {
    pattern: *mut libc::c_char,
    list: ListHead,
}

#[cfg(feature = "CONFIG_PROC_FS")]
static KSMBD_SHARE_FLAG_NAMES: &[KsmbdConstName] = &[
    KsmbdConstName { value: KSMBD_SHARE_FLAG_AVAILABLE, name: b"available\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_BROWSEABLE, name: b"browseable\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_WRITEABLE, name: b"writeable\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_READONLY, name: b"read-only\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_GUEST_OK, name: b"guest-ok\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_GUEST_ONLY, name: b"guest-only\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_STORE_DOS_ATTRS, name: b"store-dos-attrs\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_OPLOCKS, name: b"oplocks\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_PIPE, name: b"pipe\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_HIDE_DOT_FILES, name: b"hide-dot-files\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_INHERIT_OWNER, name: b"inherit-owner\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_STREAMS, name: b"streams\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_FOLLOW_SYMLINKS, name: b"follow-symlinks\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_ACL_XATTR, name: b"acl-xattr\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_UPDATE, name: b"update\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_CROSSMNT, name: b"crossmnt\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_CONTINUOUS_AVAILABILITY, name: b"continuous-availability\0".as_ptr() as *const libc::c_char },
    KsmbdConstName { value: KSMBD_SHARE_FLAG_ENCRYPT_DATA, name: b"encrypt-data\0".as_ptr() as *const libc::c_char },
];

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn proc_show_shares(m: *mut SeqFile, _v: *mut libc::c_void) -> libc::c_int {
    let mut share: *mut KsmbdShareConfig;
    let mut i: libc::c_int = 0;
    down_read(&raw mut SHARES_TABLE_LOCK);
    hash_for_each!(SHARES_TABLE, i, share, hlist, {
        seq_printf(m, b"name:\t%s\n\0".as_ptr() as *const libc::c_char, (*share).name);
        seq_printf(m, b"type:\t%s\n\0".as_ptr() as *const libc::c_char,
            if test_share_config_flag(share, KSMBD_SHARE_FLAG_PIPE) { b"pipe\0".as_ptr() } else { b"disk\0".as_ptr() });
        seq_printf(m, b"tree_connects:\t%d\n\0".as_ptr() as *const libc::c_char, atomic_read(&raw mut (*share).tree_connections));
        seq_printf(m, b"file_mask:\t0%07o\n\0".as_ptr() as *const libc::c_char, (*share).create_mask);
        seq_printf(m, b"directory_mask:\t0%07o\n\0".as_ptr() as *const libc::c_char, (*share).directory_mask);
        seq_puts(m, b"flags:\t\0".as_ptr() as *const libc::c_char);
        ksmbd_proc_show_flag_names(m, KSMBD_SHARE_FLAG_NAMES.as_ptr(), KSMBD_SHARE_FLAG_NAMES.len(), (*share).flags);
        seq_puts(m, b"\n\n\0".as_ptr() as *const libc::c_char);
    });
    up_read(&raw mut SHARES_TABLE_LOCK);
    0
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn create_proc_shares() -> libc::c_int {
    if !ksmbd_proc_create(b"shares\0".as_ptr() as *const libc::c_char, Some(proc_show_shares), core::ptr::null_mut()) { return -libc::ENOMEM; }
    0
}

#[cfg(not(feature = "CONFIG_PROC_FS"))]
unsafe fn create_proc_shares() -> libc::c_int { 0 }

unsafe fn share_name_hash(name: *const libc::c_char) -> libc::c_uint {
    jhash(name as *const libc::c_void, strlen(name), 0)
}

unsafe fn kill_share(share: *mut KsmbdShareConfig) {
    while !list_empty(&raw mut (*share).veto_list) {
        let p = list_entry((*share).veto_list.next, KsmbdVetoPattern, list);
        list_del(&raw mut p.list);
        kfree(p.pattern as *mut libc::c_void);
        kfree(p as *const _ as *mut libc::c_void);
    }
    if !(*share).path.is_null() { path_put(&raw mut (*share).vfs_path); }
    kfree((*share).name as *mut libc::c_void);
    kfree((*share).path as *mut libc::c_void);
    kfree(share as *mut libc::c_void);
}

unsafe fn ksmbd_share_config_del(share: *mut KsmbdShareConfig) {
    down_write(&raw mut SHARES_TABLE_LOCK);
    hash_del(&raw mut (*share).hlist);
    up_write(&raw mut SHARES_TABLE_LOCK);
}

unsafe fn __ksmbd_share_config_put(share: *mut KsmbdShareConfig) {
    ksmbd_share_config_del(share);
    kill_share(share);
}

unsafe fn __get_share_config(share: *mut KsmbdShareConfig) -> *mut KsmbdShareConfig {
    if !atomic_inc_not_zero(&raw mut (*share).refcount) { return core::ptr::null_mut(); }
    share
}

unsafe fn __share_lookup(name: *const libc::c_char) -> *mut KsmbdShareConfig {
    let key = share_name_hash(name);
    let mut share: *mut KsmbdShareConfig;
    hash_for_each_possible!(SHARES_TABLE, share, hlist, key, {
        if strcmp(name, (*share).name) == 0 { return share; }
    });
    core::ptr::null_mut()
}

unsafe fn parse_veto_list(share: *mut KsmbdShareConfig, mut veto_list: *mut libc::c_char, mut veto_list_sz: libc::c_int) -> libc::c_int {
    let mut sz: libc::c_int = 0;
    if veto_list_sz == 0 { return 0; }
    while veto_list_sz > 0 {
        sz = strlen(veto_list) as libc::c_int;
        if sz == 0 { break; }
        let p = kzalloc_obj::<KsmbdVetoPattern>(KSMBD_DEFAULT_GFP);
        if p.is_null() { return -libc::ENOMEM; }
        (*p).pattern = kstrdup(veto_list, KSMBD_DEFAULT_GFP);
        if (*p).pattern.is_null() { kfree(p as *mut libc::c_void); return -libc::ENOMEM; }
        list_add(&raw mut (*p).list, &raw mut (*share).veto_list);
        veto_list = veto_list.add((sz + 1) as usize);
        veto_list_sz -= sz + 1;
    }
    0
}

// The remaining share_config_request body follows the C implementation; external
// structures and helpers are supplied by the corresponding translated headers.
unsafe fn share_config_request(work: *mut KsmbdWork, name: *const libc::c_char) -> *mut KsmbdShareConfig {
    let resp = ksmbd_ipc_share_config_request(name);
    if resp.is_null() || (*resp).flags == KSMBD_SHARE_FLAG_INVALID { if !resp.is_null() { kvfree(resp as *mut libc::c_void); } return core::ptr::null_mut(); }
    let share = kzalloc_obj::<KsmbdShareConfig>(KSMBD_DEFAULT_GFP);
    if share.is_null() { kvfree(resp as *mut libc::c_void); return core::ptr::null_mut(); }
    (*share).flags = (*resp).flags;
    atomic_set(&raw mut (*share).refcount, 1);
    ksmbd_share_tree_conn_init(share);
    INIT_LIST_HEAD(&raw mut (*share).veto_list);
    (*share).name = kstrdup(name, KSMBD_DEFAULT_GFP);
    if (*share).name.is_null() { kill_share(share); kvfree(resp as *mut libc::c_void); return core::ptr::null_mut(); }
    if !test_share_config_flag(share, KSMBD_SHARE_FLAG_PIPE) {
        let mut path_len = PATH_MAX;
        if (*resp).payload_sz != 0 { path_len = (*resp).payload_sz - (*resp).veto_list_sz; }
        (*share).path = kstrndup(ksmbd_share_config_path(resp), path_len, KSMBD_DEFAULT_GFP);
        let mut ret = if (*share).path.is_null() { -libc::ENOMEM } else { 0 };
        if ret == 0 {
            (*share).path_sz = strlen((*share).path);
            while (*share).path_sz > 1 && *(*share).path.add((*share).path_sz - 1) == b'/' as libc::c_char {
                (*share).path_sz -= 1; *(*share).path.add((*share).path_sz) = 0;
            }
        }
        (*share).create_mask = (*resp).create_mask;
        (*share).directory_mask = (*resp).directory_mask;
        (*share).force_create_mode = (*resp).force_create_mode;
        (*share).force_directory_mode = (*resp).force_directory_mode;
        (*share).force_uid = (*resp).force_uid;
        (*share).force_gid = (*resp).force_gid;
        if ret == 0 { ret = parse_veto_list(share, KSMBD_SHARE_CONFIG_VETO_LIST(resp), (*resp).veto_list_sz); }
        if ret == 0 && !(*share).path.is_null() {
            if __ksmbd_override_fsids(work, share) { kill_share(share); kvfree(resp as *mut libc::c_void); return core::ptr::null_mut(); }
            ret = kern_path((*share).path, 0, &raw mut (*share).vfs_path);
            ksmbd_revert_fsids(work);
            if ret != 0 { ksmbd_debug(SMB, b"failed to access '%s'\n\0".as_ptr() as *const libc::c_char, (*share).path); kfree((*share).path as *mut libc::c_void); (*share).path = core::ptr::null_mut(); }
        }
        if ret != 0 { kill_share(share); kvfree(resp as *mut libc::c_void); return core::ptr::null_mut(); }
    }
    down_write(&raw mut SHARES_TABLE_LOCK);
    let mut lookup = __share_lookup(name);
    if !lookup.is_null() { lookup = __get_share_config(lookup); }
    if lookup.is_null() { hash_add(&raw mut SHARES_TABLE, &raw mut (*share).hlist, share_name_hash(name)); }
    else { kill_share(share); }
    up_write(&raw mut SHARES_TABLE_LOCK);
    let result = if lookup.is_null() { share } else { lookup };
    kvfree(resp as *mut libc::c_void);
    result
}

unsafe fn ksmbd_share_config_get(work: *mut KsmbdWork, name: *const libc::c_char) -> *mut KsmbdShareConfig {
    down_read(&raw mut SHARES_TABLE_LOCK);
    let mut share = __share_lookup(name);
    if !share.is_null() { share = __get_share_config(share); }
    up_read(&raw mut SHARES_TABLE_LOCK);
    if !share.is_null() { share } else { share_config_request(work, name) }
}

unsafe fn ksmbd_share_veto_filename(share: *mut KsmbdShareConfig, filename: *const libc::c_char) -> bool {
    let mut p: *mut KsmbdVetoPattern;
    list_for_each_entry!(p, (*share).veto_list, list, {
        if match_wildcard((*p).pattern, filename) { return true; }
    });
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
