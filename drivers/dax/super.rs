// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2017 Intel Corporation. All rights reserved. */

// Kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct dax_device {
    pub inode: inode,
    pub cdev: cdev,
    pub private: *mut core::ffi::c_void,
    pub flags: libc::c_ulong,
    pub ops: *const dax_operations,
    pub holder_data: *mut core::ffi::c_void,
    pub holder_ops: *const dax_holder_operations,
}

#[repr(C)] pub struct inode { pub i_rdev: dev_t, pub i_cdev: *mut cdev, pub i_mode: libc::umode_t, pub i_flags: libc::c_ulong, pub i_data: address_space }
#[repr(C)] pub struct cdev { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct dax_operations { pub direct_access: unsafe extern "C" fn(*mut dax_device, pgoff_t, libc::c_long, dax_access_mode, *mut *mut core::ffi::c_void, *mut libc::c_ulong) -> libc::c_long, pub zero_page_range: unsafe extern "C" fn(*mut dax_device, pgoff_t, usize) -> libc::c_int, pub recovery_write: Option<unsafe extern "C" fn(*mut dax_device, pgoff_t, *mut core::ffi::c_void, usize, *mut iov_iter) -> usize> }
#[repr(C)] pub struct dax_holder_operations { pub notify_failure: unsafe extern "C" fn(*mut dax_device, u64, u64, libc::c_int) -> libc::c_int }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct pseudo_fs_context { pub ops: *const super_operations }
#[repr(C)] pub struct super_operations { _private: [u8; 0] }
#[repr(C)] pub struct file_system_type { _private: [u8; 0] }
#[repr(C)] pub struct vfsmount { pub mnt_sb: *mut super_block }
#[repr(C)] pub struct dev_dax { pub dev: device }
#[repr(C)] pub struct device { pub driver: *mut device_driver }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct dax_device_driver { pub type_: libc::c_int }
#[repr(C)] pub struct block_device { pub bd_disk: *mut gendisk }
#[repr(C)] pub struct gendisk { pub queue: *mut request_queue }
#[repr(C)] pub struct request_queue { _private: [u8; 0] }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct srcu_struct { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }

pub type dev_t = libc::dev_t; pub type pgoff_t = libc::c_ulong; pub type dax_access_mode = libc::c_int;
pub const DAXDEV_ALIVE: libc::c_uint = 0;
pub const DAXDEV_WRITE_CACHE: libc::c_uint = 1;
pub const DAXDEV_SYNC: libc::c_uint = 2;
pub const DAXDEV_NOCACHE: libc::c_uint = 3;
pub const DAXDEV_NOMC: libc::c_uint = 4;

extern "C" {
    static mut dax_srcu: srcu_struct; static mut dax_mnt: *mut vfsmount; static mut dax_cache: *mut kmem_cache; static mut dax_superblock: *mut super_block;
    fn srcu_read_lock(_: *mut srcu_struct) -> libc::c_int; fn srcu_read_unlock(_: *mut srcu_struct, _: libc::c_int); fn synchronize_srcu(_: *mut srcu_struct);
    fn dax_alive(_: *mut dax_device) -> bool; fn put_dax(_: *mut dax_device); fn dax_get_private(_: *mut dax_device) -> *mut core::ffi::c_void;
    fn igrab(_: *mut inode) -> *mut inode; fn iput(_: *mut inode); fn dax_mem2blk_err(_: libc::c_int) -> libc::c_int;
    fn test_bit(_: libc::c_uint, _: *const libc::c_ulong) -> bool; fn set_bit(_: libc::c_uint, _: *mut libc::c_ulong); fn clear_bit(_: libc::c_uint, _: *mut libc::c_ulong);
    fn _copy_from_iter(_: *mut core::ffi::c_void, _: usize, _: *mut iov_iter) -> usize; fn _copy_from_iter_flushcache(_: *mut core::ffi::c_void, _: usize, _: *mut iov_iter) -> usize; fn _copy_to_iter(_: *mut core::ffi::c_void, _: usize, _: *mut iov_iter) -> usize; fn _copy_mc_to_iter(_: *mut core::ffi::c_void, _: usize, _: *mut iov_iter) -> usize;
}

#[inline] pub unsafe fn dax_read_lock() -> libc::c_int { srcu_read_lock(&mut dax_srcu) }
#[inline] pub unsafe fn dax_read_unlock(id: libc::c_int) { srcu_read_unlock(&mut dax_srcu, id); }

pub unsafe fn dax_direct_access(d: *mut dax_device, p: pgoff_t, n: libc::c_long, m: dax_access_mode, k: *mut *mut core::ffi::c_void, f: *mut libc::c_ulong) -> libc::c_long { if d.is_null() || !dax_alive(d) || (*d).ops.is_null() || n < 0 { return -libc::EOPNOTSUPP as libc::c_long; } let a = ((*(*d).ops).direct_access)(d,p,n,m,k,f); if a == 0 { -libc::ERANGE as libc::c_long } else { core::cmp::min(a,n) } }
pub unsafe fn dax_copy_from_iter(d: *mut dax_device, _p: pgoff_t, a: *mut core::ffi::c_void, b: usize, i: *mut iov_iter) -> usize { if !dax_alive(d) { 0 } else if test_bit(DAXDEV_NOCACHE, &(*d).flags) { _copy_from_iter_flushcache(a,b,i) } else { _copy_from_iter(a,b,i) } }
pub unsafe fn dax_copy_to_iter(d: *mut dax_device, _p: pgoff_t, a: *mut core::ffi::c_void, b: usize, i: *mut iov_iter) -> usize { if !dax_alive(d) { 0 } else if test_bit(DAXDEV_NOMC, &(*d).flags) { _copy_mc_to_iter(a,b,i) } else { _copy_to_iter(a,b,i) } }
pub unsafe fn dax_recovery_write(d: *mut dax_device, p: pgoff_t, a: *mut core::ffi::c_void, b: usize, i: *mut iov_iter) -> usize { if d.is_null() || (*d).ops.is_null() { return 0 } match (*(*d).ops).recovery_write { Some(f) => f(d,p,a,b,i), None => 0 } }
pub unsafe fn dax_write_cache(d: *mut dax_device, wc: bool) { if wc { set_bit(DAXDEV_WRITE_CACHE,&mut (*d).flags) } else { clear_bit(DAXDEV_WRITE_CACHE,&mut (*d).flags) } }
pub unsafe fn dax_write_cache_enabled(d: *mut dax_device) -> bool { test_bit(DAXDEV_WRITE_CACHE,&(*d).flags) }
pub unsafe fn dax_synchronous(d: *mut dax_device) -> bool { test_bit(DAXDEV_SYNC,&(*d).flags) }
pub unsafe fn set_dax_synchronous(d: *mut dax_device) { set_bit(DAXDEV_SYNC,&mut (*d).flags) }
pub unsafe fn set_dax_nocache(d: *mut dax_device) { set_bit(DAXDEV_NOCACHE,&mut (*d).flags) }
pub unsafe fn set_dax_nomc(d: *mut dax_device) { set_bit(DAXDEV_NOMC,&mut (*d).flags) }
pub unsafe fn dax_alive_local(d: *mut dax_device) -> bool { dax_alive(d) }
pub unsafe fn kill_dax(d: *mut dax_device) { if !d.is_null() { clear_bit(DAXDEV_ALIVE,&mut (*d).flags); synchronize_srcu(&mut dax_srcu); (*d).holder_ops=core::ptr::null(); (*d).holder_data=core::ptr::null_mut(); } }
pub unsafe fn run_dax(d: *mut dax_device) { set_bit(DAXDEV_ALIVE,&mut (*d).flags) }
pub unsafe fn dax_holder(d: *mut dax_device) -> *mut core::ffi::c_void { (*d).holder_data }
pub unsafe fn dax_inode(d: *mut dax_device) -> *mut inode { &mut (*d).inode }
pub unsafe fn dax_get_private_local(d: *mut dax_device) -> *mut core::ffi::c_void { if !test_bit(DAXDEV_ALIVE,&(*d).flags) { core::ptr::null_mut() } else { (*d).private } }

pub unsafe fn dax_zero_page_range(d: *mut dax_device, p: pgoff_t, n: usize) -> libc::c_int { if !dax_alive(d) { return -libc::ENXIO }; if (*d).ops.is_null() { return -libc::EOPNOTSUPP }; if n != 1 { return -libc::EIO }; dax_mem2blk_err(((*(*d).ops).zero_page_range)(d,p,n)) }
pub unsafe fn dax_holder_notify_failure(d: *mut dax_device, off: u64, len: u64, flags: libc::c_int) -> libc::c_int { let id=dax_read_lock(); let r=if !dax_alive(d) {-libc::ENXIO} else { let o=(*d).holder_ops; if o.is_null() {-libc::EOPNOTSUPP} else { ((*o).notify_failure)(d,off,len,flags) } }; dax_read_unlock(id); r }
pub unsafe fn dax_flush(_d: *mut dax_device, _a: *mut core::ffi::c_void, _s: usize) {}
pub unsafe fn dax_set_ops(d: *mut dax_device, o: *const dax_operations) -> libc::c_int { if !o.is_null() && !(*d).ops.is_null() { -libc::EBUSY } else { (*d).ops=o; 0 } }
pub unsafe fn fs_put_dax(d: *mut dax_device, h: *mut core::ffi::c_void) { if !d.is_null() && !h.is_null() { (*d).holder_ops=core::ptr::null(); if (*d).holder_data==h { (*d).holder_data=core::ptr::null_mut(); } } put_dax(d); }
pub unsafe fn fs_dax_get(d: *mut dax_device, h: *mut core::ffi::c_void, o: *const dax_holder_operations) -> libc::c_int { if d.is_null() || !dax_alive(d) { return -libc::ENODEV }; if !(*d).holder_data.is_null() { return -libc::EBUSY }; (*d).holder_data=h; (*d).holder_ops=o; 0 }
pub unsafe fn put_dax_local(d: *mut dax_device) { if !d.is_null() { iput(&mut (*d).inode) } }
pub unsafe fn inode_dax(i: *mut inode) -> *mut dax_device { (i as *mut u8).sub(core::mem::offset_of!(dax_device,cdev)) as *mut dax_device }
pub unsafe fn dax_dev_get(_devt: dev_t) -> *mut dax_device { core::ptr::null_mut() }
pub unsafe fn alloc_dax(private: *mut core::ffi::c_void, ops: *const dax_operations) -> *mut dax_device { let d=libc::malloc(core::mem::size_of::<dax_device>()) as *mut dax_device; if d.is_null() { return core::ptr::null_mut() }; core::ptr::write_bytes(d,0,1); (*d).private=private; (*d).ops=ops; d }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
