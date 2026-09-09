/* SPDX-License-Identifier: GPL-2.0-or-later */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/* Direct Rust translation of linux/jbd2.h.  Kernel-provided types and
 * functions remain external dependencies, as in the original header. */
pub const journal_oom_retry: i32 = 1;
pub const JBD2_DEFAULT_MAX_COMMIT_AGE: u32 = 5;
pub const JBD2_MIN_JOURNAL_BLOCKS: u32 = 1024;
pub const JBD2_DEFAULT_FAST_COMMIT_BLOCKS: u32 = 256;
pub const JBD2_MAGIC_NUMBER: u32 = 0xc03b3998;
pub const JBD2_DESCRIPTOR_BLOCK: u32 = 1;
pub const JBD2_COMMIT_BLOCK: u32 = 2;
pub const JBD2_SUPERBLOCK_V1: u32 = 3;
pub const JBD2_SUPERBLOCK_V2: u32 = 4;
pub const JBD2_REVOKE_BLOCK: u32 = 5;
pub const JBD2_CRC32_CHKSUM: u32 = 1;
pub const JBD2_MD5_CHKSUM: u32 = 2;
pub const JBD2_SHA1_CHKSUM: u32 = 3;
pub const JBD2_CRC32C_CHKSUM: u32 = 4;
pub const JBD2_CRC32_CHKSUM_SIZE: u32 = 4;
pub const JBD2_CHECKSUM_BYTES: usize = 8;

pub type __be16 = u16; pub type __be32 = u32; pub type __be64 = u64;
pub type __u8 = u8; pub type __u32 = u32; pub type tid_t = u32;
pub type loff_t = i64; pub type sector_t = u64; pub type pgoff_t = usize;
pub type u32_alias = u32;
pub enum jbd2_journal_handle {}
pub type handle_t = jbd2_journal_handle;
pub enum journal_s {}
pub type journal_t = journal_s;
pub enum transaction_s {}
pub type transaction_t = transaction_s;

#[repr(C)] #[derive(Copy, Clone)]
pub struct journal_header_t { pub h_magic: __be32, pub h_blocktype: __be32, pub h_sequence: __be32 }
#[repr(C)] pub struct commit_header { pub h_magic:__be32, pub h_blocktype:__be32, pub h_sequence:__be32, pub h_chksum_type:u8, pub h_chksum_size:u8, pub h_padding:[u8;2], pub h_chksum:[__be32;8], pub h_commit_sec:__be64, pub h_commit_nsec:__be32 }
#[repr(C)] pub struct journal_block_tag3_t { pub t_blocknr:__be32, pub t_flags:__be32, pub t_blocknr_high:__be32, pub t_checksum:__be32 }
#[repr(C)] pub struct journal_block_tag_t { pub t_blocknr:__be32, pub t_checksum:u16, pub t_flags:u16, pub t_blocknr_high:__be32 }
#[repr(C)] pub struct jbd2_journal_block_tail { pub t_checksum:__be32 }
#[repr(C)] pub struct jbd2_journal_revoke_header_t { pub r_header:journal_header_t, pub r_count:__be32 }

pub const JBD2_FLAG_ESCAPE:u32=1; pub const JBD2_FLAG_SAME_UUID:u32=2; pub const JBD2_FLAG_DELETED:u32=4; pub const JBD2_FLAG_LAST_TAG:u32=8;
#[repr(C)] pub struct journal_superblock_t {
 pub s_header:journal_header_t, pub s_blocksize:__be32, pub s_maxlen:__be32, pub s_first:__be32,
 pub s_sequence:__be32, pub s_start:__be32, pub s_errno:__be32, pub s_feature_compat:__be32,
 pub s_feature_incompat:__be32, pub s_feature_ro_compat:__be32, pub s_uuid:[u8;16],
 pub s_nr_users:__be32, pub s_dynsuper:__be32, pub s_max_transaction:__be32, pub s_max_trans_data:__be32,
 pub s_checksum_type:u8, pub s_padding2:[u8;3], pub s_num_fc_blks:__be32, pub s_head:__be32,
 pub s_padding:[__u32;40], pub s_checksum:__be32, pub s_users:[u8;768]
}
pub const JBD2_FEATURE_COMPAT_CHECKSUM:u32=1;
pub const JBD2_FEATURE_INCOMPAT_REVOKE:u32=1; pub const JBD2_FEATURE_INCOMPAT_64BIT:u32=2; pub const JBD2_FEATURE_INCOMPAT_ASYNC_COMMIT:u32=4; pub const JBD2_FEATURE_INCOMPAT_CSUM_V2:u32=8; pub const JBD2_FEATURE_INCOMPAT_CSUM_V3:u32=16; pub const JBD2_FEATURE_INCOMPAT_FAST_COMMIT:u32=32;
pub const JBD2_KNOWN_COMPAT_FEATURES:u32=JBD2_FEATURE_COMPAT_CHECKSUM;
pub const JBD2_KNOWN_ROCOMPAT_FEATURES:u32=0;
pub const JBD2_KNOWN_INCOMPAT_FEATURES:u32=63;

#[repr(C)] pub struct jbd2_inode { pub i_transaction:*mut transaction_t, pub i_next_transaction:*mut transaction_t, pub i_list:[usize;2], pub i_vfs_inode:*mut core::ffi::c_void, pub i_flags:usize, pub i_dirty_start_page:pgoff_t, pub i_dirty_end_page:pgoff_t }
#[repr(C)] pub union jbd2_handle_union { pub h_transaction:*mut transaction_t, pub h_journal:*mut journal_t }
#[repr(C)] pub struct jbd2_journal_handle { pub h_union:jbd2_handle_union, pub h_rsv_handle:*mut handle_t, pub h_total_credits:i32, pub h_revoke_credits:i32, pub h_revoke_credits_requested:i32, pub h_ref:i32, pub h_err:i32, pub h_sync:u8, pub h_reserved:u8, pub h_aborted:u8, pub h_invalid:u8, pub h_type:u8, pub h_line_no:u16, pub h_start_jiffies:usize, pub h_requested_credits:u32, pub saved_alloc_context:u32 }
#[repr(C)] pub struct transaction_chp_stats_s { pub cs_chp_time:usize, pub cs_forced_to_close:u32, pub cs_written:u32, pub cs_dropped:u32 }
#[repr(C)] pub struct transaction_run_stats_s { pub rs_wait:usize,pub rs_request_delay:usize,pub rs_running:usize,pub rs_locked:usize,pub rs_flushing:usize,pub rs_logging:usize,pub rs_handle_count:u32,pub rs_blocks:u32,pub rs_blocks_logged:u32 }
#[repr(C)] pub struct transaction_stats_s { pub ts_tid:usize,pub ts_requested:usize,pub run:transaction_run_stats_s }

pub const JBD2_NR_BATCH:usize=64; pub const JBD2_FC_REPLAY_STOP:i32=0; pub const JBD2_FC_REPLAY_CONTINUE:i32=1;
pub const JBD2_UNMOUNT:usize=1; pub const JBD2_ABORT:usize=2; pub const JBD2_ACK_ERR:usize=4; pub const JBD2_FLUSHED:usize=8; pub const JBD2_LOADED:usize=16; pub const JBD2_BARRIER:usize=32; pub const JBD2_CYCLE_RECORD:usize=128; pub const JBD2_FAST_COMMIT_ONGOING:usize=256; pub const JBD2_FULL_COMMIT_ONGOING:usize=512;
pub const BJ_None:i32=0; pub const BJ_Metadata:i32=1; pub const BJ_Forget:i32=2; pub const BJ_Shadow:i32=3; pub const BJ_Reserved:i32=4; pub const BJ_Types:i32=5;
pub const JOURNAL_REVOKE_DEFAULT_HASH:i32=256;

/* External declarations from the kernel and the remainder of the public API. */
extern "C" {
 pub fn jbd2_journal_start(j:*mut journal_t,nblocks:i32)->*mut handle_t;
 pub fn jbd2_journal_stop(h:*mut handle_t)->i32;
 pub fn jbd2_journal_abort(j:*mut journal_t, errno:i32);
 pub fn jbd2_journal_errno(j:*mut journal_t)->i32;
 pub fn jbd2_journal_force_commit(j:*mut journal_t)->i32;
 pub fn jbd2_log_start_commit(j:*mut journal_t, tid:tid_t)->i32;
 pub fn jbd2_log_wait_commit(j:*mut journal_t, tid:tid_t)->i32;
 pub fn jbd2_journal_destroy(j:*mut journal_t)->i32;
 pub fn jbd2_journal_recover(j:*mut journal_t)->i32;
 pub fn jbd2_journal_flush(j:*mut journal_t, flags:u32)->i32;
}

#[inline] pub fn tid_gt(x:tid_t,y:tid_t)->bool { (x.wrapping_sub(y) as i32)>0 }
#[inline] pub fn tid_geq(x:tid_t,y:tid_t)->bool { (x.wrapping_sub(y) as i32)>=0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
