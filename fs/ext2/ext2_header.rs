/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from ext2.h. Included Linux declarations remain external dependencies. */

pub type ext2_grpblk_t = ::core::ffi::c_int;
pub type ext2_fsblk_t = ::core::ffi::c_ulong;
pub const E2FSBLK: &str = "%lu";

#[repr(C)] pub struct ext2_reserve_window { pub _rsv_start: ext2_fsblk_t, pub _rsv_end: ext2_fsblk_t }
#[repr(C)] pub struct ext2_reserve_window_node { pub rsv_node: rb_node, pub rsv_goal_size: u32, pub rsv_alloc_hit: u32, pub rsv_window: ext2_reserve_window }
#[repr(C)] pub struct ext2_block_alloc_info { pub rsv_window_node: ext2_reserve_window_node, pub last_alloc_logical_block: u32, pub last_alloc_physical_block: ext2_fsblk_t }

#[repr(C)] pub struct ext2_sb_info {
    pub s_inodes_per_block: ::core::ffi::c_ulong, pub s_blocks_per_group: ::core::ffi::c_ulong,
    pub s_inodes_per_group: ::core::ffi::c_ulong, pub s_itb_per_group: ::core::ffi::c_ulong,
    pub s_gdb_count: ::core::ffi::c_ulong, pub s_desc_per_block: ::core::ffi::c_ulong,
    pub s_groups_count: ::core::ffi::c_ulong, pub s_overhead_last: ::core::ffi::c_ulong,
    pub s_blocks_last: ::core::ffi::c_ulong, pub s_sbh: *mut buffer_head, pub s_es: *mut ext2_super_block,
    pub s_group_desc: *mut *mut buffer_head, pub s_mount_opt: ::core::ffi::c_ulong,
    pub s_sb_block: ::core::ffi::c_ulong, pub s_resuid: kuid_t, pub s_resgid: kgid_t,
    pub s_mount_state: u16, pub s_pad: u16, pub s_addr_per_block_bits: ::core::ffi::c_int,
    pub s_desc_per_block_bits: ::core::ffi::c_int, pub s_inode_size: ::core::ffi::c_int,
    pub s_first_ino: ::core::ffi::c_int, pub s_next_gen_lock: spinlock_t, pub s_next_generation: u32,
    pub s_dir_count: ::core::ffi::c_ulong, pub s_debts: *mut u8,
    pub s_freeblocks_counter: percpu_counter, pub s_freeinodes_counter: percpu_counter, pub s_dirs_counter: percpu_counter,
    pub s_blockgroup_lock: *mut blockgroup_lock, pub s_rsv_window_lock: spinlock_t,
    pub s_rsv_window_root: rb_root, pub s_rsv_window_head: ext2_reserve_window_node,
    pub s_lock: spinlock_t, pub s_ea_block_cache: *mut mb_cache,
}
#[repr(C)] pub struct ext2_group_desc { pub bg_block_bitmap: __le32, pub bg_inode_bitmap: __le32, pub bg_inode_table: __le32, pub bg_free_blocks_count: __le16, pub bg_free_inodes_count: __le16, pub bg_used_dirs_count: __le16, pub bg_pad: __le16, pub bg_reserved: [__le32;3] }

#[repr(C)] pub union ext2_inode_osd1 { pub linux1: ext2_inode_osd1_linux, pub hurd1: ext2_inode_osd1_hurd, pub masix1: ext2_inode_osd1_masix }
#[repr(C)] pub struct ext2_inode_osd1_linux { pub l_i_reserved1: __le32 }
#[repr(C)] pub struct ext2_inode_osd1_hurd { pub h_i_translator: __le32 }
#[repr(C)] pub struct ext2_inode_osd1_masix { pub m_i_reserved1: __le32 }
#[repr(C)] pub union ext2_inode_osd2 { pub linux2: ext2_inode_osd2_linux, pub hurd2: ext2_inode_osd2_hurd, pub masix2: ext2_inode_osd2_masix }
#[repr(C)] pub struct ext2_inode_osd2_linux { pub l_i_frag:u8, pub l_i_fsize:u8, pub i_pad1:__le16, pub l_i_uid_high:__le16, pub l_i_gid_high:__le16, pub l_i_reserved2:__u32 }
#[repr(C)] pub struct ext2_inode_osd2_hurd { pub h_i_frag:u8, pub h_i_fsize:u8, pub h_i_mode_high:__le16, pub h_i_uid_high:__le16, pub h_i_gid_high:__le16, pub h_i_author:__le32 }
#[repr(C)] pub struct ext2_inode_osd2_masix { pub m_i_frag:u8, pub m_i_fsize:u8, pub m_pad1:__u16, pub m_i_reserved2:[__u32;2] }
#[repr(C)] pub struct ext2_inode { pub i_mode:__le16,pub i_uid:__le16,pub i_size:__le32,pub i_atime:__le32,pub i_ctime:__le32,pub i_mtime:__le32,pub i_dtime:__le32,pub i_gid:__le16,pub i_links_count:__le16,pub i_blocks:__le32,pub i_flags:__le32,pub osd1:ext2_inode_osd1,pub i_block:[__le32;15],pub i_generation:__le32,pub i_file_acl:__le32,pub i_dir_acl:__le32,pub i_faddr:__le32,pub osd2:ext2_inode_osd2 }

#[repr(C)] pub struct ext2_super_block {
    pub s_inodes_count:__le32,pub s_blocks_count:__le32,pub s_r_blocks_count:__le32,pub s_free_blocks_count:__le32,pub s_free_inodes_count:__le32,pub s_first_data_block:__le32,pub s_log_block_size:__le32,pub s_log_frag_size:__le32,pub s_blocks_per_group:__le32,pub s_frags_per_group:__le32,pub s_inodes_per_group:__le32,pub s_mtime:__le32,pub s_wtime:__le32,pub s_mnt_count:__le16,pub s_max_mnt_count:__le16,pub s_magic:__le16,pub s_state:__le16,pub s_errors:__le16,pub s_minor_rev_level:__le16,pub s_lastcheck:__le32,pub s_checkinterval:__le32,pub s_creator_os:__le32,pub s_rev_level:__le32,pub s_def_resuid:__le16,pub s_def_resgid:__le16,pub s_first_ino:__le32,pub s_inode_size:__le16,pub s_block_group_nr:__le16,pub s_feature_compat:__le32,pub s_feature_incompat:__le32,pub s_feature_ro_compat:__le32,pub s_uuid:[u8;16],pub s_volume_name:[::core::ffi::c_char;16],pub s_last_mounted:[::core::ffi::c_char;64],pub s_algorithm_usage_bitmap:__le32,pub s_prealloc_blocks:u8,pub s_prealloc_dir_blocks:u8,pub s_padding1:__le16,pub s_journal_uuid:[u8;16],pub s_journal_inum:__u32,pub s_journal_dev:__u32,pub s_last_orphan:__u32,pub s_hash_seed:[__u32;4],pub s_def_hash_version:u8,pub s_reserved_char_pad:u8,pub s_reserved_word_pad:__u16,pub s_default_mount_opts:__le32,pub s_first_meta_bg:__le32,pub s_reserved:[__u32;190]
}

#[repr(C)] pub struct ext2_dir_entry { pub inode:__le32,pub rec_len:__le16,pub name_len:__le16,pub name:[::core::ffi::c_char;0] }
#[repr(C)] pub struct ext2_dir_entry_2 { pub inode:__le32,pub rec_len:__le16,pub name_len:u8,pub file_type:u8,pub name:[::core::ffi::c_char;0] }
#[repr(C)] pub struct ext2_mount_options { pub s_mount_opt: ::core::ffi::c_ulong,pub s_resuid:kuid_t,pub s_resgid:kgid_t }
#[repr(C)] pub struct ext2_inode_info { pub i_data:[__le32;15],pub i_flags:__u32,pub i_faddr:__u32,pub i_frag_no:u8,pub i_frag_size:u8,pub i_state:__u16,pub i_file_acl:__u32,pub i_dir_acl:__u32,pub i_dtime:__u32,pub i_block_group:__u32,pub i_block_alloc_info:*mut ext2_block_alloc_info,pub i_dir_start_lookup:__u32,pub i_meta_lock:rwlock_t,pub truncate_mutex:mutex,pub vfs_inode:inode,pub i_orphan:list_head,pub i_metadata_bhs:mapping_metadata_bhs }

pub const EXT2_DEFAULT_RESERVE_BLOCKS:u32=8; pub const EXT2_MAX_RESERVE_BLOCKS:u32=1027; pub const EXT2_RESERVE_WINDOW_NOT_ALLOCATED:u32=0;
pub const EXT2_MIN_BLOCK_SIZE:u32=1024; pub const EXT2_MAX_BLOCK_SIZE:u32=65536; pub const EXT2_MIN_BLOCK_LOG_SIZE:u32=10; pub const EXT2_MAX_BLOCK_LOG_SIZE:u32=16;
pub const EXT2_BAD_INO:u32=1; pub const EXT2_ROOT_INO:u32=2; pub const EXT2_BOOT_LOADER_INO:u32=5; pub const EXT2_UNDEL_DIR_INO:u32=6; pub const EXT2_GOOD_OLD_FIRST_INO:u32=11;
pub const EXT2_NDIR_BLOCKS:usize=12; pub const EXT2_IND_BLOCK:usize=12; pub const EXT2_DIND_BLOCK:usize=13; pub const EXT2_TIND_BLOCK:usize=14; pub const EXT2_N_BLOCKS:usize=15;
pub const EXT2_VALID_FS:u32=0x0001; pub const EXT2_ERROR_FS:u32=0x0002;
pub const EXT2_GOOD_OLD_REV:u32=0; pub const EXT2_DYNAMIC_REV:u32=1; pub const EXT2_CURRENT_REV:u32=0; pub const EXT2_MAX_SUPP_REV:u32=1; pub const EXT2_GOOD_OLD_INODE_SIZE:u32=128;
pub const EXT2_DIR_PAD:u32=4; pub const EXT2_DIR_ROUND:u32=3; pub const EXT2_MAX_REC_LEN:u32=(1<<16)-1;
pub const EXT2_OS_LINUX:u32=0; pub const EXT2_OS_HURD:u32=1; pub const EXT2_OS_MASIX:u32=2; pub const EXT2_OS_FREEBSD:u32=3; pub const EXT2_OS_LITES:u32=4;
pub const EXT2_DEF_RESUID:u32=0; pub const EXT2_DEF_RESGID:u32=0; pub const EXT2_STATE_NEW:u32=1;

macro_rules! ext2_dir_rec_len { ($name_len:expr) => { (($name_len + 8 + EXT2_DIR_ROUND) & !EXT2_DIR_ROUND) }; }
macro_rules! clear_opt { ($o:expr,$opt:expr) => { $o &= !$opt }; } macro_rules! set_opt { ($o:expr,$opt:expr) => { $o |= $opt }; }

extern "C" { pub fn bgl_lock_ptr(lock:*mut blockgroup_lock, group: ::core::ffi::c_uint) -> *mut spinlock_t; pub fn container_of<T,U>(ptr:*mut T, member: *const U) -> *mut ext2_inode_info; }
#[inline] pub unsafe fn sb_bgl_lock(sbi:*mut ext2_sb_info, block_group:u32)->*mut spinlock_t { bgl_lock_ptr((*sbi).s_blockgroup_lock,block_group) }
#[inline] pub unsafe fn ext2_mask_flags(mode:umode_t, flags:__u32)->__u32 { if S_ISDIR(mode) {flags} else if S_ISREG(mode) {flags & !(FS_DIRSYNC_FL|FS_TOPDIR_FL)} else {flags & (FS_NODUMP_FL|FS_NOATIME_FL)} }
#[inline] pub unsafe fn EXT2_SB(sb:*mut super_block)->*mut ext2_sb_info { (*sb).s_fs_info as *mut ext2_sb_info }
#[inline] pub unsafe fn ext2_group_first_block_no(sb:*mut super_block, group_no:usize)->ext2_fsblk_t { group_no as ext2_fsblk_t * (*EXT2_SB(sb)).s_blocks_per_group as ext2_fsblk_t + le32_to_cpu((*EXT2_SB(sb)).s_es.as_ref().unwrap().s_first_data_block) as ext2_fsblk_t }
#[inline] pub unsafe fn ext2_group_last_block_no(sb:*mut super_block, group_no:usize)->ext2_fsblk_t { let sbi=EXT2_SB(sb); if group_no == (*sbi).s_groups_count as usize-1 { le32_to_cpu((*sbi).s_es.as_ref().unwrap().s_blocks_count) as ext2_fsblk_t-1 } else { ext2_group_first_block_no(sb,group_no)+(*sbi).s_blocks_per_group as ext2_fsblk_t-1 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
