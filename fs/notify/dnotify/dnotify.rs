// SPDX-License-Identifier: GPL-2.0-or-later
/* Directory notifications for Linux. */

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u32 = u32;
type fl_owner_t = *mut c_void;

#[repr(C)] pub struct ctl_table { pub procname: *const c_char, pub data: *mut c_void, pub maxlen: usize, pub mode: u16, pub proc_handler: Option<unsafe extern "C" fn() -> c_int> }
#[repr(C)] pub struct fsnotify_mark { pub lock: c_void, pub mask: __u32, pub connector: *mut c_void }
#[repr(C)] pub struct fsnotify_group { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_mode: u16 }
#[repr(C)] pub struct file { pub f_path: c_void }
#[repr(C)] pub struct qstr { _private: [u8; 0] }
#[repr(C)] pub struct fown_struct { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { pub files: fl_owner_t }

#[repr(C)]
pub struct dnotify_struct {
    pub dn_mask: __u32,
    pub dn_fd: c_int,
    pub dn_filp: *mut file,
    pub dn_owner: fl_owner_t,
    pub dn_next: *mut dnotify_struct,
}

#[repr(C)]
pub struct dnotify_mark { pub fsn_mark: fsnotify_mark, pub dn: *mut dnotify_struct }

extern "C" {
    static mut current: *mut task_struct;
    fn fsnotify_recalc_mask(connector: *mut c_void);
    fn fsnotify_find_inode_mark(inode: *mut inode, group: *mut fsnotify_group) -> *mut fsnotify_mark;
    fn fsnotify_group_lock(group: *mut fsnotify_group);
    fn fsnotify_group_unlock(group: *mut fsnotify_group);
    fn fsnotify_detach_mark(mark: *mut fsnotify_mark);
    fn fsnotify_free_mark(mark: *mut fsnotify_mark);
    fn fsnotify_put_mark(mark: *mut fsnotify_mark);
    fn fsnotify_add_inode_mark_locked(mark: *mut fsnotify_mark, inode: *mut inode, flags: c_uint) -> c_int;
    fn fsnotify_init_mark(mark: *mut fsnotify_mark, group: *mut fsnotify_group);
    fn fsnotify_alloc_group(ops: *const fsnotify_ops, flags: c_uint) -> *mut fsnotify_group;
    fn kmem_cache_alloc(cache: *mut c_void, flags: c_uint) -> *mut c_void;
    fn kmem_cache_free(cache: *mut c_void, object: *mut c_void);
    fn file_inode(file: *mut file) -> *mut inode;
    fn file_f_owner(file: *mut file) -> *mut fown_struct;
    fn file_f_owner_allocate(file: *mut file) -> c_int;
    fn fget_raw(fd: c_int) -> *mut file;
    fn fput(file: *mut file);
    fn __f_setown(file: *mut file, pid: *mut c_void, typ: c_int, force: c_int);
    fn task_pid(task: *mut task_struct) -> *mut c_void;
    fn security_path_notify(path: *mut c_void, mask: __u32, object_type: c_uint) -> c_int;
    fn send_sigio(fown: *mut fown_struct, fd: c_int, band: c_int);
    fn register_sysctl_init(name: *const c_char, table: *const ctl_table);
    fn panic(message: *const c_char) -> !;
}

const FS_DN_MULTISHOT: __u32 = 0x8000_0000;
const FS_EVENT_ON_CHILD: __u32 = 0x0800_0000;
const FS_ISDIR: __u32 = 0x4000_0000;
const FS_DELETE: __u32 = 0x0000_0002;
const FS_MOVED_FROM: __u32 = 0x0000_0040;
const FS_MODIFY: __u32 = 0x0000_0002;
const FS_ACCESS: __u32 = 0x0000_0001;
const FS_ATTRIB: __u32 = 0x0000_0004;
const FS_RENAME: __u32 = 0x0000_0800;
const FS_CREATE: __u32 = 0x0000_0100;
const FS_MOVED_TO: __u32 = 0x0000_0080;
const DN_MULTISHOT: c_uint = 0x8000_0000;
const DN_DELETE: c_uint = 0x0000_0001;
const DN_MODIFY: c_uint = 0x0000_0002;
const DN_ACCESS: c_uint = 0x0000_0004;
const DN_ATTRIB: c_uint = 0x0000_0008;
const DN_RENAME: c_uint = 0x0000_0010;
const DN_CREATE: c_uint = 0x0000_0020;
const FSNOTIFY_OBJ_TYPE_INODE: c_uint = 1;
const POLL_MSG: c_int = 0x0001;
const EINVAL: c_int = -22;
const ENOTDIR: c_int = -20;
const ENOMEM: c_int = -12;
const EEXIST: c_int = -17;
static mut dir_notify_enable: c_int = 1;
static mut dnotify_struct_cache: *mut c_void = core::ptr::null_mut();
static mut dnotify_mark_cache: *mut c_void = core::ptr::null_mut();
static mut dnotify_group: *mut fsnotify_group = core::ptr::null_mut();

#[repr(C)] pub struct fsnotify_ops { pub handle_inode_event: Option<unsafe extern "C" fn(*mut fsnotify_mark, __u32, *mut inode, *mut inode, *const qstr, __u32) -> c_int>, pub free_mark: Option<unsafe extern "C" fn(*mut fsnotify_mark)> }

unsafe fn dnotify_recalc_inode_mask(fsn_mark: *mut fsnotify_mark) {
    let dn_mark = fsn_mark as *mut dnotify_mark;
    let mut new_mask: __u32 = 0;
    let mut dn = (*dn_mark).dn;
    while !dn.is_null() { new_mask |= (*dn).dn_mask & !FS_DN_MULTISHOT; dn = (*dn).dn_next; }
    if (*fsn_mark).mask == new_mask { return; }
    (*fsn_mark).mask = new_mask;
    fsnotify_recalc_mask((*fsn_mark).connector);
}

unsafe extern "C" fn dnotify_handle_event(inode_mark: *mut fsnotify_mark, mask: __u32, _inode: *mut inode, dir: *mut inode, _name: *const qstr, _cookie: __u32) -> c_int {
    if dir.is_null() && mask & FS_ISDIR == 0 { return 0; }
    let dn_mark = inode_mark as *mut dnotify_mark;
    let test_mask = mask & !FS_EVENT_ON_CHILD;
    let mut prev = &mut (*dn_mark).dn as *mut *mut dnotify_struct;
    while !(*prev).is_null() {
        let dn = *prev;
        if (*dn).dn_mask & test_mask == 0 { prev = &mut (*dn).dn_next; continue; }
        send_sigio(file_f_owner((*dn).dn_filp), (*dn).dn_fd, POLL_MSG);
        if (*dn).dn_mask & FS_DN_MULTISHOT != 0 { prev = &mut (*dn).dn_next; } else { *prev = (*dn).dn_next; kmem_cache_free(dnotify_struct_cache, dn as *mut c_void); dnotify_recalc_inode_mask(inode_mark); }
    }
    0
}

unsafe extern "C" fn dnotify_free_mark(fsn_mark: *mut fsnotify_mark) { let dn_mark = fsn_mark as *mut dnotify_mark; if !(*dn_mark).dn.is_null() { panic(b"BUG_ON\0".as_ptr() as *const c_char); } kmem_cache_free(dnotify_mark_cache, dn_mark as *mut c_void); }
static DNOTIFY_FSNOTIFY_OPS: fsnotify_ops = fsnotify_ops { handle_inode_event: Some(dnotify_handle_event), free_mark: Some(dnotify_free_mark) };

pub unsafe extern "C" fn dnotify_flush(filp: *mut file, id: fl_owner_t) {
    let inode = file_inode(filp); if (*inode).i_mode & 0o170000 != 0o040000 { return; }
    let fsn_mark = fsnotify_find_inode_mark(inode, dnotify_group); if fsn_mark.is_null() { return; }
    let dn_mark = fsn_mark as *mut dnotify_mark; fsnotify_group_lock(dnotify_group);
    let mut prev = &mut (*dn_mark).dn as *mut *mut dnotify_struct;
    while !(*prev).is_null() { let dn = *prev; if (*dn).dn_owner == id && (*dn).dn_filp == filp { *prev = (*dn).dn_next; kmem_cache_free(dnotify_struct_cache, dn as *mut c_void); dnotify_recalc_inode_mask(fsn_mark); break; } prev = &mut (*dn).dn_next; }
    let free = (*dn_mark).dn.is_null(); if free { fsnotify_detach_mark(fsn_mark); } fsnotify_group_unlock(dnotify_group); if free { fsnotify_free_mark(fsn_mark); } fsnotify_put_mark(fsn_mark);
}

unsafe fn convert_arg(arg: c_uint) -> __u32 { let mut m = FS_EVENT_ON_CHILD; if arg & DN_MULTISHOT != 0 { m |= FS_DN_MULTISHOT; } if arg & DN_DELETE != 0 { m |= FS_DELETE | FS_MOVED_FROM; } if arg & DN_MODIFY != 0 { m |= FS_MODIFY; } if arg & DN_ACCESS != 0 { m |= FS_ACCESS; } if arg & DN_ATTRIB != 0 { m |= FS_ATTRIB; } if arg & DN_RENAME != 0 { m |= FS_RENAME; } if arg & DN_CREATE != 0 { m |= FS_CREATE | FS_MOVED_TO; } m }

unsafe fn attach_dn(dn: *mut dnotify_struct, dn_mark: *mut dnotify_mark, id: fl_owner_t, fd: c_int, filp: *mut file, mask: __u32) -> c_int { let mut odn = (*dn_mark).dn; while !odn.is_null() { if (*odn).dn_owner == id && (*odn).dn_filp == filp { (*odn).dn_fd = fd; (*odn).dn_mask |= mask; return EEXIST; } odn = (*odn).dn_next; } (*dn).dn_mask=mask; (*dn).dn_fd=fd; (*dn).dn_filp=filp; (*dn).dn_owner=id; (*dn).dn_next=(*dn_mark).dn; (*dn_mark).dn=dn; 0 }

pub unsafe extern "C" fn fcntl_dirnotify(fd: c_int, filp: *mut file, arg: c_uint) -> c_int {
    let id = (*current).files; let mut dn: *mut dnotify_struct = core::ptr::null_mut();
    if dir_notify_enable == 0 { return EINVAL; }
    if arg & !DN_MULTISHOT == 0 { dnotify_flush(filp, id); return 0; }
    let inode = file_inode(filp); if (*inode).i_mode & 0o170000 != 0o040000 { return ENOTDIR; }
    let mask = convert_arg(arg); let error = security_path_notify(&mut (*filp).f_path, mask, FSNOTIFY_OBJ_TYPE_INODE); if error != 0 { return error; }
    dn = kmem_cache_alloc(dnotify_struct_cache, 0) as *mut dnotify_struct; if dn.is_null() { return ENOMEM; }
    let error = file_f_owner_allocate(filp); if error != 0 { kmem_cache_free(dnotify_struct_cache, dn as *mut c_void); return error; }
    let new_dn_mark = kmem_cache_alloc(dnotify_mark_cache, 0) as *mut dnotify_mark; if new_dn_mark.is_null() { kmem_cache_free(dnotify_struct_cache, dn as *mut c_void); return ENOMEM; }
    (*new_dn_mark).dn = core::ptr::null_mut(); fsnotify_init_mark(&mut (*new_dn_mark).fsn_mark, dnotify_group); (*new_dn_mark).fsn_mark.mask = mask;
    fsnotify_group_lock(dnotify_group);
    let found = fsnotify_find_inode_mark(inode, dnotify_group); let (fsn_mark, dn_mark, new_used) = if !found.is_null() { (found, found as *mut dnotify_mark, false) } else { let e = fsnotify_add_inode_mark_locked(&mut (*new_dn_mark).fsn_mark, inode, 0); if e != 0 { fsnotify_group_unlock(dnotify_group); kmem_cache_free(dnotify_mark_cache, new_dn_mark as *mut c_void); kmem_cache_free(dnotify_struct_cache, dn as *mut c_void); return e; } (&mut (*new_dn_mark).fsn_mark, new_dn_mark, true) };
    let f = fget_raw(fd); if f != filp { if new_used { fsnotify_detach_mark(fsn_mark); fsnotify_free_mark(fsn_mark); } fsnotify_group_unlock(dnotify_group); fsnotify_put_mark(fsn_mark); kmem_cache_free(dnotify_struct_cache, dn as *mut c_void); if !f.is_null() { fput(f); } return 0; }
    __f_setown(filp, task_pid(current), 0, 0); let mut e = attach_dn(dn, dn_mark, id, fd, filp, mask); if e == 0 { dn = core::ptr::null_mut(); } else if e == EEXIST { e = 0; } dnotify_recalc_inode_mask(fsn_mark); fsnotify_group_unlock(dnotify_group); fsnotify_put_mark(fsn_mark); if !new_used { kmem_cache_free(dnotify_mark_cache, new_dn_mark as *mut c_void); } if !dn.is_null() { kmem_cache_free(dnotify_struct_cache, dn as *mut c_void); } if !f.is_null() { fput(f); } e
}

pub unsafe extern "C" fn dnotify_init() -> c_int { dnotify_group = fsnotify_alloc_group(&DNOTIFY_FSNOTIFY_OPS, 0); if dnotify_group.is_null() { panic(b"unable to allocate fsnotify group for dnotify\n\0".as_ptr() as *const c_char); } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
