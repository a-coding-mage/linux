/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of hfs_common.h. */

pub type __be16 = u16;
pub type __be32 = u32;
pub type __be64 = u64;
pub type u8_ = u8;
pub type s8 = i8;
pub type s16 = i16;
pub type u16_ = u16;
pub type u32_ = u32;

pub const HFS_DD_BLK: u32 = 0;
pub const HFS_PMAP_BLK: u32 = 1;
pub const HFS_MDB_BLK: u32 = 2;
pub const HFS_DRVR_DESC_MAGIC: u32 = 0x4552;
pub const HFS_OLD_PMAP_MAGIC: u32 = 0x5453;
pub const HFS_NEW_PMAP_MAGIC: u32 = 0x504d;
pub const HFS_SUPER_MAGIC: u32 = 0x4244;
pub const HFS_MFS_SUPER_MAGIC: u32 = 0xd2d7;
pub const HFSPLUS_VOLHEAD_SIG: u32 = 0x482b;
pub const HFSPLUS_VOLHEAD_SIGX: u32 = 0x4858;
pub const HFSPLUS_SUPER_MAGIC: u32 = 0x482b;
pub const HFSP_WRAP_MAGIC: u32 = 0x4244;
pub const HFSP_WRAP_ATTRIB_SLOCK: u32 = 0x8000;
pub const HFSP_WRAP_ATTRIB_SPARED: u32 = 0x0200;
pub const HFSP_WRAPOFF_SIG: u32 = 0x00;
pub const HFSP_WRAPOFF_ATTRIB: u32 = 0x0a;
pub const HFSP_WRAPOFF_ABLKSIZE: u32 = 0x14;
pub const HFSP_WRAPOFF_ABLKSTART: u32 = 0x1c;
pub const HFSP_WRAPOFF_EMBEDSIG: u32 = 0x7c;
pub const HFSP_WRAPOFF_EMBEDEXT: u32 = 0x7e;
pub const HFSP_HARDLINK_TYPE: u32 = 0x686c6e6b;
pub const HFSP_HFSPLUS_CREATOR: u32 = 0x6866732b;
pub const HFSP_SYMLINK_TYPE: u32 = 0x736c6e6b;
pub const HFSP_SYMLINK_CREATOR: u32 = 0x72686170;
pub const HFSP_MOUNT_VERSION: u32 = 0x482b4c78;
pub const HFSP_HIDDENDIR_NAME: &[u8] = b"\xe2\x90\x80\xe2\x90\x80\xe2\x90\x80\xe2\x90\x80HFS+ Private Data\0";
pub const HFS_SECTOR_SIZE: u32 = 512;
pub const HFS_SECTOR_SIZE_BITS: u32 = 9;
pub const HFS_MAX_VALENCE: u32 = 32767;
pub const HFSPLUS_SECTOR_SIZE: u32 = HFS_SECTOR_SIZE;
pub const HFSPLUS_SECTOR_SHIFT: u32 = HFS_SECTOR_SIZE_BITS;
pub const HFSPLUS_VOLHEAD_SECTOR: u32 = 2;
pub const HFSPLUS_MIN_VERSION: u32 = 4;
pub const HFSPLUS_CURRENT_VERSION: u32 = 5;
pub const HFS_NAMELEN: usize = 31;
pub const HFS_MAX_NAMELEN: usize = 128;
pub const HFSPLUS_MAX_STRLEN: usize = 255;
pub const HFSPLUS_ATTR_MAX_STRLEN: usize = 127;

pub const HFS_SB_ATTRIB_HLOCK: u32 = 1 << 7;
pub const HFS_SB_ATTRIB_UNMNT: u32 = 1 << 8;
pub const HFS_SB_ATTRIB_SPARED: u32 = 1 << 9;
pub const HFS_SB_ATTRIB_INCNSTNT: u32 = 1 << 11;
pub const HFS_SB_ATTRIB_SLOCK: u32 = 1 << 15;
pub const HFS_CDR_DIR: u32 = 1; pub const HFS_CDR_FIL: u32 = 2; pub const HFS_CDR_THD: u32 = 3; pub const HFS_CDR_FTH: u32 = 4;
pub const HFS_FK_DATA: u32 = 0; pub const HFS_FK_RSRC: u32 = 0xff;
pub const HFS_FIL_LOCK: u32 = 1; pub const HFS_FIL_THD: u32 = 2; pub const HFS_FIL_DOPEN: u32 = 4; pub const HFS_FIL_ROPEN: u32 = 8; pub const HFS_FIL_DIR: u32 = 0x10; pub const HFS_FIL_NOCOPY: u32 = 0x40; pub const HFS_FIL_USED: u32 = 0x80;
pub const HFS_DIR_LOCK: u32 = 1; pub const HFS_DIR_THD: u32 = 2; pub const HFS_DIR_INEXPFOLDER: u32 = 4; pub const HFS_DIR_MOUNTED: u32 = 8; pub const HFS_DIR_DIR: u32 = 0x10; pub const HFS_DIR_EXPFOLDER: u32 = 0x20;
pub const HFS_FLG_INITED: u32 = 0x100; pub const HFS_FLG_LOCKED: u32 = 0x1000; pub const HFS_FLG_INVISIBLE: u32 = 0x4000;
pub const HFS_POR_CNID: u32 = 1; pub const HFSPLUS_POR_CNID: u32 = HFS_POR_CNID; pub const HFS_ROOT_CNID: u32 = 2; pub const HFSPLUS_ROOT_CNID: u32 = HFS_ROOT_CNID; pub const HFS_EXT_CNID: u32 = 3; pub const HFSPLUS_EXT_CNID: u32 = HFS_EXT_CNID; pub const HFS_CAT_CNID: u32 = 4; pub const HFSPLUS_CAT_CNID: u32 = HFS_CAT_CNID; pub const HFS_BAD_CNID: u32 = 5; pub const HFSPLUS_BAD_CNID: u32 = HFS_BAD_CNID; pub const HFS_ALLOC_CNID: u32 = 6; pub const HFSPLUS_ALLOC_CNID: u32 = HFS_ALLOC_CNID; pub const HFS_START_CNID: u32 = 7; pub const HFSPLUS_START_CNID: u32 = HFS_START_CNID; pub const HFS_ATTR_CNID: u32 = 8; pub const HFSPLUS_ATTR_CNID: u32 = HFS_ATTR_CNID; pub const HFS_EXCH_CNID: u32 = 15; pub const HFSPLUS_EXCH_CNID: u32 = HFS_EXCH_CNID; pub const HFS_FIRSTUSER_CNID: u32 = 16; pub const HFSPLUS_FIRSTUSER_CNID: u32 = HFS_FIRSTUSER_CNID;

pub const HFSPLUS_VOL_UNMNT:u32=1<<8; pub const HFSPLUS_VOL_SPARE_BLK:u32=1<<9; pub const HFSPLUS_VOL_NOCACHE:u32=1<<10; pub const HFSPLUS_VOL_INCNSTNT:u32=1<<11; pub const HFSPLUS_VOL_NODEID_REUSED:u32=1<<12; pub const HFSPLUS_VOL_JOURNALED:u32=1<<13; pub const HFSPLUS_VOL_SOFTLOCK:u32=1<<15; pub const HFSPLUS_VOL_UNUSED_NODE_FIX:u32=1<<31;

pub type hfsplus_cnid = __be32;
pub type hfsplus_unichr = __be16;
#[repr(C, packed)] pub struct hfs_name { pub len: u8, pub name: [u8; HFS_NAMELEN] }
#[repr(C, packed)] pub struct hfsplus_unistr { pub length: __be16, pub unicode: [hfsplus_unichr; HFSPLUS_MAX_STRLEN] }
#[repr(C, packed)] pub struct hfsplus_attr_unistr { pub length: __be16, pub unicode: [hfsplus_unichr; HFSPLUS_ATTR_MAX_STRLEN] }
pub const HFS_REGULAR_NAME: u32 = 0; pub const HFS_XATTR_NAME: u32 = 1;
#[repr(C)] pub struct hfs_extent { pub block: __be16, pub count: __be16 }
pub type hfs_extent_rec = [hfs_extent; 3];
#[repr(C, packed)] pub struct hfsplus_extent { pub start_block: __be32, pub block_count: __be32 }
pub type hfsplus_extent_rec = [hfsplus_extent; 8];
#[repr(C, packed)] pub struct hfsplus_fork_raw { pub total_size: __be64, pub clump_size: __be32, pub total_blocks: __be32, pub extents: hfsplus_extent_rec }

#[repr(C, packed)] pub struct hfs_mdb {
 pub drSigWord: __be16, pub drCrDate: __be32, pub drLsMod: __be32, pub drAtrb: __be16, pub drNmFls: __be16, pub drVBMSt: __be16, pub drAllocPtr: __be16, pub drNmAlBlks: __be16, pub drAlBlkSiz: __be32, pub drClpSiz: __be32, pub drAlBlSt: __be16, pub drNxtCNID: __be32, pub drFreeBks: __be16, pub drVN: [u8;28], pub drVolBkUp: __be32, pub drVSeqNum: __be16, pub drWrCnt: __be32, pub drXTClpSiz: __be32, pub drCTClpSiz: __be32, pub drNmRtDirs: __be16, pub drFilCnt: __be32, pub drDirCnt: __be32, pub drFndrInfo: [u8;32], pub drEmbedSigWord: __be16, pub drEmbedExtent: __be32, pub drXTFlSize: __be32, pub drXTExtRec: hfs_extent_rec, pub drCTFlSize: __be32, pub drCTExtRec: hfs_extent_rec }
#[repr(C, packed)] pub struct hfsplus_vh { pub signature: __be16, pub version: __be16, pub attributes: __be32, pub last_mount_vers: __be32, pub reserved: u32, pub create_date: __be32, pub modify_date: __be32, pub backup_date: __be32, pub checked_date: __be32, pub file_count: __be32, pub folder_count: __be32, pub blocksize: __be32, pub total_blocks: __be32, pub free_blocks: __be32, pub next_alloc: __be32, pub rsrc_clump_sz: __be32, pub data_clump_sz: __be32, pub next_cnid: hfsplus_cnid, pub write_count: __be32, pub encodings_bmp: __be64, pub finder_info: [u32;8], pub alloc_file: hfsplus_fork_raw, pub ext_file: hfsplus_fork_raw, pub cat_file: hfsplus_fork_raw, pub attr_file: hfsplus_fork_raw, pub start_file: hfsplus_fork_raw }

#[repr(C, packed)] pub struct hfs_point { pub v:__be16, pub h:__be16 } pub type hfsp_point=hfs_point;
#[repr(C, packed)] pub struct hfs_rect { pub top:__be16,pub left:__be16,pub bottom:__be16,pub right:__be16 } pub type hfsp_rect=hfs_rect;
#[repr(C, packed)] pub struct hfs_finfo { pub fdType:__be32,pub fdCreator:__be32,pub fdFlags:__be16,pub fdLocation:hfs_point,pub fdFldr:__be16 } pub type FInfo=hfs_finfo;
#[repr(C, packed)] pub struct hfs_fxinfo { pub fdIconID:__be16,pub fdUnused:[u8;8],pub fdComment:__be16,pub fdPutAway:__be32 } pub type FXInfo=hfs_fxinfo;
#[repr(C, packed)] pub struct hfs_dinfo { pub frRect:hfs_rect,pub frFlags:__be16,pub frLocation:hfs_point,pub frView:__be16 } pub type DInfo=hfs_dinfo;
#[repr(C, packed)] pub struct hfs_dxinfo { pub frScroll:hfs_point,pub frOpenChain:__be32,pub frUnused:__be16,pub frComment:__be16,pub frPutAway:__be32 } pub type DXInfo=hfs_dxinfo;
#[repr(C, packed)] pub union hfs_finder_info { pub file: FinderFile, pub dir: FinderDir }
#[repr(C, packed)] pub struct FinderFile { pub finfo:hfs_finfo,pub fxinfo:hfs_fxinfo }
#[repr(C, packed)] pub struct FinderDir { pub dinfo:hfs_dinfo,pub dxinfo:hfs_dxinfo }
#[repr(C, packed)] pub struct hfs_cat_key { pub key_len:u8,pub reserved:u8,pub ParID:__be32,pub CName:hfs_name }
#[repr(C, packed)] pub struct hfsplus_cat_key { pub key_len:__be16,pub parent:hfsplus_cnid,pub name:hfsplus_unistr }
#[repr(C, packed)] pub struct hfs_ext_key { pub key_len:u8,pub FkType:u8,pub FNum:__be32,pub FABN:__be16 }
#[repr(C, packed)] pub struct hfsplus_ext_key { pub key_len:__be16,pub fork_type:u8,pub pad:u8,pub cnid:hfsplus_cnid,pub start_block:__be32 }
#[repr(C)] pub union hfs_btree_key { pub key_len:u8,pub cat:hfs_cat_key,pub ext:hfs_ext_key } pub type btree_key=hfs_btree_key;
pub const HFSPLUS_KEY_CASEFOLDING:u32=0xcf; pub const HFSPLUS_KEY_BINARY:u32=0xbc;
pub const HFSPLUS_FOLDER:u32=1; pub const HFSPLUS_FILE:u32=2; pub const HFSPLUS_FOLDER_THREAD:u32=3; pub const HFSPLUS_FILE_THREAD:u32=4;
pub const HFSPLUS_XATTR_FINDER_INFO_NAME:&str="com.apple.FinderInfo"; pub const HFSPLUS_XATTR_ACL_NAME:&str="com.apple.system.Security";
pub const HFSPLUS_ATTR_INLINE_DATA:u32=0x10; pub const HFSPLUS_ATTR_FORK_DATA:u32=0x20; pub const HFSPLUS_ATTR_EXTENTS:u32=0x30;
pub const HFSPLUS_MAX_INLINE_DATA_SIZE:usize=3802;
#[repr(C, packed)] pub struct hfsplus_perm { pub owner:__be32,pub group:__be32,pub rootflags:u8,pub userflags:u8,pub mode:__be16,pub dev:__be32 }
#[repr(C, packed)] pub struct hfs_bnode_desc { pub next:__be32,pub prev:__be32,pub r#type:u8,pub height:u8,pub num_recs:__be16,pub reserved:u16 }
pub const HFS_NODE_INDEX:u32=0; pub const HFS_NODE_HEADER:u32=1; pub const HFS_NODE_MAP:u32=2; pub const HFS_NODE_LEAF:u32=0xff;
#[repr(C, packed)] pub struct hfs_btree_header_rec { pub depth:__be16,pub root:__be32,pub leaf_count:__be32,pub leaf_head:__be32,pub leaf_tail:__be32,pub node_size:__be16,pub max_key_len:__be16,pub node_count:__be32,pub free_nodes:__be32,pub reserved1:u16,pub clump_size:__be32,pub btree_type:u8,pub key_type:u8,pub attributes:__be32,pub reserved3:[u32;16] }
pub const BTREE_ATTR_BADCLOSE:u32=1; pub const HFS_TREE_BIGKEYS:u32=2; pub const HFS_TREE_VARIDXKEYS:u32=4; pub const HFS_TREE_HEAD:u32=0; pub const HFS_BTREE_HDR_MAP_REC_INDEX:u32=2; pub const HFS_BTREE_MAP_NODE_REC_INDEX:u32=0; pub const HFSPLUS_NODE_MXSZ:u32=32768; pub const HFSPLUS_NODE_MINSZ:u32=512; pub const HFSPLUS_ATTR_TREE_NODE_SIZE:u32=8192; pub const HFSPLUS_BTREE_HDR_NODE_RECS_COUNT:u32=3; pub const HFSPLUS_BTREE_HDR_USER_BYTES:u32=128; pub const HFSPLUS_BTREE_MAP_NODE_RECS_COUNT:u32=2; pub const HFSPLUS_BTREE_MAP_NODE_RESERVED_BYTES:u32=2;
pub const HFSPLUS_CAT_KEYLEN:usize=core::mem::size_of::<hfsplus_cat_key>(); pub const HFSPLUS_EXT_KEYLEN:usize=core::mem::size_of::<hfsplus_ext_key>(); pub const HFS_MAX_CAT_KEYLEN:usize=core::mem::size_of::<hfs_cat_key>()-1; pub const HFS_MAX_EXT_KEYLEN:usize=core::mem::size_of::<hfs_ext_key>()-1;
pub const HFSPLUS_FLG_NODUMP:u32=1; pub const HFSPLUS_FLG_IMMUTABLE:u32=2; pub const HFSPLUS_FLG_APPEND:u32=4;
pub const HFSPLUS_FILE_LOCKED:u32=1; pub const HFSPLUS_FILE_THREAD_EXISTS:u32=2; pub const HFSPLUS_XATTR_EXISTS:u32=4; pub const HFSPLUS_ACL_EXISTS:u32=8; pub const HFSPLUS_HAS_FOLDER_COUNT:u32=0x10; pub const HFSPLUS_MIN_THREAD_SZ:u32=10;
#[repr(C, packed)] pub struct hfsplus_cat_thread { pub r#type:__be16,pub reserved:i16,pub parentID:hfsplus_cnid,pub nodeName:hfsplus_unistr }
#[repr(C, packed)] pub struct hfsplus_cat_folder { pub r#type:__be16,pub flags:__be16,pub valence:__be32,pub id:hfsplus_cnid,pub create_date:__be32,pub content_mod_date:__be32,pub attribute_mod_date:__be32,pub access_date:__be32,pub backup_date:__be32,pub permissions:hfsplus_perm,pub user_info:DInfo,pub finder_info:DXInfo,pub text_encoding:__be32,pub subfolders:__be32 }
#[repr(C, packed)] pub struct hfsplus_cat_file { pub r#type:__be16,pub flags:__be16,pub reserved1:u32,pub id:hfsplus_cnid,pub create_date:__be32,pub content_mod_date:__be32,pub attribute_mod_date:__be32,pub access_date:__be32,pub backup_date:__be32,pub permissions:hfsplus_perm,pub user_info:FInfo,pub finder_info:FXInfo,pub text_encoding:__be32,pub reserved2:u32,pub data_fork:hfsplus_fork_raw,pub rsrc_fork:hfsplus_fork_raw }
#[repr(C, packed)] pub union hfsplus_cat_entry { pub r#type:__be16,pub folder:hfsplus_cat_folder,pub file:hfsplus_cat_file,pub thread:hfsplus_cat_thread }
#[repr(C, packed)] pub struct hfsplus_attr_key { pub key_len:__be16,pub pad:__be16,pub cnid:hfsplus_cnid,pub start_block:__be32,pub key_name:hfsplus_attr_unistr }
#[repr(C, packed)] pub struct hfsplus_attr_fork_data { pub record_type:__be32,pub reserved:__be32,pub the_fork:hfsplus_fork_raw }
#[repr(C, packed)] pub struct hfsplus_attr_extents { pub record_type:__be32,pub reserved:__be32,pub extents:hfsplus_extent }
#[repr(C, packed)] pub struct hfsplus_attr_inline_data { pub record_type:__be32,pub reserved1:__be32,pub reserved2:[u8;6],pub length:__be16,pub raw_bytes:[u8;3802] }
#[repr(C, packed)] pub union hfsplus_attr_entry { pub record_type:__be32,pub fork_data:hfsplus_attr_fork_data,pub extents:hfsplus_attr_extents,pub inline_data:hfsplus_attr_inline_data }
#[repr(C, packed)] pub union hfsplus_btree_key { pub key_len:__be16,pub cat:hfsplus_cat_key,pub ext:hfsplus_ext_key,pub attr:hfsplus_attr_key }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
