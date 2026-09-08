// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2010 IBM Corporation */

use core::ffi::c_char;
use core::ptr;

extern "C" {
    type dentry; type file; type audit_buffer; type iattr; type inode; type mnt_idmap;
    type file_operations;
    static mut evm_initialized: u32;
    static mut integrity_dir: *mut dentry;
    static nop_mnt_idmap: mnt_idmap;
    static mut evm_config_xattrnames: list_head;
    fn simple_read_from_buffer(*mut c_char, usize, *mut i64, *const c_char, usize) -> isize;
    fn simple_setattr(*const mnt_idmap, *mut dentry, *mut iattr) -> i32;
    fn inode_lock(*mut inode); fn inode_unlock(*mut inode);
    fn mutex_lock_interruptible(*mut mutex) -> i32; fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex);
    fn kmalloc(usize, u32) -> *mut core::ffi::c_void; fn kfree(*mut core::ffi::c_void);
    fn strlen(*const c_char) -> usize; fn sprintf(*mut c_char, *const c_char, ...) -> i32;
    fn strncmp(*const c_char, *const c_char, usize) -> i32; fn strcmp(*const c_char, *const c_char) -> i32;
    fn snprintf(*mut c_char, usize, *const c_char, ...) -> i32;
    fn memdup_user_nul(*const c_char, usize) -> *mut c_char;
    fn audit_context() -> *mut core::ffi::c_void;
    fn audit_log_start(*mut core::ffi::c_void, u32, u32) -> *mut audit_buffer;
    fn audit_log_format(*mut audit_buffer, *const c_char, ...);
    fn audit_log_untrustedstring(*mut audit_buffer, *const c_char); fn audit_log_end(*mut audit_buffer);
    fn securityfs_create_dir(*const c_char, *mut dentry) -> *mut dentry;
    fn securityfs_create_file(*const c_char, u32, *mut dentry, *mut core::ffi::c_void, *const file_operations) -> *mut dentry;
    fn securityfs_create_symlink(*const c_char, *mut dentry, *const c_char, *mut core::ffi::c_void) -> *mut dentry;
    fn securityfs_remove(*mut dentry); fn integrity_fs_init() -> i32; fn integrity_fs_fini(); fn evm_init_key() -> i32;
    fn capable(u32) -> bool; fn kstrtouint_from_user(*const c_char, usize, u32, *mut u32) -> i32;
    fn IS_ENABLED(u32) -> bool; fn IS_ERR(*const core::ffi::c_void) -> bool; fn PTR_ERR(*const core::ffi::c_void) -> i32;
}

#[repr(C)] pub struct list_head { next: *mut list_head, prev: *mut list_head }
#[repr(C)] pub struct mutex { _data: [u8; 0] }
#[repr(C)] pub struct xattr_list { pub list: list_head, pub name: *mut c_char, pub enabled: bool }

static mut evm_dir: *mut dentry = ptr::null_mut();
static mut evm_symlink: *mut dentry = ptr::null_mut();
#[cfg(CONFIG_EVM_ADD_XATTRS)] static mut evm_xattrs: *mut dentry = ptr::null_mut();
#[cfg(CONFIG_EVM_ADD_XATTRS)] static mut xattr_list_mutex: mutex = mutex { _data: [] };
#[cfg(CONFIG_EVM_ADD_XATTRS)] static mut evm_xattrs_locked: i32 = 0;

const EVM_SETUP_COMPLETE:u32=1; const EVM_INIT_MASK:u32=7; const EVM_ALLOW_METADATA_WRITES:u32=2; const EVM_INIT_HMAC:u32=4;
const XATTR_NAME_MAX:usize=255; const XATTR_SECURITY_PREFIX_LEN:usize=9; const GFP_KERNEL:u32=0x0120;
const CAP_SYS_ADMIN:u32=21; const AUDIT_INTEGRITY_EVM_XATTR:u32=1800; const ATTR_MODE:u32=0x20; const S_IFREG:u32=0o100000;
const EPERM:i32=-1; const EINVAL:i32=-22; const ENOMEM:i32=-12; const EFAULT:i32=-14; const ERESTARTSYS:i32=-512; const EEXIST:i32=-17; const E2BIG:i32=-7;
const XATTR_SECURITY_PREFIX:&[u8]=b"security.";

unsafe extern "C" fn evm_read_key(_: *mut file, buf:*mut c_char, count:usize, ppos:*mut i64)->isize {
    let mut temp=[0u8;80]; if *ppos!=0{return 0;} sprintf(temp.as_mut_ptr() as *mut c_char,b"%d\0".as_ptr() as *const c_char,(evm_initialized & !EVM_SETUP_COMPLETE) as i32); simple_read_from_buffer(buf,count,ppos,temp.as_ptr() as *const c_char,strlen(temp.as_ptr() as *const c_char))
}
unsafe extern "C" fn evm_write_key(_: *mut file,buf:*const c_char,count:usize,_:*mut i64)->isize {
    let mut i=0u32; if !capable(CAP_SYS_ADMIN)||(evm_initialized&EVM_SETUP_COMPLETE)!=0{return EPERM as isize;} let mut ret=kstrtouint_from_user(buf,count,0,&mut i); if ret!=0{return ret as isize;} if i==0||(i&!EVM_INIT_MASK)!=0{return EINVAL as isize;} if (i&EVM_ALLOW_METADATA_WRITES)!=0&&(evm_initialized&EVM_INIT_HMAC)!=0{return EPERM as isize;} if (i&EVM_INIT_HMAC)!=0 {ret=evm_init_key();if ret!=0{return ret as isize;}i|=EVM_SETUP_COMPLETE;} evm_initialized|=i;if (evm_initialized&EVM_INIT_HMAC)!=0{evm_initialized&=!EVM_ALLOW_METADATA_WRITES;} count as isize
}

#[cfg(CONFIG_EVM_ADD_XATTRS)] unsafe extern "C" fn evm_read_xattrs(_: *mut file,buf:*mut c_char,count:usize,ppos:*mut i64)->isize {
    if *ppos!=0{return 0;} if mutex_lock_interruptible(&mut xattr_list_mutex)!=0{return ERESTARTSYS as isize;} let mut size=0; let mut p=(*evm_config_xattrnames).next; while p!=&mut evm_config_xattrnames {let x=p as *mut xattr_list;if (*x).enabled{size+=strlen((*x).name)+1;}p=(*p).next;} let temp=kmalloc(size+1,GFP_KERNEL) as *mut c_char;if temp.is_null(){mutex_unlock(&mut xattr_list_mutex);return ENOMEM as isize;} *temp.add(size)=0;let mut off=0;p=(*evm_config_xattrnames).next;while p!=&mut evm_config_xattrnames{let x=p as *mut xattr_list;if (*x).enabled{off+=snprintf(temp.add(off),size+1-off,b"%s\n\0".as_ptr() as *const c_char,(*x).name) as usize;}p=(*p).next;}mutex_unlock(&mut xattr_list_mutex);let r=simple_read_from_buffer(buf,count,ppos,temp,off);kfree(temp as *mut _);r
}

#[cfg(CONFIG_EVM_ADD_XATTRS)] unsafe extern "C" fn evm_write_xattrs(_: *mut file,buf:*const c_char,count:usize,ppos:*mut i64)->isize {
    if !capable(CAP_SYS_ADMIN)||evm_xattrs_locked!=0{return EPERM as isize;}if *ppos!=0{return EINVAL as isize;}if count>XATTR_NAME_MAX{return E2BIG as isize;}let ab=audit_log_start(audit_context(),GFP_KERNEL,AUDIT_INTEGRITY_EVM_XATTR);if ab.is_null()&&IS_ENABLED(1){return ENOMEM as isize;}let x=kmalloc(core::mem::size_of::<xattr_list>(),GFP_KERNEL) as *mut xattr_list;if x.is_null(){return ENOMEM as isize;}(*x).enabled=true;(*x).name=memdup_user_nul(buf,count);if IS_ERR((*x).name as *const _){let e=PTR_ERR((*x).name as *const _);(*x).name=ptr::null_mut();audit_log_format(ab,b" res=%d\0".as_ptr() as *const _,e);audit_log_end(ab);kfree(x as *mut _);return e as isize;}let len=strlen((*x).name);if len!=0&&*(*x).name.add(len-1)==b'\n' as c_char{*(*x).name.add(len-1)=0;}audit_log_format(ab,b"xattr=\0".as_ptr() as *const _);audit_log_untrustedstring(ab,(*x).name);if strcmp((*x).name,b".\0".as_ptr() as *const _)==0{evm_xattrs_locked=1;let mut a:iattr=core::mem::zeroed();a.ia_mode=S_IFREG|0o440;a.ia_valid=ATTR_MODE;let i=(*(evm_xattrs)).d_inode;inode_lock(i);let mut e=simple_setattr(&nop_mnt_idmap,evm_xattrs,&mut a);inode_unlock(i);if e==0{e=count as i32;}audit_log_format(ab,b" res=%d\0".as_ptr() as *const _,if e<0{e}else{0});audit_log_end(ab);kfree((*x).name as *mut _);kfree(x as *mut _);return e as isize;}if strncmp((*x).name,XATTR_SECURITY_PREFIX.as_ptr() as *const _,XATTR_SECURITY_PREFIX_LEN)!=0{audit_log_format(ab,b" res=-22\0".as_ptr() as *const _);audit_log_end(ab);kfree((*x).name as *mut _);kfree(x as *mut _);return EINVAL as isize;}mutex_lock(&mut xattr_list_mutex);let mut p=(*evm_config_xattrnames).next;while p!=&mut evm_config_xattrnames{let t=p as *mut xattr_list;if strcmp((*x).name,(*t).name)==0{let e=if !(*t).enabled{(*t).enabled=true;count as i32}else{EEXIST};mutex_unlock(&mut xattr_list_mutex);audit_log_format(ab,b" res=%d\0".as_ptr() as *const _,if e<0{e}else{0});audit_log_end(ab);kfree((*x).name as *mut _);kfree(x as *mut _);return e as isize;}p=(*p).next;}let tail=(*evm_config_xattrnames).prev;(*x).list.next=&mut evm_config_xattrnames;(*x).list.prev=tail;(*tail).next=&mut (*x).list;(*evm_config_xattrnames).prev=&mut (*x).list;mutex_unlock(&mut xattr_list_mutex);audit_log_format(ab,b" res=0\0".as_ptr() as *const _);audit_log_end(ab);count as isize
}

static mut evm_key_ops:file_operations=unsafe{core::mem::zeroed()};
#[cfg(CONFIG_EVM_ADD_XATTRS)] static mut evm_xattr_ops:file_operations=unsafe{core::mem::zeroed()};
#[cfg(CONFIG_EVM_ADD_XATTRS)] unsafe fn evm_init_xattrs()->i32{evm_xattrs=securityfs_create_file(b"evm_xattrs\0".as_ptr() as *const _,0o660,evm_dir,ptr::null_mut(),&evm_xattr_ops);if IS_ERR(evm_xattrs as *const _){-EFAULT}else{0}}
#[cfg(not(CONFIG_EVM_ADD_XATTRS))] unsafe fn evm_init_xattrs()->i32{0}

#[no_mangle] pub unsafe extern "C" fn evm_init_secfs()->i32{let mut error=integrity_fs_init();if error<0{return -EFAULT;}evm_dir=securityfs_create_dir(b"evm\0".as_ptr() as *const _,integrity_dir);if IS_ERR(evm_dir as *const _){error=-EFAULT;}else{let d=securityfs_create_file(b"evm\0".as_ptr() as *const _,0o660,evm_dir,ptr::null_mut(),&evm_key_ops);if IS_ERR(d as *const _){error=-EFAULT;}else{evm_symlink=securityfs_create_symlink(b"evm\0".as_ptr() as *const _,ptr::null_mut(),b"integrity/evm/evm\0".as_ptr() as *const _,ptr::null_mut());if IS_ERR(evm_symlink as *const _){error=-EFAULT;}else if evm_init_xattrs()!=0{error=-EFAULT;}else{return 0;}}}securityfs_remove(evm_symlink);securityfs_remove(evm_dir);integrity_fs_fini();error}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
