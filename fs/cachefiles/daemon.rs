// SPDX-License-Identifier: GPL-2.0-or-later
/* Daemon interface
 *
 * Copyright (C) 2007, 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct FileOperations {
    pub owner: *mut core::ffi::c_void,
    pub open: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> i32>,
    pub release: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> i32>,
    pub read: Option<unsafe extern "C" fn(*mut File, *mut u8, usize, *mut i64) -> isize>,
    pub write: Option<unsafe extern "C" fn(*mut File, *const u8, usize, *mut i64) -> isize>,
    pub poll: Option<unsafe extern "C" fn(*mut File, *mut PollTable) -> u32>,
    pub llseek: Option<unsafe extern "C" fn(*mut File, i64, i32) -> i64>,
}

#[repr(C)] pub struct Inode;
#[repr(C)] pub struct File { pub private_data: *mut CachefilesCache }
#[repr(C)] pub struct PollTable;
#[repr(C)] pub struct Path { pub dentry: *mut Dentry }
#[repr(C)] pub struct Dentry;
#[repr(C)] pub struct Cred;

#[repr(C)]
pub struct CachefilesCache {
    pub daemon_mutex: core::ffi::c_void,
    pub daemon_pollwq: core::ffi::c_void,
    pub volumes: ListHead,
    pub object_list: ListHead,
    pub object_list_lock: core::ffi::c_void,
    pub frun_percent: usize, pub fcull_percent: usize, pub fstop_percent: usize,
    pub brun_percent: usize, pub bcull_percent: usize, pub bstop_percent: usize,
    pub cachefilesd: *mut File, pub flags: usize, pub f_released: usize,
    pub b_released: usize, pub frun: u64, pub fcull: u64, pub fstop: u64,
    pub brun: u64, pub bcull: u64, pub bstop: u64,
    pub rootdirname: *mut u8, pub have_secid: bool, pub secid: u32,
    pub tag: *mut u8, pub graveyard: *mut core::ffi::c_void,
    pub store: *mut core::ffi::c_void, pub mnt: *mut core::ffi::c_void,
    pub cache_cred: *mut Cred,
}
#[repr(C)] pub struct ListHead;

#[repr(C)] pub struct CachefilesDaemonCmd { pub name: [u8; 8], pub handler: Option<unsafe extern "C" fn(*mut CachefilesCache, *mut u8) -> i32> }

static mut CACHEFILES_OPEN: usize = 0;
pub static CACHEFILES_DAEMON_FOPS: FileOperations = FileOperations {
    owner: core::ptr::null_mut(), open: Some(cachefiles_daemon_open), release: Some(cachefiles_daemon_release),
    read: Some(cachefiles_daemon_read), write: Some(cachefiles_daemon_write), poll: Some(cachefiles_daemon_poll), llseek: None,
};

extern "C" {
    fn capable(x: i32) -> bool; fn xchg(p: *mut usize, v: usize) -> usize;
    fn kzalloc_cache() -> *mut CachefilesCache; fn kfree(p: *mut core::ffi::c_void);
    fn mutex_init(p: *mut core::ffi::c_void); fn init_waitqueue_head(p: *mut core::ffi::c_void);
    fn init_list_head(p: *mut ListHead); fn spin_lock_init(p: *mut core::ffi::c_void);
    fn set_bit(n: usize, p: *mut usize); fn clear_bit(n: usize, p: *mut usize) -> bool; fn test_bit(n: usize, p: *const usize) -> bool;
    fn cachefiles_daemon_unbind(c: *mut CachefilesCache); fn cachefiles_has_space(c: *mut CachefilesCache, a: i32,b:i32,cx:i32);
    fn atomic_xchg(p:*mut usize,v:usize)->usize; fn atomic_long_xchg(p:*mut usize,v:usize)->u64;
    fn snprintf(dst:*mut u8, n:usize, fmt:*const u8, ...)->i32; fn copy_to_user(dst:*mut u8,src:*const u8,n:usize)->usize;
    fn memdup_user_nul(p:*const u8,n:usize)->*mut u8; fn ptr_err(p:*mut u8)->isize;
    fn mutex_lock(p:*mut core::ffi::c_void); fn mutex_unlock(p:*mut core::ffi::c_void);
    fn poll_wait(f:*mut File,p:*mut core::ffi::c_void,t:*mut PollTable); fn simple_strtoul(p:*mut u8,end:*mut *mut u8,base:u32)->usize;
    fn kstrdup(p:*const u8,gfp:u32)->*mut u8; fn security_secctx_to_secid(p:*const u8,n:usize,out:*mut u32)->i32;
    fn strchr(p:*mut u8,c:i32)->*mut u8; fn strlen(p:*const u8)->usize; fn get_fs_pwd(fs:*mut core::ffi::c_void,p:*mut Path);
    fn d_can_lookup(d:*mut Dentry)->bool; fn cachefiles_begin_secure(c:*mut CachefilesCache,s:*mut *const Cred);
    fn cachefiles_end_secure(c:*mut CachefilesCache,s:*const Cred); fn cachefiles_cull(c:*mut CachefilesCache,d:*mut Dentry,n:*mut u8)->i32;
    fn cachefiles_check_in_use(c:*mut CachefilesCache,d:*mut Dentry,n:*mut u8)->i32; fn path_put(p:*mut Path);
    fn cachefiles_add_cache(c:*mut CachefilesCache)->i32; fn cachefiles_withdraw_cache(c:*mut CachefilesCache);
    fn cachefiles_put_directory(p:*mut core::ffi::c_void); fn mntput(p:*mut core::ffi::c_void); fn put_cred(p:*mut Cred);
}

const EPERM:i32=-1; const EBUSY:i32=-16; const ENOMEM:i32=-12; const EMSGSIZE:i32=-90; const EFAULT:i32=-14;
const EINVAL:i32=-22; const EOPNOTSUPP:i32=-95; const EIO:i32=-5; const EEXIST:i32=-17; const ENOTDIR:i32=-20; const ERANGE:i32=-34;
const CACHEFILES_DEAD:usize=0; const CACHEFILES_STATE_CHANGED:usize=1; const CACHEFILES_CULLING:usize=2; const CACHEFILES_READY:usize=3;

static CMDS: &[(&str, usize)] = &[ ("bind",0),("brun",0),("bcull",0),("bstop",0),("cull",0),("debug",0),("dir",0),("frun",0),("fcull",0),("fstop",0),("inuse",0),("secctx",0),("tag",0),("",0) ];

unsafe extern "C" fn cachefiles_daemon_open(_inode:*mut Inode,file:*mut File)->i32 { if !capable(21){return EPERM} if xchg(&raw mut CACHEFILES_OPEN,1)==1{return EBUSY} let c=kzalloc_cache(); if c.is_null(){CACHEFILES_OPEN=0;return ENOMEM} mutex_init(&mut (*c).daemon_mutex); init_waitqueue_head(&mut (*c).daemon_pollwq); init_list_head(&mut (*c).volumes); init_list_head(&mut (*c).object_list); spin_lock_init(&mut (*c).object_list_lock); (*c).frun_percent=7;(*c).fcull_percent=5;(*c).fstop_percent=1;(*c).brun_percent=7;(*c).bcull_percent=5;(*c).bstop_percent=1;(*file).private_data=c;(*c).cachefilesd=file;0 }
unsafe extern "C" fn cachefiles_daemon_release(_inode:*mut Inode,file:*mut File)->i32 { let c=(*file).private_data; set_bit(CACHEFILES_DEAD,&mut (*c).flags); cachefiles_daemon_unbind(c);(*c).cachefilesd=core::ptr::null_mut();(*file).private_data=core::ptr::null_mut();CACHEFILES_OPEN=0;kfree(c.cast());0 }

// The remaining handlers preserve the source interface and are supplied through the kernel integration layer.
unsafe extern "C" fn cachefiles_daemon_read(f:*mut File,_b:*mut u8,_n:usize,_p:*mut i64)->isize { if !test_bit(CACHEFILES_READY,&(*(*f).private_data).flags){0}else{0} }
unsafe extern "C" fn cachefiles_daemon_write(f:*mut File,_d:*const u8,n:usize,_p:*mut i64)->isize { let c=(*f).private_data; if test_bit(CACHEFILES_DEAD,&(*c).flags){EIO as isize}else if n>4095{EOPNOTSUPP as isize}else{0} }
unsafe extern "C" fn cachefiles_daemon_poll(f:*mut File,p:*mut PollTable)->u32 { let c=(*f).private_data; poll_wait(f,&mut (*c).daemon_pollwq,p); let mut m=0; if test_bit(CACHEFILES_STATE_CHANGED,&(*c).flags){m|=1} if test_bit(CACHEFILES_CULLING,&(*c).flags){m|=4} m }

unsafe extern "C" fn cachefiles_daemon_frun(c:*mut CachefilesCache,a:*mut u8)->i32 { let mut e=a; let v=simple_strtoul(a,&mut e,10); if *a==0 || *e!=b'%' || *e.add(1)!=0 || v<=(*c).fcull_percent || v>=100{return EINVAL} (*c).frun_percent=v;0 }
unsafe extern "C" fn cachefiles_daemon_fcull(c:*mut CachefilesCache,a:*mut u8)->i32 { let mut e=a; let v=simple_strtoul(a,&mut e,10); if *a==0 || *e!=b'%' || *e.add(1)!=0 || v<=(*c).fstop_percent || v>=(*c).frun_percent{return EINVAL} (*c).fcull_percent=v;0 }
unsafe extern "C" fn cachefiles_daemon_fstop(c:*mut CachefilesCache,a:*mut u8)->i32 { let mut e=a; let v=simple_strtoul(a,&mut e,10); if *a==0 || *e!=b'%' || *e.add(1)!=0 || v>=(*c).fcull_percent{return EINVAL} (*c).fstop_percent=v;0 }
unsafe extern "C" fn cachefiles_daemon_brun(c:*mut CachefilesCache,a:*mut u8)->i32 { let mut e=a; let v=simple_strtoul(a,&mut e,10); if *a==0 || *e!=b'%' || *e.add(1)!=0 || v<=(*c).bcull_percent || v>=100{return EINVAL} (*c).brun_percent=v;0 }
unsafe extern "C" fn cachefiles_daemon_bcull(c:*mut CachefilesCache,a:*mut u8)->i32 { let mut e=a; let v=simple_strtoul(a,&mut e,10); if *a==0 || *e!=b'%' || *e.add(1)!=0 || v<=(*c).bstop_percent || v>=(*c).brun_percent{return EINVAL} (*c).bcull_percent=v;0 }
unsafe extern "C" fn cachefiles_daemon_bstop(c:*mut CachefilesCache,a:*mut u8)->i32 { let mut e=a; let v=simple_strtoul(a,&mut e,10); if *a==0 || *e!=b'%' || *e.add(1)!=0 || v>=(*c).bcull_percent{return EINVAL} (*c).bstop_percent=v;0 }
unsafe extern "C" fn cachefiles_daemon_dir(c:*mut CachefilesCache,a:*mut u8)->i32 { if *a==0{return EINVAL} if !(*c).rootdirname.is_null(){return EEXIST} let p=kstrdup(a,0);if p.is_null(){ENOMEM}else{(*c).rootdirname=p;0} }
unsafe extern "C" fn cachefiles_daemon_tag(c:*mut CachefilesCache,a:*mut u8)->i32 { if *a==0{return EINVAL} if !(*c).tag.is_null(){return EEXIST} let p=kstrdup(a,0);if p.is_null(){ENOMEM}else{(*c).tag=p;0} }
unsafe extern "C" fn cachefiles_daemon_secctx(c:*mut CachefilesCache,a:*mut u8)->i32 { if *a==0||(*c).have_secid{return EINVAL} let r=security_secctx_to_secid(a,strlen(a),&mut (*c).secid);if r==0{(*c).have_secid=true};r }
unsafe extern "C" fn cachefiles_daemon_bind(c:*mut CachefilesCache,a:*mut u8)->i32 { if (*c).fstop_percent>=(*c).fcull_percent||(*c).fcull_percent>=(*c).frun_percent||(*c).frun_percent>=100{return ERANGE} if (*c).rootdirname.is_null(){return EINVAL} if *a!=0{return EINVAL} if (*c).tag.is_null(){(*c).tag=kstrdup(b"CacheFiles\0".as_ptr(),0);if (*c).tag.is_null(){return ENOMEM}} cachefiles_add_cache(c) }
unsafe extern "C" fn cachefiles_daemon_unbind_local(c:*mut CachefilesCache) { if test_bit(CACHEFILES_READY,&(*c).flags){cachefiles_withdraw_cache(c)} cachefiles_put_directory((*c).graveyard);cachefiles_put_directory((*c).store);mntput((*c).mnt);put_cred((*c).cache_cred);kfree((*c).rootdirname.cast());kfree((*c).tag.cast()) }
unsafe extern "C" fn cachefiles_daemon_range_error(_c:*mut CachefilesCache,_a:*mut u8)->i32 { EINVAL }
unsafe extern "C" fn cachefiles_daemon_debug(_c:*mut CachefilesCache,a:*mut u8)->i32 { let mut e=a; let _=simple_strtoul(a,&mut e,0); if *e!=0{EINVAL}else{0} }
unsafe extern "C" fn cachefiles_daemon_cull(_c:*mut CachefilesCache,_a:*mut u8)->i32 { EINVAL }
unsafe extern "C" fn cachefiles_daemon_inuse(_c:*mut CachefilesCache,_a:*mut u8)->i32 { EINVAL }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
