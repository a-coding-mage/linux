/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Authentication token and access key management */

/* C header dependencies are supplied by other translated headers. */

#[cfg(any())]
const __KERNEL__: () = ();

pub type KeySerialT = i32;
pub type KeyPermT = u32;

#[repr(C)] pub struct key;
#[repr(C)] pub struct net;
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct user_struct;
#[repr(C)] pub struct signal_struct;
#[repr(C)] pub struct cred;
#[repr(C)] pub struct key_type;
#[repr(C)] pub struct key_owner;
#[repr(C)] pub struct keyring_list;
#[repr(C)] pub struct keyring_name;
#[repr(C)] pub struct user_namespace;
#[repr(C)] pub struct key_user;
#[repr(C)] pub struct watch_list;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct rb_node;
#[repr(C)] pub struct rw_semaphore;
#[repr(C)] pub struct assoc_array;
#[repr(C)] pub struct rcu_head;
#[repr(C)] pub struct refcount_t;

pub const KEY_POS_VIEW: u32 = 0x01000000;
pub const KEY_POS_READ: u32 = 0x02000000;
pub const KEY_POS_WRITE: u32 = 0x04000000;
pub const KEY_POS_SEARCH: u32 = 0x08000000;
pub const KEY_POS_LINK: u32 = 0x10000000;
pub const KEY_POS_SETATTR: u32 = 0x20000000;
pub const KEY_POS_ALL: u32 = 0x3f000000;
pub const KEY_USR_VIEW: u32 = 0x00010000;
pub const KEY_USR_READ: u32 = 0x00020000;
pub const KEY_USR_WRITE: u32 = 0x00040000;
pub const KEY_USR_SEARCH: u32 = 0x00080000;
pub const KEY_USR_LINK: u32 = 0x00100000;
pub const KEY_USR_SETATTR: u32 = 0x00200000;
pub const KEY_USR_ALL: u32 = 0x003f0000;
pub const KEY_GRP_VIEW: u32 = 0x00000100;
pub const KEY_GRP_READ: u32 = 0x00000200;
pub const KEY_GRP_WRITE: u32 = 0x00000400;
pub const KEY_GRP_SEARCH: u32 = 0x00000800;
pub const KEY_GRP_LINK: u32 = 0x00001000;
pub const KEY_GRP_SETATTR: u32 = 0x00002000;
pub const KEY_GRP_ALL: u32 = 0x00003f00;
pub const KEY_OTH_VIEW: u32 = 0x00000001;
pub const KEY_OTH_READ: u32 = 0x00000002;
pub const KEY_OTH_WRITE: u32 = 0x00000004;
pub const KEY_OTH_SEARCH: u32 = 0x00000008;
pub const KEY_OTH_LINK: u32 = 0x00000010;
pub const KEY_OTH_SETATTR: u32 = 0x00000020;
pub const KEY_OTH_ALL: u32 = 0x0000003f;
pub const KEY_PERM_UNDEF: u32 = 0xffffffff;

#[repr(C)] #[derive(Copy, Clone)] pub enum key_need_perm { KEY_NEED_UNSPECIFIED, KEY_NEED_VIEW, KEY_NEED_READ, KEY_NEED_WRITE, KEY_NEED_SEARCH, KEY_NEED_LINK, KEY_NEED_SETATTR, KEY_NEED_UNLINK, KEY_SYSADMIN_OVERRIDE, KEY_AUTHTOKEN_OVERRIDE, KEY_DEFER_PERM_CHECK }
#[repr(C)] #[derive(Copy, Clone)] pub enum key_lookup_flag { KEY_LOOKUP_CREATE = 1, KEY_LOOKUP_PARTIAL = 2, KEY_LOOKUP_ALL = 3 }

#[repr(C)] pub struct key_tag { pub rcu: rcu_head, pub usage: refcount_t, pub removed: bool }
#[repr(C)] pub struct keyring_index_key { pub hash: usize, pub x: usize, pub type_: *mut key_type, pub domain_tag: *mut key_tag, pub description: *const i8 }
#[repr(C)] pub union key_payload { pub rcu_data0: *mut core::ffi::c_void, pub data: [*mut core::ffi::c_void; 4] }
pub type key_ref_t = *mut __key_reference_with_attributes;
#[repr(C)] pub struct __key_reference_with_attributes;

#[inline] pub unsafe fn make_key_ref(key: *const key, possession: bool) -> key_ref_t { ((key as usize) | possession as usize) as key_ref_t }
#[inline] pub unsafe fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key { ((key_ref as usize) & !1usize) as *mut key }
#[inline] pub unsafe fn is_key_possessed(key_ref: key_ref_t) -> bool { (key_ref as usize & 1) != 0 }

pub type key_restrict_link_func_t = unsafe extern "C" fn(*mut key, *const key_type, *const key_payload, *mut key) -> i32;
#[repr(C)] pub struct key_restriction { pub check: Option<key_restrict_link_func_t>, pub key: *mut key, pub keytype: *mut key_type }
#[repr(C)] #[derive(Copy, Clone)] pub enum key_state { KEY_IS_UNINSTANTIATED, KEY_IS_POSITIVE }

#[repr(C)] pub struct key {
    pub usage: refcount_t, pub serial: KeySerialT,
    pub link: key_link_union, pub sem: rw_semaphore, pub user: *mut key_user, pub security: *mut core::ffi::c_void,
    pub time: key_time_union, pub last_used_at: i64, pub uid: u32, pub gid: u32, pub perm: KeyPermT,
    pub quotalen: u16, pub datalen: u16, pub state: i16, pub flags: usize, pub description_data: key_description_union,
    pub payload_data: key_payload_union, pub restrict_link: *mut key_restriction,
}
#[repr(C)] pub union key_link_union { pub graveyard_link: list_head, pub serial_node: rb_node }
#[repr(C)] pub union key_time_union { pub expiry: i64, pub revoked_at: i64 }
#[repr(C)] pub union key_description_union { pub index_key: keyring_index_key, pub fields: key_description_fields }
#[repr(C)] pub struct key_description_fields { pub hash: usize, pub len_desc: usize, pub type_: *mut key_type, pub domain_tag: *mut key_tag, pub description: *mut i8 }
#[repr(C)] pub union key_payload_union { pub payload: key_payload, pub keyring: keyring_data }
#[repr(C)] pub struct keyring_data { pub name_link: list_head, pub keys: assoc_array }

pub const KEY_ALLOC_IN_QUOTA: u32 = 0; pub const KEY_ALLOC_QUOTA_OVERRUN: u32 = 1; pub const KEY_ALLOC_NOT_IN_QUOTA: u32 = 2;
pub const KEY_ALLOC_BUILT_IN: u32 = 4; pub const KEY_ALLOC_BYPASS_RESTRICTION: u32 = 8; pub const KEY_ALLOC_UID_KEYRING: u32 = 16; pub const KEY_ALLOC_SET_KEEP: u32 = 32;

extern "C" {
    pub fn key_alloc(type_: *mut key_type, desc: *const i8, uid: u32, gid: u32, cred: *const cred, perm: KeyPermT, flags: usize, restrict_link: *mut key_restriction) -> *mut key;
    pub fn key_revoke(key: *mut key); pub fn key_invalidate(key: *mut key); pub fn key_put(key: *mut key);
    pub fn key_put_tag(tag: *mut key_tag) -> bool; pub fn key_remove_domain(tag: *mut key_tag);
    pub fn request_key_tag(type_: *mut key_type, description: *const i8, domain_tag: *mut key_tag, callout_info: *const i8) -> *mut key;
    pub fn request_key_rcu(type_: *mut key_type, description: *const i8, domain_tag: *mut key_tag) -> *mut key;
    pub fn request_key_with_auxdata(type_: *mut key_type, description: *const i8, domain_tag: *mut key_tag, callout_info: *const core::ffi::c_void, callout_len: usize, aux: *mut core::ffi::c_void) -> *mut key;
    pub fn wait_for_key_construction(key: *mut key, intr: bool) -> i32;
    pub fn key_validate(key: *const key) -> i32;
    pub fn key_create(keyring: key_ref_t, type_: *const i8, description: *const i8, payload: *const core::ffi::c_void, plen: usize, perm: KeyPermT, flags: usize) -> key_ref_t;
    pub fn key_create_or_update(keyring: key_ref_t, type_: *const i8, description: *const i8, payload: *const core::ffi::c_void, plen: usize, perm: KeyPermT, flags: usize) -> key_ref_t;
    pub fn key_update(key: key_ref_t, payload: *const core::ffi::c_void, plen: usize) -> i32;
    pub fn key_link(keyring: *mut key, key: *mut key) -> i32; pub fn key_move(key: *mut key, from: *mut key, to: *mut key, flags: u32) -> i32;
    pub fn key_unlink(keyring: *mut key, key: *mut key) -> i32;
    pub fn keyring_alloc(description: *const i8, uid: u32, gid: u32, cred: *const cred, perm: KeyPermT, flags: usize, restrict_link: *mut key_restriction, dest: *mut key) -> *mut key;
    pub fn restrict_link_reject(keyring: *mut key, type_: *const key_type, payload: *const key_payload, restriction_key: *mut key) -> i32;
    pub fn keyring_clear(keyring: *mut key) -> i32; pub fn keyring_search(keyring: key_ref_t, type_: *mut key_type, description: *const i8, recurse: bool) -> key_ref_t;
    pub fn keyring_restrict(keyring: key_ref_t, type_: *const i8, restriction: *const i8) -> i32;
    pub fn key_lookup(id: KeySerialT) -> *mut key; pub fn key_set_timeout(key: *mut key, timeout: u32);
    pub fn lookup_user_key(id: KeySerialT, flags: usize, need_perm: key_need_perm) -> key_ref_t;
    pub fn key_free_user_ns(ns: *mut user_namespace);
}

#[inline] pub unsafe fn __key_get(k: *mut key) -> *mut key { k }
#[inline] pub unsafe fn key_get(k: *mut key) -> *mut key { k }
#[inline] pub unsafe fn key_ref_put(r: key_ref_t) { key_put(key_ref_to_ptr(r)); }
#[inline] pub unsafe fn request_key(type_: *mut key_type, description: *const i8, callout_info: *const i8) -> *mut key { request_key_tag(type_, description, core::ptr::null_mut(), callout_info) }
#[inline] pub unsafe fn key_serial(k: *const key) -> KeySerialT { if k.is_null() { 0 } else { (*k).serial } }
#[inline] pub unsafe fn key_read_state(k: *const key) -> i16 { (*k).state }
#[inline] pub unsafe fn key_is_positive(k: *const key) -> bool { key_read_state(k) == KEY_IS_POSITIVE as i16 }
#[inline] pub unsafe fn key_is_negative(k: *const key) -> bool { key_read_state(k) < 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
