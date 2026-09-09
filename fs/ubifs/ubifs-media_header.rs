/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of ubifs-media.h. */

pub const UBIFS_NODE_MAGIC: u32 = 0x06101831;
pub const UBIFS_FORMAT_VERSION: u32 = 5;
pub const UBIFS_RO_COMPAT_VERSION: u32 = 0;
pub const UBIFS_MIN_LEB_SZ: usize = 15 * 1024;
pub const UBIFS_CRC32_INIT: u32 = 0xFFFF_FFFF;
pub const UBIFS_MIN_COMPR_LEN: usize = 128;
pub const UBIFS_MIN_COMPRESS_DIFF: usize = 64;
pub const UBIFS_ROOT_INO: u32 = 1;
pub const UBIFS_FIRST_INO: u32 = 64;
pub const UBIFS_MAX_NLEN: usize = 255;
pub const UBIFS_MAX_JHEADS: usize = 1;
pub const UBIFS_BLOCK_SIZE: usize = 4096;
pub const UBIFS_BLOCK_SHIFT: usize = 12;
pub const UBIFS_PADDING_BYTE: u8 = 0xCE;
pub const UBIFS_MAX_KEY_LEN: usize = 16;
pub const UBIFS_SK_LEN: usize = 8;
pub const UBIFS_MIN_FANOUT: usize = 3;
pub const UBIFS_MAX_LEVELS: usize = 512;
pub const UBIFS_MAX_INO_DATA: usize = UBIFS_BLOCK_SIZE;
pub const UBIFS_LPT_FANOUT: usize = 4;
pub const UBIFS_LPT_FANOUT_SHIFT: usize = 2;
pub const UBIFS_LPT_CRC_BITS: usize = 16;
pub const UBIFS_LPT_CRC_BYTES: usize = 2;
pub const UBIFS_LPT_TYPE_BITS: usize = 4;
/* UBIFS_KEY_OFFSET is offsetof(struct ubifs_ino_node, key). */
pub const UBIFS_KEY_OFFSET: usize = core::mem::size_of::<ubifs_ch>();
pub const UBIFS_GC_HEAD: usize = 0;
pub const UBIFS_BASE_HEAD: usize = 1;
pub const UBIFS_DATA_HEAD: usize = 2;

pub const UBIFS_LPT_PNODE: usize = 0;
pub const UBIFS_LPT_NNODE: usize = 1;
pub const UBIFS_LPT_LTAB: usize = 2;
pub const UBIFS_LPT_LSAVE: usize = 3;
pub const UBIFS_LPT_NODE_CNT: usize = 4;
pub const UBIFS_LPT_NOT_A_NODE: usize = (1 << UBIFS_LPT_TYPE_BITS) - 1;

pub const UBIFS_ITYPE_REG: usize = 0;
pub const UBIFS_ITYPE_DIR: usize = 1;
pub const UBIFS_ITYPE_LNK: usize = 2;
pub const UBIFS_ITYPE_BLK: usize = 3;
pub const UBIFS_ITYPE_CHR: usize = 4;
pub const UBIFS_ITYPE_FIFO: usize = 5;
pub const UBIFS_ITYPE_SOCK: usize = 6;
pub const UBIFS_ITYPES_CNT: usize = 7;
pub const UBIFS_KEY_HASH_R5: usize = 0;
pub const UBIFS_KEY_HASH_TEST: usize = 1;
pub const UBIFS_SIMPLE_KEY_FMT: usize = 0;
pub const UBIFS_S_KEY_BLOCK_BITS: usize = 29;
pub const UBIFS_S_KEY_BLOCK_MASK: u32 = 0x1FFFFFFF;
pub const UBIFS_S_KEY_HASH_BITS: usize = UBIFS_S_KEY_BLOCK_BITS;
pub const UBIFS_S_KEY_HASH_MASK: u32 = UBIFS_S_KEY_BLOCK_MASK;
pub const UBIFS_INO_KEY: usize = 0;
pub const UBIFS_DATA_KEY: usize = 1;
pub const UBIFS_DENT_KEY: usize = 2;
pub const UBIFS_XENT_KEY: usize = 3;
pub const UBIFS_KEY_TYPES_CNT: usize = 4;

pub const UBIFS_SB_LEBS: usize = 1;
pub const UBIFS_MST_LEBS: usize = 2;
pub const UBIFS_SB_LNUM: usize = 0;
pub const UBIFS_MST_LNUM: usize = UBIFS_SB_LNUM + UBIFS_SB_LEBS;
pub const UBIFS_LOG_LNUM: usize = UBIFS_MST_LNUM + UBIFS_MST_LEBS;
pub const UBIFS_MIN_LOG_LEBS: usize = 2;
pub const UBIFS_MIN_BUD_LEBS: usize = 3;
pub const UBIFS_MIN_JNL_LEBS: usize = UBIFS_MIN_LOG_LEBS + UBIFS_MIN_BUD_LEBS;
pub const UBIFS_MIN_LPT_LEBS: usize = 2;
pub const UBIFS_MIN_ORPH_LEBS: usize = 1;
pub const UBIFS_MIN_MAIN_LEBS: usize = UBIFS_MIN_BUD_LEBS + 6;
pub const UBIFS_MIN_LEB_CNT: usize = UBIFS_SB_LEBS + UBIFS_MST_LEBS + UBIFS_MIN_LOG_LEBS + UBIFS_MIN_LPT_LEBS + UBIFS_MIN_ORPH_LEBS + UBIFS_MIN_MAIN_LEBS;

pub const UBIFS_MAX_HASH_LEN: usize = 64;
pub const UBIFS_MAX_HMAC_LEN: usize = 64;
pub const UBIFS_XATTR_NAME_ENCRYPTION_CONTEXT: &[u8] = b"c\0";
pub const UBIFS_SIGNATURE_TYPE_PKCS7: u32 = 1;

pub const UBIFS_COMPR_FL: u32 = 0x01;
pub const UBIFS_SYNC_FL: u32 = 0x02;
pub const UBIFS_IMMUTABLE_FL: u32 = 0x04;
pub const UBIFS_APPEND_FL: u32 = 0x08;
pub const UBIFS_DIRSYNC_FL: u32 = 0x10;
pub const UBIFS_XATTR_FL: u32 = 0x20;
pub const UBIFS_CRYPT_FL: u32 = 0x40;
pub const UBIFS_FL_MASK: u32 = 0x0000001F;
pub const UBIFS_COMPR_NONE: u32 = 0;
pub const UBIFS_COMPR_LZO: u32 = 1;
pub const UBIFS_COMPR_ZLIB: u32 = 2;
pub const UBIFS_COMPR_ZSTD: u32 = 3;
pub const UBIFS_COMPR_TYPES_CNT: u32 = 4;
pub const UBIFS_INO_NODE: u32 = 0;
pub const UBIFS_DATA_NODE: u32 = 1;
pub const UBIFS_DENT_NODE: u32 = 2;
pub const UBIFS_XENT_NODE: u32 = 3;
pub const UBIFS_TRUN_NODE: u32 = 4;
pub const UBIFS_PAD_NODE: u32 = 5;
pub const UBIFS_SB_NODE: u32 = 6;
pub const UBIFS_MST_NODE: u32 = 7;
pub const UBIFS_REF_NODE: u32 = 8;
pub const UBIFS_IDX_NODE: u32 = 9;
pub const UBIFS_CS_NODE: u32 = 10;
pub const UBIFS_ORPH_NODE: u32 = 11;
pub const UBIFS_AUTH_NODE: u32 = 12;
pub const UBIFS_SIG_NODE: u32 = 13;
pub const UBIFS_NODE_TYPES_CNT: u32 = 14;
pub const UBIFS_MST_DIRTY: u32 = 1;
pub const UBIFS_MST_NO_ORPHS: u32 = 2;
pub const UBIFS_MST_RCVRY: u32 = 4;
pub const UBIFS_NO_NODE_GROUP: u32 = 0;
pub const UBIFS_IN_NODE_GROUP: u32 = 1;
pub const UBIFS_LAST_OF_NODE_GROUP: u32 = 2;
pub const UBIFS_FLG_BIGLPT: u32 = 0x02;
pub const UBIFS_FLG_SPACE_FIXUP: u32 = 0x04;
pub const UBIFS_FLG_DOUBLE_HASH: u32 = 0x08;
pub const UBIFS_FLG_ENCRYPTION: u32 = 0x10;
pub const UBIFS_FLG_AUTHENTICATION: u32 = 0x20;
pub const UBIFS_FLG_MASK: u32 = UBIFS_FLG_BIGLPT | UBIFS_FLG_SPACE_FIXUP | UBIFS_FLG_DOUBLE_HASH | UBIFS_FLG_ENCRYPTION | UBIFS_FLG_AUTHENTICATION;

#[repr(C, packed)]
pub struct ubifs_ch { pub magic: u32, pub crc: u32, pub sqnum: u64, pub len: u32, pub node_type: u8, pub group_type: u8, pub padding: [u8; 2] }
#[repr(C, packed)]
pub union ubifs_dev_desc { pub new: u32, pub huge: u64 }
#[repr(C, packed)]
pub struct ubifs_ino_node {
    pub ch: ubifs_ch, pub key: [u8; UBIFS_MAX_KEY_LEN], pub creat_sqnum: u64, pub size: u64,
    pub atime_sec: u64, pub ctime_sec: u64, pub mtime_sec: u64, pub atime_nsec: u32,
    pub ctime_nsec: u32, pub mtime_nsec: u32, pub nlink: u32, pub uid: u32, pub gid: u32,
    pub mode: u32, pub flags: u32, pub data_len: u32, pub xattr_cnt: u32, pub xattr_size: u32,
    pub padding1: [u8; 4], pub xattr_names: u32, pub compr_type: u16, pub padding2: [u8; 26], pub data: [u8; 0],
}
#[repr(C, packed)] pub struct ubifs_dent_node { pub ch: ubifs_ch, pub key: [u8; 16], pub inum: u64, pub padding1: u8, pub type_: u8, pub nlen: u16, pub cookie: u32, pub name: [u8; 0] }
#[repr(C, packed)] pub struct ubifs_data_node { pub ch: ubifs_ch, pub key: [u8; 16], pub size: u32, pub compr_type: u16, pub compr_size: u16, pub data: [u8; 0] }
#[repr(C, packed)] pub struct ubifs_trun_node { pub ch: ubifs_ch, pub inum: u32, pub padding: [u8; 12], pub old_size: u64, pub new_size: u64 }
#[repr(C, packed)] pub struct ubifs_pad_node { pub ch: ubifs_ch, pub pad_len: u32 }
#[repr(C, packed)] pub struct ubifs_sb_node { pub ch: ubifs_ch, pub padding: [u8; 2], pub key_hash: u8, pub key_fmt: u8, pub flags: u32, pub min_io_size: u32, pub leb_size: u32, pub leb_cnt: u32, pub max_leb_cnt: u32, pub max_bud_bytes: u64, pub log_lebs: u32, pub lpt_lebs: u32, pub orph_lebs: u32, pub jhead_cnt: u32, pub fanout: u32, pub lsave_cnt: u32, pub fmt_version: u32, pub default_compr: u16, pub padding1: [u8; 2], pub rp_uid: u32, pub rp_gid: u32, pub rp_size: u64, pub time_gran: u32, pub uuid: [u8; 16], pub ro_compat_version: u32, pub hmac: [u8; 64], pub hmac_wkm: [u8; 64], pub hash_algo: u16, pub hash_mst: [u8; 64], pub padding2: [u8; 3774] }
#[repr(C, packed)] pub struct ubifs_mst_node { pub ch: ubifs_ch, pub highest_inum: u64, pub cmt_no: u64, pub flags: u32, pub log_lnum: u32, pub root_lnum: u32, pub root_offs: u32, pub root_len: u32, pub gc_lnum: u32, pub ihead_lnum: u32, pub ihead_offs: u32, pub index_size: u64, pub total_free: u64, pub total_dirty: u64, pub total_used: u64, pub total_dead: u64, pub total_dark: u64, pub lpt_lnum: u32, pub lpt_offs: u32, pub nhead_lnum: u32, pub nhead_offs: u32, pub ltab_lnum: u32, pub ltab_offs: u32, pub lsave_lnum: u32, pub lsave_offs: u32, pub lscan_lnum: u32, pub empty_lebs: u32, pub idx_lebs: u32, pub leb_cnt: u32, pub hash_root_idx: [u8; 64], pub hash_lpt: [u8; 64], pub hmac: [u8; 64], pub padding: [u8; 152] }
#[repr(C, packed)] pub struct ubifs_ref_node { pub ch: ubifs_ch, pub lnum: u32, pub offs: u32, pub jhead: u32, pub padding: [u8; 28] }
#[repr(C, packed)] pub struct ubifs_auth_node { pub ch: ubifs_ch, pub hmac: [u8; 0] }
#[repr(C, packed)] pub struct ubifs_sig_node { pub ch: ubifs_ch, pub type_: u32, pub len: u32, pub padding: [u8; 32], pub sig: [u8; 0] }
#[repr(C, packed)] pub struct ubifs_branch { pub lnum: u32, pub offs: u32, pub len: u32, pub key: [u8; 0] }
#[repr(C, packed)] pub struct ubifs_idx_node { pub ch: ubifs_ch, pub child_cnt: u16, pub level: u16, pub branches: [u8; 0] }
#[repr(C, packed)] pub struct ubifs_cs_node { pub ch: ubifs_ch, pub cmt_no: u64 }
#[repr(C, packed)] pub struct ubifs_orph_node { pub ch: ubifs_ch, pub cmt_no: u64, pub inos: [u64; 0] }

pub const UBIFS_CH_SZ: usize = core::mem::size_of::<ubifs_ch>();
pub const UBIFS_INO_NODE_SZ: usize = core::mem::size_of::<ubifs_ino_node>();
pub const UBIFS_DATA_NODE_SZ: usize = core::mem::size_of::<ubifs_data_node>();
pub const UBIFS_DENT_NODE_SZ: usize = core::mem::size_of::<ubifs_dent_node>();
pub const UBIFS_TRUN_NODE_SZ: usize = core::mem::size_of::<ubifs_trun_node>();
pub const UBIFS_PAD_NODE_SZ: usize = core::mem::size_of::<ubifs_pad_node>();
pub const UBIFS_SB_NODE_SZ: usize = core::mem::size_of::<ubifs_sb_node>();
pub const UBIFS_MST_NODE_SZ: usize = core::mem::size_of::<ubifs_mst_node>();
pub const UBIFS_REF_NODE_SZ: usize = core::mem::size_of::<ubifs_ref_node>();
pub const UBIFS_IDX_NODE_SZ: usize = core::mem::size_of::<ubifs_idx_node>();
pub const UBIFS_CS_NODE_SZ: usize = core::mem::size_of::<ubifs_cs_node>();
pub const UBIFS_ORPH_NODE_SZ: usize = core::mem::size_of::<ubifs_orph_node>();
pub const UBIFS_AUTH_NODE_SZ: usize = core::mem::size_of::<ubifs_auth_node>();
pub const UBIFS_SIG_NODE_SZ: usize = core::mem::size_of::<ubifs_sig_node>();
pub const UBIFS_XENT_NODE_SZ: usize = UBIFS_DENT_NODE_SZ;
pub const UBIFS_BRANCH_SZ: usize = core::mem::size_of::<ubifs_branch>();
pub const UBIFS_MAX_DATA_NODE_SZ: usize = UBIFS_DATA_NODE_SZ + UBIFS_BLOCK_SIZE;
pub const UBIFS_MAX_INO_NODE_SZ: usize = UBIFS_INO_NODE_SZ + UBIFS_MAX_INO_DATA;
pub const UBIFS_MAX_DENT_NODE_SZ: usize = UBIFS_DENT_NODE_SZ + UBIFS_MAX_NLEN + 1;
pub const UBIFS_MAX_XENT_NODE_SZ: usize = UBIFS_MAX_DENT_NODE_SZ;
pub const UBIFS_MAX_NODE_SZ: usize = UBIFS_MAX_INO_NODE_SZ;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
