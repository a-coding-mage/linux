// SPDX-License-Identifier: GPL-2.0
// Translation of linux/f2fs_fs.h. Kernel-provided types, constants, and
// helpers referenced below remain external dependencies.

pub const F2FS_SUPER_OFFSET: usize = 1024;
pub const F2FS_MIN_LOG_SECTOR_SIZE: usize = 9;
pub const F2FS_MAX_LOG_SECTOR_SIZE: usize = PAGE_SHIFT;
pub const F2FS_LOG_SECTORS_PER_BLOCK: usize = PAGE_SHIFT - 9;
pub const F2FS_BLKSIZE: usize = PAGE_SIZE;
pub const F2FS_BLKSIZE_BITS: usize = PAGE_SHIFT;
pub const F2FS_MAX_EXTENSION: usize = 64;
pub const F2FS_EXTENSION_LEN: usize = 8;

pub const NULL_ADDR: block_t = 0;
pub const NEW_ADDR: block_t = (-1i32) as block_t;
pub const COMPRESS_ADDR: block_t = (-2i32) as block_t;
pub const F2FS_BLKSIZE_MASK: usize = F2FS_BLKSIZE - 1;
pub const F2FS_RESERVED_NODE_NUM: usize = 3;
pub const F2FS_MAX_QUOTAS: usize = 3;
pub const F2FS_ENC_UTF8_12_1: usize = 1;
pub const MAX_ACTIVE_LOGS: usize = 16;
pub const MAX_ACTIVE_NODE_LOGS: usize = 8;
pub const MAX_ACTIVE_DATA_LOGS: usize = 8;
pub const VERSION_LEN: usize = 256;
pub const MAX_VOLUME_NAME: usize = 512;
pub const MAX_PATH_LEN: usize = 64;
pub const MAX_DEVICES: usize = 8;
pub const MAX_STOP_REASON: usize = 32;
pub const MAX_F2FS_ERRORS: usize = 16;

#[repr(C, packed)]
pub struct f2fs_device { pub path: [__u8; MAX_PATH_LEN], pub total_segments: __le32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum stop_cp_reason { STOP_CP_REASON_SHUTDOWN, STOP_CP_REASON_FAULT_INJECT, STOP_CP_REASON_META_PAGE, STOP_CP_REASON_WRITE_FAIL, STOP_CP_REASON_CORRUPTED_SUMMARY, STOP_CP_REASON_UPDATE_INODE, STOP_CP_REASON_FLUSH_FAIL, STOP_CP_REASON_NO_SEGMENT, STOP_CP_REASON_CORRUPTED_FREE_BITMAP, STOP_CP_REASON_CORRUPTED_NID, STOP_CP_REASON_READ_META, STOP_CP_REASON_READ_NODE, STOP_CP_REASON_READ_DATA, STOP_CP_REASON_MAX }
pub const MAX_F2FS_ERROR_REASON: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum f2fs_error { ERROR_CORRUPTED_CLUSTER, ERROR_FAIL_DECOMPRESSION, ERROR_INVALID_BLKADDR, ERROR_CORRUPTED_DIRENT, ERROR_CORRUPTED_INODE, ERROR_INCONSISTENT_SUMMARY, ERROR_INCONSISTENT_FOOTER, ERROR_INCONSISTENT_SUM_TYPE, ERROR_CORRUPTED_JOURNAL, ERROR_INCONSISTENT_NODE_COUNT, ERROR_INCONSISTENT_BLOCK_COUNT, ERROR_INVALID_CURSEG, ERROR_INCONSISTENT_SIT, ERROR_CORRUPTED_VERITY_XATTR, ERROR_CORRUPTED_XATTR, ERROR_INVALID_NODE_REFERENCE, ERROR_INCONSISTENT_NAT, ERROR_INCONSISTENT_ORPHAN, ERROR_MAX }

#[repr(C, packed)]
pub struct f2fs_super_block {
    pub magic: __le32, pub major_ver: __le16, pub minor_ver: __le16,
    pub log_sectorsize: __le32, pub log_sectors_per_block: __le32, pub log_blocksize: __le32, pub log_blocks_per_seg: __le32,
    pub segs_per_sec: __le32, pub secs_per_zone: __le32, pub checksum_offset: __le32, pub block_count: __le64,
    pub section_count: __le32, pub segment_count: __le32, pub segment_count_ckpt: __le32, pub segment_count_sit: __le32,
    pub segment_count_nat: __le32, pub segment_count_ssa: __le32, pub segment_count_main: __le32,
    pub segment0_blkaddr: __le32, pub cp_blkaddr: __le32, pub sit_blkaddr: __le32, pub nat_blkaddr: __le32, pub ssa_blkaddr: __le32, pub main_blkaddr: __le32,
    pub root_ino: __le32, pub node_ino: __le32, pub meta_ino: __le32, pub uuid: [__u8; 16], pub volume_name: [__le16; MAX_VOLUME_NAME],
    pub extension_count: __le32, pub extension_list: [[__u8; F2FS_EXTENSION_LEN]; F2FS_MAX_EXTENSION], pub cp_payload: __le32,
    pub version: [__u8; VERSION_LEN], pub init_version: [__u8; VERSION_LEN], pub feature: __le32, pub encryption_level: __u8, pub encrypt_pw_salt: [__u8; 16],
    pub devs: [f2fs_device; MAX_DEVICES], pub qf_ino: [__le32; F2FS_MAX_QUOTAS], pub hot_ext_count: __u8, pub s_encoding: __le16, pub s_encoding_flags: __le16,
    pub s_stop_reason: [__u8; MAX_STOP_REASON], pub s_errors: [__u8; MAX_F2FS_ERRORS], pub reserved: [__u8; 258], pub crc: __le32,
}

pub const CP_RESIZEFS_FLAG: u32 = 0x00004000; pub const CP_DISABLED_QUICK_FLAG: u32 = 0x00002000; pub const CP_DISABLED_FLAG: u32 = 0x00001000;
pub const CP_QUOTA_NEED_FSCK_FLAG: u32 = 0x00000800; pub const CP_LARGE_NAT_BITMAP_FLAG: u32 = 0x00000400; pub const CP_NOCRC_RECOVERY_FLAG: u32 = 0x00000200;
pub const CP_TRIMMED_FLAG: u32 = 0x00000100; pub const CP_NAT_BITS_FLAG: u32 = 0x80; pub const CP_CRC_RECOVERY_FLAG: u32 = 0x40;
pub const CP_FASTBOOT_FLAG: u32 = 0x20; pub const CP_FSCK_FLAG: u32 = 0x10; pub const CP_ERROR_FLAG: u32 = 8; pub const CP_COMPACT_SUM_FLAG: u32 = 4;
pub const CP_ORPHAN_PRESENT_FLAG: u32 = 2; pub const CP_UMOUNT_FLAG: u32 = 1; pub const F2FS_CP_PACKS: usize = 2;

#[repr(C, packed)]
pub struct f2fs_checkpoint { pub checkpoint_ver: __le64, pub user_block_count: __le64, pub valid_block_count: __le64, pub rsvd_segment_count: __le32, pub overprov_segment_count: __le32, pub free_segment_count: __le32, pub cur_node_segno: [__le32; MAX_ACTIVE_NODE_LOGS], pub cur_node_blkoff: [__le16; MAX_ACTIVE_NODE_LOGS], pub cur_data_segno: [__le32; MAX_ACTIVE_DATA_LOGS], pub cur_data_blkoff: [__le16; MAX_ACTIVE_DATA_LOGS], pub ckpt_flags: __le32, pub cp_pack_total_block_count: __le32, pub cp_pack_start_sum: __le32, pub valid_node_count: __le32, pub valid_inode_count: __le32, pub next_free_nid: __le32, pub sit_ver_bitmap_bytesize: __le32, pub nat_ver_bitmap_bytesize: __le32, pub checksum_offset: __le32, pub elapsed_time: __le64, pub alloc_type: [u8; MAX_ACTIVE_LOGS], pub sit_nat_version_bitmap: [u8; 0] }
pub const CP_CHKSUM_OFFSET: usize = F2FS_BLKSIZE - core::mem::size_of::<__le32>();
pub const F2FS_ORPHANS_PER_BLOCK: usize = (F2FS_BLKSIZE - 4 * core::mem::size_of::<__le32>()) / core::mem::size_of::<__le32>();
pub const F2FS_NAME_LEN: usize = 255; pub const DEFAULT_INLINE_XATTR_ADDRS: usize = 50; pub const OFFSET_OF_END_OF_I_EXT: usize = 360; pub const SIZE_OF_I_NID: usize = 20;

#[repr(C, packed)] pub struct f2fs_orphan_block { pub ino: [__le32; F2FS_ORPHANS_PER_BLOCK], pub reserved: __le32, pub blk_addr: __le16, pub blk_count: __le16, pub entry_count: __le32, pub check_sum: __le32 }
#[repr(C, packed)] pub struct f2fs_extent { pub fofs: __le32, pub blk: __le32, pub len: __le32 }
#[repr(C, packed)] pub struct node_footer { pub nid: __le32, pub ino: __le32, pub flag: __le32, pub cp_ver: __le64, pub next_blkaddr: __le32 }
pub const DEF_ADDRS_PER_INODE: usize = (F2FS_BLKSIZE - OFFSET_OF_END_OF_I_EXT - SIZE_OF_I_NID - core::mem::size_of::<node_footer>()) / core::mem::size_of::<__le32>();
pub const DEF_NIDS_PER_INODE: usize = 5; pub const DEF_ADDRS_PER_BLOCK: usize = (F2FS_BLKSIZE - core::mem::size_of::<node_footer>()) / core::mem::size_of::<__le32>(); pub const NIDS_PER_BLOCK: usize = DEF_ADDRS_PER_BLOCK;

#[repr(C)] pub union f2fs_inode_extra { pub extra: f2fs_inode_extra_fields, pub i_addr: [__le32; DEF_ADDRS_PER_INODE] }
#[repr(C, packed)] pub struct f2fs_inode_extra_fields { pub i_extra_isize: __le16, pub i_inline_xattr_size: __le16, pub i_projid: __le32, pub i_inode_checksum: __le32, pub i_crtime: __le64, pub i_crtime_nsec: __le32, pub i_compr_blocks: __le64, pub i_compress_algorithm: __u8, pub i_log_cluster_size: __u8, pub i_compress_flag: __le16, pub i_extra_end: [__le32; 0] }
#[repr(C, packed)] pub struct f2fs_inode { pub i_mode: __le16, pub i_advise: __u8, pub i_inline: __u8, pub i_uid: __le32, pub i_gid: __le32, pub i_links: __le32, pub i_size: __le64, pub i_blocks: __le64, pub i_atime: __le64, pub i_ctime: __le64, pub i_mtime: __le64, pub i_atime_nsec: __le32, pub i_ctime_nsec: __le32, pub i_mtime_nsec: __le32, pub i_generation: __le32, pub i_current_depth: __le32, pub i_xattr_nid: __le32, pub i_flags: __le32, pub i_pino: __le32, pub i_namelen: __le32, pub i_name: [__u8; F2FS_NAME_LEN], pub i_dir_level: __u8, pub i_ext: f2fs_extent, pub extra: f2fs_inode_extra, pub i_nid: [__le32; DEF_NIDS_PER_INODE] }
#[repr(C, packed)] pub struct direct_node { pub addr: [__le32; DEF_ADDRS_PER_BLOCK] }
#[repr(C, packed)] pub struct indirect_node { pub nid: [__le32; NIDS_PER_BLOCK] }
pub const COLD_BIT_SHIFT: usize = 0; pub const FSYNC_BIT_SHIFT: usize = 1; pub const DENT_BIT_SHIFT: usize = 2; pub const OFFSET_BIT_SHIFT: usize = 3;
#[repr(C)] pub union f2fs_node_body { pub i: f2fs_inode, pub dn: direct_node, pub indirect: indirect_node }
#[repr(C, packed)] pub struct f2fs_node { pub body: f2fs_node_body, pub footer: node_footer }

pub const SIT_VBLOCK_MAP_SIZE: usize = 64;
#[repr(C, packed)] pub struct f2fs_nat_entry { pub version: __u8, pub ino: __le32, pub block_addr: __le32 }
pub const NAT_ENTRY_PER_BLOCK: usize = F2FS_BLKSIZE / core::mem::size_of::<f2fs_nat_entry>();
#[repr(C, packed)] pub struct f2fs_nat_block { pub entries: [f2fs_nat_entry; NAT_ENTRY_PER_BLOCK] }
pub const SIT_ENTRY_PER_BLOCK: usize = F2FS_BLKSIZE / core::mem::size_of::<f2fs_sit_entry>();
pub const F2FS_MAX_SEGMENT: usize = (16 * 1024 * 1024) / 2;
#[repr(C, packed)] pub struct f2fs_sit_entry { pub vblocks: __le16, pub valid_map: [__u8; SIT_VBLOCK_MAP_SIZE], pub mtime: __le64 }
#[repr(C, packed)] pub struct f2fs_sit_block { pub entries: [f2fs_sit_entry; SIT_ENTRY_PER_BLOCK] }
pub const SUMMARY_SIZE: usize = 7; pub const SUM_FOOTER_SIZE: usize = 5;
#[repr(C, packed)] pub struct f2fs_summary { pub nid: __le32, pub reserved: [__u8; 3] }
#[repr(C, packed)] pub struct summary_footer { pub entry_type: u8, pub check_sum: __le32 }
#[repr(C, packed)] pub struct nat_journal_entry { pub nid: __le32, pub ne: f2fs_nat_entry }
#[repr(C, packed)] pub struct nat_journal { pub entries: [nat_journal_entry; 0] }
#[repr(C, packed)] pub struct sit_journal_entry { pub segno: __le32, pub se: f2fs_sit_entry }
#[repr(C, packed)] pub struct sit_journal { pub entries: [sit_journal_entry; 0] }
#[repr(C, packed)] pub struct f2fs_extra_info { pub kbytes_written: __le64, pub reserved: [u8; 0] }
#[repr(C)] pub union f2fs_journal_body { pub nat_j: nat_journal, pub sit_j: sit_journal, pub info: f2fs_extra_info }
#[repr(C, packed)] pub struct f2fs_journal { pub n_nats: __le16, pub body: f2fs_journal_body }
#[repr(C, packed)] pub struct f2fs_summary_block { pub entries: [f2fs_summary; 0] }

pub const F2FS_DOT_HASH: u64 = 0; pub const F2FS_DDOT_HASH: u64 = F2FS_DOT_HASH; pub const F2FS_MAX_HASH: u64 = !(0x3u64 << 62); pub const F2FS_HASH_COL_BIT: u64 = 1u64 << 63;
pub type f2fs_hash_t = __le32; pub const F2FS_SLOT_LEN: usize = 8; pub const F2FS_SLOT_LEN_BITS: usize = 3; pub const MAX_DIR_HASH_DEPTH: usize = 63;
pub const SIZE_OF_DIR_ENTRY: usize = 11; pub const MIN_INLINE_DENTRY_SIZE: usize = 40; pub const F2FS_DEF_PROJID: usize = 0;
#[repr(C, packed)] pub struct f2fs_dir_entry { pub hash_code: __le32, pub ino: __le32, pub name_len: __le16, pub file_type: __u8 }
// The following directory-size constants preserve the source formulas and depend on kernel page constants.
pub const NR_DENTRY_IN_BLOCK: usize = (BITS_PER_BYTE * F2FS_BLKSIZE) / ((SIZE_OF_DIR_ENTRY + F2FS_SLOT_LEN) * BITS_PER_BYTE + 1);
pub const SIZE_OF_DENTRY_BITMAP: usize = (NR_DENTRY_IN_BLOCK + BITS_PER_BYTE - 1) / BITS_PER_BYTE;
pub const SIZE_OF_RESERVED: usize = F2FS_BLKSIZE - ((SIZE_OF_DIR_ENTRY + F2FS_SLOT_LEN) * NR_DENTRY_IN_BLOCK + SIZE_OF_DENTRY_BITMAP);
#[repr(C, packed)] pub struct f2fs_dentry_block { pub dentry_bitmap: [__u8; SIZE_OF_DENTRY_BITMAP], pub reserved: [__u8; SIZE_OF_RESERVED], pub dentry: [f2fs_dir_entry; NR_DENTRY_IN_BLOCK], pub filename: [[__u8; F2FS_SLOT_LEN]; NR_DENTRY_IN_BLOCK] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
