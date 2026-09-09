/* Translated from hfs_fs.h. Linux dependencies are supplied externally. */

#[repr(C)]
pub struct hfs_inode_info {
    pub opencnt: atomic_t,
    pub flags: c_uint,
    pub tz_secondswest: c_int,
    pub cat_key: hfs_cat_key,
    pub rsrc_inode: *mut inode,
    pub extents_lock: mutex,
    pub alloc_blocks: u16,
    pub clump_blocks: u16,
    pub fs_blocks: sector_t,
    /* Allocation extents from catlog record or volume header */
    pub first_extents: hfs_extent_rec,
    pub first_blocks: u16,
    pub cached_extents: hfs_extent_rec,
    pub cached_start: u16,
    pub cached_blocks: u16,
    pub phys_size: loff_t,
    pub vfs_inode: inode,
}

pub const HFS_FLG_RSRC: c_uint = 0x0001;
pub const HFS_FLG_EXT_DIRTY: c_uint = 0x0002;
pub const HFS_FLG_EXT_NEW: c_uint = 0x0004;

#[inline]
pub unsafe fn HFS_IS_RSRC(inode: *mut inode) -> c_uint {
    ((*HFS_I(inode)).flags & HFS_FLG_RSRC)
}

#[repr(C)]
pub struct hfs_sb_info {
    pub mdb_lock: mutex,
    pub mdb_bh: *mut buffer_head,
    pub mdb_offset: c_uint,
    pub mdb: *mut hfs_mdb,
    pub alt_mdb_bh: *mut buffer_head,
    pub alt_mdb_offset: c_uint,
    pub alt_mdb: *mut hfs_mdb,
    pub bitmap: *mut __be32,
    pub ext_tree: *mut hfs_btree,
    pub cat_tree: *mut hfs_btree,
    pub file_count: atomic64_t,
    pub folder_count: atomic64_t,
    pub next_id: atomic64_t,
    pub clumpablks: u32,
    pub fs_start: u32,
    pub part_start: u32,
    pub root_files: u16,
    pub root_dirs: u16,
    pub fs_ablocks: u16,
    pub free_ablocks: u16,
    pub alloc_blksz: u32,
    pub s_quiet: c_int,
    pub s_type: __be32,
    pub s_creator: __be32,
    pub s_file_umask: umode_t,
    pub s_dir_umask: umode_t,
    pub s_uid: kuid_t,
    pub s_gid: kgid_t,
    pub session: c_int,
    pub part: c_int,
    pub nls_io: *mut nls_table,
    pub nls_disk: *mut nls_table,
    pub bitmap_lock: mutex,
    pub flags: c_ulong,
    pub blockoffset: u16,
    pub fs_div: c_int,
    pub sb: *mut super_block,
    pub work_queued: c_int,
    pub mdb_work: delayed_work,
    pub work_lock: spinlock_t,
}

pub const HFS_FLG_BITMAP_DIRTY: c_uint = 0;
pub const HFS_FLG_MDB_DIRTY: c_uint = 1;
pub const HFS_FLG_ALT_MDB_DIRTY: c_uint = 2;

extern "C" {
    pub fn hfs_vbm_search_free(sb: *mut super_block, goal: u32, num_bits: *mut u32) -> u32;
    pub fn hfs_clear_vbm_bits(sb: *mut super_block, start: u16, count: u16) -> c_int;
    pub fn hfs_cat_keycmp(key1: *const btree_key, key2: *const btree_key) -> c_int;
    pub fn hfs_cat_find_brec(sb: *mut super_block, cnid: u32, fd: *mut hfs_find_data) -> c_int;
    pub fn hfs_cat_create(cnid: u32, dir: *mut inode, str_: *const qstr, inode: *mut inode) -> c_int;
    pub fn hfs_cat_delete(cnid: u32, dir: *mut inode, str_: *const qstr) -> c_int;
    pub fn hfs_cat_move(cnid: u32, src_dir: *mut inode, src_name: *const qstr, dst_dir: *mut inode, dst_name: *const qstr) -> c_int;
    pub fn hfs_cat_build_key(sb: *mut super_block, key: *mut btree_key, parent: u32, name: *const qstr);
}

#[inline]
pub fn hfs_is_valid_cnid(cnid: u32, ty: u8) -> bool {
    if cnid >= HFS_FIRSTUSER_CNID { return true; }
    match cnid {
        HFS_ROOT_CNID => ty == HFS_CDR_DIR,
        HFS_EXT_CNID | HFS_CAT_CNID => ty == HFS_CDR_FIL,
        _ => false,
    }
}

extern "C" {
    pub static hfs_dir_operations: file_operations;
    pub static hfs_dir_inode_operations: inode_operations;
    pub fn hfs_ext_keycmp(key1: *const btree_key, key2: *const btree_key) -> c_int;
    pub fn hfs_ext_find_block(ext: *mut hfs_extent, off: u16) -> u16;
    pub fn hfs_free_fork(sb: *mut super_block, file: *mut hfs_cat_file, ty: c_int) -> c_int;
    pub fn hfs_ext_write_extent(inode: *mut inode) -> c_int;
    pub fn hfs_extend_file(inode: *mut inode) -> c_int;
    pub fn hfs_file_truncate(inode: *mut inode);
    pub fn hfs_get_block(inode: *mut inode, block: sector_t, bh_result: *mut buffer_head, create: c_int) -> c_int;
    pub static hfs_aops: address_space_operations;
    pub static hfs_btree_aops: address_space_operations;
    pub fn hfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
    pub fn hfs_write_begin(iocb: *const kiocb, mapping: *mut address_space, pos: loff_t, len: c_uint, foliop: *mut *mut folio, fsdata: *mut *mut c_void) -> c_int;
    pub fn hfs_new_inode(dir: *mut inode, name: *const qstr, mode: umode_t) -> *mut inode;
    pub fn hfs_inode_write_fork(inode: *mut inode, ext: *mut hfs_extent, log_size: *mut __be32, phys_size: *mut __be32);
    pub fn hfs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int;
    pub fn hfs_inode_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int;
    pub fn hfs_inode_read_fork(inode: *mut inode, ext: *mut hfs_extent, log_size: __be32, phys_size: __be32, clump_size: u32);
    pub fn hfs_iget(sb: *mut super_block, key: *mut hfs_cat_key, rec: *mut hfs_cat_rec) -> *mut inode;
    pub fn hfs_evict_inode(inode: *mut inode);
    pub fn hfs_delete_inode(inode: *mut inode);
    pub static hfs_xattr_handlers: *const *const xattr_handler;
    pub fn is_hfs_cnid_counts_valid(sb: *mut super_block) -> bool;
    pub fn hfs_mdb_get(sb: *mut super_block) -> c_int;
    pub fn hfs_mdb_commit(sb: *mut super_block) -> c_int;
    pub fn hfs_mdb_close(sb: *mut super_block);
    pub fn hfs_mdb_put(sb: *mut super_block);
    pub fn hfs_part_find(sb: *mut super_block, part_start: *mut sector_t, part_size: *mut sector_t) -> c_int;
    pub static hfs_dentry_operations: dentry_operations;
    pub fn hfs_hash_dentry(dentry: *const dentry, this: *mut qstr) -> c_int;
    pub fn hfs_strcmp(s1: *const u8, len1: c_uint, s2: *const u8, len2: c_uint) -> c_int;
    pub fn hfs_compare_dentry(dentry: *const dentry, len: c_uint, str_: *const c_char, name: *const qstr) -> c_int;
    pub fn hfs_asc2mac(sb: *mut super_block, out: *mut hfs_name, input: *const qstr);
    pub fn hfs_mac2asc(sb: *mut super_block, out: *mut c_char, input: *const hfs_name) -> c_int;
    pub fn hfs_mark_mdb_dirty(sb: *mut super_block);
}

pub const HFS_UTC_OFFSET: u32 = 2082844800;

#[inline]
pub unsafe fn __hfs_m_to_utime(mt: __be32) -> time64_t {
    let ut = (be32_to_cpu(mt).wrapping_sub(HFS_UTC_OFFSET)) as u32 as time64_t;
    ut + sys_tz.tz_minuteswest as time64_t * 60
}

#[inline]
pub unsafe fn __hfs_u_to_mtime(mut ut: time64_t) -> __be32 {
    ut -= sys_tz.tz_minuteswest as time64_t * 60;
    cpu_to_be32((lower_32_bits(ut)).wrapping_add(HFS_UTC_OFFSET))
}

#[inline]
pub unsafe fn hfs_mdb_name(sb: *mut super_block) -> *const c_char { (*sb).s_id }

#[inline]
pub unsafe fn hfs_bitmap_dirty(sb: *mut super_block) {
    set_bit(HFS_FLG_BITMAP_DIRTY, &mut (*HFS_SB(sb)).flags);
    hfs_mark_mdb_dirty(sb);
}

// C macro sb_bread512: computes the block and offset, reads it, and assigns data.
#[inline]
pub unsafe fn sb_bread512(sb: *mut super_block, sec: sector_t, data: *mut *mut c_void) -> *mut buffer_head {
    let start = (sec as loff_t) << HFS_SECTOR_SIZE_BITS;
    let block = start >> (*sb).s_blocksize_bits;
    let offset = (start & ((*sb).s_blocksize as loff_t - 1)) as isize;
    let bh = sb_bread(sb, block as sector_t);
    if !bh.is_null() { *data = ((*bh).b_data.offset(offset)) as *mut c_void; } else { *data = core::ptr::null_mut(); }
    bh
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
