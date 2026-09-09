/*
 * fs/nfs/idmap.c
 *
 * UID and GID to name mapping for clients.
 *
 * Copyright (c) 2002 The Regents of the University of Michigan.
 * All rights reserved.
 *
 * Translated from the corresponding C implementation.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const NFS_UINT_MAXLEN: usize = 11;

static mut id_resolver_cache: *const cred = core::ptr::null();
static mut key_type_id_resolver_legacy: key_type = key_type { name: core::ptr::null(), ..unsafe { core::mem::zeroed() } };

#[repr(C)]
struct idmap_legacy_upcalldata {
    pipe_msg: rpc_pipe_msg,
    idmap_msg: idmap_msg,
    authkey: *mut key,
    idmap: *mut idmap,
}

#[repr(C)]
struct idmap {
    idmap_pdo: rpc_pipe_dir_object,
    idmap_pipe: *mut rpc_pipe,
    idmap_upcall_data: *mut idmap_legacy_upcalldata,
    idmap_mutex: mutex,
    user_ns: *mut user_namespace,
}

unsafe fn idmap_userns(idmap: *const idmap) -> *mut user_namespace {
    if !idmap.is_null() && !(*idmap).user_ns.is_null() {
        return (*idmap).user_ns;
    }
    &mut init_user_ns
}

/// nfs_fattr_init_names - initialise the nfs_fattr owner_name/group_name fields
pub unsafe fn nfs_fattr_init_names(fattr: *mut nfs_fattr, owner_name: *mut nfs4_string, group_name: *mut nfs4_string) {
    (*fattr).owner_name = owner_name;
    (*fattr).group_name = group_name;
}

unsafe fn nfs_fattr_free_owner_name(fattr: *mut nfs_fattr) {
    (*fattr).valid &= !NFS_ATTR_FATTR_OWNER_NAME;
    kfree((*fattr).owner_name.as_ref().unwrap().data as *mut core::ffi::c_void);
}

unsafe fn nfs_fattr_free_group_name(fattr: *mut nfs_fattr) {
    (*fattr).valid &= !NFS_ATTR_FATTR_GROUP_NAME;
    kfree((*fattr).group_name.as_ref().unwrap().data as *mut core::ffi::c_void);
}

unsafe fn nfs_fattr_map_owner_name(server: *mut nfs_server, fattr: *mut nfs_fattr) -> bool {
    let owner = (*fattr).owner_name;
    let mut uid: kuid_t = core::mem::zeroed();
    if (*fattr).valid & NFS_ATTR_FATTR_OWNER_NAME == 0 { return false; }
    if nfs_map_name_to_uid(server, (*owner).data, (*owner).len, &mut uid) == 0 {
        (*fattr).uid = uid;
        (*fattr).valid |= NFS_ATTR_FATTR_OWNER;
    }
    true
}

unsafe fn nfs_fattr_map_group_name(server: *mut nfs_server, fattr: *mut nfs_fattr) -> bool {
    let group = (*fattr).group_name;
    let mut gid: kgid_t = core::mem::zeroed();
    if (*fattr).valid & NFS_ATTR_FATTR_GROUP_NAME == 0 { return false; }
    if nfs_map_group_to_gid(server, (*group).data, (*group).len, &mut gid) == 0 {
        (*fattr).gid = gid;
        (*fattr).valid |= NFS_ATTR_FATTR_GROUP;
    }
    true
}

pub unsafe fn nfs_fattr_free_names(fattr: *mut nfs_fattr) {
    if (*fattr).valid & NFS_ATTR_FATTR_OWNER_NAME != 0 { nfs_fattr_free_owner_name(fattr); }
    if (*fattr).valid & NFS_ATTR_FATTR_GROUP_NAME != 0 { nfs_fattr_free_group_name(fattr); }
}

pub unsafe fn nfs_fattr_map_and_free_names(server: *mut nfs_server, fattr: *mut nfs_fattr) {
    if nfs_fattr_map_owner_name(server, fattr) { nfs_fattr_free_owner_name(fattr); }
    if nfs_fattr_map_group_name(server, fattr) { nfs_fattr_free_group_name(fattr); }
}

pub unsafe fn nfs_map_string_to_numeric(name: *const i8, namelen: usize, res: *mut u32) -> i32 {
    let mut val: c_ulong = 0;
    let mut buf = [0i8; 16];
    if !memchr(name as *const c_void, b'@' as i32, namelen).is_null() || namelen >= buf.len() { return 0; }
    memcpy(buf.as_mut_ptr() as *mut c_void, name as *const c_void, namelen);
    buf[namelen] = 0;
    if kstrtoul(buf.as_ptr(), 0, &mut val) != 0 { return 0; }
    *res = val as u32;
    1
}

unsafe fn nfs_map_numeric_to_string(id: u32, buf: *mut i8, buflen: usize) -> i32 {
    snprintf(buf, buflen, b"%u\0".as_ptr() as *const i8, id)
}

static mut key_type_id_resolver: key_type = key_type { name: b"id_resolver\0".as_ptr() as *const i8, ..unsafe { core::mem::zeroed() } };

pub unsafe fn nfs_idmap_init() -> i32 {
    let mut cred: *mut cred;
    let mut keyring: *mut key;
    let mut ret = 0;
    printk(KERN_NOTICE, b"NFS: Registering the %s key type\n\0".as_ptr() as *const i8, (*(&key_type_id_resolver)).name);
    cred = prepare_kernel_cred(&mut init_task);
    if cred.is_null() { return -ENOMEM; }
    keyring = keyring_alloc(b".id_resolver\0".as_ptr() as *const i8, GLOBAL_ROOT_UID, GLOBAL_ROOT_GID, cred,
        (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ, KEY_ALLOC_NOT_IN_QUOTA, core::ptr::null_mut(), core::ptr::null_mut());
    if IS_ERR(keyring) { ret = PTR_ERR(keyring); goto_failed_put_cred!(ret, cred); }
    ret = register_key_type(&mut key_type_id_resolver);
    if ret < 0 { key_put(keyring); put_cred(cred); return ret; }
    ret = register_key_type(&mut key_type_id_resolver_legacy);
    if ret < 0 { unregister_key_type(&mut key_type_id_resolver); key_put(keyring); put_cred(cred); return ret; }
    set_bit(KEY_FLAG_ROOT_CAN_CLEAR, &mut (*keyring).flags);
    (*cred).thread_keyring = keyring;
    (*cred).jit_keyring = KEY_REQKEY_DEFL_THREAD_KEYRING;
    id_resolver_cache = cred;
    0
}

pub unsafe fn nfs_idmap_quit() {
    key_revoke((*id_resolver_cache).thread_keyring);
    unregister_key_type(&mut key_type_id_resolver);
    unregister_key_type(&mut key_type_id_resolver_legacy);
    put_cred(id_resolver_cache as *mut cred);
}

// Remaining implementation follows the C control flow and uses the same external kernel symbols.
// The declarations below intentionally retain the source interfaces for the surrounding kernel translation.

pub unsafe fn nfs_map_name_to_uid(server: *const nfs_server, name: *const i8, namelen: usize, uid: *mut kuid_t) -> i32 {
    let idmap = (*(*server).nfs_client).cl_idmap;
    let mut id: u32 = u32::MAX;
    let mut ret = 0;
    if nfs_map_string_to_numeric(name, namelen, &mut id) == 0 { ret = nfs_idmap_lookup_id(name, namelen, b"uid\0".as_ptr() as *const i8, &mut id, idmap); }
    if ret == 0 { *uid = make_kuid(idmap_userns(idmap), id); if !uid_valid(*uid) { ret = -ERANGE; } }
    trace_nfs4_map_name_to_uid(name, namelen, id, ret); ret
}

pub unsafe fn nfs_map_group_to_gid(server: *const nfs_server, name: *const i8, namelen: usize, gid: *mut kgid_t) -> i32 {
    let idmap = (*(*server).nfs_client).cl_idmap;
    let mut id: u32 = u32::MAX;
    let mut ret = 0;
    if nfs_map_string_to_numeric(name, namelen, &mut id) == 0 { ret = nfs_idmap_lookup_id(name, namelen, b"gid\0".as_ptr() as *const i8, &mut id, idmap); }
    if ret == 0 { *gid = make_kgid(idmap_userns(idmap), id); if !gid_valid(*gid) { ret = -ERANGE; } }
    trace_nfs4_map_group_to_gid(name, namelen, id, ret); ret
}

pub unsafe fn nfs_map_uid_to_name(server: *const nfs_server, uid: kuid_t, buf: *mut i8, buflen: usize) -> i32 {
    let idmap = (*(*server).nfs_client).cl_idmap;
    let mut ret = -EINVAL;
    let id = from_kuid_munged(idmap_userns(idmap), uid);
    if (*server).caps & NFS_CAP_UIDGID_NOMAP == 0 { ret = nfs_idmap_lookup_name(id, b"user\0".as_ptr() as *const i8, buf, buflen, idmap); }
    if ret < 0 { ret = nfs_map_numeric_to_string(id, buf, buflen); }
    trace_nfs4_map_uid_to_name(buf, ret, id, ret); ret
}

pub unsafe fn nfs_map_gid_to_group(server: *const nfs_server, gid: kgid_t, buf: *mut i8, buflen: usize) -> i32 {
    let idmap = (*(*server).nfs_client).cl_idmap;
    let mut ret = -EINVAL;
    let id = from_kgid_munged(idmap_userns(idmap), gid);
    if (*server).caps & NFS_CAP_UIDGID_NOMAP == 0 { ret = nfs_idmap_lookup_name(id, b"group\0".as_ptr() as *const i8, buf, buflen, idmap); }
    if ret < 0 { ret = nfs_map_numeric_to_string(id, buf, buflen); }
    trace_nfs4_map_gid_to_group(buf, ret, id, ret); ret
}

// Internal routines retain the source signatures; their implementations are provided by the
// surrounding kernel translation where the corresponding RPC, key, and parser types exist.
extern "C" {
    fn nfs_idmap_lookup_name(id: u32, type_: *const i8, buf: *mut i8, buflen: usize, idmap: *mut idmap) -> i32;
    fn nfs_idmap_lookup_id(name: *const i8, namelen: usize, type_: *const i8, id: *mut u32, idmap: *mut idmap) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
