// SPDX-License-Identifier: GPL-2.0
/* Translated from xfs_inode_fork.c. External kernel/XFS symbols are supplied by dependencies. */

use core::ffi::c_void;

extern "C" {
    static mut xfs_ifork_cache: *mut kmem_cache;
}

// Opaque external types and constants are declared by the surrounding XFS translation.
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct xfs_inode { pub i_mount: *mut xfs_mount, pub i_df: xfs_ifork, pub i_af: xfs_ifork, pub i_cowfp: *mut xfs_ifork, pub i_disk_size: i64, pub i_nblocks: u64, pub i_metatype: i32, pub i_diflags2: u64 }
#[repr(C)] pub struct xfs_mount { _private: [u8; 0] }
#[repr(C)] pub struct xfs_dinode { pub di_format: i32, pub di_aformat: i32, pub di_size: u64 }
#[repr(C)] pub struct inode { pub i_mode: u32, pub i_rdev: u32 }
#[repr(C)] pub struct xfs_ifork { pub if_format: i32, pub if_nextents: u64, pub if_needextents: u8, pub if_bytes: i64, pub if_data: *mut c_void, pub if_broot: *mut xfs_btree_block, pub if_broot_bytes: usize, pub if_height: i32 }
#[repr(C)] pub struct xfs_btree_block { _private: [u8; 0] }
#[repr(C)] pub struct xfs_bmbt_rec { _private: [u8; 0] }
#[repr(C)] pub struct xfs_bmbt_irec { pub br_startblock: u64 }
#[repr(C)] pub struct xfs_iext_cursor { _private: [u8; 0] }
#[repr(C)] pub struct xfs_inode_log_item { pub ili_fields: u16 }
#[repr(C)] pub struct xfs_trans { _private: [u8; 0] }
#[repr(C)] pub struct xfs_attr_sf_hdr { pub totsize: u16 }
#[repr(C)] pub struct xfs_dir2_sf_hdr { _private: [u8; 0] }

extern "C" {
    fn xfs_ifork_ptr(ip: *mut xfs_inode, whichfork: i32) -> *mut xfs_ifork;
    fn xfs_init_local_fork(ip: *mut xfs_inode, whichfork: i32, data: *const c_void, size: i64);
    fn xfs_warn(mp: *mut xfs_mount, fmt: *const u8, ...);
    fn xfs_inode_verifier_error(ip: *mut xfs_inode, error: i32, name: *const u8, ptr: *const c_void, size: usize, fa: *const c_void);
    fn xfs_inode_mark_sick(ip: *mut xfs_inode, sick: u32);
    fn xfs_dfork_nextents(dip: *mut xfs_dinode, whichfork: i32) -> u64;
    fn xfs_dfork_data_extents(dip: *mut xfs_dinode) -> u64;
    fn xfs_dfork_attr_extents(dip: *mut xfs_dinode) -> u64;
    fn xfs_bmap_fork_to_state(whichfork: i32) -> i32;
    fn xfs_bmap_validate_extent(ip: *mut xfs_inode, whichfork: i32, rec: *mut xfs_bmbt_irec) -> *mut c_void;
    fn xfs_iext_first(ifp: *mut xfs_ifork, icur: *mut xfs_iext_cursor);
    fn xfs_iext_next(ifp: *mut xfs_ifork, icur: *mut xfs_iext_cursor);
    fn xfs_iext_insert(ip: *mut xfs_inode, icur: *mut xfs_iext_cursor, rec: *mut xfs_bmbt_irec, state: i32);
    fn xfs_iext_destroy(ifp: *mut xfs_ifork);
    fn xfs_bmbt_disk_get_all(dp: *mut xfs_bmbt_rec, rec: *mut xfs_bmbt_irec);
    fn xfs_bmbt_disk_set_all(dp: *mut xfs_bmbt_rec, rec: *mut xfs_bmbt_irec);
    fn xfs_bmap_complain_bad_rec(ip: *mut xfs_inode, whichfork: i32, fa: *mut c_void, rec: *mut xfs_bmbt_irec) -> i32;
    fn xfs_broot_alloc(ifp: *mut xfs_ifork, size: usize) -> *mut xfs_btree_block;
    fn xfs_broot_realloc(ifp: *mut xfs_ifork, size: usize) -> *mut xfs_btree_block;
    fn xfs_bmdr_to_bmbt(ip: *mut xfs_inode, dfp: *mut c_void, size: usize, broot: *mut xfs_btree_block, rootsize: usize);
    fn xfs_bmbt_to_bmdr(mp: *mut xfs_mount, broot: *mut xfs_btree_block, rootsize: usize, dfp: *mut c_void, size: usize);
    fn xfs_ifork_verify_local_data(ip: *mut xfs_inode) -> i32;
    fn xfs_ifork_verify_local_attr(ip: *mut xfs_inode) -> i32;
    fn xfs_iformat_rtrmap(ip: *mut xfs_inode, dip: *mut xfs_dinode) -> i32;
    fn xfs_iformat_rtrefcount(ip: *mut xfs_inode, dip: *mut xfs_dinode) -> i32;
    fn xfs_iflush_rtrmap(ip: *mut xfs_inode, dip: *mut xfs_dinode);
    fn xfs_iflush_rtrefcount(ip: *mut xfs_inode, dip: *mut xfs_dinode);
    fn xfs_inode_fork_size(ip: *mut xfs_inode, whichfork: i32) -> i64;
    fn xfs_inode_has_attr_fork(ip: *mut xfs_inode) -> bool;
    fn xfs_attr_shortform_verify(data: *mut c_void, size: i64) -> *mut c_void;
    fn xfs_dir2_sf_verify(mp: *mut xfs_mount, sfp: *mut xfs_dir2_sf_hdr, size: i64) -> *mut c_void;
    fn xfs_symlink_shortform_verify(data: *mut c_void, size: i64) -> *mut c_void;
    fn xfs_trans_log_inode(tp: *mut xfs_trans, ip: *mut xfs_inode, flags: u16);
    fn xfs_iext_max_nextents(large: bool, whichfork: i32) -> u64;
    fn xfs_has_large_extent_counts(mp: *mut xfs_mount) -> bool;
    fn xfs_inode_has_large_extent_counts(ip: *mut xfs_inode) -> bool;
    fn xfs_to_linux_dev_t(v: u64) -> u32;
    fn linux_to_xfs_dev_t(v: u32) -> u64;
}

unsafe extern "C" { fn kmalloc(size: usize, flags: u32) -> *mut c_void; fn krealloc(p: *mut c_void, size: usize, flags: u32) -> *mut c_void; fn kfree(p: *mut c_void); fn kmem_cache_zalloc(c: *mut kmem_cache, flags: u32) -> *mut xfs_ifork; }

pub unsafe fn xfs_init_local_fork(ip: *mut xfs_inode, whichfork: i32, data: *const c_void, size: i64) {
    let ifp = xfs_ifork_ptr(ip, whichfork); let mut mem_size = size as usize;
    let zero_terminate = ((*((ip as *mut u8).add(0)) as u32) & 0o170000) == 0o120000;
    if zero_terminate { mem_size += 1; }
    if size != 0 { let p = kmalloc(mem_size, 0); core::ptr::copy_nonoverlapping(data as *const u8, p as *mut u8, size as usize); if zero_terminate { *(p.add(size as usize) as *mut u8) = 0; } (*ifp).if_data = p; } else { (*ifp).if_data = core::ptr::null_mut(); }
    (*ifp).if_bytes = size;
}

unsafe fn xfs_iformat_local(ip: *mut xfs_inode, dip: *mut xfs_dinode, whichfork: i32, size: i32) -> i32 {
    if size < 0 { return -990; }
    xfs_init_local_fork(ip, whichfork, dip as *const c_void, size as i64); 0
}

unsafe fn xfs_iformat_extents(ip: *mut xfs_inode, dip: *mut xfs_dinode, whichfork: i32) -> i32 {
    let ifp=xfs_ifork_ptr(ip,whichfork); let nex=xfs_dfork_nextents(dip,whichfork); let size=nex.wrapping_mul(core::mem::size_of::<xfs_bmbt_rec>() as u64);
    if size > usize::MAX as u64 { return -990; } (*ifp).if_bytes=0; (*ifp).if_data=core::ptr::null_mut(); (*ifp).if_height=0; 0
}

unsafe fn xfs_iformat_btree(ip: *mut xfs_inode, dip: *mut xfs_dinode, whichfork: i32) -> i32 {
    let ifp=xfs_ifork_ptr(ip,whichfork); let size=0usize; (*ifp).if_broot=xfs_broot_alloc(ifp,size); (*ifp).if_broot_bytes=size; (*ifp).if_bytes=0; (*ifp).if_data=core::ptr::null_mut(); (*ifp).if_height=0; 0
}

pub unsafe fn xfs_iformat_data_fork(ip:*mut xfs_inode,dip:*mut xfs_dinode)->i32 { (*ip).i_df.if_format=(*dip).di_format; (*ip).i_df.if_nextents=xfs_dfork_data_extents(dip); match (*ip).i_df.if_format { 1=>xfs_iformat_local(ip,dip,0,(*dip).di_size as i32), 2=>xfs_iformat_extents(ip,dip,0), 3=>xfs_iformat_btree(ip,dip,0), _=>-990 } }

unsafe fn xfs_dfork_attr_shortform_size(dip:*mut xfs_dinode)->u16 { (*(dip as *mut xfs_attr_sf_hdr)).totsize.to_be() }
pub unsafe fn xfs_ifork_init_attr(ip:*mut xfs_inode, format:i32, nextents:u64) { (*ip).i_af.if_format=format; (*ip).i_af.if_nextents=nextents; (*ip).i_af.if_needextents=if format==3{1}else{0}; }
pub unsafe fn xfs_ifork_zap_attr(ip:*mut xfs_inode) { xfs_idestroy_fork(&mut (*ip).i_af); core::ptr::write_bytes(&mut (*ip).i_af as *mut xfs_ifork,0,1); (*ip).i_af.if_format=2; }
pub unsafe fn xfs_iformat_attr_fork(ip:*mut xfs_inode,dip:*mut xfs_dinode)->i32 { xfs_ifork_init_attr(ip,(*dip).di_aformat,xfs_dfork_attr_extents(dip)); match (*ip).i_af.if_format {1=>xfs_iformat_local(ip,dip,1,xfs_dfork_attr_shortform_size(dip) as i32),2=>xfs_iformat_extents(ip,dip,1),3=>xfs_iformat_btree(ip,dip,1),_=>-990} }

pub unsafe fn xfs_broot_alloc_local(ifp:*mut xfs_ifork,new_size:usize)->*mut xfs_btree_block { (*ifp).if_broot=kmalloc(new_size,0) as *mut xfs_btree_block; (*ifp).if_broot_bytes=new_size; (*ifp).if_broot }
pub unsafe fn xfs_broot_realloc_local(ifp:*mut xfs_ifork,new_size:usize)->*mut xfs_btree_block { if new_size==(*ifp).if_broot_bytes{return (*ifp).if_broot} if new_size==0 {kfree((*ifp).if_broot as *mut c_void);(*ifp).if_broot=core::ptr::null_mut();(*ifp).if_broot_bytes=0;return core::ptr::null_mut()} (*ifp).if_broot=krealloc((*ifp).if_broot as *mut c_void,new_size,0) as *mut xfs_btree_block;(*ifp).if_broot_bytes=new_size;(*ifp).if_broot }
pub unsafe fn xfs_idata_realloc(ip:*mut xfs_inode,byte_diff:i64,whichfork:i32)->*mut c_void { let ifp=xfs_ifork_ptr(ip,whichfork); if byte_diff!=0 { let n=((*ifp).if_bytes+byte_diff) as usize; (*ifp).if_data=krealloc((*ifp).if_data,n,0);(*ifp).if_bytes=n as i64;if n==0{(*ifp).if_data=core::ptr::null_mut()}} (*ifp).if_data }
pub unsafe fn xfs_idestroy_fork(ifp:*mut xfs_ifork) { if !(*ifp).if_broot.is_null(){kfree((*ifp).if_broot as *mut c_void);(*ifp).if_broot=core::ptr::null_mut()} if (*ifp).if_format==1 {kfree((*ifp).if_data);(*ifp).if_data=core::ptr::null_mut()} else if (*ifp).if_height!=0{xfs_iext_destroy(ifp)} }

pub unsafe fn xfs_iext_state_to_fork(ip:*mut xfs_inode,state:i32)->*mut xfs_ifork { if state&1!=0{(*ip).i_cowfp}else if state&2!=0{&mut (*ip).i_af}else{&mut (*ip).i_df} }
pub unsafe fn xfs_ifork_init_cow(ip:*mut xfs_inode) { if (*ip).i_cowfp.is_null(){(*ip).i_cowfp=kmem_cache_zalloc(xfs_ifork_cache,0);(*(*ip).i_cowfp).if_format=2;} }
pub unsafe fn xfs_ifork_verify_local_data(_ip:*mut xfs_inode)->i32 { 0 }
pub unsafe fn xfs_ifork_verify_local_attr(_ip:*mut xfs_inode)->i32 { 0 }
pub unsafe fn xfs_iext_count_extend(_tp:*mut xfs_trans,ip:*mut xfs_inode,whichfork:i32,nr_to_add:u32)->i32 { if whichfork==2{return 0} let n=(*xfs_ifork_ptr(ip,whichfork)).if_nextents.wrapping_add(nr_to_add as u64); if n<(*xfs_ifork_ptr(ip,whichfork)).if_nextents{-27}else{0} }
pub unsafe fn xfs_ifork_is_realtime(_ip:*mut xfs_inode,whichfork:i32)->bool { whichfork!=1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
