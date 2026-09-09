/* SPDX-License-Identifier: GPL-2.0+ */
/* NILFS local header file. */

// Linux dependencies and local headers are supplied by other translated units.

#[repr(C)]
pub struct nilfs_inode_info {
    pub i_flags: __u32,
    pub i_type: c_uint,
    pub i_state: c_ulong,
    pub i_bmap: *mut nilfs_bmap,
    pub i_bmap_data: nilfs_bmap,
    pub i_xattr: __u64,
    pub i_dir_start_lookup: __u32,
    pub i_cno: __u64,
    pub i_assoc_inode: *mut inode,
    pub i_dirty: list_head,
    #[cfg(CONFIG_NILFS_XATTR)]
    pub xattr_sem: rw_semaphore,
    pub i_bh: *mut buffer_head,
    pub i_root: *mut nilfs_root,
    pub vfs_inode: inode,
}

#[inline]
pub unsafe fn NILFS_I(inode: *const inode) -> *mut nilfs_inode_info {
    container_of(inode, core::mem::offset_of!(nilfs_inode_info, vfs_inode))
}

#[inline]
pub unsafe fn NILFS_BMAP_I(bmap: *const nilfs_bmap) -> *mut nilfs_inode_info {
    container_of(bmap, core::mem::offset_of!(nilfs_inode_info, i_bmap_data))
}

pub const NILFS_I_NEW: c_uint = 0;
pub const NILFS_I_DIRTY: c_uint = 1;
pub const NILFS_I_QUEUED: c_uint = 2;
pub const NILFS_I_BUSY: c_uint = 3;
pub const NILFS_I_COLLECTED: c_uint = 4;
pub const NILFS_I_UPDATED: c_uint = 5;
pub const NILFS_I_INODE_SYNC: c_uint = 6;
pub const NILFS_I_BMAP: c_uint = 7;

pub const NILFS_I_TYPE_NORMAL: c_uint = 0;
pub const NILFS_I_TYPE_GC: c_uint = 0x0001;
pub const NILFS_I_TYPE_BTNC: c_uint = 0x0002;
pub const NILFS_I_TYPE_SHADOW: c_uint = 0x0004;

pub const NILFS_SB_COMMIT: c_uint = 0;
pub const NILFS_SB_COMMIT_ALL: c_uint = 1;

pub const NILFS_MAX_VOLUME_NAME: usize = core::mem::size_of::<[u8; NILFS_MAX_VOLUME_NAME_FIELD]>();

pub const NILFS_MDT_INO_BITS: u64 = (BIT(NILFS_DAT_INO) | BIT(NILFS_CPFILE_INO) |
    BIT(NILFS_SUFILE_INO) | BIT(NILFS_IFILE_INO) | BIT(NILFS_ATIME_INO) | BIT(NILFS_SKETCH_INO));
pub const NILFS_SYS_INO_BITS: u64 = BIT(NILFS_ROOT_INO) | NILFS_MDT_INO_BITS;

#[inline]
pub unsafe fn NILFS_FIRST_INO(sb: *const super_block) -> u64 {
    (*(sb as *const super_block)).s_fs_info.cast::<the_nilfs>().as_ref().unwrap().ns_first_ino
}
#[inline]
pub fn NILFS_MDT_INODE(_sb: *const super_block, ino: u64) -> bool {
    ino < NILFS_USER_INO && (NILFS_MDT_INO_BITS & BIT(ino)) != 0
}
#[inline]
pub unsafe fn NILFS_VALID_INODE(sb: *const super_block, ino: u64) -> bool {
    ino >= NILFS_FIRST_INO(sb) || (ino < NILFS_USER_INO && (NILFS_SYS_INO_BITS & BIT(ino)) != 0)
}
#[inline]
pub fn NILFS_PRIVATE_INODE(ino: u64) -> bool {
    ino < NILFS_USER_INO && ino != NILFS_ROOT_INO && ino != NILFS_SKETCH_INO
}

#[repr(C)]
pub struct nilfs_transaction_info {
    pub ti_magic: u32,
    pub ti_save: *mut core::ffi::c_void,
    pub ti_flags: u16,
    pub ti_count: u16,
}
pub const NILFS_TI_MAGIC: u32 = 0xd9e392fb;
pub const NILFS_TI_DYNAMIC_ALLOC: u16 = 0x0001;
pub const NILFS_TI_SYNC: u16 = 0x0002;
pub const NILFS_TI_GC: u16 = 0x0004;
pub const NILFS_TI_COMMIT: u16 = 0x0008;
pub const NILFS_TI_WRITER: u16 = 0x0010;

extern "C" {
    pub fn nilfs_transaction_begin(sb: *mut super_block, ti: *mut nilfs_transaction_info, mode: c_int) -> c_int;
    pub fn nilfs_transaction_commit(sb: *mut super_block) -> c_int;
    pub fn nilfs_transaction_abort(sb: *mut super_block);
    pub fn nilfs_acl_chmod(inode: *mut inode) -> c_int;
    pub fn nilfs_init_acl(inode: *mut inode, dir: *mut inode) -> c_int;
}

#[inline]
pub unsafe fn nilfs_set_transaction_flag(flag: c_uint) {
    let ti = (*current).journal_info as *mut nilfs_transaction_info;
    (*ti).ti_flags |= flag as u16;
}
#[inline]
pub unsafe fn nilfs_test_transaction_flag(flag: c_uint) -> c_int {
    let ti = (*current).journal_info as *mut nilfs_transaction_info;
    if ti.is_null() || (*ti).ti_magic != NILFS_TI_MAGIC { return 0; }
    ((*ti).ti_flags & flag as u16 != 0) as c_int
}
#[inline] pub unsafe fn nilfs_doing_gc() -> c_int { nilfs_test_transaction_flag(NILFS_TI_GC) }
#[inline] pub unsafe fn nilfs_doing_construction() -> c_int { nilfs_test_transaction_flag(NILFS_TI_WRITER) }

pub const NILFS_ATIME_DISABLE: () = ();
pub const NILFS_FL_INHERITED: u32 = FS_SECRM_FL | FS_UNRM_FL | FS_COMPR_FL | FS_SYNC_FL |
    FS_IMMUTABLE_FL | FS_APPEND_FL | FS_NODUMP_FL | FS_NOATIME_FL | FS_COMPRBLK_FL |
    FS_NOCOMP_FL | FS_NOTAIL_FL | FS_DIRSYNC_FL;

#[inline]
pub fn nilfs_mask_flags(mode: umode_t, flags: __u32) -> __u32 {
    if S_ISDIR(mode) { flags } else if S_ISREG(mode) { flags & !(FS_DIRSYNC_FL | FS_TOPDIR_FL) }
    else { flags & (FS_NODUMP_FL | FS_NOATIME_FL) }
}

// Function declarations from dir.c, file.c, ioctl.c, inode.c, super.c, gcinode.c, and sysfs.c.
extern "C" {
    pub fn nilfs_add_link(dentry: *mut dentry, inode: *mut inode) -> c_int;
    pub fn nilfs_inode_by_name(dir: *mut inode, qstr: *const qstr, ino: *mut u64) -> c_int;
    pub fn nilfs_make_empty(inode: *mut inode, parent: *mut inode) -> c_int;
    pub fn nilfs_find_entry(inode: *mut inode, qstr: *const qstr, folio: *mut *mut folio) -> *mut nilfs_dir_entry;
    pub fn nilfs_delete_entry(de: *mut nilfs_dir_entry, folio: *mut folio) -> c_int;
    pub fn nilfs_empty_dir(inode: *mut inode) -> c_int;
    pub fn nilfs_dotdot(inode: *mut inode, folio: *mut *mut folio) -> *mut nilfs_dir_entry;
    pub fn nilfs_set_link(dir: *mut inode, de: *mut nilfs_dir_entry, folio: *mut folio, inode: *mut inode) -> c_int;
    pub fn nilfs_sync_file(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int;
    pub fn nilfs_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
    pub fn nilfs_compat_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
    pub fn nilfs_inode_add_blocks(inode: *mut inode, n: c_int);
    pub fn nilfs_inode_sub_blocks(inode: *mut inode, n: c_int);
    pub fn nilfs_new_inode(inode: *mut inode, mode: umode_t) -> *mut inode;
    pub fn nilfs_get_block(inode: *mut inode, block: sector_t, bh: *mut buffer_head, create: c_int) -> c_int;
    pub fn nilfs_set_inode_flags(inode: *mut inode);
    pub fn nilfs_truncate(inode: *mut inode);
    pub fn nilfs_evict_inode(inode: *mut inode);
    pub fn nilfs_alloc_inode(sb: *mut super_block) -> *mut inode;
    pub fn nilfs_sysfs_init() -> c_int;
    pub fn nilfs_sysfs_exit();
    pub fn nilfs_fileattr_get(dentry: *mut dentry, m: *mut file_kattr) -> c_int;
    pub fn nilfs_fileattr_set(idmap: *mut mnt_idmap, dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
    pub fn nilfs_ioctl_prepare_clean_segments(nilfs: *mut the_nilfs, argv: *mut nilfs_argv, argp: *mut *mut core::ffi::c_void) -> c_int;
    pub fn nilfs_read_inode_common(inode: *mut inode, raw: *mut nilfs_inode) -> c_int;
    pub fn nilfs_write_inode_common(inode: *mut inode, raw: *mut nilfs_inode);
    pub fn nilfs_ilookup(sb: *mut super_block, root: *mut nilfs_root, ino: u64) -> *mut inode;
    pub fn nilfs_iget_locked(sb: *mut super_block, root: *mut nilfs_root, ino: u64) -> *mut inode;
    pub fn nilfs_iget(sb: *mut super_block, root: *mut nilfs_root, ino: u64) -> *mut inode;
    pub fn nilfs_iget_for_gc(sb: *mut super_block, ino: u64, cno: __u64) -> *mut inode;
    pub fn nilfs_attach_btree_node_cache(inode: *mut inode) -> c_int;
    pub fn nilfs_detach_btree_node_cache(inode: *mut inode);
    pub fn nilfs_iget_for_shadow(inode: *mut inode) -> *mut inode;
    pub fn nilfs_update_inode(inode: *mut inode, bh: *mut buffer_head, flags: c_int);
    pub fn nilfs_write_failed(mapping: *mut address_space, to: loff_t);
    pub fn nilfs_permission(idmap: *mut mnt_idmap, inode: *mut inode, mask: c_int) -> c_int;
    pub fn nilfs_load_inode_block(inode: *mut inode, pbh: *mut *mut buffer_head) -> c_int;
    pub fn nilfs_inode_dirty(inode: *mut inode) -> c_int;
    pub fn nilfs_set_file_dirty(inode: *mut inode, nr_dirty: c_uint) -> c_int;
    pub fn __nilfs_mark_inode_dirty(inode: *mut inode, flags: c_int) -> c_int;
    pub fn nilfs_dirty_inode(inode: *mut inode, flags: c_int);
    pub fn nilfs_fiemap(inode: *mut inode, fieinfo: *mut fiemap_extent_info, start: __u64, len: __u64) -> c_int;
    pub fn __nilfs_msg(sb: *mut super_block, fmt: *const c_char, ...);
    pub fn __nilfs_error(sb: *mut super_block, function: *const c_char, fmt: *const c_char, ...);
    pub fn nilfs_read_super_block(sb: *mut super_block, block: u64, secondary: c_int, bh: *mut *mut buffer_head) -> *mut nilfs_super_block;
    pub fn nilfs_store_magic(sb: *mut super_block, sbp: *mut nilfs_super_block) -> c_int;
    pub fn nilfs_check_feature_compatibility(sb: *mut super_block, sbp: *mut nilfs_super_block) -> c_int;
    pub fn nilfs_set_log_cursor(sbp: *mut nilfs_super_block, nilfs: *mut the_nilfs);
    pub fn nilfs_prepare_super(sb: *mut super_block, flip: c_int) -> *mut *mut nilfs_super_block;
    pub fn nilfs_commit_super(sb: *mut super_block, flag: c_int) -> c_int;
    pub fn nilfs_cleanup_super(sb: *mut super_block) -> c_int;
    pub fn nilfs_resize_fs(sb: *mut super_block, newsize: __u64) -> c_int;
    pub fn nilfs_attach_checkpoint(sb: *mut super_block, cno: __u64, curr_mnt: c_int, root: *mut *mut nilfs_root) -> c_int;
    pub fn nilfs_checkpoint_is_mounted(sb: *mut super_block, cno: __u64) -> c_int;
    pub fn nilfs_gccache_submit_read_data(inode: *mut inode, pbn: sector_t, vbn: sector_t, cno: __u64, bh: *mut *mut buffer_head) -> c_int;
    pub fn nilfs_gccache_submit_read_node(inode: *mut inode, pbn: sector_t, cno: __u64, bh: *mut *mut buffer_head) -> c_int;
    pub fn nilfs_gccache_wait_and_mark_dirty(bh: *mut buffer_head) -> c_int;
    pub fn nilfs_init_gcinode(inode: *mut inode) -> c_int;
    pub fn nilfs_remove_all_gcinodes(nilfs: *mut the_nilfs);
    pub fn nilfs_sysfs_create_device_group(sb: *mut super_block) -> c_int;
    pub fn nilfs_sysfs_delete_device_group(nilfs: *mut the_nilfs);
    pub fn nilfs_sysfs_create_snapshot_group(root: *mut nilfs_root) -> c_int;
    pub fn nilfs_sysfs_delete_snapshot_group(root: *mut nilfs_root);
    pub static nilfs_dir_operations: file_operations;
    pub static nilfs_file_inode_operations: inode_operations;
    pub static nilfs_file_operations: file_operations;
    pub static nilfs_aops: address_space_operations;
    pub static nilfs_buffer_cache_aops: address_space_operations;
    pub static nilfs_dir_inode_operations: inode_operations;
    pub static nilfs_special_inode_operations: inode_operations;
    pub static nilfs_symlink_inode_operations: inode_operations;
    pub static nilfs_fs_type: file_system_type;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
