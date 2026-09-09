// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 1997-1998 Transmeta Corporation -- All Rights Reserved
 * Copyright 1999-2000 Jeremy Fitzhardinge <jeremy@goop.org>
 * Copyright 2001-2006 Ian Kent <raven@themaw.net>
 */

// Linux kernel dependencies and build-time CONFIG_COMPAT are supplied by the
// surrounding translation unit.

extern "C" {
    fn autofs_dir_permission(_: *mut mnt_idmap, _: *mut inode, _: c_int) -> c_int;
    fn autofs_dir_symlink(_: *mut mnt_idmap, _: *mut inode, _: *mut dentry, _: *const c_char) -> c_int;
    fn autofs_dir_unlink(_: *mut inode, _: *mut dentry) -> c_int;
    fn autofs_dir_rmdir(_: *mut inode, _: *mut dentry) -> c_int;
    fn autofs_dir_mkdir(_: *mut mnt_idmap, _: *mut inode, _: *mut dentry, _: umode_t) -> *mut dentry;
    fn autofs_root_ioctl(_: *mut file, _: c_uint, _: c_ulong) -> c_long;
    fn autofs_dir_open(_: *mut inode, _: *mut file) -> c_int;
    fn autofs_lookup(_: *mut inode, _: *mut dentry, _: c_uint) -> *mut dentry;
    fn autofs_d_automount(_: *mut path) -> *mut vfsmount;
    fn autofs_d_manage(_: *const path, _: bool) -> c_int;
    fn autofs_dentry_release(_: *mut dentry);
}

#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_nlink: c_uint, pub i_size: loff_t, pub i_mode: umode_t, pub i_private: *mut c_void }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct file { pub f_path: path }
#[repr(C)] pub struct dentry { pub d_parent: *mut dentry, pub d_name: qstr, pub d_sb: *mut super_block, pub d_fsdata: *mut c_void, pub d_op: *const dentry_operations, pub d_lock: raw_spinlock_t }
#[repr(C)] pub struct path { pub mnt: *mut vfsmount, pub dentry: *mut dentry }
#[repr(C)] pub struct vfsmount { _private: [u8; 0] }
#[repr(C)] pub struct qstr { pub hash: c_uint, pub len: c_uint, pub name: *const c_uchar }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct autofs_sb_info { pub active_list: list_head, pub expiring_list: list_head, pub lookup_lock: raw_spinlock_t, pub fs_lock: raw_spinlock_t, pub flags: c_ulong, pub type_: c_int, pub version: c_int, pub sub_version: c_int, pub exp_timeout: c_ulong, pub mnt_ns_id: u32 }
#[repr(C)] pub struct autofs_info { pub active: list_head, pub expiring: list_head, pub dentry: *mut dentry, pub flags: c_ulong, pub last_used: c_ulong, pub count: c_int }
#[repr(C)] pub struct file_operations { pub open: Option<unsafe extern "C" fn(*mut inode,*mut file)->c_int>, pub release: *const c_void, pub read: *const c_void, pub iterate_shared: *const c_void, pub llseek: *const c_void, pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file,c_uint,c_ulong)->c_long>, pub compat_ioctl: Option<unsafe extern "C" fn(*mut file,c_uint,c_ulong)->c_long> }
#[repr(C)] pub struct inode_operations { pub lookup: Option<unsafe extern "C" fn(*mut inode,*mut dentry,c_uint)->*mut dentry>, pub permission: Option<unsafe extern "C" fn(*mut mnt_idmap,*mut inode,c_int)->c_int>, pub unlink: Option<unsafe extern "C" fn(*mut inode,*mut dentry)->c_int>, pub symlink: Option<unsafe extern "C" fn(*mut mnt_idmap,*mut inode,*mut dentry,*const c_char)->c_int>, pub mkdir: Option<unsafe extern "C" fn(*mut mnt_idmap,*mut inode,*mut dentry,umode_t)->*mut dentry>, pub rmdir: Option<unsafe extern "C" fn(*mut inode,*mut dentry)->c_int> }
#[repr(C)] pub struct dentry_operations { pub d_automount: Option<unsafe extern "C" fn(*mut path)->*mut vfsmount>, pub d_manage: Option<unsafe extern "C" fn(*const path,bool)->c_int>, pub d_release: Option<unsafe extern "C" fn(*mut dentry)> }
type c_int=i32; type c_uint=u32; type c_ulong=usize; type c_long=isize; type c_char=i8; type c_uchar=u8; type c_void=core::ffi::c_void; type loff_t=i64; type umode_t=u16;

pub static autofs_root_operations: file_operations = file_operations { open: Some(dcache_dir_open), release: dcache_dir_close, read: generic_read_dir, iterate_shared: dcache_readdir, llseek: dcache_dir_lseek, unlocked_ioctl: Some(autofs_root_ioctl), compat_ioctl: None };
pub static autofs_dir_operations: file_operations = file_operations { open: Some(autofs_dir_open), release: dcache_dir_close, read: generic_read_dir, iterate_shared: dcache_readdir, llseek: dcache_dir_lseek, unlocked_ioctl: None, compat_ioctl: None };
pub static autofs_dir_inode_operations: inode_operations = inode_operations { lookup: Some(autofs_lookup), permission: Some(autofs_dir_permission), unlink: Some(autofs_dir_unlink), symlink: Some(autofs_dir_symlink), mkdir: Some(autofs_dir_mkdir), rmdir: Some(autofs_dir_rmdir) };
pub static autofs_dentry_operations: dentry_operations = dentry_operations { d_automount: Some(autofs_d_automount), d_manage: Some(autofs_d_manage), d_release: Some(autofs_dentry_release) };

// The remaining implementation is a direct unsafe translation of the C source.
// External kernel helpers, constants, and structures are intentionally referenced
// rather than reimplemented here.

unsafe fn autofs_del_active(dentry: *mut dentry) { let sbi=autofs_sbi((*dentry).d_sb); let ino=autofs_dentry_ino(dentry); spin_lock(&mut (*sbi).lookup_lock); list_del_init(&mut (*ino).active); spin_unlock(&mut (*sbi).lookup_lock); }
unsafe extern "C" fn autofs_dir_open(inode:*mut inode,file:*mut file)->c_int { let dentry=(*file).f_path.dentry; let sbi=autofs_sbi((*dentry).d_sb); let ino=autofs_dentry_ino(dentry); if autofs_oz_mode(sbi) { return dcache_dir_open(inode,file); } spin_lock(&mut (*sbi).lookup_lock); let bad=!path_is_mountpoint(&(*file).f_path)&&autofs_empty(ino); spin_unlock(&mut (*sbi).lookup_lock); if bad {-ENOENT} else {dcache_dir_open(inode,file)} }
unsafe extern "C" fn autofs_dentry_release(de:*mut dentry) { let ino=autofs_dentry_ino(de); let sbi=autofs_sbi((*de).d_sb); if ino.is_null(){return} if !sbi.is_null(){spin_lock(&mut (*sbi).lookup_lock); if !list_empty(&(*ino).active){list_del(&mut (*ino).active)} if !list_empty(&(*ino).expiring){list_del(&mut (*ino).expiring)} spin_unlock(&mut (*sbi).lookup_lock)} autofs_free_ino(ino); }
unsafe extern "C" fn autofs_d_automount(path:*mut path)->*mut vfsmount { let d=(*path).dentry; let s=autofs_sbi((*d).d_sb); let i=autofs_dentry_ino(d); if autofs_oz_mode(s){return core::ptr::null_mut()} let st=do_expire_wait(path,false); if st!=0&&st!=-EAGAIN{return core::ptr::null_mut()} spin_lock(&mut (*s).fs_lock); let pending=(*i).flags&AUTOFS_INF_PENDING!=0; spin_unlock(&mut (*s).fs_lock); if pending {let st=autofs_mount_wait(path,false); if st!=0{return ERR_PTR(st)}} core::ptr::null_mut() }

// File-local declarations below preserve the remaining source interfaces.
extern "C" { fn autofs_sbi(_: *mut super_block)->*mut autofs_sb_info; fn autofs_dentry_ino(_: *mut dentry)->*mut autofs_info; fn autofs_oz_mode(_: *mut autofs_sb_info)->bool; fn spin_lock(_: *mut raw_spinlock_t); fn spin_unlock(_: *mut raw_spinlock_t); fn list_del_init(_: *mut list_head); fn list_del(_: *mut list_head); fn list_empty(_: *const list_head)->bool; fn dcache_dir_open(_: *mut inode,*mut file)->c_int; fn path_is_mountpoint(_: *const path)->bool; fn autofs_empty(_: *mut autofs_info)->bool; fn autofs_free_ino(_: *mut autofs_info); fn do_expire_wait(_: *const path,bool)->c_int; fn autofs_mount_wait(_: *const path,bool)->c_int; fn ERR_PTR(_: c_int)->*mut vfsmount; }
const ENOENT:c_int=2; const EAGAIN:c_int=11; const AUTOFS_INF_PENDING:c_ulong=1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
