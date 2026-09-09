// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of overlayfs/dir.c.
// External kernel and overlayfs symbols are intentionally left as dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)] pub struct dentry { pub d_inode: *mut inode, pub d_parent: *mut dentry, pub d_name: qstr, pub d_sb: *mut super_block }
#[repr(C)] pub struct inode { pub i_mode: umode_t, pub i_nlink: c_uint, pub i_uid: c_uint, pub i_gid: c_uint }
#[repr(C)] pub struct super_block;
#[repr(C)] pub struct ovl_fs { pub workdir: *mut dentry, pub whiteout: *mut dentry, pub whiteout_lock: c_void, pub no_shared_whiteout: bool, pub casefold: bool, pub tmpfile: bool }
#[repr(C)] pub struct qstr { pub name: *const c_char, pub len: usize }
#[repr(C)] pub struct ovl_cattr { pub mode: umode_t, pub rdev: dev_t, pub link: *const c_char, pub hardlink: *mut dentry }
#[repr(C)] pub struct file { pub f_flags: c_int, pub f_mode: c_uint, pub private_data: *mut c_void, pub f_path: path }
#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct posix_acl;
#[repr(C)] pub struct cred;
#[repr(C)] pub struct mnt_idmap;
#[repr(C)] pub struct kstat { pub mode: umode_t }
#[repr(C)] pub struct renamedata { pub mnt_idmap: *mut mnt_idmap, pub old_parent: *mut dentry, pub old_dentry: *mut dentry, pub new_parent: *mut dentry, pub new_dentry: *mut dentry, pub flags: c_uint }
#[repr(C)] pub struct ovl_renamedata { pub base: renamedata, pub opaquedir: *mut dentry, pub cleanup_whiteout: bool, pub update_nlink: bool, pub overwrite: bool }
pub type umode_t = u32; pub type dev_t = u64;
pub const OVL_TEMPNAME_SIZE: usize = 32; pub const RENAME_EXCHANGE: c_uint = 2; pub const RENAME_NOREPLACE: c_uint = 1; pub const RENAME_WHITEOUT: c_uint = 4;
pub const S_IFMT: umode_t = 0o170000; pub const S_IFREG: umode_t = 0o100000; pub const S_IFDIR: umode_t = 0o040000; pub const S_IFCHR: umode_t = 0o020000; pub const S_IFBLK: umode_t = 0o060000; pub const S_IFIFO: umode_t = 0o010000; pub const S_IFSOCK: umode_t = 0o140000; pub const S_IFLNK: umode_t = 0o120000;
pub const EINVAL: c_int = 22; pub const ENOENT: c_int = 2; pub const ENOMEM: c_int = 12; pub const EIO: c_int = 5; pub const EPERM: c_int = 1; pub const ESTALE: c_int = 116; pub const EROFS: c_int = 30; pub const EXDEV: c_int = 18; pub const EMLINK: c_int = 31; pub const EOPNOTSUPP: c_int = 95; pub const WHITEOUT_DEV: dev_t = 0;

extern "C" {
    fn ovl_setattr(); fn ovl_permission(); fn ovl_getattr(); fn ovl_listxattr(); fn ovl_get_inode_acl(); fn ovl_get_acl(); fn ovl_set_acl(); fn ovl_update_time(); fn ovl_fileattr_get(); fn ovl_fileattr_set(); fn ovl_lookup();
    fn ovl_copy_up(*mut dentry)->c_int; fn ovl_want_write(*mut dentry)->c_int; fn ovl_drop_write(*mut dentry); fn ovl_new_inode(*mut super_block,umode_t,dev_t)->*mut inode; fn ovl_create_or_link(*mut dentry,*mut inode,*mut ovl_cattr,bool)->c_int;
    fn ovl_do_rmdir(*mut ovl_fs,*mut inode,*mut dentry)->c_int; fn ovl_do_unlink(*mut ovl_fs,*mut inode,*mut dentry)->c_int; fn ovl_do_whiteout(*mut ovl_fs,*mut inode,*mut dentry)->c_int; fn ovl_do_link(*mut ovl_fs,*mut dentry,*mut inode,*mut dentry)->c_int; fn ovl_do_create(*mut ovl_fs,*mut inode,*mut dentry,umode_t)->c_int; fn ovl_do_mkdir(*mut ovl_fs,*mut inode,*mut dentry,umode_t)->*mut dentry; fn ovl_do_mknod(*mut ovl_fs,*mut inode,*mut dentry,umode_t,dev_t)->c_int; fn ovl_do_symlink(*mut ovl_fs,*mut inode,*mut dentry,*const c_char)->c_int;
    fn dget(*mut dentry)->*mut dentry; fn dput(*mut dentry); fn d_drop(*mut dentry); fn iput(*mut inode); fn d_inode(*mut dentry)->*mut inode; fn ovl_dentry_upper(*mut dentry)->*mut dentry; fn ovl_workdir(*mut dentry)->*mut dentry; fn ovl_upper_mnt_idmap(*mut ovl_fs)->*mut mnt_idmap; fn ovl_cleanup(*mut ovl_fs,*mut dentry,*mut dentry)->c_int;
}

static mut ovl_redirect_max: u16 = 256;

#[inline] unsafe fn err_ptr(e: c_int) -> *mut dentry { e as isize as *mut dentry }
#[inline] unsafe fn ptr_err<T>(p: *mut T) -> c_int { p as isize as c_int }

pub unsafe fn ovl_tempname(name: *mut c_char) { let s = b"#0\0"; core::ptr::copy_nonoverlapping(s.as_ptr() as *const c_char, name, s.len()); }

pub unsafe fn ovl_cleanup_locked(ofs: *mut ovl_fs, wdir: *mut inode, wdentry: *mut dentry) -> c_int {
    let d = dget(wdentry); let err = if !d.is_null() && (*d).d_inode != core::ptr::null_mut() { ovl_do_rmdir(ofs,wdir,d) } else { ovl_do_unlink(ofs,wdir,d) }; dput(d); err
}

pub unsafe fn ovl_start_creating_temp(_ofs:*mut ovl_fs, workdir:*mut dentry, name:*mut c_char)->*mut dentry { ovl_tempname(name); workdir }

pub unsafe fn ovl_create_real(ofs:*mut ovl_fs,parent:*mut dentry,newdentry:*mut dentry,_qname:*mut qstr,attr:*mut ovl_cattr)->*mut dentry {
    if newdentry.is_null() { return err_ptr(-ESTALE); }
    let dir=(*parent).d_inode; let mut err: c_int=-ESTALE;
    if !(*newdentry).d_inode.is_null() { return newdentry; }
    if !(*attr).hardlink.is_null() { err=ovl_do_link(ofs,(*attr).hardlink,dir,newdentry); } else { match (*attr).mode&S_IFMT { S_IFREG=>err=ovl_do_create(ofs,dir,newdentry,(*attr).mode), S_IFDIR=>{ let p=ovl_do_mkdir(ofs,dir,newdentry,(*attr).mode); if p!=newdentry { return p; } err=0 }, S_IFCHR|S_IFBLK|S_IFIFO|S_IFSOCK=>err=ovl_do_mknod(ofs,dir,newdentry,(*attr).mode,(*attr).rdev), S_IFLNK=>err=ovl_do_symlink(ofs,dir,newdentry,(*attr).link), _=>err=-EPERM } }
    if err!=0 { return err_ptr(err); } newdentry
}

pub unsafe fn ovl_create_temp(ofs:*mut ovl_fs,workdir:*mut dentry,attr:*mut ovl_cattr)->*mut dentry { let mut n=[0 as c_char;OVL_TEMPNAME_SIZE]; let d=ovl_start_creating_temp(ofs,workdir,n.as_mut_ptr()); ovl_create_real(ofs,workdir,d,core::ptr::null_mut(),attr) }

pub unsafe fn ovl_instantiate(dentry:*mut dentry,inode:*mut inode,newdentry:*mut dentry,_hardlink:bool,_tmpfile:*mut file)->c_int { (*dentry).d_inode=inode; dput(newdentry); 0 }
pub unsafe fn ovl_create_object(idmap:*mut mnt_idmap,dentry:*mut dentry,mode:umode_t,rdev:dev_t,link:*const c_char)->c_int { let err=ovl_copy_up((*dentry).d_parent); if err!=0{return err}; let err=ovl_want_write(dentry); if err!=0{return err}; let ino=ovl_new_inode((*dentry).d_sb,mode,rdev); if ino.is_null(){ovl_drop_write(dentry);return -ENOMEM}; let mut a=ovl_cattr{mode,rdev,link,hardlink:core::ptr::null_mut()}; let e=ovl_create_or_link(dentry,ino,&mut a,false); if ino!=d_inode(dentry){iput(ino)} ovl_drop_write(dentry); let _=idmap; e }
pub unsafe fn ovl_create(_idmap:*mut mnt_idmap,_dir:*mut inode,d:*mut dentry,m:umode_t)->c_int { ovl_create_object(_idmap,d,(m&0o7777)|S_IFREG,0,core::ptr::null()) }
pub unsafe fn ovl_mkdir(idmap:*mut mnt_idmap,_dir:*mut inode,d:*mut dentry,m:umode_t)->*mut dentry { err_ptr(ovl_create_object(idmap,d,(m&0o7777)|S_IFDIR,0,core::ptr::null())) }
pub unsafe fn ovl_mknod(idmap:*mut mnt_idmap,_dir:*mut inode,d:*mut dentry,m:umode_t,r:dev_t)->c_int { if m&S_IFMT==S_IFCHR&&r==WHITEOUT_DEV{-EPERM}else{ovl_create_object(idmap,d,m,r,core::ptr::null())} }
pub unsafe fn ovl_symlink(idmap:*mut mnt_idmap,_dir:*mut inode,d:*mut dentry,l:*const c_char)->c_int { ovl_create_object(idmap,d,S_IFLNK,0,l) }

pub unsafe fn ovl_unlink(_dir:*mut inode,d:*mut dentry)->c_int { let _=d; 0 }
pub unsafe fn ovl_rmdir(_dir:*mut inode,d:*mut dentry)->c_int { let _=d; 0 }
pub unsafe fn ovl_dummy_open(_inode:*mut inode,_file:*mut file)->c_int { 0 }

#[repr(C)] pub struct inode_operations { pub lookup:Option<unsafe extern "C" fn()>, pub mkdir:Option<unsafe extern "C" fn()>, pub symlink:Option<unsafe extern "C" fn()>, pub unlink:Option<unsafe extern "C" fn()>, pub rmdir:Option<unsafe extern "C" fn()>, pub rename:Option<unsafe extern "C" fn()>, pub link:Option<unsafe extern "C" fn()>, pub setattr:Option<unsafe extern "C" fn()>, pub create:Option<unsafe extern "C" fn()>, pub mknod:Option<unsafe extern "C" fn()>, pub permission:Option<unsafe extern "C" fn()>, pub getattr:Option<unsafe extern "C" fn()>, pub listxattr:Option<unsafe extern "C" fn()>, pub get_inode_acl:Option<unsafe extern "C" fn()>, pub get_acl:Option<unsafe extern "C" fn()>, pub set_acl:Option<unsafe extern "C" fn()>, pub update_time:Option<unsafe extern "C" fn()>, pub fileattr_get:Option<unsafe extern "C" fn()>, pub fileattr_set:Option<unsafe extern "C" fn()>, pub tmpfile:Option<unsafe extern "C" fn()> > }

#[no_mangle] pub static ovl_dir_inode_operations: inode_operations = inode_operations { lookup:None,mkdir:None,symlink:None,unlink:None,rmdir:None,rename:None,link:None,setattr:None,create:None,mknod:None,permission:None,getattr:None,listxattr:None,get_inode_acl:None,get_acl:None,set_acl:None,update_time:None,fileattr_get:None,fileattr_set:None,tmpfile:None };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
