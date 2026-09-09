/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from btrfs_inode.h. Kernel-provided types and helpers are external. */

pub const BTRFS_DIR_START_INDEX: u64 = 2;

pub const BTRFS_INODE_FLUSH_ON_CLOSE: u32 = 0;
pub const BTRFS_INODE_DUMMY: u32 = 1;
pub const BTRFS_INODE_IN_DEFRAG: u32 = 2;
pub const BTRFS_INODE_HAS_ASYNC_EXTENT: u32 = 3;
pub const BTRFS_INODE_NEEDS_FULL_SYNC: u32 = 4;
pub const BTRFS_INODE_COPY_EVERYTHING: u32 = 5;
pub const BTRFS_INODE_HAS_PROPS: u32 = 6;
pub const BTRFS_INODE_SNAPSHOT_FLUSH: u32 = 7;
pub const BTRFS_INODE_NO_XATTRS: u32 = 8;
pub const BTRFS_INODE_NO_DELALLOC_FLUSH: u32 = 9;
pub const BTRFS_INODE_VERITY_IN_PROGRESS: u32 = 10;
pub const BTRFS_INODE_FREE_SPACE_INODE: u32 = 11;
pub const BTRFS_INODE_NO_CAP_XATTR: u32 = 12;
pub const BTRFS_INODE_COW_WRITE_ERROR: u32 = 13;
pub const BTRFS_INODE_ROOT_STUB: u32 = 14;

#[repr(C)]
pub struct btrfs_inode {
    pub root: *mut btrfs_root,
    pub prop_compress: u8,
    pub defrag_compress: u8,
    pub defrag_compress_level: i8,
    pub lock: spinlock_t,
    pub extent_tree: extent_map_tree,
    pub io_tree: extent_io_tree,
    pub file_extent_tree: *mut extent_io_tree,
    pub log_mutex: mutex,
    pub outstanding_extents: u32,
    pub ordered_tree_lock: spinlock_t,
    pub ordered_tree: rb_root,
    pub ordered_tree_last: *mut rb_node,
    pub delalloc_inodes: list_head,
    pub runtime_flags: c_ulong,
    pub generation: u64,
    pub last_trans: u64,
    pub logged_trans: u64,
    pub last_sub_trans: i32,
    pub last_log_commit: i32,
    pub delalloc_bytes: u64,
    pub new_delalloc_bytes: u64,
    pub defrag_bytes: u64,
    pub disk_i_size: u64,
    pub index_cnt: u64,
    pub dir_index: u64,
    pub last_unlink_trans: u64,
    pub last_reflink_trans: u64,
    pub flags: u32,
    pub ro_flags: u32,
    pub block_rsv: btrfs_block_rsv,
    pub delayed_node: *mut btrfs_delayed_node,
    pub i_otime_sec: u64,
    pub i_otime_nsec: u32,
    pub delayed_iput: list_head,
    pub i_mmap_lock: rw_semaphore,
    pub vfs_inode: inode,
}

/* Anonymous C unions are represented by the corresponding overlapping fields. */
#[allow(non_camel_case_types)]
pub type btrfs_inode_union_u64 = u64;

#[inline]
pub unsafe fn btrfs_get_first_dir_index_to_log(inode: *const btrfs_inode) -> u64 {
    core::ptr::read_volatile(&(*inode).delalloc_bytes)
}
#[inline]
pub unsafe fn btrfs_set_first_dir_index_to_log(inode: *mut btrfs_inode, index: u64) {
    core::ptr::write_volatile(&mut (*inode).delalloc_bytes, index)
}

#[inline]
pub unsafe fn btrfs_ino(inode: *const btrfs_inode) -> u64 { (*inode).vfs_inode.i_ino }
#[inline]
pub unsafe fn btrfs_get_inode_key(inode: *const btrfs_inode, key: *mut btrfs_key) {
    (*key).objectid = btrfs_ino(inode);
    (*key).type_ = BTRFS_INODE_ITEM_KEY;
    (*key).offset = 0;
}
#[inline]
pub unsafe fn btrfs_set_inode_number(inode: *mut btrfs_inode, ino: u64) { (*inode).vfs_inode.i_ino = ino; }
#[inline]
pub unsafe fn btrfs_i_size_write(inode: *mut btrfs_inode, size: u64) {
    i_size_write(&mut (*inode).vfs_inode, size); (*inode).disk_i_size = size;
}
#[inline]
pub unsafe fn btrfs_is_free_space_inode(inode: *const btrfs_inode) -> bool {
    test_bit(BTRFS_INODE_FREE_SPACE_INODE, &(*inode).runtime_flags)
}
#[inline]
pub unsafe fn is_data_inode(inode: *const btrfs_inode) -> bool { btrfs_ino(inode) != BTRFS_BTREE_INODE_OBJECTID }

/* The following declarations preserve the header's externally visible interface. */
extern "C" {
    pub fn btrfs_calculate_block_csum_folio(fs_info: *mut btrfs_fs_info, paddr: phys_addr_t, dest: *mut u8);
    pub fn btrfs_calculate_block_csum_pages(fs_info: *mut btrfs_fs_info, paddrs: *const phys_addr_t, dest: *mut u8);
    pub fn btrfs_check_block_csum(fs_info: *mut btrfs_fs_info, paddr: phys_addr_t, csum: *mut u8, expected: *const u8) -> i32;
    pub fn btrfs_data_csum_ok(bbio: *mut btrfs_bio, dev: *mut btrfs_device, bio_offset: u32, paddrs: *const phys_addr_t) -> bool;
    pub fn can_nocow_extent(inode: *mut btrfs_inode, offset: u64, len: *mut u64, file_extent: *mut btrfs_file_extent, nowait: bool) -> i32;
    pub fn btrfs_del_delalloc_inode(inode: *mut btrfs_inode);
    pub fn btrfs_lookup_dentry(dir: *mut inode, dentry: *mut dentry) -> *mut inode;
    pub fn btrfs_set_inode_index(dir: *mut btrfs_inode, index: *mut u64) -> i32;
    pub fn btrfs_unlink_inode(trans: *mut btrfs_trans_handle, dir: *mut btrfs_inode, inode: *mut btrfs_inode, name: *const fscrypt_str) -> i32;
    pub fn btrfs_add_link(trans: *mut btrfs_trans_handle, parent: *mut btrfs_inode, inode: *mut btrfs_inode, name: *const fscrypt_str, add_backref: bool, index: u64) -> i32;
    pub fn btrfs_delete_subvolume(dir: *mut btrfs_inode, dentry: *mut dentry) -> i32;
    pub fn btrfs_truncate_block(inode: *mut btrfs_inode, offset: u64, start: u64, end: u64) -> i32;
    pub fn btrfs_start_delalloc_snapshot(root: *mut btrfs_root, reclaim: bool) -> i32;
    pub fn btrfs_start_delalloc_roots(fs_info: *mut btrfs_fs_info, nr: c_long, reclaim: bool) -> i32;
    pub fn btrfs_set_extent_delalloc(inode: *mut btrfs_inode, start: u64, end: u64, extra_bits: u32, cached: *mut *mut extent_state) -> i32;
    pub fn btrfs_reset_extent_delalloc(inode: *mut btrfs_inode, start: u64, end: u64, extra_bits: u32, cached: *mut *mut extent_state) -> i32;
}

#[repr(C)]
pub struct btrfs_new_inode_args {
    pub dir: *mut inode, pub dentry: *mut dentry, pub inode: *mut inode,
    pub orphan: bool, pub subvol: bool, pub default_acl: *mut posix_acl,
    pub acl: *mut posix_acl, pub fname: fscrypt_name,
}

#[repr(C)]
pub enum btrfs_ilock_type { BTRFS_ILOCK_SHARED = 1, BTRFS_ILOCK_TRY = 2, BTRFS_ILOCK_MMAP = 4 }

extern "C" {
    pub fn btrfs_new_inode_prepare(args: *mut btrfs_new_inode_args, items: *mut u32) -> i32;
    pub fn btrfs_create_new_inode(trans: *mut btrfs_trans_handle, args: *mut btrfs_new_inode_args) -> i32;
    pub fn btrfs_new_inode_args_destroy(args: *mut btrfs_new_inode_args);
    pub fn btrfs_new_subvol_inode(idmap: *mut mnt_idmap, dir: *mut inode) -> *mut inode;
    pub fn btrfs_evict_inode(inode: *mut inode); pub fn btrfs_alloc_inode(sb: *mut super_block) -> *mut inode;
    pub fn btrfs_destroy_inode(inode: *mut inode); pub fn btrfs_free_inode(inode: *mut inode); pub fn btrfs_drop_inode(inode: *mut inode) -> i32;
    pub fn btrfs_init_cachep() -> i32; pub fn btrfs_destroy_cachep();
    pub fn btrfs_iget_path(ino: u64, root: *mut btrfs_root, path: *mut btrfs_path) -> *mut btrfs_inode;
    pub fn btrfs_iget(ino: u64, root: *mut btrfs_root) -> *mut btrfs_inode;
    pub fn btrfs_update_inode(trans: *mut btrfs_trans_handle, inode: *mut btrfs_inode) -> i32;
    pub fn btrfs_update_inode_fallback(trans: *mut btrfs_trans_handle, inode: *mut btrfs_inode) -> i32;
    pub fn btrfs_orphan_add(trans: *mut btrfs_trans_handle, inode: *mut btrfs_inode) -> i32;
    pub fn btrfs_orphan_cleanup(root: *mut btrfs_root) -> i32;
    pub fn btrfs_add_delayed_iput(inode: *mut btrfs_inode); pub fn btrfs_run_delayed_iputs(fs: *mut btrfs_fs_info); pub fn btrfs_wait_on_delayed_iputs(fs: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_find_first_inode(root: *mut btrfs_root, min_ino: u64) -> *mut btrfs_inode;
    pub fn btrfs_inode_lock(inode: *mut btrfs_inode, flags: u32) -> i32; pub fn btrfs_inode_unlock(inode: *mut btrfs_inode, flags: u32);
    pub fn btrfs_update_inode_bytes(inode: *mut btrfs_inode, add_bytes: u64, del_bytes: u64);
    pub fn btrfs_assert_inode_range_clean(inode: *mut btrfs_inode, start: u64, end: u64);
    pub fn btrfs_get_extent_allocation_hint(inode: *mut btrfs_inode, start: u64, num_bytes: u64) -> u64;
    pub fn btrfs_create_io_em(inode: *mut btrfs_inode, start: u64, extent: *const btrfs_file_extent, type_: i32) -> *mut extent_map;
    pub fn btrfs_cont_expand(inode: *mut btrfs_inode, oldsize: loff_t, size: loff_t) -> i32;
    pub fn btrfs_prealloc_file_range(inode: *mut inode, mode: i32, start: u64, num_bytes: u64, min_size: u64, actual_len: loff_t, alloc_hint: *mut u64) -> i32;
    pub fn btrfs_prealloc_file_range_trans(inode: *mut inode, trans: *mut btrfs_trans_handle, mode: i32, start: u64, num_bytes: u64, min_size: u64, actual_len: loff_t, alloc_hint: *mut u64) -> i32;
    pub fn btrfs_run_delalloc_range(inode: *mut btrfs_inode, folio: *mut folio, start: u64, end: u64, wbc: *mut writeback_control) -> i32;
    pub fn btrfs_queue_writepage_fixup(inode: *mut btrfs_inode, folio: *mut folio);
    pub fn btrfs_encoded_io_compression_from_extent(fs: *mut btrfs_fs_info, compress_type: i32) -> i32;
    pub fn btrfs_get_extent(inode: *mut btrfs_inode, folio: *mut folio, start: u64, len: u64) -> *mut extent_map;
    pub fn btrfs_set_delalloc_extent(inode: *mut btrfs_inode, state: *mut extent_state, bits: u32);
    pub fn btrfs_clear_delalloc_extent(inode: *mut btrfs_inode, state: *mut extent_state, bits: u32);
    pub fn btrfs_merge_delalloc_extent(inode: *mut btrfs_inode, new: *mut extent_state, other: *mut extent_state);
    pub fn btrfs_split_delalloc_extent(inode: *mut btrfs_inode, orig: *mut extent_state, split: u64);
    pub fn btrfs_encoded_read(iocb: *mut kiocb, iter: *mut iov_iter, encoded: *mut btrfs_ioctl_encoded_io_args, cached: *mut *mut extent_state, disk_bytenr: *mut u64, disk_io_size: *mut u64) -> ssize_t;
    pub fn btrfs_encoded_read_regular(iocb: *mut kiocb, iter: *mut iov_iter, start: u64, lockend: u64, cached: *mut *mut extent_state, disk_bytenr: u64, disk_io_size: u64, count: usize, compressed: bool, unlocked: *mut bool) -> ssize_t;
    pub fn btrfs_do_encoded_write(iocb: *mut kiocb, from: *mut iov_iter, encoded: *const btrfs_ioctl_encoded_io_args) -> ssize_t;
    pub static btrfs_dentry_operations: dentry_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
