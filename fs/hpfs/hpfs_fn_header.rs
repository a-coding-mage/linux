/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of linux/fs/hpfs/hpfs_fn.h. Kernel includes are external dependencies. */

pub const EIOERROR: i32 = EIO;
pub const EFSERROR: i32 = EUCLEAN;
pub const ANODE_ALLOC_FWD: u32 = 512;
pub const FNODE_ALLOC_FWD: u32 = 0;
pub const ALLOC_FWD_MIN: u32 = 16;
pub const ALLOC_FWD_MAX: u32 = 128;
pub const ALLOC_M: u32 = 1;
pub const FNODE_RD_AHEAD: u32 = 16;
pub const ANODE_RD_AHEAD: u32 = 0;
pub const DNODE_RD_AHEAD: u32 = 72;
pub const COUNT_RD_AHEAD: u32 = 62;
pub const FREE_DNODES_ADD: u32 = 58;
pub const FREE_DNODES_DEL: u32 = 29;

#[macro_export]
macro_rules! CHKCOND { ($x:expr, $msg:expr) => { if !$x { unsafe { printk($msg); } } }; }

#[repr(C)]
pub struct hpfs_inode_info {
    pub mmu_private: loff_t,
    pub i_parent_dir: ino_t,
    pub i_dno: ::core::ffi::c_uint,
    pub i_dpos: ::core::ffi::c_uint,
    pub i_dsubdno: ::core::ffi::c_uint,
    pub i_file_sec: ::core::ffi::c_uint,
    pub i_disk_sec: ::core::ffi::c_uint,
    pub i_n_secs: ::core::ffi::c_uint,
    pub i_ea_size: ::core::ffi::c_uint,
    pub i_ea_mode: ::core::ffi::c_uint,
    pub i_ea_uid: ::core::ffi::c_uint,
    pub i_ea_gid: ::core::ffi::c_uint,
    pub i_dirty: ::core::ffi::c_uint,
    pub i_rddir_off: *mut *mut loff_t,
    pub vfs_inode: inode,
}

#[repr(C)]
pub struct hpfs_sb_info {
    pub hpfs_mutex: mutex,
    pub sb_root: ino_t,
    pub sb_fs_size: ::core::ffi::c_uint,
    pub sb_bitmaps: ::core::ffi::c_uint,
    pub sb_dirband_start: ::core::ffi::c_uint,
    pub sb_dirband_size: ::core::ffi::c_uint,
    pub sb_dmap: ::core::ffi::c_uint,
    pub sb_n_free: ::core::ffi::c_uint,
    pub sb_n_free_dnodes: ::core::ffi::c_uint,
    pub sb_uid: kuid_t,
    pub sb_gid: kgid_t,
    pub sb_mode: umode_t,
    pub sb_eas: ::core::ffi::c_uint,
    pub sb_err: ::core::ffi::c_uint,
    pub sb_chk: ::core::ffi::c_uint,
    pub sb_lowercase: ::core::ffi::c_uint,
    pub sb_was_error: ::core::ffi::c_uint,
    pub sb_chkdsk: ::core::ffi::c_uint,
    pub sb_cp_table: *mut ::core::ffi::c_uchar,
    pub sb_bmp_dir: *mut __le32,
    pub sb_c_bitmap: ::core::ffi::c_uint,
    pub sb_max_fwd_alloc: ::core::ffi::c_uint,
    pub sb_timeshift: ::core::ffi::c_int,
    pub rcu: rcu_head,
    pub n_hotfixes: ::core::ffi::c_uint,
    pub hotfix_from: [secno; 256],
    pub hotfix_to: [secno; 256],
}

#[repr(C)]
pub struct quad_buffer_head {
    pub bh: [*mut buffer_head; 4],
    pub data: *mut ::core::ffi::c_void,
}

#[inline]
pub unsafe fn de_down_pointer(de: *mut hpfs_dirent) -> dnode_secno {
    CHKCOND((*de).down != 0, "HPFS: de_down_pointer: !de->down\n");
    le32_to_cpu(*((de as *mut u8).add(le16_to_cpu((*de).length) as usize - 4) as *const __le32))
}

#[inline]
pub unsafe fn dnode_first_de(dnode: *mut dnode) -> *mut hpfs_dirent { (*dnode).dirent as *mut hpfs_dirent }

#[inline]
pub unsafe fn dnode_end_de(dnode: *mut dnode) -> *mut hpfs_dirent {
    let p = le32_to_cpu((*dnode).first_free);
    CHKCOND(p >= 0x14 && p <= 0xa00, "HPFS: dnode_end_de: invalid first_free\n");
    (dnode as *mut u8).add(p as usize) as *mut hpfs_dirent
}

#[inline]
pub unsafe fn de_next_de(de: *mut hpfs_dirent) -> *mut hpfs_dirent {
    let n = le16_to_cpu((*de).length);
    CHKCOND(n >= 0x20 && n < 0x800, "HPFS: de_next_de: invalid length\n");
    (de as *mut u8).add(n as usize) as *mut hpfs_dirent
}

#[inline]
pub unsafe fn fnode_ea(fnode: *mut fnode) -> *mut extended_attribute {
    (fnode as *mut u8).add((le16_to_cpu((*fnode).ea_offs) + le16_to_cpu((*fnode).acl_size_s)) as usize) as *mut extended_attribute
}
#[inline]
pub unsafe fn fnode_end_ea(fnode: *mut fnode) -> *mut extended_attribute {
    (fnode as *mut u8).add((le16_to_cpu((*fnode).ea_offs) + le16_to_cpu((*fnode).acl_size_s) + le16_to_cpu((*fnode).ea_size_s)) as usize) as *mut extended_attribute
}
#[inline]
pub unsafe fn ea_valuelen(ea: *mut extended_attribute) -> u32 { (*ea).valuelen_lo as u32 + 256 * (*ea).valuelen_hi as u32 }
#[inline]
pub unsafe fn next_ea(ea: *mut extended_attribute) -> *mut extended_attribute { (ea as *mut u8).add(5 + (*ea).namelen as usize + ea_valuelen(ea) as usize) as *mut extended_attribute }
#[inline]
pub unsafe fn ea_sec(ea: *mut extended_attribute) -> secno { le32_to_cpu(get_unaligned((ea as *mut u8).add(9 + (*ea).namelen as usize) as *const __le32)) }
#[inline]
pub unsafe fn ea_len(ea: *mut extended_attribute) -> secno { le32_to_cpu(get_unaligned((ea as *mut u8).add(5 + (*ea).namelen as usize) as *const __le32)) }
#[inline]
pub unsafe fn ea_data(ea: *mut extended_attribute) -> *mut i8 { (ea as *mut u8).add(5 + (*ea).namelen as usize) as *mut i8 }
#[inline]
pub fn de_size(namelen: i32, down_ptr: secno) -> u32 { (((0x1f + namelen + 3) as u32) & !3) + if down_ptr != 0 { 4 } else { 0 } }
#[inline]
pub unsafe fn copy_de(dst: *mut hpfs_dirent, src: *mut hpfs_dirent) {
    if dst.is_null() || src.is_null() { return; }
    let a = (*dst).down; let n = (*dst).not_8x3;
    core::ptr::copy_nonoverlapping((src as *const u8).add(2), (dst as *mut u8).add(2), 28);
    (*dst).down = a; (*dst).not_8x3 = n;
}
#[inline]
pub unsafe fn tstbits(bmp: *mut __le32, b: u32, n: u32) -> i32 {
    if b >= 0x4000 || b + n - 1 >= 0x4000 { return n as i32; }
    if ((le32_to_cpu(*bmp.add(((b & 0x3fff) >> 5) as usize)) >> (b & 0x1f)) & 1) == 0 { return 1; }
    let mut i = 1; while i < n { let x = b + i; if ((le32_to_cpu(*bmp.add(((x & 0x3fff) >> 5) as usize)) >> (x & 0x1f)) & 1) == 0 { return (i + 1) as i32; } i += 1; } 0
}

extern "C" {
    pub fn hpfs_chk_sectors(_: *mut super_block, _: secno, _: i32, _: *mut i8) -> i32;
    pub fn hpfs_alloc_sector(_: *mut super_block, _: secno, _: u32, _: i32) -> secno;
    pub fn hpfs_alloc_if_possible(_: *mut super_block, _: secno) -> i32;
    pub fn hpfs_free_sectors(_: *mut super_block, _: secno, _: u32);
    pub fn hpfs_check_free_dnodes(_: *mut super_block, _: i32) -> i32;
    pub fn hpfs_free_dnode(_: *mut super_block, _: secno);
    pub fn hpfs_alloc_dnode(_: *mut super_block, _: secno, _: *mut dnode_secno, _: *mut quad_buffer_head) -> *mut dnode;
    pub fn hpfs_alloc_fnode(_: *mut super_block, _: secno, _: *mut fnode_secno, _: *mut *mut buffer_head) -> *mut fnode;
    pub fn hpfs_alloc_anode(_: *mut super_block, _: secno, _: *mut anode_secno, _: *mut *mut buffer_head) -> *mut anode;
    pub fn hpfs_trim_fs(_: *mut super_block, _: u64, _: u64, _: u64, _: *mut u32) -> i32;
    pub fn hpfs_bplus_lookup(_: *mut super_block, _: *mut inode, _: *mut bplus_header, _: u32, _: *mut buffer_head) -> secno;
    pub fn hpfs_add_sector_to_btree(_: *mut super_block, _: secno, _: i32, _: u32) -> secno;
    pub fn hpfs_remove_btree(_: *mut super_block, _: *mut bplus_header);
    pub fn hpfs_ea_read(_: *mut super_block, _: secno, _: i32, _: u32, _: u32, _: *mut i8) -> i32;
    pub fn hpfs_ea_write(_: *mut super_block, _: secno, _: i32, _: u32, _: u32, _: *const i8) -> i32;
    pub fn hpfs_ea_remove(_: *mut super_block, _: secno, _: i32, _: u32);
    pub fn hpfs_truncate_btree(_: *mut super_block, _: secno, _: i32, _: u32);
    pub fn hpfs_remove_fnode(_: *mut super_block, _: fnode_secno);
    pub fn hpfs_search_hotfix_map(_: *mut super_block, _: secno) -> secno;
    pub fn hpfs_search_hotfix_map_for_range(_: *mut super_block, _: secno, _: u32) -> u32;
    pub fn hpfs_prefetch_sectors(_: *mut super_block, _: u32, _: i32);
    pub fn hpfs_map_sector(_: *mut super_block, _: u32, _: *mut *mut buffer_head, _: i32) -> *mut ::core::ffi::c_void;
    pub fn hpfs_get_sector(_: *mut super_block, _: u32, _: *mut *mut buffer_head) -> *mut ::core::ffi::c_void;
    pub fn hpfs_map_4sectors(_: *mut super_block, _: u32, _: *mut quad_buffer_head, _: i32) -> *mut ::core::ffi::c_void;
    pub fn hpfs_get_4sectors(_: *mut super_block, _: u32, _: *mut quad_buffer_head) -> *mut ::core::ffi::c_void;
    pub fn hpfs_brelse4(_: *mut quad_buffer_head);
    pub fn hpfs_mark_4buffers_dirty(_: *mut quad_buffer_head);
    pub static hpfs_dentry_operations: dentry_operations;
    pub fn hpfs_lookup(_: *mut inode, _: *mut dentry, _: u32) -> *mut dentry;
    pub static hpfs_dir_ops: file_operations;
    pub fn hpfs_add_pos(_: *mut inode, _: *mut loff_t) -> i32;
    pub fn hpfs_del_pos(_: *mut inode, _: *mut loff_t);
    pub fn hpfs_add_de(_: *mut super_block, _: *mut dnode, _: *const u8, _: u32, _: secno) -> *mut hpfs_dirent;
    pub fn hpfs_add_dirent(_: *mut inode, _: *const u8, _: u32, _: *mut hpfs_dirent) -> i32;
    pub fn hpfs_remove_dirent(_: *mut inode, _: dnode_secno, _: *mut hpfs_dirent, _: *mut quad_buffer_head, _: i32) -> i32;
    pub fn hpfs_count_dnodes(_: *mut super_block, _: dnode_secno, _: *mut i32, _: *mut i32, _: *mut i32);
    pub fn hpfs_de_as_down_as_possible(_: *mut super_block, _: dnode_secno) -> dnode_secno;
    pub fn map_pos_dirent(_: *mut inode, _: *mut loff_t, _: *mut quad_buffer_head) -> *mut hpfs_dirent;
    pub fn map_dirent(_: *mut inode, _: dnode_secno, _: *const u8, _: u32, _: *mut dnode_secno, _: *mut quad_buffer_head) -> *mut hpfs_dirent;
    pub fn hpfs_remove_dtree(_: *mut super_block, _: dnode_secno);
    pub fn map_fnode_dirent(_: *mut super_block, _: fnode_secno, _: *mut fnode, _: *mut quad_buffer_head) -> *mut hpfs_dirent;
    pub fn hpfs_ea_ext_remove(_: *mut super_block, _: secno, _: i32, _: u32);
    pub fn hpfs_read_ea(_: *mut super_block, _: *mut fnode, _: *mut i8, _: *mut i8, _: i32) -> i32;
    pub fn hpfs_get_ea(_: *mut super_block, _: *mut fnode, _: *mut i8, _: *mut i32) -> *mut i8;
    pub fn hpfs_set_ea(_: *mut inode, _: *mut fnode, _: *const i8, _: *const i8, _: i32);
    pub fn hpfs_file_fsync(_: *mut file, _: loff_t, _: loff_t, _: i32) -> i32;
    pub fn hpfs_truncate(_: *mut inode);
    pub static hpfs_file_ops: file_operations;
    pub static hpfs_file_iops: inode_operations;
    pub static hpfs_aops: address_space_operations;
    pub fn hpfs_init_inode(_: *mut inode);
    pub fn hpfs_read_inode(_: *mut inode);
    pub fn hpfs_write_inode(_: *mut inode);
    pub fn hpfs_write_inode_nolock(_: *mut inode);
    pub fn hpfs_setattr(_: *mut mnt_idmap, _: *mut dentry, _: *mut iattr) -> i32;
    pub fn hpfs_write_if_changed(_: *mut inode);
    pub fn hpfs_evict_inode(_: *mut inode);
    pub fn hpfs_map_dnode_bitmap(_: *mut super_block, _: *mut quad_buffer_head) -> *mut __le32;
    pub fn hpfs_map_bitmap(_: *mut super_block, _: u32, _: *mut quad_buffer_head, _: *mut i8) -> *mut __le32;
    pub fn hpfs_prefetch_bitmap(_: *mut super_block, _: u32);
    pub fn hpfs_load_code_page(_: *mut super_block, _: secno) -> *mut u8;
    pub fn hpfs_load_bitmap_directory(_: *mut super_block, _: secno) -> *mut __le32;
    pub fn hpfs_load_hotfix_map(_: *mut super_block, _: *mut hpfs_spare_block);
    pub fn hpfs_map_fnode(_: *mut super_block, _: ino_t, _: *mut *mut buffer_head) -> *mut fnode;
    pub fn hpfs_map_anode(_: *mut super_block, _: anode_secno, _: *mut *mut buffer_head) -> *mut anode;
    pub fn hpfs_map_dnode(_: *mut super_block, _: dnode_secno, _: *mut quad_buffer_head) -> *mut dnode;
    pub fn hpfs_fnode_dno(_: *mut super_block, _: ino_t) -> dnode_secno;
    pub fn hpfs_upcase(_: *mut u8, _: u8) -> u8;
    pub fn hpfs_chk_name(_: *const u8, _: *mut u32) -> i32;
    pub fn hpfs_translate_name(_: *mut super_block, _: *mut u8, _: u32, _: i32, _: i32) -> *mut u8;
    pub fn hpfs_compare_names(_: *mut super_block, _: *const u8, _: u32, _: *const u8, _: u32, _: i32) -> i32;
    pub fn hpfs_is_name_long(_: *const u8, _: u32) -> i32;
    pub fn hpfs_adjust_length(_: *const u8, _: *mut u32);
    pub static hpfs_dir_iops: inode_operations;
    pub static hpfs_symlink_aops: address_space_operations;
    pub fn hpfs_error(_: *mut super_block, _: *const i8, ...);
    pub fn hpfs_stop_cycles(_: *mut super_block, _: i32, _: *mut i32, _: *mut i32, _: *mut i8) -> i32;
    pub fn hpfs_get_free_dnodes(_: *mut super_block) -> u32;
    pub fn hpfs_ioctl(_: *mut file, _: u32, _: c_ulong) -> c_long;
}

#[inline] pub unsafe fn hpfs_i(inode: *mut inode) -> *mut hpfs_inode_info { container_of(inode, hpfs_inode_info, vfs_inode) }
#[inline] pub unsafe fn hpfs_sb(sb: *mut super_block) -> *mut hpfs_sb_info { (*sb).s_fs_info as *mut hpfs_sb_info }
#[inline] pub unsafe fn local_to_gmt(s: *mut super_block, t: time64_t) -> time64_t { t + sys_tz.tz_minuteswest as i64 * 60 + (*hpfs_sb(s)).sb_timeshift as i64 }
#[inline] pub unsafe fn gmt_to_local(s: *mut super_block, t: time64_t) -> time32_t { (t - sys_tz.tz_minuteswest as i64 * 60 - (*hpfs_sb(s)).sb_timeshift as i64) as time32_t }
#[inline] pub unsafe fn local_get_seconds(s: *mut super_block) -> time32_t { gmt_to_local(s, ktime_get_real_seconds()) }
#[inline] pub unsafe fn hpfs_lock(s: *mut super_block) { mutex_lock(&mut (*hpfs_sb(s)).hpfs_mutex); }
#[inline] pub unsafe fn hpfs_unlock(s: *mut super_block) { mutex_unlock(&mut (*hpfs_sb(s)).hpfs_mutex); }
#[inline] pub unsafe fn hpfs_lock_assert(s: *mut super_block) { WARN_ON(!mutex_is_locked(&(*hpfs_sb(s)).hpfs_mutex)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
