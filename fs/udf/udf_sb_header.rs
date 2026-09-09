/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: linux/mutex.h, linux/bitops.h, and linux/magic.h. */

/* Even UDF 2.6 media should have version <= 0x250, but broken filesystems
 * with version 0x260 exist; accommodate those. */
pub const UDF_MAX_READ_VERSION: u32 = 0x0260;
pub const UDF_MAX_WRITE_VERSION: u32 = 0x0201;

pub const UDF_FLAG_USE_EXTENDED_FE: i32 = 0;
pub const UDF_VERS_USE_EXTENDED_FE: u32 = 0x0200;
pub const UDF_FLAG_USE_STREAMS: i32 = 1;
pub const UDF_VERS_USE_STREAMS: u32 = 0x0200;
pub const UDF_FLAG_USE_SHORT_AD: i32 = 2;
pub const UDF_FLAG_USE_AD_IN_ICB: i32 = 3;
pub const UDF_FLAG_USE_FILE_CTIME_EA: i32 = 4;
pub const UDF_FLAG_STRICT: i32 = 5;
pub const UDF_FLAG_UNDELETE: i32 = 6;
pub const UDF_FLAG_UNHIDE: i32 = 7;
pub const UDF_FLAG_NOVRS: i32 = 8;
pub const UDF_FLAG_UID_FORGET: i32 = 11; // save -1 for uid to disk
pub const UDF_FLAG_GID_FORGET: i32 = 12;
pub const UDF_FLAG_UID_SET: i32 = 13;
pub const UDF_FLAG_GID_SET: i32 = 14;
pub const UDF_FLAG_SESSION_SET: i32 = 15;
pub const UDF_FLAG_LASTBLOCK_SET: i32 = 16;
pub const UDF_FLAG_BLOCKSIZE_SET: i32 = 17;
pub const UDF_FLAG_INCONSISTENT: i32 = 18;
pub const UDF_FLAG_RW_INCOMPAT: i32 = 19; // Set when an RW incompatible feature is found.

pub const UDF_PART_FLAG_UNALLOC_BITMAP: u16 = 0x0001;
pub const UDF_PART_FLAG_UNALLOC_TABLE: u16 = 0x0002;
pub const UDF_PART_FLAG_READ_ONLY: u16 = 0x0010;
pub const UDF_PART_FLAG_WRITE_ONCE: u16 = 0x0020;
pub const UDF_PART_FLAG_REWRITABLE: u16 = 0x0040;
pub const UDF_PART_FLAG_OVERWRITABLE: u16 = 0x0080;
pub const UDF_MAX_BLOCK_LOADED: usize = 8;

pub const UDF_TYPE1_MAP15: u16 = 0x1511;
pub const UDF_VIRTUAL_MAP15: u16 = 0x1512;
pub const UDF_VIRTUAL_MAP20: u16 = 0x2012;
pub const UDF_SPARABLE_MAP15: u16 = 0x1522;
pub const UDF_METADATA_MAP25: u16 = 0x2511;

pub const UDF_INVALID_MODE: umode_t = !0 as umode_t;
pub const MF_DUPLICATE_MD: i32 = 0x01;
pub const MF_MIRROR_FE_LOADED: i32 = 0x02;

#[repr(C)]
pub struct udf_meta_data {
    pub s_meta_file_loc: __u32,
    pub s_mirror_file_loc: __u32,
    pub s_bitmap_file_loc: __u32,
    pub s_alloc_unit_size: __u32,
    pub s_align_unit_size: __u16,
    /* Partition Reference Number of the associated physical / sparable partition. */
    pub s_phys_partition_ref: __u16,
    pub s_flags: i32,
    pub s_metadata_fe: *mut inode,
    pub s_mirror_fe: *mut inode,
    pub s_bitmap_fe: *mut inode,
}

#[repr(C)]
pub struct udf_sparing_data {
    pub s_packet_len: __u16,
    pub s_spar_map: [*mut buffer_head; 4],
}

#[repr(C)]
pub struct udf_virtual_data {
    pub s_num_entries: __u32,
    pub s_start_offset: __u16,
}

#[repr(C)]
pub struct udf_bitmap {
    pub s_extPosition: __u32,
    pub s_nr_groups: i32,
    pub s_block_bitmap: [*mut buffer_head; 0],
}

#[repr(C)]
pub union udf_part_map_s_uspace {
    pub s_bitmap: *mut udf_bitmap,
    pub s_table: *mut inode,
}

#[repr(C)]
pub union udf_part_map_s_type_specific {
    pub s_sparing: udf_sparing_data,
    pub s_virtual: udf_virtual_data,
    pub s_metadata: udf_meta_data,
}

#[repr(C)]
pub struct udf_part_map {
    pub s_uspace: udf_part_map_s_uspace,
    pub s_partition_root: __u32,
    pub s_partition_len: __u32,
    pub s_partition_type: __u16,
    pub s_partition_num: __u16,
    pub s_type_specific: udf_part_map_s_type_specific,
    pub s_partition_func: Option<unsafe extern "C" fn(*mut super_block, __u32, __u16, __u32) -> __u32>,
    pub s_volumeseqnum: __u16,
    pub s_partition_flags: __u16,
}

#[repr(C)]
pub struct udf_sb_info {
    pub s_partmaps: *mut udf_part_map,
    pub s_volume_ident: [__u8; 32],
    pub s_partitions: __u16,
    pub s_partition: __u16,
    pub s_session: __s32,
    pub s_anchor: __u32,
    pub s_last_block: __u32,
    pub s_lvid_bh: *mut buffer_head,
    pub s_umask: umode_t,
    pub s_gid: kgid_t,
    pub s_uid: kuid_t,
    pub s_fmode: umode_t,
    pub s_dmode: umode_t,
    pub s_cred_lock: rwlock_t,
    pub s_record_time: timespec64,
    pub s_serial_number: __u16,
    pub s_udfrev: __u16,
    pub s_flags: c_ulong,
    pub s_nls_map: *mut nls_table,
    pub s_vat_inode: *mut inode,
    pub s_alloc_mutex: mutex,
    pub s_lvid_dirty: c_uint,
}

#[inline]
pub unsafe fn UDF_SB(sb: *mut super_block) -> *mut udf_sb_info {
    (*sb).s_fs_info as *mut udf_sb_info
}

extern "C" {
    pub fn udf_sb_lvidiu(sb: *mut super_block) -> *mut logicalVolIntegrityDescImpUse;
    pub fn udf_compute_nr_groups(sb: *mut super_block, partition: u32) -> i32;
}

#[inline]
pub unsafe fn UDF_QUERY_FLAG(sb: *mut super_block, flag: i32) -> bool {
    test_bit(flag, &(*UDF_SB(sb)).s_flags) != 0
}

#[inline]
pub unsafe fn UDF_SET_FLAG(sb: *mut super_block, flag: i32) {
    set_bit(flag, &mut (*UDF_SB(sb)).s_flags);
}

#[inline]
pub unsafe fn UDF_CLEAR_FLAG(sb: *mut super_block, flag: i32) {
    clear_bit(flag, &mut (*UDF_SB(sb)).s_flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
