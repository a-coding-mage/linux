/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations corresponding to the Linux FAT header. */

pub const VFAT_SFN_DISPLAY_LOWER: u32 = 0x0001;
pub const VFAT_SFN_DISPLAY_WIN95: u32 = 0x0002;
pub const VFAT_SFN_DISPLAY_WINNT: u32 = 0x0004;
pub const VFAT_SFN_CREATE_WIN95: u32 = 0x0100;
pub const VFAT_SFN_CREATE_WINNT: u32 = 0x0200;

pub const FAT_ERRORS_CONT: i32 = 1;
pub const FAT_ERRORS_PANIC: i32 = 2;
pub const FAT_ERRORS_RO: i32 = 3;
pub const FAT_NFS_STALE_RW: i32 = 1;
pub const FAT_NFS_NOSTALE_RO: i32 = 2;

#[repr(C)]
pub struct fat_mount_options {
    pub fs_uid: kuid_t, pub fs_gid: kgid_t,
    pub fs_fmask: libc::c_ushort, pub fs_dmask: libc::c_ushort,
    pub codepage: libc::c_ushort, pub time_offset: libc::c_int,
    pub iocharset: *mut libc::c_char, pub shortname: libc::c_ushort,
    pub name_check: libc::c_uchar, pub errors: libc::c_uchar,
    pub nfs: libc::c_uchar, pub allow_utime: libc::c_ushort,
    /* C unsigned bit-fields; each member is represented by its flag value. */
    pub flags: u32,
}

pub const FAT_HASH_BITS: u32 = 8;
pub const FAT_HASH_SIZE: usize = 1usize << FAT_HASH_BITS;

#[repr(C)]
pub struct msdos_sb_info {
    pub sec_per_clus: libc::c_ushort, pub cluster_bits: libc::c_ushort,
    pub cluster_size: libc::c_uint, pub fats: libc::c_uchar, pub fat_bits: libc::c_uchar,
    pub fat_start: libc::c_ushort, pub fat_length: libc::c_ulong,
    pub dir_start: libc::c_ulong, pub dir_entries: libc::c_ushort,
    pub data_start: libc::c_ulong, pub max_cluster: libc::c_ulong,
    pub root_cluster: libc::c_ulong, pub fsinfo_sector: libc::c_ulong,
    pub fat_lock: mutex, pub nfs_build_inode_lock: mutex, pub s_lock: mutex,
    pub prev_free: libc::c_uint, pub free_clusters: libc::c_uint,
    pub free_clus_valid: libc::c_uint, pub options: fat_mount_options,
    pub nls_disk: *mut nls_table, pub nls_io: *mut nls_table,
    pub dir_ops: *const libc::c_void, pub dir_per_block: libc::c_int,
    pub dir_per_block_bits: libc::c_int, pub vol_id: libc::c_uint,
    pub fatent_shift: libc::c_int, pub fatent_ops: *const fatent_operations,
    pub fat_inode: *mut inode, pub fsinfo_inode: *mut inode,
    pub ratelimit: ratelimit_state, pub inode_hash_lock: spinlock_t,
    pub inode_hashtable: [hlist_head; FAT_HASH_SIZE], pub dir_hash_lock: spinlock_t,
    pub dir_hashtable: [hlist_head; FAT_HASH_SIZE], pub dirty: libc::c_uint,
    pub rcu: rcu_head,
}

pub const FAT_CACHE_VALID: i32 = 0;

#[repr(C)]
pub struct msdos_inode_info {
    pub cache_lru_lock: spinlock_t, pub cache_lru: list_head, pub nr_caches: libc::c_int,
    pub cache_valid_id: libc::c_uint, pub mmu_private: loff_t,
    pub i_start: libc::c_int, pub i_logstart: libc::c_int, pub i_attrs: libc::c_int,
    pub i_pos: loff_t, pub i_fat_hash: hlist_node, pub i_dir_hash: hlist_node,
    pub truncate_lock: rw_semaphore, pub i_crtime: timespec64,
    pub i_metadata_bhs: mapping_metadata_bhs, pub vfs_inode: inode,
}

#[repr(C)]
pub struct fat_slot_info {
    pub i_pos: loff_t, pub slot_off: loff_t, pub nr_slots: libc::c_int,
    pub de: *mut msdos_dir_entry, pub bh: *mut buffer_head,
}

#[inline]
pub unsafe fn MSDOS_SB(sb: *mut super_block) -> *mut msdos_sb_info {
    (*sb).s_fs_info as *mut msdos_sb_info
}
#[inline] pub unsafe fn is_fat12(sbi: *const msdos_sb_info) -> bool { (*sbi).fat_bits == 12 }
#[inline] pub unsafe fn is_fat16(sbi: *const msdos_sb_info) -> bool { (*sbi).fat_bits == 16 }
#[inline] pub unsafe fn is_fat32(sbi: *const msdos_sb_info) -> bool { (*sbi).fat_bits == 32 }
#[inline] pub unsafe fn max_fat(sb: *mut super_block) -> u32 {
    let sbi = MSDOS_SB(sb);
    if is_fat32(sbi) { MAX_FAT32 } else if is_fat16(sbi) { MAX_FAT16 } else { MAX_FAT12 }
}
#[inline] pub unsafe fn MSDOS_I(inode: *mut inode) -> *mut msdos_inode_info {
    container_of!(inode, msdos_inode_info, vfs_inode)
}

#[inline] pub unsafe fn fat_mode_can_hold_ro(inode: *mut inode) -> libc::c_int {
    let sbi = MSDOS_SB((*inode).i_sb); let mask: umode_t;
    if S_ISDIR!((*inode).i_mode) { if !(*sbi).options.rodir { return 0; } mask = !(*sbi).options.fs_dmask as umode_t; }
    else { mask = !(*sbi).options.fs_fmask as umode_t; }
    if mask & S_IWUGO == 0 { 0 } else { 1 }
}
#[inline] pub unsafe fn fat_make_mode(sbi: *mut msdos_sb_info, attrs: u8, mut mode: umode_t) -> umode_t {
    if attrs & ATTR_RO != 0 && !(attrs & ATTR_DIR != 0 && !(*sbi).options.rodir) { mode &= !S_IWUGO; }
    if attrs & ATTR_DIR != 0 { (mode & !(*sbi).options.fs_dmask as umode_t) | S_IFDIR } else { (mode & !(*sbi).options.fs_fmask as umode_t) | S_IFREG }
}
#[inline] pub unsafe fn fat_make_attrs(inode: *mut inode) -> u8 {
    let mut attrs = (*MSDOS_I(inode)).i_attrs as u8;
    if S_ISDIR!((*inode).i_mode) { attrs |= ATTR_DIR; }
    if fat_mode_can_hold_ro(inode) != 0 && (*inode).i_mode & S_IWUGO == 0 { attrs |= ATTR_RO; } attrs
}
#[inline] pub unsafe fn fat_save_attrs(inode: *mut inode, attrs: u8) { if fat_mode_can_hold_ro(inode) != 0 { (*MSDOS_I(inode)).i_attrs = (attrs & ATTR_UNUSED) as i32; } else { (*MSDOS_I(inode)).i_attrs = (attrs & (ATTR_UNUSED | ATTR_RO)) as i32; } }

#[inline] pub unsafe fn fat_checksum(name: *const u8) -> u8 { let mut s = *name; for i in 1..11 { s = s.wrapping_shl(7).wrapping_add(s.wrapping_shr(1)).wrapping_add(*name.add(i)); } s }
#[inline] pub unsafe fn fat_clus_to_blknr(sbi: *const msdos_sb_info, clus: libc::c_int) -> sector_t { ((clus as sector_t).wrapping_sub(FAT_START_ENT as sector_t)) * (*sbi).sec_per_clus as sector_t + (*sbi).data_start as sector_t }
#[inline] pub unsafe fn fat_get_blknr_offset(sbi: *const msdos_sb_info, i_pos: loff_t, blknr: *mut sector_t, offset: *mut libc::c_int) { *blknr = (i_pos >> (*sbi).dir_per_block_bits) as sector_t; *offset = (i_pos & ((*sbi).dir_per_block - 1) as loff_t) as libc::c_int; }

#[repr(C)] pub union fat_entry_u { pub ent12_p: [*mut u8; 2], pub ent16_p: *mut __le16, pub ent32_p: *mut __le32 }
#[repr(C)] pub struct fat_entry { pub entry: libc::c_int, pub u: fat_entry_u, pub nr_bhs: libc::c_int, pub bhs: [*mut buffer_head; 2], pub fat_inode: *mut inode }
#[inline] pub unsafe fn fatent_init(f: *mut fat_entry) { (*f).nr_bhs=0; (*f).entry=0; (*f).u.ent32_p=core::ptr::null_mut(); (*f).bhs=[core::ptr::null_mut();2]; (*f).fat_inode=core::ptr::null_mut(); }
#[inline] pub unsafe fn fatent_set_entry(f: *mut fat_entry, entry: libc::c_int) { (*f).entry=entry; (*f).u.ent32_p=core::ptr::null_mut(); }
#[inline] pub unsafe fn fat_valid_entry(sbi: *mut msdos_sb_info, entry: libc::c_int) -> bool { FAT_START_ENT <= entry && entry < (*sbi).max_cluster as libc::c_int }

extern "C" {
    pub fn fat_cache_inval_inode(inode: *mut inode); pub fn fat_get_cluster(inode: *mut inode, cluster: libc::c_int, fclus: *mut libc::c_int, dclus: *mut libc::c_int) -> libc::c_int;
    pub fn fat_get_mapped_cluster(inode: *mut inode, sector: sector_t, last_block: sector_t, mapped_blocks: *mut libc::c_ulong, bmap: *mut sector_t) -> libc::c_int;
    pub fn fat_bmap(inode: *mut inode, sector: sector_t, phys: *mut sector_t, mapped_blocks: *mut libc::c_ulong, create: libc::c_int, from_bmap: bool) -> libc::c_int;
    pub fn fat_ent_access_init(sb: *mut super_block); pub fn fat_ent_read(inode: *mut inode, fatent: *mut fat_entry, entry: libc::c_int) -> libc::c_int;
    pub fn fat_ent_write(inode: *mut inode, fatent: *mut fat_entry, new: libc::c_int, wait: libc::c_int) -> libc::c_int;
    pub fn fat_alloc_clusters(inode: *mut inode, cluster: *mut libc::c_int, nr_cluster: libc::c_int) -> libc::c_int;
    pub fn fat_free_clusters(inode: *mut inode, cluster: libc::c_int) -> libc::c_int; pub fn fat_count_free_clusters(sb: *mut super_block) -> libc::c_int;
    pub fn fat_trim_fs(inode: *mut inode, range: *mut fstrim_range) -> libc::c_int;
    pub fn fat_add_cluster(inode: *mut inode) -> libc::c_int; pub fn fat_clusters_flush(sb: *mut super_block) -> libc::c_int;
    pub fn fat_chain_add(inode: *mut inode, new_dclus: libc::c_int, nr_cluster: libc::c_int) -> libc::c_int;
    pub fn fat_search_long(inode: *mut inode, name: *const libc::c_uchar, name_len: libc::c_int, sinfo: *mut fat_slot_info) -> libc::c_int;
    pub fn fat_dir_empty(dir: *mut inode) -> libc::c_int; pub fn fat_subdirs(dir: *mut inode) -> libc::c_int;
    pub fn fat_scan(dir: *mut inode, name: *const libc::c_uchar, sinfo: *mut fat_slot_info) -> libc::c_int;
    pub fn fat_scan_logstart(dir: *mut inode, i_logstart: libc::c_int, sinfo: *mut fat_slot_info) -> libc::c_int;
    pub fn fat_get_dotdot_entry(dir: *mut inode, bh: *mut *mut buffer_head, de: *mut *mut msdos_dir_entry) -> libc::c_int;
    pub fn fat_alloc_new_dir(dir: *mut inode, ts: *mut timespec64) -> libc::c_int;
    pub fn fat_add_entries(dir: *mut inode, slots: *mut libc::c_void, nr_slots: libc::c_int, sinfo: *mut fat_slot_info) -> libc::c_int;
    pub fn fat_remove_entries(dir: *mut inode, sinfo: *mut fat_slot_info) -> libc::c_int;
    pub fn fat_block_truncate_page(inode: *mut inode, from: loff_t) -> libc::c_int;
    pub fn fat_attach(inode: *mut inode, i_pos: loff_t); pub fn fat_detach(inode: *mut inode);
    pub fn fat_iget(sb: *mut super_block, i_pos: loff_t) -> *mut inode;
    pub fn fat_build_inode(sb: *mut super_block, de: *mut msdos_dir_entry, i_pos: loff_t) -> *mut inode;
    pub fn fat_fill_super(sb: *mut super_block, fc: *mut fs_context, setup: Option<unsafe extern "C" fn(*mut super_block)>) -> libc::c_int;
    pub fn fat_fill_inode(inode: *mut inode, de: *mut msdos_dir_entry) -> libc::c_int;
    pub fn fat_flush_inodes(sb: *mut super_block, i1: *mut inode, i2: *mut inode) -> libc::c_int;
    pub fn fat_init_fs_context(fc: *mut fs_context, is_vfat: bool) -> libc::c_int;
    pub fn fat_free_fc(fc: *mut fs_context); pub fn fat_reconfigure(fc: *mut fs_context) -> libc::c_int;
    pub fn fat_parse_param(fc: *mut fs_context, param: *mut fs_parameter, is_vfat: bool) -> libc::c_int;
    pub fn __fat_fs_error(sb: *mut super_block, report: libc::c_int, fmt: *const libc::c_char, ...);
    pub fn _fat_msg(sb: *mut super_block, level: *const libc::c_char, fmt: *const libc::c_char, ...);
    pub fn fat_time_fat2unix(sbi: *mut msdos_sb_info, ts: *mut timespec64, time: __le16, date: __le16, time_cs: u8);
    pub fn fat_time_unix2fat(sbi: *mut msdos_sb_info, ts: *mut timespec64, time: *mut __le16, date: *mut __le16, time_cs: *mut u8);
    pub fn fat_truncate_atime(sbi: *const msdos_sb_info, ts: *const timespec64) -> timespec64;
    pub fn fat_truncate_time(inode: *mut inode, now: *mut timespec64, flags: libc::c_uint);
    pub fn fat_update_time(inode: *mut inode, ty: fs_update_time, flags: libc::c_uint) -> libc::c_int;
    pub fn fat_sync_bhs(bhs: *mut *mut buffer_head, nr_bhs: libc::c_int) -> libc::c_int;
}

pub const FAT_UPDATE_ATIME: u32 = 1u32 << 0;
pub const FAT_UPDATE_CMTIME: u32 = 1u32 << 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
