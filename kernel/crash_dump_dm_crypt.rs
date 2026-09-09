// SPDX-License-Identifier: GPL-2.0-only
// Kernel header dependencies are supplied by the surrounding build.

use core::ffi::{c_char, c_int, c_void};

const KEY_NUM_MAX: u32 = 128;
const KEY_SIZE_MAX: usize = 256;
const KEY_DESC_MAX_LEN: usize = 128;

static mut KEY_COUNT: u32 = 0;

#[repr(C)]
pub struct dm_crypt_key {
    pub key_size: u32,
    pub key_desc: [c_char; KEY_DESC_MAX_LEN],
    pub data: [u8; KEY_SIZE_MAX],
}

#[repr(C)]
pub struct keys_header {
    pub total_keys: u32,
    pub keys: [dm_crypt_key; 0],
}

static mut KEYS_HEADER: *mut keys_header = core::ptr::null_mut();
pub static mut dm_crypt_keys_addr: u64 = 0;

#[repr(C)] pub struct key_ref_t { _private: [u8; 0] }
#[repr(C)] pub struct key { _private: [u8; 0] }
#[repr(C)] pub struct user_key_payload { pub datalen: u32, pub data: [u8; 0] }
#[repr(C)] pub struct config_item { _private: [u8; 0] }
#[repr(C)] pub struct config_group { _private: [u8; 0] }
#[repr(C)] pub struct configfs_attribute { _private: [u8; 0] }
#[repr(C)] pub struct config_item_type { _private: [u8; 0] }
#[repr(C)] pub struct configfs_subsystem { _private: [u8; 0] }
#[repr(C)] pub struct kimage { pub dm_crypt_keys_addr: u64, pub dm_crypt_keys_sz: usize }
#[repr(C)] pub struct kexec_buf { pub image: *mut kimage, pub buffer: *mut c_void, pub bufsz: usize, pub memsz: usize, pub buf_min: u64, pub buf_max: u64, pub top_down: bool, pub random: bool, pub buf_align: usize, pub mem: u64 }

extern "C" {
    fn memparse(s: *mut c_char, end: *mut *mut c_char) -> u64;
    fn read_from_oldmem(iter: *mut c_void, count: usize, pos: *mut u64, encrypted: bool) -> isize;
    fn cc_platform_has(attr: c_int) -> bool;
    fn lookup_user_key(spec: c_int, perm: c_int, need: c_int) -> key_ref_t;
    fn key_create_or_update(ring: key_ref_t, typ: *const c_char, desc: *const c_char, data: *const u8, len: u32, perm: u32, flags: u32) -> key_ref_t;
    fn key_ref_to_ptr(r: key_ref_t) -> *mut key;
    fn key_ref_put(r: key_ref_t);
    fn key_put(k: *mut key);
    fn request_key(typ: *const c_void, desc: *const c_char, callout: *const c_char) -> *mut key;
    fn down_read(sem: *mut c_void);
    fn up_read(sem: *mut c_void);
    fn user_key_payload_locked(k: *mut key) -> *const user_key_payload;
    fn kmap_local_page(page: *mut c_void) -> *mut c_void;
    fn kunmap_local(addr: *mut c_void);
    fn pfn_to_page(pfn: u64) -> *mut c_void;
    fn arch_kexec_unprotect_crashkres();
    fn arch_kexec_protect_crashkres();
    fn kexec_add_buffer(buf: *mut kexec_buf) -> c_int;
    fn configfs_register_subsystem(s: *mut configfs_subsystem) -> c_int;
    fn configfs_unregister_subsystem(s: *mut configfs_subsystem);
    fn config_group_init(g: *mut config_group);
    fn mutex_init(m: *mut c_void);
    fn is_kdump_kernel() -> bool;
}

unsafe fn get_keys_header_size(total_keys: usize) -> usize {
    core::mem::size_of::<keys_header>() + total_keys * core::mem::size_of::<dm_crypt_key>()
}

#[no_mangle]
pub unsafe extern "C" fn setup_dmcryptkeys(arg: *mut c_char) -> c_int {
    if arg.is_null() { return -22; }
    let mut end = core::ptr::null_mut();
    dm_crypt_keys_addr = memparse(arg, &mut end);
    if end > arg { return 0; }
    dm_crypt_keys_addr = 0;
    -22
}

pub unsafe extern "C" fn dm_crypt_keys_read(buf: *mut c_char, count: usize, ppos: *mut u64) -> isize {
    read_from_oldmem(core::ptr::null_mut(), count, ppos, cc_platform_has(0))
}

unsafe fn add_key_to_keyring(dm_key: *mut dm_crypt_key, keyring_ref: key_ref_t) -> c_int {
    let key_ref = key_create_or_update(keyring_ref, b"user\0".as_ptr() as *const c_char,
        (*dm_key).key_desc.as_ptr(), (*dm_key).data.as_ptr(), (*dm_key).key_size, 0, 0);
    let r;
    if !core::ptr::eq(&key_ref as *const _, core::ptr::null()) {
        r = 0;
        key_ref_put(key_ref);
    } else { r = -1; }
    r
}

unsafe fn get_keys_from_kdump_reserved_memory() {
    arch_kexec_unprotect_crashkres();
    let loaded = kmap_local_page(pfn_to_page(dm_crypt_keys_addr >> 12));
    core::ptr::copy_nonoverlapping(loaded as *const u8, KEYS_HEADER as *mut u8,
        get_keys_header_size(KEY_COUNT as usize));
    kunmap_local(loaded);
    arch_kexec_protect_crashkres();
}

unsafe fn restore_dm_crypt_keys_to_thread_keyring() -> c_int {
    let keyring_ref = lookup_user_key(0, 1, 1);
    let mut ret = 0;
    if KEY_COUNT > KEY_NUM_MAX { ret = -1; }
    let size = get_keys_header_size(KEY_COUNT as usize);
    let _ = size;
    key_ref_put(keyring_ref);
    ret
}

unsafe fn read_key_from_user_keyring(dm_key: *mut dm_crypt_key) -> c_int {
    let key = request_key(core::ptr::null(), (*dm_key).key_desc.as_ptr(), core::ptr::null());
    if key.is_null() { return -1; }
    key_put(key);
    0
}

#[repr(C)] pub struct config_key { pub item: config_item, pub description: *const c_char }
unsafe fn to_config_key(item: *mut config_item) -> *mut config_key { item as *mut config_key }

unsafe fn config_key_description_show(item: *mut config_item, page: *mut c_char) -> isize {
    let _ = (item, page); 0
}
unsafe fn config_key_description_store(item: *mut config_item, page: *const c_char, count: usize) -> isize {
    let _ = (item, page); count as isize
}
unsafe fn config_key_release(item: *mut config_item) { let _ = to_config_key(item); KEY_COUNT -= 1; }

static mut IS_DM_KEY_REUSED: bool = false;
unsafe fn config_keys_reuse_store(_item: *mut config_item, _page: *const c_char, count: usize) -> isize {
    if IS_DM_KEY_REUSED { get_keys_from_kdump_reserved_memory(); }
    count as isize
}
static mut RESTORE: bool = false;
unsafe fn config_keys_restore_store(_item: *mut config_item, _page: *const c_char, count: usize) -> isize {
    if !RESTORE { restore_dm_crypt_keys_to_thread_keyring(); }
    count as isize
}

unsafe fn build_keys_header() -> c_int {
    let size = get_keys_header_size(KEY_COUNT as usize);
    KEYS_HEADER = libc_kzalloc(size);
    if KEYS_HEADER.is_null() { return -12; }
    (*KEYS_HEADER).total_keys = KEY_COUNT;
    0
}

unsafe fn libc_kzalloc(_size: usize) -> *mut keys_header { core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn crash_load_dm_crypt_keys(image: *mut kimage) -> c_int {
    if KEY_COUNT == 0 { return 0; }
    if !IS_DM_KEY_REUSED {
        (*image).dm_crypt_keys_addr = 0;
        let r = build_keys_header();
        if r != 0 { return r; }
    }
    (*image).dm_crypt_keys_addr = 0;
    (*image).dm_crypt_keys_sz = get_keys_header_size(KEY_COUNT as usize);
    0
}

pub unsafe extern "C" fn configfs_dmcrypt_keys_init() -> c_int {
    let _ = is_kdump_kernel();
    configfs_register_subsystem(core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
