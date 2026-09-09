// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */
// Translated from xfs_inode.h. Included C dependencies are supplied elsewhere.

#[repr(C)]
pub struct xfs_inode {
    pub i_mount: *mut xfs_mount,
    pub i_udquot: *mut xfs_dquot,
    pub i_gdquot: *mut xfs_dquot,
    pub i_pdquot: *mut xfs_dquot,
    pub i_imap: xfs_imap,
    pub i_cowfp: *mut xfs_ifork,
    pub i_df: xfs_ifork,
    pub i_af: xfs_ifork,
    pub i_itemp: *mut xfs_inode_log_item,
    pub i_lock: rw_semaphore,
    pub i_pincount: atomic_t,
    pub i_gclist: llist_node,
    pub i_checked: u16,
    pub i_sick: u16,
    pub i_flags_lock: spinlock_t,
    pub i_flags: c_ulong,
    pub i_delayed_blks: u64,
    pub i_disk_size: xfs_fsize_t,
    pub i_nblocks: xfs_rfsblock_t,
    pub i_projid: prid_t,
    pub i_extsize: xfs_extlen_t,
    pub i_extent_union: xfs_inode_extent_union,
    pub i_forkoff: u8,
    pub i_metatype: xfs_metafile_type,
    pub i_diflags: u16,
    pub i_diflags2: u64,
    pub i_crtime: timespec64,
    pub i_next_unlinked: xfs_agino_t,
    pub i_prev_unlinked: xfs_agino_t,
    pub i_vnode: inode,
    pub i_ioend_lock: spinlock_t,
    pub i_ioend_work: work_struct,
    pub i_ioend_list: list_head,
}

#[repr(C)]
pub union xfs_inode_extent_union {
    pub i_used_blocks: u32,
    pub i_cowextsize: xfs_extlen_t,
    pub i_flushiter: u16,
}

pub type xfs_inode_t = xfs_inode;

pub unsafe fn xfs_inode_on_unlinked_list(ip: *const xfs_inode) -> bool { (*ip).i_prev_unlinked != 0 }
pub unsafe fn xfs_inode_has_attr_fork(ip: *const xfs_inode) -> bool { (*ip).i_forkoff > 0 }

pub unsafe fn xfs_ifork_ptr(ip: *mut xfs_inode, whichfork: c_int) -> *mut xfs_ifork {
    match whichfork {
        XFS_DATA_FORK => &mut (*ip).i_df,
        XFS_ATTR_FORK => if !xfs_inode_has_attr_fork(ip) { core::ptr::null_mut() } else { &mut (*ip).i_af },
        XFS_COW_FORK => (*ip).i_cowfp,
        _ => { ASSERT(0); core::ptr::null_mut() }
    }
}
pub unsafe fn xfs_inode_fork_boff(ip: *mut xfs_inode) -> c_uint { (*ip).i_forkoff.wrapping_shl(3) as c_uint }
pub unsafe fn xfs_inode_data_fork_size(ip: *mut xfs_inode) -> c_uint { if xfs_inode_has_attr_fork(ip) { xfs_inode_fork_boff(ip) } else { XFS_LITINO((*ip).i_mount) } }
pub unsafe fn xfs_inode_attr_fork_size(ip: *mut xfs_inode) -> c_uint { if xfs_inode_has_attr_fork(ip) { XFS_LITINO((*ip).i_mount) - xfs_inode_fork_boff(ip) } else { 0 } }
pub unsafe fn xfs_inode_fork_size(ip: *mut xfs_inode, whichfork: c_int) -> c_uint { match whichfork { XFS_DATA_FORK => xfs_inode_data_fork_size(ip), XFS_ATTR_FORK => xfs_inode_attr_fork_size(ip), _ => 0 } }

pub unsafe fn XFS_I(inode: *mut inode) -> *mut xfs_inode { container_of(inode, xfs_inode, i_vnode) }
pub unsafe fn VFS_I(ip: *mut xfs_inode) -> *mut inode { &mut (*ip).i_vnode }
pub unsafe fn VFS_IC(ip: *const xfs_inode) -> *const inode { &(*ip).i_vnode }
pub unsafe fn I_INO(ip: *const xfs_inode) -> xfs_ino_t { (*VFS_IC(ip)).i_ino }
pub unsafe fn XFS_ISIZE(ip: *mut xfs_inode) -> xfs_fsize_t { if S_ISREG((*VFS_I(ip)).i_mode) { i_size_read(VFS_I(ip)) } else { (*ip).i_disk_size } }
pub unsafe fn xfs_new_eof(ip: *mut xfs_inode, mut new_size: xfs_fsize_t) -> xfs_fsize_t { let i_size = i_size_read(VFS_I(ip)); if new_size > i_size || new_size < 0 { new_size = i_size; } if new_size > (*ip).i_disk_size { new_size } else { 0 } }

pub unsafe fn __xfs_iflags_set(ip: *mut xfs_inode, flags: c_ulong) { (*ip).i_flags |= flags; }
pub unsafe fn xfs_iflags_set(ip: *mut xfs_inode, flags: c_ulong) { spin_lock(&mut (*ip).i_flags_lock); __xfs_iflags_set(ip, flags); spin_unlock(&mut (*ip).i_flags_lock); }
pub unsafe fn xfs_iflags_clear(ip: *mut xfs_inode, flags: c_ulong) { spin_lock(&mut (*ip).i_flags_lock); (*ip).i_flags &= !flags; spin_unlock(&mut (*ip).i_flags_lock); }
pub unsafe fn __xfs_iflags_test(ip: *const xfs_inode, flags: c_ulong) -> c_int { ((*ip).i_flags & flags) as c_int }
pub unsafe fn xfs_iflags_test(ip: *mut xfs_inode, flags: c_ulong) -> c_int { spin_lock(&mut (*ip).i_flags_lock); let ret = __xfs_iflags_test(ip, flags); spin_unlock(&mut (*ip).i_flags_lock); ret }
pub unsafe fn xfs_iflags_test_and_clear(ip: *mut xfs_inode, flags: c_ulong) -> c_int { spin_lock(&mut (*ip).i_flags_lock); let ret = ((*ip).i_flags & flags) as c_int; if ret != 0 { (*ip).i_flags &= !flags; } spin_unlock(&mut (*ip).i_flags_lock); ret }
pub unsafe fn xfs_iflags_test_and_set(ip: *mut xfs_inode, flags: c_ulong) -> c_int { spin_lock(&mut (*ip).i_flags_lock); let ret = ((*ip).i_flags & flags) as c_int; if ret == 0 { (*ip).i_flags |= flags; } spin_unlock(&mut (*ip).i_flags_lock); ret }

pub unsafe fn xfs_is_reflink_inode(ip: *const xfs_inode) -> bool { (*ip).i_diflags2 & XFS_DIFLAG2_REFLINK != 0 }
pub unsafe fn xfs_is_metadir_inode(ip: *const xfs_inode) -> bool { (*ip).i_diflags2 & XFS_DIFLAG2_METADATA != 0 }
pub unsafe fn xfs_is_internal_inode(ip: *const xfs_inode) -> bool { let mp=(*ip).i_mount; if xfs_has_metadir(mp) { return xfs_is_metadir_inode(ip); } I_INO(ip)==(*mp).m_sb.sb_rbmino || I_INO(ip)==(*mp).m_sb.sb_rsumino || xfs_is_quota_inode(&(*mp).m_sb, I_INO(ip)) }
pub unsafe fn xfs_is_zoned_inode(ip: *const xfs_inode) -> bool { xfs_has_zoned((*ip).i_mount) && XFS_IS_REALTIME_INODE(ip) }
extern "C" { pub fn xfs_is_always_cow_inode(ip: *const xfs_inode) -> bool; }
pub unsafe fn xfs_is_cow_inode(ip: *const xfs_inode) -> bool { xfs_is_reflink_inode(ip) || xfs_is_always_cow_inode(ip) }
pub unsafe fn xfs_inode_has_filedata(ip: *const xfs_inode) -> bool { (*ip).i_df.if_nextents > 0 || (*ip).i_delayed_blks > 0 }
pub unsafe fn xfs_inode_has_cow_data(ip: *const xfs_inode) -> bool { !(*ip).i_cowfp.is_null() && (*(*ip).i_cowfp).if_bytes != 0 }
pub unsafe fn xfs_inode_has_bigtime(ip: *const xfs_inode) -> bool { (*ip).i_diflags2 & XFS_DIFLAG2_BIGTIME != 0 }
pub unsafe fn xfs_inode_has_large_extent_counts(ip: *const xfs_inode) -> bool { (*ip).i_diflags2 & XFS_DIFLAG2_NREXT64 != 0 }
pub unsafe fn xfs_inode_has_bigrtalloc(ip: *const xfs_inode) -> bool { XFS_IS_REALTIME_INODE(ip) && (*(*ip).i_mount).m_sb.sb_rextsize > 1 }

pub const XFS_IRECLAIM: c_ulong = 1<<0; pub const XFS_ISTALE: c_ulong=1<<1; pub const XFS_IRECLAIMABLE:c_ulong=1<<2; pub const XFS_INEW:c_ulong=1<<3; pub const XFS_IPRESERVE_DM_FIELDS:c_ulong=1<<4; pub const XFS_ITRUNCATED:c_ulong=1<<5; pub const XFS_EOFBLOCKS_RELEASED:c_ulong=1<<6; pub const XFS_IFLUSHING:c_ulong=1<<7; pub const __XFS_IPINNED_BIT:u32=8; pub const XFS_IPINNED:c_ulong=1<<8; pub const XFS_IEOFBLOCKS:c_ulong=1<<9; pub const XFS_NEED_INACTIVE:c_ulong=1<<10; pub const XFS_IRECOVERY:c_ulong=1<<11; pub const XFS_ICOWBLOCKS:c_ulong=1<<12; pub const XFS_INACTIVATING:c_ulong=1<<13; pub const XFS_IQUOTAUNCHECKED:c_ulong=1<<14; pub const XFS_IREMAPPING:c_ulong=1<<15;
pub const XFS_ALL_IRECLAIM_FLAGS:c_ulong=XFS_IRECLAIMABLE|XFS_IRECLAIM|XFS_NEED_INACTIVE|XFS_INACTIVATING;
pub const XFS_IRECLAIM_RESET_FLAGS:c_ulong=XFS_IRECLAIMABLE|XFS_IRECLAIM|XFS_EOFBLOCKS_RELEASED|XFS_ITRUNCATED|XFS_NEED_INACTIVE|XFS_INACTIVATING|XFS_IQUOTAUNCHECKED;
pub const XFS_IOLOCK_EXCL:u32=1<<0; pub const XFS_IOLOCK_SHARED:u32=1<<1; pub const XFS_ILOCK_EXCL:u32=1<<2; pub const XFS_ILOCK_SHARED:u32=1<<3; pub const XFS_MMAPLOCK_EXCL:u32=1<<4; pub const XFS_MMAPLOCK_SHARED:u32=1<<5;
pub const XFS_LOCK_MASK:u32=XFS_IOLOCK_EXCL|XFS_IOLOCK_SHARED|XFS_ILOCK_EXCL|XFS_ILOCK_SHARED|XFS_MMAPLOCK_EXCL|XFS_MMAPLOCK_SHARED;
pub const XFS_IOLOCK_SHIFT:u32=16; pub const XFS_IOLOCK_MAX_SUBCLASS:u32=3; pub const XFS_IOLOCK_DEP_MASK:u32=0x000f0000; pub const XFS_MMAPLOCK_SHIFT:u32=20; pub const XFS_MMAPLOCK_NUMORDER:u32=0; pub const XFS_MMAPLOCK_MAX_SUBCLASS:u32=3; pub const XFS_MMAPLOCK_DEP_MASK:u32=0x00f00000; pub const XFS_ILOCK_SHIFT:u32=24; pub const XFS_ILOCK_PARENT_VAL:u32=5; pub const XFS_ILOCK_MAX_SUBCLASS:u32=4; pub const XFS_ILOCK_DEP_MASK:u32=0xff000000; pub const XFS_ILOCK_PARENT:u32=5<<24; pub const XFS_LOCK_SUBCLASS_MASK:u32=XFS_IOLOCK_DEP_MASK|XFS_MMAPLOCK_DEP_MASK|XFS_ILOCK_DEP_MASK;
pub const XFS_DEFAULT_COWEXTSZ_HINT:u32=32;

#[repr(C)] pub enum layout_break_reason { BREAK_WRITE, BREAK_UNMAP }

extern "C" {
    pub fn xfs_inactive(ip:*mut xfs_inode)->c_int; pub fn xfs_lookup(dp:*mut xfs_inode,name:*const xfs_name,ipp:*mut *mut xfs_inode,ci_name:*mut xfs_name)->c_int; pub fn xfs_create(iargs:*const xfs_icreate_args,name:*mut xfs_name,ipp:*mut *mut xfs_inode)->c_int; pub fn xfs_create_tmpfile(iargs:*const xfs_icreate_args,ipp:*mut *mut xfs_inode)->c_int; pub fn xfs_remove(dp:*mut xfs_inode,name:*mut xfs_name,ip:*mut xfs_inode)->c_int; pub fn xfs_link(tdp:*mut xfs_inode,sip:*mut xfs_inode,target_name:*mut xfs_name)->c_int; pub fn xfs_rename(idmap:*mut mnt_idmap,src_dp:*mut xfs_inode,src_name:*mut xfs_name,src_ip:*mut xfs_inode,target_dp:*mut xfs_inode,target_name:*mut xfs_name,target_ip:*mut xfs_inode,flags:c_uint)->c_int;
    pub fn xfs_ilock(ip:*mut xfs_inode,mode:c_uint); pub fn xfs_ilock_nowait(ip:*mut xfs_inode,mode:c_uint)->c_int; pub fn xfs_iunlock(ip:*mut xfs_inode,mode:c_uint); pub fn xfs_ilock_demote(ip:*mut xfs_inode,mode:c_uint); pub fn xfs_assert_ilocked(ip:*mut xfs_inode,mode:c_uint); pub fn xfs_ilock_data_map_shared(ip:*mut xfs_inode)->c_uint; pub fn xfs_ilock_attr_map_shared(ip:*mut xfs_inode)->c_uint; pub fn xfs_ifree(tp:*mut xfs_trans,ip:*mut xfs_inode)->c_int; pub fn xfs_itruncate_extents_flags(tpp:*mut *mut xfs_trans,ip:*mut xfs_inode,whichfork:c_int,new_size:xfs_fsize_t,flags:c_int)->c_int; pub fn xfs_iext_realloc(ip:*mut xfs_inode,whichfork:c_int,nr:c_int); pub fn xfs_log_force_inode(ip:*mut xfs_inode)->c_int; pub fn xfs_iunpin_wait(ip:*mut xfs_inode); pub fn xfs_iflush_cluster(bp:*mut xfs_buf)->c_int; pub fn xfs_lock_two_inodes(ip0:*mut xfs_inode,ip0_mode:c_uint,ip1:*mut xfs_inode,ip1_mode:c_uint); pub fn xfs_icreate(tp:*mut xfs_trans,ino:xfs_ino_t,args:*const xfs_icreate_args,ipp:*mut *mut xfs_inode)->c_int; pub fn xfs_break_dax_layouts(inode:*mut inode)->c_int; pub fn xfs_break_layouts(inode:*mut inode,iolock:*mut c_uint,reason:layout_break_reason)->c_int; pub fn xfs_irele(ip:*mut xfs_inode); pub static mut xfs_inode_cache:*mut kmem_cache; pub fn xfs_inode_needs_inactive(ip:*mut xfs_inode)->bool; pub fn xfs_iunlink_lookup(pag:*mut xfs_perag,agino:xfs_agino_t)->*mut xfs_inode; pub fn xfs_iunlink_reload_next(tp:*mut xfs_trans,agibp:*mut xfs_buf,prev_agino:xfs_agino_t,next_agino:xfs_agino_t)->c_int; pub fn xfs_end_io(work:*mut work_struct); pub fn xfs_ilock2_io_mmap(ip1:*mut xfs_inode,ip2:*mut xfs_inode)->c_int; pub fn xfs_iunlock2_io_mmap(ip1:*mut xfs_inode,ip2:*mut xfs_inode); pub fn xfs_iunlock2_remapping(ip1:*mut xfs_inode,ip2:*mut xfs_inode); pub fn xfs_lock_inodes(ips:*mut *mut xfs_inode,inodes:c_int,lock_mode:c_uint); pub fn xfs_sort_inodes(i_tab:*mut *mut xfs_inode,num_inodes:c_uint); pub fn xfs_inode_reload_unlinked_bucket(tp:*mut xfs_trans,ip:*mut xfs_inode)->c_int; pub fn xfs_inode_reload_unlinked(ip:*mut xfs_inode)->c_int; pub fn xfs_ifork_zapped(ip:*const xfs_inode,whichfork:c_int)->bool; pub fn xfs_inode_count_blocks(tp:*mut xfs_trans,ip:*mut xfs_inode,dblocks:*mut xfs_filblks_t,rblocks:*mut xfs_filblks_t); pub fn xfs_inode_alloc_unitsize(ip:*mut xfs_inode)->c_uint; pub fn xfs_icreate_dqalloc(args:*const xfs_icreate_args,udqpp:*mut *mut xfs_dquot,gdqpp:*mut *mut xfs_dquot,pdqpp:*mut *mut xfs_dquot)->c_int;
}

pub unsafe fn xfs_inode_buftarg(ip: *const xfs_inode) -> *mut xfs_buftarg { if XFS_IS_REALTIME_INODE(ip) { (*(*ip).i_mount).m_rtdev_targp } else { (*(*ip).i_mount).m_ddev_targp } }
pub unsafe fn xfs_inode_can_hw_atomic_write(ip: *const xfs_inode) -> bool { if IS_DAX(VFS_IC(ip)) { return false; } (*xfs_inode_buftarg(ip)).bt_awu_max > 0 }
pub unsafe fn xfs_inode_can_sw_atomic_write(ip: *const xfs_inode) -> bool { if IS_DAX(VFS_IC(ip)) { return false; } xfs_can_sw_atomic_write((*ip).i_mount) }
pub unsafe fn xfs_itruncate_extents(tpp:*mut *mut xfs_trans,ip:*mut xfs_inode,whichfork:c_int,new_size:xfs_fsize_t)->c_int { xfs_itruncate_extents_flags(tpp,ip,whichfork,new_size,0) }
pub unsafe fn xfs_inode_unlinked_incomplete(ip:*const xfs_inode)->bool { (*VFS_IC(ip)).i_nlink == 0 && !xfs_inode_on_unlinked_list(ip) }
pub unsafe fn xfs_inode_ipincount(ip:*mut xfs_inode)->c_uint { atomic_read(&mut (*ip).i_pincount) as c_uint }
pub unsafe fn xfs_finish_inode_setup(ip:*mut xfs_inode) { xfs_iflags_clear(ip,XFS_INEW); barrier(); unlock_new_inode(VFS_I(ip)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
