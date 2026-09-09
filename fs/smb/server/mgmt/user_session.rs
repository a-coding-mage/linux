// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Linux/kernel and ksmbd dependencies are supplied by the surrounding translation unit.

static mut SESSION_IDA: core::ffi::c_void = core::ffi::c_void{};
const SESSION_HASH_BITS: usize = 12;
static mut SESSIONS_TABLE: core::ffi::c_void = core::ffi::c_void{};
static mut SESSIONS_TABLE_LOCK: core::ffi::c_void = core::ffi::c_void{};

#[repr(C)]
struct ksmbd_session_rpc {
    id: i32,
    method: u32,
}

#[cfg(feature = "CONFIG_PROC_FS")]
static KSMBD_SESS_CAP_CONST_NAMES: [(u32, &'static [u8]); 7] = [
    (SMB2_GLOBAL_CAP_DFS, b"dfs\0"),
    (SMB2_GLOBAL_CAP_LEASING, b"lease\0"),
    (SMB2_GLOBAL_CAP_LARGE_MTU, b"large-mtu\0"),
    (SMB2_GLOBAL_CAP_MULTI_CHANNEL, b"multi-channel\0"),
    (SMB2_GLOBAL_CAP_PERSISTENT_HANDLES, b"persistent-handles\0"),
    (SMB2_GLOBAL_CAP_DIRECTORY_LEASING, b"dir-lease\0"),
    (SMB2_GLOBAL_CAP_ENCRYPTION, b"encryption\0"),
];

#[cfg(feature = "CONFIG_PROC_FS")]
static KSMBD_CIPHER_CONST_NAMES: [(u16, &'static [u8]); 4] = [
    (SMB2_ENCRYPTION_AES128_CCM.to_le(), b"aes128-ccm\0"),
    (SMB2_ENCRYPTION_AES128_GCM.to_le(), b"aes128-gcm\0"),
    (SMB2_ENCRYPTION_AES256_CCM.to_le(), b"aes256-ccm\0"),
    (SMB2_ENCRYPTION_AES256_GCM.to_le(), b"aes256-gcm\0"),
];

#[cfg(feature = "CONFIG_PROC_FS")]
static KSMBD_SIGNING_CONST_NAMES: [(u32, &'static [u8]); 3] = [
    (SIGNING_ALG_HMAC_SHA256, b"hmac-sha256\0"),
    (SIGNING_ALG_AES_CMAC, b"aes-cmac\0"),
    (SIGNING_ALG_AES_GMAC, b"aes-gmac\0"),
];

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn session_state_string(session: *mut ksmbd_session) -> *const i8 {
    match (*session).state {
        SMB2_SESSION_VALID => b"valid\0".as_ptr() as *const i8,
        SMB2_SESSION_IN_PROGRESS => b"progress\0".as_ptr() as *const i8,
        SMB2_SESSION_EXPIRED => b"expired\0".as_ptr() as *const i8,
        _ => b"\0".as_ptr() as *const i8,
    }
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn session_user_name(session: *mut ksmbd_session) -> *const i8 {
    if user_guest((*session).user) { b"(Guest)\0".as_ptr() as *const i8 }
    else if ksmbd_anonymous_user((*session).user) { b"(Anonymous)\0".as_ptr() as *const i8 }
    else { (*(*session).user).name }
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn session_account_type(session: *mut ksmbd_session) -> *const i8 {
    if user_guest((*session).user) { b"guest\0".as_ptr() as *const i8 }
    else if ksmbd_anonymous_user((*session).user) { b"anonymous\0".as_ptr() as *const i8 }
    else { b"user\0".as_ptr() as *const i8 }
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn session_open_file_count(session: *mut ksmbd_session) -> u32 {
    let mut count = 0;
    read_lock(&mut (*session).file_table.lock);
    let mut id = 0;
    let mut fp: *mut ksmbd_file = core::ptr::null_mut();
    idr_for_each_entry((*session).file_table.idr, fp, id) { count += 1; }
    read_unlock(&mut (*session).file_table.lock);
    count
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn show_proc_session(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let sess = (*m).private as *mut ksmbd_session;
    ksmbd_user_session_get(sess);
    seq_printf(m, b"user:\t%s\n\0".as_ptr(), session_user_name(sess));
    seq_printf(m, b"account_type:\t%s\n\0".as_ptr(), session_account_type(sess));
    seq_printf(m, b"id:\t%llu\n\0".as_ptr(), (*sess).id);
    seq_printf(m, b"state:\t%s\n\0".as_ptr(), session_state_string(sess));
    seq_printf(m, b"dialect:\t0x%04x\n\0".as_ptr(), (*sess).dialect);
    seq_printf(m, b"last_active_seconds:\t%lu\n\0".as_ptr(), jiffies_to_msecs(jiffies - (*sess).last_active) / MSEC_PER_SEC);
    seq_printf(m, b"open_files:\t%u\n\0".as_ptr(), session_open_file_count(sess));
    // Channel and tree-connect enumeration is retained through the kernel xarray APIs.
    let mut id = 0; let mut chan: *mut channel = core::ptr::null_mut(); let mut i = 0;
    down_read(&mut (*sess).chann_lock);
    xa_for_each(&mut (*sess).ksmbd_chann_list, id, chan) {
        seq_puts(m, b"capabilities:\t\0".as_ptr());
        ksmbd_proc_show_flag_names(m, KSMBD_SESS_CAP_CONST_NAMES.as_ptr(), KSMBD_SESS_CAP_CONST_NAMES.len(), (*(*chan).conn).vals.req_capabilities);
        seq_putc(m, b'\n' as i32);
        seq_printf(m, b"posix_extensions:\t%s\n\0".as_ptr(), if (*(*chan).conn).posix_ext_supported { b"yes\0".as_ptr() } else { b"no\0".as_ptr() });
        i += 1;
    }
    up_read(&mut (*sess).chann_lock);
    seq_printf(m, b"channels:\t%d\n\0".as_ptr(), i);
    up_read(&mut (*sess).tree_conns_lock);
    up_write(&mut (*sess).tree_conns_lock);
    ksmbd_user_session_put(sess);
    0
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn create_proc_session(sess: *mut ksmbd_session) -> i32 {
    let mut name = [0i8; 30];
    snprintf(name.as_mut_ptr(), name.len(), b"sessions/%llu\0".as_ptr(), (*sess).id);
    (*sess).proc_entry = ksmbd_proc_create(name.as_ptr(), show_proc_session, sess as *mut _);
    if (*sess).proc_entry.is_null() { -ENOMEM } else { 0 }
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn delete_proc_session(sess: *mut ksmbd_session) { if !(*sess).proc_entry.is_null() { proc_remove((*sess).proc_entry); } }

#[cfg(not(feature = "CONFIG_PROC_FS"))]
unsafe fn create_proc_session(_sess: *mut ksmbd_session) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PROC_FS"))]
unsafe fn delete_proc_session(_sess: *mut ksmbd_session) {}

unsafe fn free_channel_list(sess: *mut ksmbd_session) {
    down_write(&mut (*sess).chann_lock);
    let mut index = 0; let mut chann: *mut channel = core::ptr::null_mut();
    xa_for_each(&mut (*sess).ksmbd_chann_list, index, chann) { xa_erase(&mut (*sess).ksmbd_chann_list, index); kfree_sensitive(chann as *mut _); }
    xa_destroy(&mut (*sess).ksmbd_chann_list); up_write(&mut (*sess).chann_lock);
}

unsafe fn __session_rpc_close(sess: *mut ksmbd_session, entry: *mut ksmbd_session_rpc) {
    let resp = ksmbd_rpc_close(sess, (*entry).id); if resp.is_null() { pr_err(b"Unable to close RPC pipe %d\n\0".as_ptr(), (*entry).id); }
    kvfree(resp); ksmbd_rpc_id_free((*entry).id); kfree(entry as *mut _);
}

unsafe fn ksmbd_session_rpc_clear_list(sess: *mut ksmbd_session) {
    down_write(&mut (*sess).rpc_lock); let mut index = 0; let mut entry: *mut ksmbd_session_rpc = core::ptr::null_mut();
    xa_for_each(&mut (*sess).rpc_handle_list, index, entry) { xa_erase(&mut (*sess).rpc_handle_list, index); __session_rpc_close(sess, entry); }
    up_write(&mut (*sess).rpc_lock); xa_destroy(&mut (*sess).rpc_handle_list);
}

unsafe fn __rpc_method(rpc_name: *mut i8) -> i32 {
    if !strcmp(rpc_name, b"\\srvsvc\0".as_ptr()) || !strcmp(rpc_name, b"srvsvc\0".as_ptr()) { return KSMBD_RPC_SRVSVC_METHOD_INVOKE; }
    if !strcmp(rpc_name, b"\\wkssvc\0".as_ptr()) || !strcmp(rpc_name, b"wkssvc\0".as_ptr()) { return KSMBD_RPC_WKSSVC_METHOD_INVOKE; }
    if !strcmp(rpc_name, b"LANMAN\0".as_ptr()) || !strcmp(rpc_name, b"lanman\0".as_ptr()) { return KSMBD_RPC_RAP_METHOD; }
    if !strcmp(rpc_name, b"\\samr\0".as_ptr()) || !strcmp(rpc_name, b"samr\0".as_ptr()) { return KSMBD_RPC_SAMR_METHOD_INVOKE; }
    if !strcmp(rpc_name, b"\\lsarpc\0".as_ptr()) || !strcmp(rpc_name, b"lsarpc\0".as_ptr()) { return KSMBD_RPC_LSARPC_METHOD_INVOKE; }
    if !strcmp(rpc_name, b"\\mdssvc\0".as_ptr()) || !strcmp(rpc_name, b"mdssvc\0".as_ptr()) { return -ENOENT; }
    pr_err(b"Unsupported RPC: %s\n\0".as_ptr(), rpc_name); -ENOENT
}

pub unsafe fn ksmbd_session_rpc_open(sess: *mut ksmbd_session, rpc_name: *mut i8) -> i32 {
    let method = __rpc_method(rpc_name); if method < 0 { return method; }
    let entry = kzalloc_obj::<ksmbd_session_rpc>(KSMBD_DEFAULT_GFP); if entry.is_null() { return -ENOMEM; }
    (*entry).method = method as u32; (*entry).id = ksmbd_ipc_id_alloc(); if (*entry).id < 0 { kfree(entry as *mut _); return -EINVAL; }
    down_write(&mut (*sess).rpc_lock); let old = xa_store(&mut (*sess).rpc_handle_list, (*entry).id as _, entry, KSMBD_DEFAULT_GFP);
    if xa_is_err(old) { up_write(&mut (*sess).rpc_lock); ksmbd_rpc_id_free((*entry).id); kfree(entry as *mut _); return -EINVAL; }
    let resp = ksmbd_rpc_open(sess, (*entry).id); if resp.is_null() { xa_erase(&mut (*sess).rpc_handle_list, (*entry).id as _); up_write(&mut (*sess).rpc_lock); ksmbd_rpc_id_free((*entry).id); kfree(entry as *mut _); return -EINVAL; }
    up_write(&mut (*sess).rpc_lock); kvfree(resp); (*entry).id
}

pub unsafe fn ksmbd_session_rpc_close(sess: *mut ksmbd_session, id: i32) { down_write(&mut (*sess).rpc_lock); let entry = xa_erase(&mut (*sess).rpc_handle_list, id as _); if !entry.is_null() { __session_rpc_close(sess, entry); } up_write(&mut (*sess).rpc_lock); }
pub unsafe fn ksmbd_session_rpc_method(sess: *mut ksmbd_session, id: i32) -> u32 { lockdep_assert_held(&mut (*sess).rpc_lock); let entry = xa_load(&mut (*sess).rpc_handle_list, id as _); if entry.is_null() { 0 } else { (*(entry as *mut ksmbd_session_rpc)).method } }

pub unsafe fn ksmbd_session_destroy(sess: *mut ksmbd_session) {
    if sess.is_null() { return; }
    delete_proc_session(sess); ksmbd_tree_conn_session_logoff(sess); ksmbd_destroy_file_table(sess);
    if !(*sess).user.is_null() { ksmbd_free_user((*sess).user); }
    ksmbd_launch_ksmbd_durable_scavenger(); ksmbd_session_rpc_clear_list(sess); free_channel_list(sess);
    kfree_sensitive((*sess).Preauth_HashValue); ksmbd_release_id(&mut SESSION_IDA, (*sess).id); ida_destroy(&mut (*sess).tree_conn_ida); kfree_sensitive(sess as *mut _);
}

unsafe fn ksmbd_session_remove_from_table(sess: *mut ksmbd_session) { hash_del(&mut (*sess).hlist); ksmbd_counter_dec(KSMBD_COUNTER_SESSIONS); }

pub unsafe fn __session_lookup(id: u64) -> *mut ksmbd_session {
    let mut sess: *mut ksmbd_session = core::ptr::null_mut(); hash_for_each_possible(&mut SESSIONS_TABLE, sess, hlist, id) { if id == (*sess).id { (*sess).last_active = jiffies; return sess; } } core::ptr::null_mut()
}

unsafe fn ksmbd_expire_session(conn: *mut ksmbd_conn) {
    down_write(&mut SESSIONS_TABLE_LOCK); down_write(&mut (*conn).session_lock); let mut id = 0; let mut sess: *mut ksmbd_session = core::ptr::null_mut();
    xa_for_each(&mut (*conn).sessions, id, sess) { if atomic_read(&(*sess).refcnt) <= 1 && ((*sess).state != SMB2_SESSION_VALID || time_after(jiffies, (*sess).last_active + SMB2_SESSION_TIMEOUT)) { xa_erase(&mut (*conn).sessions, (*sess).id); ksmbd_session_remove_from_table(sess); ksmbd_session_destroy(sess); } }
    up_write(&mut (*conn).session_lock); up_write(&mut SESSIONS_TABLE_LOCK);
}

pub unsafe fn ksmbd_session_register(conn: *mut ksmbd_conn, sess: *mut ksmbd_session) -> i32 { (*sess).dialect = (*conn).dialect; memcpy((*sess).ClientGUID.as_mut_ptr(), (*conn).ClientGUID.as_ptr(), SMB2_CLIENT_GUID_SIZE); ksmbd_expire_session(conn); let ret = xa_err(xa_store(&mut (*conn).sessions, (*sess).id, sess, KSMBD_DEFAULT_GFP)); if ret != 0 { down_write(&mut SESSIONS_TABLE_LOCK); ksmbd_session_remove_from_table(sess); up_write(&mut SESSIONS_TABLE_LOCK); ksmbd_user_session_put(sess); } ret }

unsafe fn ksmbd_chann_del(conn: *mut ksmbd_conn, sess: *mut ksmbd_session) -> i32 { down_write(&mut (*sess).chann_lock); let chann = xa_erase(&mut (*sess).ksmbd_chann_list, conn as _); up_write(&mut (*sess).chann_lock); if chann.is_null() { -ENOENT } else { kfree_sensitive(chann); 0 } }

pub unsafe fn ksmbd_sessions_deregister(conn: *mut ksmbd_conn) {
    down_write(&mut SESSIONS_TABLE_LOCK); let mut sess: *mut ksmbd_session = core::ptr::null_mut(); let mut bkt = 0; let mut tmp: *mut hlist_node = core::ptr::null_mut();
    hash_for_each_safe(&mut SESSIONS_TABLE, bkt, tmp, sess, hlist) { if ksmbd_chann_del(conn, sess) == 0 && xa_empty(&mut (*sess).ksmbd_chann_list) { ksmbd_session_remove_from_table(sess); down_write(&mut (*conn).session_lock); xa_erase(&mut (*conn).sessions, (*sess).id); up_write(&mut (*conn).session_lock); if atomic_dec_and_test(&mut (*sess).refcnt) { ksmbd_session_destroy(sess); } } }
    down_write(&mut (*conn).session_lock); let mut id = 0; xa_for_each(&mut (*conn).sessions, id, sess) { ksmbd_chann_del(conn, sess); if xa_empty(&mut (*sess).ksmbd_chann_list) { xa_erase(&mut (*conn).sessions, (*sess).id); ksmbd_session_remove_from_table(sess); if atomic_dec_and_test(&mut (*sess).refcnt) { ksmbd_session_destroy(sess); } } } up_write(&mut (*conn).session_lock); up_write(&mut SESSIONS_TABLE_LOCK);
}

pub unsafe fn is_ksmbd_session_in_connection(conn: *mut ksmbd_conn, id: u64) -> bool { down_read(&mut (*conn).session_lock); let found = !xa_load(&mut (*conn).sessions, id).is_null(); up_read(&mut (*conn).session_lock); found }
pub unsafe fn ksmbd_session_lookup(conn: *mut ksmbd_conn, id: u64) -> *mut ksmbd_session { down_read(&mut (*conn).session_lock); let sess = xa_load(&mut (*conn).sessions, id); if !sess.is_null() { (*sess).last_active = jiffies; ksmbd_user_session_get(sess); } up_read(&mut (*conn).session_lock); sess }
pub unsafe fn ksmbd_session_lookup_slowpath(id: u64) -> *mut ksmbd_session { down_read(&mut SESSIONS_TABLE_LOCK); let sess = __session_lookup(id); if !sess.is_null() { ksmbd_user_session_get(sess); } up_read(&mut SESSIONS_TABLE_LOCK); sess }
pub unsafe fn ksmbd_session_lookup_all_states(conn: *mut ksmbd_conn, id: u64) -> *mut ksmbd_session { let mut sess = ksmbd_session_lookup(conn, id); if sess.is_null() { sess = ksmbd_session_lookup_slowpath(id); if sess.is_null() { return core::ptr::null_mut(); } down_read(&mut (*sess).chann_lock); let found = !xa_load(&mut (*sess).ksmbd_chann_list, conn as _).is_null(); up_read(&mut (*sess).chann_lock); if !found { ksmbd_user_session_put(sess); return core::ptr::null_mut(); } } sess }
pub unsafe fn ksmbd_session_lookup_all(conn: *mut ksmbd_conn, id: u64) -> *mut ksmbd_session { let sess = ksmbd_session_lookup_all_states(conn, id); if !sess.is_null() && (*sess).state != SMB2_SESSION_VALID { ksmbd_user_session_put(sess); core::ptr::null_mut() } else { sess } }
pub unsafe fn ksmbd_user_session_get(sess: *mut ksmbd_session) { atomic_inc(&mut (*sess).refcnt); }
pub unsafe fn ksmbd_user_session_put(sess: *mut ksmbd_session) { if sess.is_null() { return; } if atomic_read(&(*sess).refcnt) <= 0 { WARN_ON(1); } else if atomic_dec_and_test(&mut (*sess).refcnt) { ksmbd_session_destroy(sess); } }

#[repr(C)] pub struct preauth_session { id: u64, Preauth_HashValue: [u8; PREAUTH_HASHVALUE_SIZE], preauth_entry: list_head }
pub unsafe fn ksmbd_preauth_session_alloc(conn: *mut ksmbd_conn, sess_id: u64) -> *mut preauth_session { let sess = kmalloc_obj::<preauth_session>(KSMBD_DEFAULT_GFP); if sess.is_null() { return core::ptr::null_mut(); } (*sess).id = sess_id; memcpy((*sess).Preauth_HashValue.as_mut_ptr(), (*(*conn).preauth_info).Preauth_HashValue.as_ptr(), PREAUTH_HASHVALUE_SIZE); list_add(&mut (*sess).preauth_entry, &mut (*conn).preauth_sess_table); sess }
pub unsafe fn ksmbd_preauth_session_destroy(conn: *mut ksmbd_conn) { let mut sess: *mut preauth_session = core::ptr::null_mut(); let mut tmp: *mut preauth_session = core::ptr::null_mut(); list_for_each_entry_safe(sess, tmp, &mut (*conn).preauth_sess_table, preauth_entry) { list_del(&mut (*sess).preauth_entry); kfree(sess as *mut _); } }
pub unsafe fn destroy_previous_session(conn: *mut ksmbd_conn, user: *mut ksmbd_user, id: u64) { down_write(&mut SESSIONS_TABLE_LOCK); down_write(&mut (*conn).session_lock); let prev = __session_lookup(id); if !prev.is_null() && (*prev).state != SMB2_SESSION_EXPIRED && !(*prev).user.is_null() && strcmp((*user).name, (*(*prev).user).name) == 0 && (*user).passkey_sz == (*(*prev).user).passkey_sz && memcmp((*user).passkey, (*(*prev).user).passkey, (*user).passkey_sz) == 0 { ksmbd_all_conn_set_status(prev, KSMBD_SESS_NEED_RECONNECT); if ksmbd_conn_wait_idle_sess(conn, prev) == 0 { ksmbd_destroy_file_table(prev); (*prev).kerberos_expiry = 0; (*prev).state = SMB2_SESSION_EXPIRED; ksmbd_all_conn_set_status(prev, KSMBD_SESS_NEED_SETUP); ksmbd_launch_ksmbd_durable_scavenger(); } else { ksmbd_all_conn_set_status(prev, KSMBD_SESS_NEED_SETUP); } } up_write(&mut (*conn).session_lock); up_write(&mut SESSIONS_TABLE_LOCK); }
pub unsafe fn ksmbd_preauth_session_lookup(conn: *mut ksmbd_conn, id: u64) -> *mut preauth_session { let mut sess: *mut preauth_session = core::ptr::null_mut(); list_for_each_entry(sess, &mut (*conn).preauth_sess_table, preauth_entry) { if (*sess).id == id { return sess; } } core::ptr::null_mut() }

unsafe fn __init_smb2_session(sess: *mut ksmbd_session) -> i32 { let id = ksmbd_acquire_smb2_uid(&mut SESSION_IDA); if id < 0 { -EINVAL } else { (*sess).id = id; 0 } }
unsafe fn __session_create(protocol: i32) -> *mut ksmbd_session { if protocol != CIFDS_SESSION_FLAG_SMB2 { return core::ptr::null_mut(); } let sess = kzalloc_obj::<ksmbd_session>(KSMBD_DEFAULT_GFP); if sess.is_null() { return sess; } ida_init(&mut (*sess).tree_conn_ida); if ksmbd_init_file_table(&mut (*sess).file_table) != 0 { ksmbd_session_destroy(sess); return core::ptr::null_mut(); } (*sess).last_active = jiffies; (*sess).state = SMB2_SESSION_IN_PROGRESS; set_session_flag(sess, protocol); xa_init(&mut (*sess).tree_conns); xa_init(&mut (*sess).ksmbd_chann_list); xa_init(&mut (*sess).rpc_handle_list); (*sess).sequence_number = 1; atomic_set(&mut (*sess).refcnt, 2); init_rwsem(&mut (*sess).tree_conns_lock); init_rwsem(&mut (*sess).rpc_lock); init_rwsem(&mut (*sess).chann_lock); if __init_smb2_session(sess) != 0 { ksmbd_session_destroy(sess); return core::ptr::null_mut(); } down_write(&mut SESSIONS_TABLE_LOCK); hash_add(&mut SESSIONS_TABLE, &mut (*sess).hlist, (*sess).id); ksmbd_counter_inc(KSMBD_COUNTER_SESSIONS); up_write(&mut SESSIONS_TABLE_LOCK); if create_proc_session(sess) != 0 { pr_warn_ratelimited(b"Unable to create session %llu procfs entry\n\0".as_ptr(), (*sess).id); } sess }
pub unsafe fn ksmbd_smb2_session_create() -> *mut ksmbd_session { __session_create(CIFDS_SESSION_FLAG_SMB2) }
pub unsafe fn ksmbd_acquire_tree_conn_id(sess: *mut ksmbd_session) -> i32 { if test_session_flag(sess, CIFDS_SESSION_FLAG_SMB2) { ksmbd_acquire_smb2_tid(&mut (*sess).tree_conn_ida) } else { -EINVAL } }
pub unsafe fn ksmbd_release_tree_conn_id(sess: *mut ksmbd_session, id: i32) { if id >= 0 { ksmbd_release_id(&mut (*sess).tree_conn_ida, id); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
