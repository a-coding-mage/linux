/* SPDX-License-Identifier: GPL-2.0 */
// Linux dependencies and preprocessor configuration are supplied externally.

pub const AFFS_CACHE_SIZE: usize = PAGE_SIZE;
pub const AFFS_LC_SIZE: usize = AFFS_CACHE_SIZE / core::mem::size_of::<u32>() / 2;
pub const AFFS_AC_SIZE: usize = AFFS_CACHE_SIZE / core::mem::size_of::<affs_ext_key>() / 2;
pub const AFFS_AC_MASK: usize = AFFS_AC_SIZE - 1;
pub const AFFSNAMEMAX: u32 = 30;

#[inline] pub unsafe fn AFFS_BLOCK(sb: *mut super_block, bh: *mut buffer_head, blk: usize) -> u32 { (*(((*bh).b_data) as *mut affs_head)).table[(*AFFS_SB(sb)).s_hashsize as usize - 1 - blk] }
#[inline] pub unsafe fn AFFS_HEAD(bh: *mut buffer_head) -> *mut affs_head { (*bh).b_data as *mut affs_head }
#[inline] pub unsafe fn AFFS_TAIL(sb: *mut super_block, bh: *mut buffer_head) -> *mut affs_tail { ((*bh).b_data.add((*sb).s_blocksize as usize - core::mem::size_of::<affs_tail>())) as *mut affs_tail }
#[inline] pub unsafe fn AFFS_ROOT_HEAD(bh: *mut buffer_head) -> *mut affs_root_head { (*bh).b_data as *mut affs_root_head }
#[inline] pub unsafe fn AFFS_ROOT_TAIL(sb: *mut super_block, bh: *mut buffer_head) -> *mut affs_root_tail { (*bh).b_data.add((*sb).s_blocksize as usize - core::mem::size_of::<affs_root_tail>()) as *mut affs_root_tail }
#[inline] pub unsafe fn AFFS_DATA_HEAD(bh: *mut buffer_head) -> *mut affs_data_head { (*bh).b_data as *mut affs_data_head }
#[inline] pub unsafe fn AFFS_DATA(bh: *mut buffer_head) -> *mut u8 { (*AFFS_DATA_HEAD(bh)).data.as_mut_ptr() }

#[repr(C)]
pub struct affs_ext_key {
    pub ext: u32, // idx of the extended block
    pub key: u32, // block number
}

#[repr(C)]
pub struct affs_inode_info {
    pub i_opencnt: atomic_t,
    pub i_link_lock: mutex,
    pub i_ext_lock: mutex,
    pub i_blkcnt: u32,
    pub i_extcnt: u32,
    pub i_lc: *mut u32,
    pub i_lc_size: u32,
    pub i_lc_shift: u32,
    pub i_lc_mask: u32,
    pub i_ac: *mut affs_ext_key,
    pub i_ext_last: u32,
    pub i_ext_bh: *mut buffer_head,
    pub mmu_private: loff_t,
    pub i_protect: u32,
    pub i_lastalloc: u32,
    pub i_pa_cnt: core::ffi::c_int,
    pub vfs_inode: inode,
}

#[inline]
pub unsafe fn AFFS_I(inode: *mut inode) -> *mut affs_inode_info {
    container_of(inode, core::ptr::addr_of_mut!((*core::ptr::null_mut::<affs_inode_info>()).vfs_inode))
}

#[repr(C)]
pub struct affs_bm_info { pub bm_key: u32, pub bm_free: u32 }

#[repr(C)]
pub struct affs_sb_info {
    pub s_partition_size: core::ffi::c_int,
    pub s_reserved: core::ffi::c_int,
    pub s_data_blksize: u32,
    pub s_root_block: u32,
    pub s_hashsize: core::ffi::c_int,
    pub s_flags: usize,
    pub s_uid: kuid_t,
    pub s_gid: kgid_t,
    pub s_mode: umode_t,
    pub s_root_bh: *mut buffer_head,
    pub s_bmlock: mutex,
    pub s_bitmap: *mut affs_bm_info,
    pub s_bmap_count: u32,
    pub s_bmap_bits: u32,
    pub s_last_bmap: u32,
    pub s_bmap_bh: *mut buffer_head,
    pub s_prefix: *mut core::ffi::c_char,
    pub s_volume: [core::ffi::c_char; 32],
    pub symlink_lock: spinlock_t,
    pub sb: *mut super_block,
    pub work_queued: core::ffi::c_int,
    pub sb_work: delayed_work,
    pub work_lock: spinlock_t,
    pub rcu: rcu_head,
}

pub const AFFS_MOUNT_SF_INTL: usize = 0x0001;
pub const AFFS_MOUNT_SF_BM_VALID: usize = 0x0002;
pub const AFFS_MOUNT_SF_IMMUTABLE: usize = 0x0004;
pub const AFFS_MOUNT_SF_QUIET: usize = 0x0008;
pub const AFFS_MOUNT_SF_SETUID: usize = 0x0010;
pub const AFFS_MOUNT_SF_SETGID: usize = 0x0020;
pub const AFFS_MOUNT_SF_SETMODE: usize = 0x0040;
pub const AFFS_MOUNT_SF_MUFS: usize = 0x0100;
pub const AFFS_MOUNT_SF_OFS: usize = 0x0200;
pub const AFFS_MOUNT_SF_PREFIX: usize = 0x0400;
pub const AFFS_MOUNT_SF_VERBOSE: usize = 0x0800;
pub const AFFS_MOUNT_SF_NO_TRUNCATE: usize = 0x1000;

#[inline] pub fn affs_clear_opt(o: &mut usize, opt: usize) { *o &= !opt; }
#[inline] pub fn affs_set_opt(o: &mut usize, opt: usize) { *o |= opt; }
#[inline] pub fn affs_test_opt(o: usize, opt: usize) -> usize { o & opt }

#[inline]
pub unsafe fn AFFS_SB(sb: *mut super_block) -> *mut affs_sb_info { (*sb).s_fs_info as *mut affs_sb_info }

pub unsafe extern "C" { pub fn affs_mark_sb_dirty(sb: *mut super_block); }

// External declarations from amigaffs.c, bitmap.c, namei.c, inode.c, file.c, and dir.c.
pub unsafe extern "C" {
    pub fn affs_insert_hash(inode: *mut inode, bh: *mut buffer_head) -> core::ffi::c_int;
    pub fn affs_remove_hash(dir: *mut inode, rem_bh: *mut buffer_head) -> core::ffi::c_int;
    pub fn affs_remove_header(dentry: *mut dentry) -> core::ffi::c_int;
    pub fn affs_checksum_block(sb: *mut super_block, bh: *mut buffer_head) -> u32;
    pub fn affs_fix_checksum(sb: *mut super_block, bh: *mut buffer_head);
    pub fn affs_secs_to_datestamp(secs: time64_t, ds: *mut affs_date);
    pub fn affs_prot_to_mode(prot: u32) -> umode_t;
    pub fn affs_mode_to_prot(inode: *mut inode);
    pub fn affs_nofilenametruncate(dentry: *const dentry) -> bool;
    pub fn affs_check_name(name: *const u8, len: core::ffi::c_int, notruncate: bool) -> core::ffi::c_int;
    pub fn affs_copy_name(bstr: *mut u8, dentry: *mut dentry) -> core::ffi::c_int;
    pub fn affs_count_free_blocks(s: *mut super_block) -> u32;
    pub fn affs_free_block(sb: *mut super_block, block: u32);
    pub fn affs_alloc_block(inode: *mut inode, goal: u32) -> u32;
    pub fn affs_init_bitmap(sb: *mut super_block, flags: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn affs_free_bitmap(sb: *mut super_block);
    pub fn affs_hash_name(sb: *mut super_block, name: *const u8, len: u32) -> core::ffi::c_int;
    pub fn affs_lookup(dir: *mut inode, dentry: *mut dentry, flags: u32) -> *mut dentry;
    pub fn affs_unlink(dir: *mut inode, dentry: *mut dentry) -> core::ffi::c_int;
    pub fn affs_new_inode(dir: *mut inode) -> *mut inode;
    pub fn affs_evict_inode(inode: *mut inode);
    pub fn affs_iget(sb: *mut super_block, ino: usize) -> *mut inode;
    pub fn affs_truncate(inode: *mut inode);
    pub fn affs_free_prealloc(inode: *mut inode);
    pub fn affs_dir_truncate(inode: *mut inode);
}

#[inline] pub unsafe fn affs_validblock(sb: *mut super_block, block: core::ffi::c_int) -> bool { block >= (*AFFS_SB(sb)).s_reserved && block < (*AFFS_SB(sb)).s_partition_size }
#[inline] pub unsafe fn affs_bread(sb: *mut super_block, block: core::ffi::c_int) -> *mut buffer_head { if affs_validblock(sb, block) { sb_bread(sb, block) } else { core::ptr::null_mut() } }
#[inline] pub unsafe fn affs_getblk(sb: *mut super_block, block: core::ffi::c_int) -> *mut buffer_head { if affs_validblock(sb, block) { sb_getblk(sb, block) } else { core::ptr::null_mut() } }
#[inline] pub unsafe fn affs_brelse(bh: *mut buffer_head) { brelse(bh); }

#[inline] pub unsafe fn affs_getzeroblk(sb: *mut super_block, block: core::ffi::c_int) -> *mut buffer_head {
    if affs_validblock(sb, block) { let bh = sb_getblk(sb, block); lock_buffer(bh); core::ptr::write_bytes((*bh).b_data, 0, (*sb).s_blocksize as usize); set_buffer_uptodate(bh); unlock_buffer(bh); bh } else { core::ptr::null_mut() }
}
#[inline] pub unsafe fn affs_getemptyblk(sb: *mut super_block, block: core::ffi::c_int) -> *mut buffer_head {
    if affs_validblock(sb, block) { let bh = sb_getblk(sb, block); wait_on_buffer(bh); set_buffer_uptodate(bh); bh } else { core::ptr::null_mut() }
}

#[inline] pub unsafe fn affs_adjust_checksum(bh: *mut buffer_head, val: u32) { let p = (*bh).b_data as *mut u32; *p.add(5) = (*p.add(5)).to_be().wrapping_sub(val).to_be(); }
#[inline] pub unsafe fn affs_adjust_bitmapchecksum(bh: *mut buffer_head, val: u32) { let p = (*bh).b_data as *mut u32; *p = (*p).to_be().wrapping_sub(val).to_be(); }
#[inline] pub unsafe fn affs_lock_link(inode: *mut inode) { mutex_lock(&mut (*AFFS_I(inode)).i_link_lock); }
#[inline] pub unsafe fn affs_unlock_link(inode: *mut inode) { mutex_unlock(&mut (*AFFS_I(inode)).i_link_lock); }
#[inline] pub unsafe fn affs_lock_ext(inode: *mut inode) { mutex_lock(&mut (*AFFS_I(inode)).i_ext_lock); }
#[inline] pub unsafe fn affs_unlock_ext(inode: *mut inode) { mutex_unlock(&mut (*AFFS_I(inode)).i_ext_lock); }
#[inline] pub unsafe fn affs_lock_dir(inode: *mut inode) { mutex_lock_nested(&mut (*AFFS_I(inode)).i_ext_lock, SINGLE_DEPTH_NESTING); }
#[inline] pub unsafe fn affs_unlock_dir(inode: *mut inode) { mutex_unlock(&mut (*AFFS_I(inode)).i_ext_lock); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
