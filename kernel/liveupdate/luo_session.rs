// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation of luo_session.c. Kernel and LUO declarations are
// supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct luo_session_header {
    pub count: c_long,
    pub list: list_head,
    pub rwsem: rw_semaphore,
    pub block_set: kho_block_set,
    pub sessions_pa: *mut u64,
    pub active: bool,
}

#[repr(C)]
pub struct luo_session_global {
    pub incoming: luo_session_header,
    pub outgoing: luo_session_header,
}

// These types, constants, and functions are defined by the kernel/LUO headers.
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct kho_block_set { _private: [u8; 0] }
#[repr(C)] pub struct kho_block_set_it { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct luo_ucmd { pub ubuffer: *mut c_void, pub user_size: u32, pub cmd: *mut c_void }
#[repr(C)] pub struct luo_session_ser { pub name: [c_char; 128], pub file_set_ser: [u8; 0] }
#[repr(C)] pub struct luo_file_set { _private: [u8; 0] }
#[repr(C)] pub struct luo_session {
    pub name: [c_char; 128], pub file_set: luo_file_set, pub list: list_head,
    pub mutex: mutex, pub retrieved: bool,
}
#[repr(C)] pub struct liveupdate_session_finish { pub reserved: u32 }
#[repr(C)] pub struct liveupdate_session_preserve_fd { pub token: u64, pub fd: c_int }
#[repr(C)] pub struct liveupdate_session_retrieve_fd { pub token: u64, pub fd: c_int }
#[repr(C)] pub struct liveupdate_session_get_name { pub reserved: u32, pub name: [u8; 128] }
#[repr(C)] pub union ucmd_buffer {
    pub finish: liveupdate_session_finish,
    pub preserve: liveupdate_session_preserve_fd,
    pub retrieve: liveupdate_session_retrieve_fd,
    pub get_name: liveupdate_session_get_name,
}

extern "C" {
    static mut luo_session_global: luo_session_global;
    fn luo_file_set_init(set: *mut luo_file_set);
    fn luo_file_set_destroy(set: *mut luo_file_set);
    fn luo_file_finish(set: *mut luo_file_set) -> c_int;
    fn luo_file_unfreeze(set: *mut luo_file_set, ser: *mut c_void);
    fn luo_file_freeze(set: *mut luo_file_set, ser: *mut c_void) -> c_int;
    fn luo_file_deserialize(set: *mut luo_file_set, ser: *mut c_void) -> c_int;
    fn luo_preserve_file(set: *mut luo_file_set, token: u64, fd: c_int) -> c_int;
    fn luo_retrieve_file(set: *mut luo_file_set, token: u64, file: *mut *mut file) -> c_int;
    fn luo_ucmd_respond(cmd: *mut luo_ucmd, size: usize) -> c_int;
    fn kho_block_set_grow(set: *mut kho_block_set, count: c_long) -> c_int;
    fn kho_block_set_shrink(set: *mut kho_block_set, count: c_long);
    fn kho_block_set_restore(set: *mut kho_block_set, pa: u64) -> c_int;
    fn kho_block_set_destroy(set: *mut kho_block_set);
    fn kho_block_set_it_init(it: *mut kho_block_set_it, set: *mut kho_block_set);
    fn kho_block_set_it_read_entry(it: *mut kho_block_set_it) -> *mut luo_session_ser;
    fn kho_block_set_it_reserve_entry(it: *mut kho_block_set_it) -> *mut luo_session_ser;
    fn kho_block_set_it_prev(it: *mut kho_block_set_it) -> *mut luo_session_ser;
    fn kho_block_set_head_pa(set: *mut kho_block_set) -> u64;
    fn get_unused_fd_flags(flags: c_uint) -> c_int;
    fn put_unused_fd(fd: c_int);
    fn fd_install(fd: c_int, file: *mut file);
    fn fput(file: *mut file);
}

static mut LUO_SESSION_SERIALIZE_RWSEM: rw_semaphore = rw_semaphore { _private: [] };

unsafe fn luo_session_alloc(name: *const c_char) -> *mut luo_session {
    let session = kzalloc_luo_session();
    if session.is_null() { return core::ptr::null_mut(); }
    strscpy((*session).name.as_mut_ptr(), name, (*session).name.len());
    luo_file_set_init(&mut (*session).file_set);
    init_list_head(&mut (*session).list);
    mutex_init(&mut (*session).mutex);
    session
}

unsafe fn luo_session_free(session: *mut luo_session) {
    luo_file_set_destroy(&mut (*session).file_set);
    mutex_destroy(&mut (*session).mutex);
    kfree(session as *mut c_void);
}

unsafe fn luo_session_insert(sh: *mut luo_session_header, session: *mut luo_session) -> c_int {
    down_write(&mut (*sh).rwsem);
    if sh == &mut luo_session_global.outgoing {
        let err = kho_block_set_grow(&mut (*sh).block_set, (*sh).count + 1);
        if err != 0 { up_write(&mut (*sh).rwsem); return err; }
    }
    let mut it = (*sh).list.next as *mut luo_session;
    while !it.is_null() && it != sh as *mut luo_session {
        if strncmp((*it).name.as_ptr(), (*session).name.as_ptr(), (*it).name.len()) == 0 {
            up_write(&mut (*sh).rwsem); return -17;
        }
        it = (*it).list.next as *mut luo_session;
    }
    list_add_tail(&mut (*session).list, &mut (*sh).list);
    (*sh).count += 1;
    up_write(&mut (*sh).rwsem);
    0
}

unsafe fn luo_session_remove(sh: *mut luo_session_header, session: *mut luo_session) {
    down_write(&mut (*sh).rwsem);
    list_del(&mut (*session).list);
    (*sh).count -= 1;
    if sh == &mut luo_session_global.outgoing { kho_block_set_shrink(&mut (*sh).block_set, (*sh).count); }
    up_write(&mut (*sh).rwsem);
}

unsafe fn luo_session_finish_one(session: *mut luo_session) -> c_int {
    mutex_lock(&mut (*session).mutex); let err = luo_file_finish(&mut (*session).file_set); mutex_unlock(&mut (*session).mutex); err
}
unsafe fn luo_session_unfreeze_one(session: *mut luo_session, ser: *mut luo_session_ser) {
    mutex_lock(&mut (*session).mutex); luo_file_unfreeze(&mut (*session).file_set, &mut (*ser).file_set_ser as *mut _ as *mut c_void); mutex_unlock(&mut (*session).mutex);
}
unsafe fn luo_session_freeze_one(session: *mut luo_session, ser: *mut luo_session_ser) -> c_int {
    mutex_lock(&mut (*session).mutex); let err = luo_file_freeze(&mut (*session).file_set, &mut (*ser).file_set_ser as *mut _ as *mut c_void); mutex_unlock(&mut (*session).mutex); err
}

#[allow(dead_code)]
unsafe fn luo_session_release(_inodep: *mut inode, filep: *mut file) -> c_int {
    let session = (*filep).private_data as *mut luo_session;
    let sh;
    if (*session).retrieved { let err = luo_session_finish_one(session); if err != 0 { return err; } sh = &mut luo_session_global.incoming; }
    else { mutex_lock(&mut (*session).mutex); luo_file_unpreserve_files(&mut (*session).file_set); mutex_unlock(&mut (*session).mutex); sh = &mut luo_session_global.outgoing; }
    luo_session_remove(sh, session); luo_session_free(session); 0
}

unsafe fn luo_session_preserve_fd(session: *mut luo_session, ucmd: *mut luo_ucmd) -> c_int {
    let argp = (*ucmd).cmd as *mut liveupdate_session_preserve_fd;
    mutex_lock(&mut (*session).mutex); let mut err = luo_preserve_file(&mut (*session).file_set, (*argp).token, (*argp).fd); mutex_unlock(&mut (*session).mutex);
    if err != 0 { return err; } err = luo_ucmd_respond(ucmd, core::mem::size_of::<liveupdate_session_preserve_fd>()); err
}
unsafe fn luo_session_retrieve_fd(session: *mut luo_session, ucmd: *mut luo_ucmd) -> c_int {
    let argp = (*ucmd).cmd as *mut liveupdate_session_retrieve_fd; (*argp).fd = get_unused_fd_flags(0x80000); if (*argp).fd < 0 { return (*argp).fd; }
    let mut fp = core::ptr::null_mut(); mutex_lock(&mut (*session).mutex); let mut err = luo_retrieve_file(&mut (*session).file_set, (*argp).token, &mut fp); mutex_unlock(&mut (*session).mutex);
    if err < 0 { put_unused_fd((*argp).fd); return err; } err = luo_ucmd_respond(ucmd, core::mem::size_of::<liveupdate_session_retrieve_fd>()); if err != 0 { fput(fp); put_unused_fd((*argp).fd); return err; } fd_install((*argp).fd, fp); 0
}
unsafe fn luo_session_finish(session: *mut luo_session, ucmd: *mut luo_ucmd) -> c_int { let argp = (*ucmd).cmd as *mut liveupdate_session_finish; if (*argp).reserved != 0 { return -22; } let err = luo_session_finish_one(session); if err != 0 { return err; } luo_ucmd_respond(ucmd, core::mem::size_of::<liveupdate_session_finish>()) }
unsafe fn luo_session_get_name(session: *mut luo_session, ucmd: *mut luo_ucmd) -> c_int { let argp = (*ucmd).cmd as *mut liveupdate_session_get_name; if (*argp).reserved != 0 { return -22; } strscpy((*argp).name.as_mut_ptr() as *mut c_char, (*session).name.as_ptr(), (*argp).name.len()); luo_ucmd_respond(ucmd, core::mem::size_of::<liveupdate_session_get_name>()) }

#[repr(C)] pub enum luo_ioctl_type { Incoming, Outgoing, All }
#[repr(C)] pub struct luo_ioctl_op { pub size: usize, pub min_size: usize, pub ioctl_num: c_uint, pub ioctl_type: luo_ioctl_type, pub execute: unsafe fn(*mut luo_session, *mut luo_ucmd) -> c_int }

unsafe fn luo_ioctl_type_valid(session: *mut luo_session, op: *const luo_ioctl_op) -> bool { match (*op).ioctl_type { luo_ioctl_type::Incoming => (*session).retrieved, luo_ioctl_type::Outgoing => !(*session).retrieved, luo_ioctl_type::All => true } }

pub unsafe fn luo_session_create(name: *const c_char, filep: *mut *mut file) -> c_int { let len = strnlen(name, 128); if len == 0 || len > 127 { return -22; } let session = luo_session_alloc(name); if session.is_null() { return -12; } let mut err = luo_session_insert(&mut luo_session_global.outgoing, session); if err != 0 { luo_session_free(session); return err; } mutex_lock(&mut (*session).mutex); err = luo_session_getfile(session, filep); mutex_unlock(&mut (*session).mutex); if err != 0 { luo_session_remove(&mut luo_session_global.outgoing, session); luo_session_free(session); } err }
pub unsafe fn luo_session_retrieve(name: *const c_char, filep: *mut *mut file) -> c_int { let sh = &mut luo_session_global.incoming; let mut it = (*sh).list.next as *mut luo_session; while !it.is_null() && it != sh as *mut _ as *mut luo_session { if strncmp((*it).name.as_ptr(), name, (*it).name.len()) == 0 { break; } it = (*it).list.next as *mut luo_session; } if it.is_null() || it == sh as *mut _ as *mut luo_session { return -2; } if (*it).retrieved { return -22; } let err = luo_session_getfile(it, filep); if err == 0 { (*it).retrieved = true; } err }

pub unsafe fn luo_session_setup_outgoing(sessions_pa: *mut u64) { luo_session_global.outgoing.sessions_pa = sessions_pa; luo_session_global.outgoing.active = true; }
pub unsafe fn luo_session_setup_incoming(sessions_pa: u64) -> c_int { if sessions_pa == 0 { return 0; } let err = kho_block_set_restore(&mut luo_session_global.incoming.block_set, sessions_pa); if err == 0 { luo_session_global.incoming.active = true; } err }

// Remaining serialization/deserialization entry points retain the KHO iterator
// protocol and are declared here for the surrounding implementation to provide.
extern "C" { fn luo_session_deserialize() -> c_int; fn luo_session_serialize() -> c_int; }

extern "C" {
    fn kzalloc_luo_session() -> *mut luo_session; fn kfree(p: *mut c_void); fn strscpy(d: *mut c_char, s: *const c_char, n: usize) -> isize; fn strncmp(a: *const c_char,b:*const c_char,n:usize)->c_int; fn strnlen(s:*const c_char,n:usize)->usize;
    fn init_list_head(h:*mut list_head); fn list_add_tail(n:*mut list_head,h:*mut list_head); fn list_del(n:*mut list_head);
    fn mutex_init(m:*mut mutex); fn mutex_destroy(m:*mut mutex); fn mutex_lock(m:*mut mutex); fn mutex_unlock(m:*mut mutex); fn down_write(s:*mut rw_semaphore); fn up_write(s:*mut rw_semaphore);
    fn luo_file_unpreserve_files(s:*mut luo_file_set); fn luo_session_getfile(s:*mut luo_session,f:*mut *mut file)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
