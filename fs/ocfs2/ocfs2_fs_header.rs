#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Rust translation of ocfs2_fs.h. External kernel types/constants are supplied
 * by the surrounding translation unit. */
pub type __u8 = u8; pub type __u16 = u16; pub type __u32 = u32; pub type __u64 = u64;
pub type __le16 = u16; pub type __le32 = u32; pub type __le64 = u64;

pub const OCFS2_MAJOR_REV_LEVEL:u32=0; pub const OCFS2_MINOR_REV_LEVEL:u32=90;
pub const OCFS2_SUPER_BLOCK_BLKNO:u32=2; pub const OCFS2_MIN_CLUSTERSIZE:u32=4096;
pub const OCFS2_MAX_CLUSTERSIZE:u32=1048576; pub const OCFS2_MIN_BLOCKSIZE:u32=512;
pub const OCFS2_MAX_BLOCKSIZE:u32=OCFS2_MIN_CLUSTERSIZE;
pub const OCFS2_SUPER_BLOCK_SIGNATURE:&[u8]=b"OCFSV2"; pub const OCFS2_INODE_SIGNATURE:&[u8]=b"INODE01";
pub const OCFS2_EXTENT_BLOCK_SIGNATURE:&[u8]=b"EXBLK01"; pub const OCFS2_GROUP_DESC_SIGNATURE:&[u8]=b"GROUP01";
pub const OCFS2_XATTR_BLOCK_SIGNATURE:&[u8]=b"XATTR01"; pub const OCFS2_DIR_TRAILER_SIGNATURE:&[u8]=b"DIRTRL1";
pub const OCFS2_DX_ROOT_SIGNATURE:&[u8]=b"DXDIR01"; pub const OCFS2_DX_LEAF_SIGNATURE:&[u8]=b"DXLEAF1";
pub const OCFS2_REFCOUNT_BLOCK_SIGNATURE:&[u8]=b"REFCNT1";

pub const OCFS2_FEATURE_INCOMPAT_HEARTBEAT_DEV:u32=0x0002; pub const OCFS2_FEATURE_INCOMPAT_RESIZE_INPROG:u32=4;
pub const OCFS2_FEATURE_INCOMPAT_LOCAL_MOUNT:u32=8; pub const OCFS2_FEATURE_INCOMPAT_SPARSE_ALLOC:u32=0x10;
pub const OCFS2_FEATURE_INCOMPAT_TUNEFS_INPROG:u32=0x20; pub const OCFS2_FEATURE_INCOMPAT_INLINE_DATA:u32=0x40;
pub const OCFS2_FEATURE_INCOMPAT_USERSPACE_STACK:u32=0x80; pub const OCFS2_FEATURE_INCOMPAT_EXTENDED_SLOT_MAP:u32=0x100;
pub const OCFS2_FEATURE_INCOMPAT_XATTR:u32=0x200; pub const OCFS2_FEATURE_INCOMPAT_INDEXED_DIRS:u32=0x400;
pub const OCFS2_FEATURE_INCOMPAT_META_ECC:u32=0x800; pub const OCFS2_FEATURE_INCOMPAT_REFCOUNT_TREE:u32=0x1000;
pub const OCFS2_FEATURE_INCOMPAT_DISCONTIG_BG:u32=0x2000; pub const OCFS2_FEATURE_INCOMPAT_CLUSTERINFO:u32=0x4000;
pub const OCFS2_FEATURE_INCOMPAT_APPEND_DIO:u32=0x8000; pub const OCFS2_FEATURE_COMPAT_BACKUP_SB:u32=1;
pub const OCFS2_FEATURE_COMPAT_JBD2_SB:u32=2; pub const OCFS2_FEATURE_RO_COMPAT_UNWRITTEN:u32=1;
pub const OCFS2_FEATURE_RO_COMPAT_USRQUOTA:u32=2; pub const OCFS2_FEATURE_RO_COMPAT_GRPQUOTA:u32=4;
pub const OCFS2_FEATURE_COMPAT_SUPP:u32=OCFS2_FEATURE_COMPAT_BACKUP_SB|OCFS2_FEATURE_COMPAT_JBD2_SB;
pub const OCFS2_FEATURE_INCOMPAT_SUPP:u32=OCFS2_FEATURE_INCOMPAT_LOCAL_MOUNT|OCFS2_FEATURE_INCOMPAT_SPARSE_ALLOC|OCFS2_FEATURE_INCOMPAT_INLINE_DATA|OCFS2_FEATURE_INCOMPAT_EXTENDED_SLOT_MAP|OCFS2_FEATURE_INCOMPAT_USERSPACE_STACK|OCFS2_FEATURE_INCOMPAT_XATTR|OCFS2_FEATURE_INCOMPAT_META_ECC|OCFS2_FEATURE_INCOMPAT_INDEXED_DIRS|OCFS2_FEATURE_INCOMPAT_REFCOUNT_TREE|OCFS2_FEATURE_INCOMPAT_DISCONTIG_BG|OCFS2_FEATURE_INCOMPAT_CLUSTERINFO|OCFS2_FEATURE_INCOMPAT_APPEND_DIO;
pub const OCFS2_FEATURE_RO_COMPAT_SUPP:u32=OCFS2_FEATURE_RO_COMPAT_UNWRITTEN|OCFS2_FEATURE_RO_COMPAT_USRQUOTA|OCFS2_FEATURE_RO_COMPAT_GRPQUOTA;
pub const OCFS2_BACKUP_SB_START:u64=1<<30; pub const OCFS2_MAX_BACKUP_SUPERBLOCKS:usize=6;
pub const OCFS2_VALID_FL:u32=1; pub const OCFS2_ORPHANED_FL:u32=4; pub const OCFS2_SYSTEM_FL:u32=0x10;
pub const OCFS2_SUPER_BLOCK_FL:u32=0x20; pub const OCFS2_LOCAL_ALLOC_FL:u32=0x40; pub const OCFS2_BITMAP_FL:u32=0x80;
pub const OCFS2_JOURNAL_FL:u32=0x100; pub const OCFS2_HEARTBEAT_FL:u32=0x200; pub const OCFS2_CHAIN_FL:u32=0x400;
pub const OCFS2_DEALLOC_FL:u32=0x800; pub const OCFS2_QUOTA_FL:u32=0x1000; pub const OCFS2_DIO_ORPHANED_FL:u32=0x2000;
pub const OCFS2_INLINE_DATA_FL:u16=1; pub const OCFS2_HAS_XATTR_FL:u16=2; pub const OCFS2_INLINE_XATTR_FL:u16=4;
pub const OCFS2_INDEXED_DIR_FL:u16=8; pub const OCFS2_HAS_REFCOUNT_FL:u16=0x10;
pub const OCFS2_EXT_UNWRITTEN:u8=1; pub const OCFS2_EXT_REFCOUNTED:u8=2; pub const OCFS2_JOURNAL_DIRTY_FL:u32=1;
pub const OCFS2_ERROR_FS:u16=1; pub const OCFS2_MAX_FILENAME_LEN:usize=255; pub const OCFS2_MAX_SLOTS:u16=255;
pub const OCFS2_INVALID_SLOT:u16=0xffff; pub const OCFS2_VOL_UUID_LEN:usize=16; pub const OCFS2_MAX_VOL_LABEL_LEN:usize=64;
pub const OCFS2_STACK_LABEL_LEN:usize=4; pub const OCFS2_CLUSTER_NAME_LEN:usize=16; pub const OCFS2_CLASSIC_CLUSTER_STACK:&[u8]=b"o2cb";
pub const OCFS2_MIN_JOURNAL_SIZE:u32=4*1024*1024; pub const OCFS2_MIN_XATTR_INLINE_SIZE:u16=256; pub const OCFS2_CLUSTER_O2CB_GLOBAL_HEARTBEAT:u8=1;

#[repr(C)] pub struct ocfs2_system_inode_info { pub si_name:*const u8,pub si_iflags:i32,pub si_mode:i32 }
pub const BAD_BLOCK_SYSTEM_INODE:usize=0; pub const GLOBAL_INODE_ALLOC_SYSTEM_INODE:usize=1; pub const OCFS2_FIRST_ONLINE_SYSTEM_INODE:usize=1;
pub const SLOT_MAP_SYSTEM_INODE:usize=2; pub const HEARTBEAT_SYSTEM_INODE:usize=3; pub const GLOBAL_BITMAP_SYSTEM_INODE:usize=4; pub const USER_QUOTA_SYSTEM_INODE:usize=5; pub const GROUP_QUOTA_SYSTEM_INODE:usize=6; pub const OCFS2_LAST_GLOBAL_SYSTEM_INODE:usize=6; pub const OCFS2_FIRST_LOCAL_SYSTEM_INODE:usize=7; pub const ORPHAN_DIR_SYSTEM_INODE:usize=7; pub const EXTENT_ALLOC_SYSTEM_INODE:usize=8; pub const INODE_ALLOC_SYSTEM_INODE:usize=9; pub const JOURNAL_SYSTEM_INODE:usize=10; pub const LOCAL_ALLOC_SYSTEM_INODE:usize=11; pub const TRUNCATE_LOG_SYSTEM_INODE:usize=12; pub const LOCAL_USER_QUOTA_SYSTEM_INODE:usize=13; pub const LOCAL_GROUP_QUOTA_SYSTEM_INODE:usize=14; pub const OCFS2_LAST_LOCAL_SYSTEM_INODE:usize=14; pub const NUM_SYSTEM_INODES:usize=15;
pub const NUM_GLOBAL_SYSTEM_INODES:usize=7; pub const NUM_LOCAL_SYSTEM_INODES:usize=8;

#[repr(C)] pub struct ocfs2_block_check { pub bc_crc32e:__le32,pub bc_ecc:__le16,pub bc_reserved1:__le16 }
#[repr(C)] pub union ocfs2_extent_rec_u { pub e_int_clusters:__le32,pub leaf:ocfs2_extent_rec_leaf }
#[repr(C)] pub struct ocfs2_extent_rec_leaf { pub e_leaf_clusters:__le16,pub e_reserved1:__u8,pub e_flags:__u8 }
#[repr(C)] pub struct ocfs2_extent_rec { pub e_cpos:__le32,pub u:ocfs2_extent_rec_u,pub e_blkno:__le64 }
#[repr(C)] pub struct ocfs2_chain_rec {pub c_free:__le32,pub c_total:__le32,pub c_blkno:__le64}
#[repr(C)] pub struct ocfs2_truncate_rec {pub t_start:__le32,pub t_clusters:__le32}
#[repr(C)] pub struct ocfs2_extent_list {pub l_tree_depth:__le16,pub l_count:__le16,pub l_next_free_rec:__le16,pub l_reserved1:__le16,pub l_reserved2:__le64,pub l_recs:[ocfs2_extent_rec;0]}
#[repr(C)] pub struct ocfs2_chain_list {pub cl_cpg:__le16,pub cl_bpc:__le16,pub cl_count:__le16,pub cl_next_free_rec:__le16,pub cl_reserved1:__le64,pub cl_recs:[ocfs2_chain_rec;0]}
#[repr(C)] pub struct ocfs2_truncate_log {pub tl_count:__le16,pub tl_used:__le16,pub tl_reserved1:__le32,pub tl_recs:[ocfs2_truncate_rec;0]}
#[repr(C)] pub struct ocfs2_extent_block {pub h_signature:[u8;8],pub h_check:ocfs2_block_check,pub h_suballoc_slot:__le16,pub h_suballoc_bit:__le16,pub h_fs_generation:__le32,pub h_blkno:__le64,pub h_suballoc_loc:__le64,pub h_next_leaf_blk:__le64,pub h_list:ocfs2_extent_list}
#[repr(C)] pub struct ocfs2_slot_map {pub sm_slots:[__le16;0]}
#[repr(C)] pub struct ocfs2_extended_slot {pub es_valid:u8,pub es_reserved1:[u8;3],pub es_node_num:__le32}
#[repr(C)] pub struct ocfs2_slot_map_extended {pub se_slots:[ocfs2_extended_slot;0]}
#[repr(C)] pub union ocfs2_cluster_info_u {pub ci_reserved:__le32,pub ci_fields:ocfs2_cluster_info_fields}
#[repr(C)] pub struct ocfs2_cluster_info_fields {pub ci_stackflags:u8,pub ci_reserved1:u8,pub ci_reserved2:u8,pub ci_reserved3:u8}
#[repr(C)] pub struct ocfs2_cluster_info {pub ci_stack:[u8;4],pub u:ocfs2_cluster_info_u,pub ci_cluster:[u8;16]}

/* The remaining on-disk records retain C layout and flexible-array tails. */
#[repr(C)] pub struct ocfs2_super_block {pub s_major_rev_level:__le16,pub s_minor_rev_level:__le16,pub s_mnt_count:__le16,pub s_max_mnt_count:__le16,pub s_state:__le16,pub s_errors:__le16,pub s_checkinterval:__le32,pub s_lastcheck:__le64,pub s_creator_os:__le32,pub s_feature_compat:__le32,pub s_feature_incompat:__le32,pub s_feature_ro_compat:__le32,pub s_root_blkno:__le64,pub s_system_dir_blkno:__le64,pub s_blocksize_bits:__le32,pub s_clustersize_bits:__le32,pub s_max_slots:__le16,pub s_tunefs_flag:__le16,pub s_uuid_hash:__le32,pub s_first_cluster_group:__le64,pub s_label:[u8;64],pub s_uuid:[u8;16],pub s_cluster_info:ocfs2_cluster_info,pub s_xattr_inline_size:__le16,pub s_reserved0:__le16,pub s_dx_seed:[__le32;3],pub s_reserved2:[__le64;15]}
#[repr(C)] pub struct ocfs2_local_alloc {pub la_bm_off:__le32,pub la_size:__le16,pub la_reserved1:__le16,pub la_reserved2:__le64,pub la_bitmap:[u8;0]}
#[repr(C)] pub struct ocfs2_inline_data {pub id_count:__le16,pub id_reserved0:__le16,pub id_reserved1:__le32,pub id_data:[u8;0]}
#[repr(C)] pub union ocfs2_inode_id1 {pub i_pad1:__le64,pub i_rdev:__le64,pub bitmap1:[__le32;2],pub journal1:[__le32;2]}
#[repr(C)] pub union ocfs2_inode_id2 {pub i_super:ocfs2_super_block,pub i_lab:ocfs2_local_alloc,pub i_chain:ocfs2_chain_list,pub i_list:ocfs2_extent_list,pub i_dealloc:ocfs2_truncate_log,pub i_data:ocfs2_inline_data,pub i_symlink:[u8;0]}
#[repr(C)] pub struct ocfs2_dinode {pub i_signature:[u8;8],pub i_generation:__le32,pub i_suballoc_slot:__le16,pub i_suballoc_bit:__le16,pub i_links_count_hi:__le16,pub i_xattr_inline_size:__le16,pub i_clusters:__le32,pub i_uid:__le32,pub i_gid:__le32,pub i_size:__le64,pub i_mode:__le16,pub i_links_count:__le16,pub i_flags:__le32,pub i_atime:__le64,pub i_ctime:__le64,pub i_mtime:__le64,pub i_dtime:__le64,pub i_blkno:__le64,pub i_last_eb_blk:__le64,pub i_fs_generation:__le32,pub i_atime_nsec:__le32,pub i_ctime_nsec:__le32,pub i_mtime_nsec:__le32,pub i_attr:__le32,pub i_orphaned_slot:__le16,pub i_dyn_features:__le16,pub i_xattr_loc:__le64,pub i_check:ocfs2_block_check,pub i_dx_root:__le64,pub i_refcount_loc:__le64,pub i_suballoc_loc:__le64,pub i_dio_orphaned_slot:__le16,pub i_reserved1:[__le16;3],pub i_reserved2:[__le64;2],pub id1:ocfs2_inode_id1,pub id2:ocfs2_inode_id2}
#[repr(C,packed)] pub struct ocfs2_dir_entry {pub inode:__le64,pub rec_len:__le16,pub name_len:u8,pub file_type:u8,pub name:[u8;255]}
#[repr(C)] pub struct ocfs2_dir_block_trailer {pub db_compat_inode:__le64,pub db_compat_rec_len:__le16,pub db_compat_name_len:u8,pub db_reserved0:u8,pub db_reserved1:__le16,pub db_free_rec_len:__le16,pub db_signature:[u8;8],pub db_reserved2:__le64,pub db_free_next:__le64,pub db_blkno:__le64,pub db_parent_dinode:__le64,pub db_check:ocfs2_block_check}
#[repr(C)] pub struct ocfs2_dx_entry {pub dx_major_hash:__le32,pub dx_minor_hash:__le32,pub dx_dirent_blk:__le64}
#[repr(C)] pub struct ocfs2_dx_entry_list {pub de_reserved:__le32,pub de_count:__le16,pub de_num_used:__le16,pub de_entries:[ocfs2_dx_entry;0]}
pub const OCFS2_DX_FLAG_INLINE:u8=1;
#[repr(C)] pub struct ocfs2_dx_root_block {pub dr_signature:[u8;8],pub dr_check:ocfs2_block_check,pub dr_suballoc_slot:__le16,pub dr_suballoc_bit:__le16,pub dr_fs_generation:__le32,pub dr_blkno:__le64,pub dr_last_eb_blk:__le64,pub dr_clusters:__le32,pub dr_flags:u8,pub dr_reserved0:u8,pub dr_reserved1:__le16,pub dr_dir_blkno:__le64,pub dr_num_entries:__le32,pub dr_reserved2:__le32,pub dr_free_blk:__le64,pub dr_suballoc_loc:__le64,pub dr_reserved3:[__le64;14],pub dr_list:ocfs2_extent_list}
#[repr(C)] pub struct ocfs2_dx_leaf {pub dl_signature:[u8;8],pub dl_check:ocfs2_block_check,pub dl_blkno:__le64,pub dl_fs_generation:__le32,pub dl_reserved0:__le32,pub dl_reserved1:__le64,pub dl_list:ocfs2_dx_entry_list}
pub const OCFS2_MAX_BG_BITMAP_SIZE:usize=256;
#[repr(C)] pub struct ocfs2_group_desc {pub bg_signature:[u8;8],pub bg_size:__le16,pub bg_bits:__le16,pub bg_free_bits_count:__le16,pub bg_chain:__le16,pub bg_generation:__le32,pub bg_contig_free_bits:__le16,pub bg_reserved1:__le16,pub bg_next_group:__le64,pub bg_parent_dinode:__le64,pub bg_blkno:__le64,pub bg_check:ocfs2_block_check,pub bg_reserved2:__le64,pub bg_bitmap:[u8;256],pub bg_list:ocfs2_extent_list}
#[repr(C)] pub struct ocfs2_refcount_rec {pub r_cpos:__le64,pub r_clusters:__le32,pub r_refcount:__le32}
#[repr(C)] pub struct ocfs2_refcount_list {pub rl_count:__le16,pub rl_used:__le16,pub rl_reserved2:__le32,pub rl_reserved1:__le64,pub rl_recs:[ocfs2_refcount_rec;0]}
#[repr(C)] pub struct ocfs2_refcount_block {pub rf_signature:[u8;8],pub rf_suballoc_slot:__le16,pub rf_suballoc_bit:__le16,pub rf_fs_generation:__le32,pub rf_blkno:__le64,pub rf_parent:__le64,pub rf_check:ocfs2_block_check,pub rf_last_eb_blk:__le64,pub rf_count:__le32,pub rf_flags:__le32,pub rf_clusters:__le32,pub rf_cpos:__le32,pub rf_generation:__le32,pub rf_reserved0:__le32,pub rf_suballoc_loc:__le64,pub rf_reserved1:[__le64;6],pub rf_records:ocfs2_refcount_list}
pub const OCFS2_32BIT_POS_MASK:u64=0xffff_ffff; pub const OCFS2_REFCOUNT_LEAF_FL:u32=1; pub const OCFS2_REFCOUNT_TREE_FL:u32=2;
#[repr(C)] pub struct ocfs2_xattr_entry {pub xe_name_hash:__le32,pub xe_name_offset:__le16,pub xe_name_len:u8,pub xe_type:u8,pub xe_value_size:__le64}
#[repr(C)] pub struct ocfs2_xattr_header {pub xh_count:__le16,pub xh_free_start:__le16,pub xh_name_value_len:__le16,pub xh_num_buckets:__le16,pub xh_check:ocfs2_block_check,pub xh_entries:[ocfs2_xattr_entry;0]}
#[repr(C)] pub struct ocfs2_xattr_value_root {pub xr_clusters:__le32,pub xr_reserved0:__le32,pub xr_last_eb_blk:__le64,pub xr_list:ocfs2_extent_list}
#[repr(C)] pub struct ocfs2_xattr_tree_root {pub xt_clusters:__le32,pub xt_reserved0:__le32,pub xt_last_eb_blk:__le64,pub xt_list:ocfs2_extent_list}
pub const OCFS2_XATTR_INDEXED:u32=1; pub const OCFS2_HASH_SHIFT:u32=5; pub const OCFS2_XATTR_ROUND:u32=3; pub const OCFS2_XATTR_BUCKET_SIZE:u32=4096; pub const OCFS2_XATTR_MAX_BLOCKS_PER_BUCKET:u32=8;
#[repr(C)] pub struct ocfs2_xattr_block {pub xb_signature:[u8;8],pub xb_suballoc_slot:__le16,pub xb_suballoc_bit:__le16,pub xb_fs_generation:__le32,pub xb_blkno:__le64,pub xb_check:ocfs2_block_check,pub xb_flags:__le16,pub xb_reserved0:__le16,pub xb_reserved1:__le32,pub xb_suballoc_loc:__le64,pub xb_attrs:ocfs2_xattr_tree_root}
pub const OCFS2_XATTR_ENTRY_LOCAL:u8=0x80; pub const OCFS2_XATTR_TYPE_MASK:u8=0x7f;
pub unsafe fn ocfs2_xattr_set_local(xe:*mut ocfs2_xattr_entry,local:i32){if local!=0{(*xe).xe_type|=OCFS2_XATTR_ENTRY_LOCAL}else{(*xe).xe_type&=!OCFS2_XATTR_ENTRY_LOCAL}}
pub unsafe fn ocfs2_xattr_is_local(xe:*mut ocfs2_xattr_entry)->i32{((*xe).xe_type&OCFS2_XATTR_ENTRY_LOCAL) as i32}
pub unsafe fn ocfs2_xattr_set_type(xe:*mut ocfs2_xattr_entry,t:i32){(*xe).xe_type|=(t as u8)&OCFS2_XATTR_TYPE_MASK}
pub unsafe fn ocfs2_xattr_get_type(xe:*mut ocfs2_xattr_entry)->i32{((*xe).xe_type&OCFS2_XATTR_TYPE_MASK) as i32}
#[repr(C)] pub struct ocfs2_disk_dqheader {pub dqh_magic:__le32,pub dqh_version:__le32}
#[repr(C)] pub struct ocfs2_global_disk_dqinfo {pub dqi_bgrace:__le32,pub dqi_igrace:__le32,pub dqi_syncms:__le32,pub dqi_blocks:__le32,pub dqi_free_blk:__le32,pub dqi_free_entry:__le32}
#[repr(C)] pub struct ocfs2_global_disk_dqblk {pub dqb_id:__le32,pub dqb_use_count:__le32,pub dqb_ihardlimit:__le64,pub dqb_isoftlimit:__le64,pub dqb_curinodes:__le64,pub dqb_bhardlimit:__le64,pub dqb_bsoftlimit:__le64,pub dqb_curspace:__le64,pub dqb_btime:__le64,pub dqb_itime:__le64,pub dqb_pad1:__le64,pub dqb_pad2:__le64}
#[repr(C)] pub struct ocfs2_local_disk_dqinfo {pub dqi_flags:__le32,pub dqi_chunks:__le32,pub dqi_blocks:__le32}
#[repr(C)] pub struct ocfs2_local_disk_chunk {pub dqc_free:__le32,pub dqc_bitmap:[u8;0]}
#[repr(C)] pub struct ocfs2_local_disk_dqblk {pub dqb_id:__le64,pub dqb_spacemod:__le64,pub dqb_inodemod:__le64}
#[repr(C)] pub struct ocfs2_disk_dqtrailer {pub dq_check:ocfs2_block_check}
pub const OCFS2_QBLK_RESERVED_SPACE:usize=8;
pub const OCFS2_TUNEFS_INPROG_REMOVE_SLOT:u16=1;
pub const OCFS2_HB_NONE:&[u8]=b"heartbeat=none"; pub const OCFS2_HB_LOCAL:&[u8]=b"heartbeat=local"; pub const OCFS2_HB_GLOBAL:&[u8]=b"heartbeat=global";
pub const OCFS2_DIR_PAD:usize=4; pub const OCFS2_DIR_ROUND:usize=3; pub const OCFS2_LINK_MAX:u32=32000; pub const OCFS2_DX_LINK_MAX:u32=0x7fff_ffff; pub const OCFS2_LINKS_HI_SHIFT:u32=16; pub const OCFS2_DX_ENTRIES_MAX:u32=0xffff_ffff;
pub const OCFS2_XATTR_ENTRY_LOCAL_MASK:u8=0x80; pub const OLQF_CLEAN:u32=1;
pub const OCFS2_GLOBAL_QMAGICS:[u32;2]=[0x0cf52470,0x0cf52471]; pub const OCFS2_GLOBAL_QVERSIONS:[u32;2]=[0,0];
pub const OCFS2_LOCAL_QMAGICS:[u32;2]=[0x0cf524c0,0x0cf524c1]; pub const OCFS2_LOCAL_QVERSIONS:[u32;2]=[0,0];
pub const OCFS2_GLOBAL_INFO_OFF:usize=core::mem::size_of::<ocfs2_disk_dqheader>(); pub const OCFS2_LOCAL_INFO_OFF:usize=core::mem::size_of::<ocfs2_disk_dqheader>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
