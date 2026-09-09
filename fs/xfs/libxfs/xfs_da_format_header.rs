/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from xfs_da_format.h. External kernel types and helpers are supplied by dependencies. */

pub const XFS_DA_NODE_MAGIC: u16 = 0xfebe;
pub const XFS_ATTR_LEAF_MAGIC: u16 = 0xfbee;
pub const XFS_DIR2_LEAF1_MAGIC: u16 = 0xd2f1;
pub const XFS_DIR2_LEAFN_MAGIC: u16 = 0xd2ff;

#[repr(C)] pub struct xfs_da_blkinfo { pub forw: __be32, pub back: __be32, pub magic: __be16, pub pad: __be16 }
pub type xfs_da_blkinfo_t = xfs_da_blkinfo;
pub const XFS_DA3_NODE_MAGIC: u16 = 0x3ebe;
pub const XFS_ATTR3_LEAF_MAGIC: u16 = 0x3bee;
pub const XFS_DIR3_LEAF1_MAGIC: u16 = 0x3df1;
pub const XFS_DIR3_LEAFN_MAGIC: u16 = 0x3dff;
#[repr(C)] pub struct xfs_da3_blkinfo { pub hdr: xfs_da_blkinfo, pub crc: __be32, pub blkno: __be64, pub lsn: __be64, pub uuid: uuid_t, pub owner: __be64 }

pub const XFS_DA_NODE_MAXDEPTH: u32 = 5;
#[repr(C)] pub struct xfs_da_node_hdr { pub info: xfs_da_blkinfo, pub __count: __be16, pub __level: __be16 }
#[repr(C)] pub struct xfs_da3_node_hdr { pub info: xfs_da3_blkinfo, pub __count: __be16, pub __level: __be16, pub __pad32: __be32 }
pub const XFS_DA3_NODE_CRC_OFF: usize = core::mem::offset_of!(xfs_da3_node_hdr, info) + core::mem::offset_of!(xfs_da3_blkinfo, crc);
#[repr(C)] pub struct xfs_da_node_entry { pub hashval: __be32, pub before: __be32 }
#[repr(C)] pub struct xfs_da_intnode { pub hdr: xfs_da_node_hdr, pub __btree: [xfs_da_node_entry; 0] }
#[repr(C)] pub struct xfs_da3_intnode { pub hdr: xfs_da3_node_hdr, pub __btree: [xfs_da_node_entry; 0] }

pub const XFS_DIR2_BLOCK_MAGIC: u32 = 0x58443242;
pub const XFS_DIR2_DATA_MAGIC: u32 = 0x58443244;
pub const XFS_DIR2_FREE_MAGIC: u32 = 0x58443246;
pub const XFS_DIR3_BLOCK_MAGIC: u32 = 0x58444233;
pub const XFS_DIR3_DATA_MAGIC: u32 = 0x58444433;
pub const XFS_DIR3_FREE_MAGIC: u32 = 0x58444633;
pub const XFS_DIR3_FT_UNKNOWN: u8 = 0; pub const XFS_DIR3_FT_REG_FILE: u8 = 1; pub const XFS_DIR3_FT_DIR: u8 = 2; pub const XFS_DIR3_FT_CHRDEV: u8 = 3; pub const XFS_DIR3_FT_BLKDEV: u8 = 4; pub const XFS_DIR3_FT_FIFO: u8 = 5; pub const XFS_DIR3_FT_SOCK: u8 = 6; pub const XFS_DIR3_FT_SYMLINK: u8 = 7; pub const XFS_DIR3_FT_WHT: u8 = 8; pub const XFS_DIR3_FT_MAX: u8 = 9;
pub type xfs_dir2_data_off_t = u16; pub const NULLDATAOFF: u16 = 0xffff; pub type xfs_dir2_data_aoff_t = uint;
pub type xfs_dir2_dataptr_t = u32; pub const XFS_DIR2_MAX_DATAPTR: u32 = u32::MAX; pub const XFS_DIR2_NULL_DATAPTR: u32 = 0;
pub type xfs_dir2_off_t = xfs_off_t; pub type xfs_dir2_db_t = u32;
pub const XFS_INO32_SIZE: usize = 4; pub const XFS_INO64_SIZE: usize = 8; pub const XFS_INO64_DIFF: usize = 4;
pub const XFS_DIR2_MAX_SHORT_INUM: u64 = 0xffff_ffff;
#[repr(C, packed)] pub struct xfs_dir2_sf_hdr { pub count: u8, pub i8count: u8, pub parent: [u8; 8] }
pub type xfs_dir2_sf_hdr_t = xfs_dir2_sf_hdr;
#[repr(C, packed)] pub struct xfs_dir2_sf_entry { pub namelen: u8, pub offset: [u8; 2], pub name: [u8; 0] }
pub type xfs_dir2_sf_entry_t = xfs_dir2_sf_entry;
pub unsafe fn xfs_dir2_sf_hdr_size(i8count: i32) -> usize { core::mem::size_of::<xfs_dir2_sf_hdr>() - if i8count == 0 { XFS_INO64_DIFF } else { 0 } }
pub unsafe fn xfs_dir2_sf_get_offset(sfep: *mut xfs_dir2_sf_entry) -> xfs_dir2_data_aoff_t { get_unaligned_be16((*sfep).offset.as_ptr()) as _ }
pub unsafe fn xfs_dir2_sf_put_offset(sfep: *mut xfs_dir2_sf_entry, off: xfs_dir2_data_aoff_t) { put_unaligned_be16(off as _, (*sfep).offset.as_mut_ptr()); }
pub unsafe fn xfs_dir2_sf_firstentry(hdr: *mut xfs_dir2_sf_hdr) -> *mut xfs_dir2_sf_entry { (hdr as *mut u8).add(xfs_dir2_sf_hdr_size((*hdr).i8count as _)) as _ }

pub const XFS_DIR2_DATA_ALIGN_LOG: u32 = 3; pub const XFS_DIR2_DATA_ALIGN: u32 = 1 << XFS_DIR2_DATA_ALIGN_LOG; pub const XFS_DIR2_DATA_FREE_TAG: u16 = 0xffff; pub const XFS_DIR2_DATA_FD_COUNT: usize = 3;
pub const XFS_DIR2_MAX_SPACES: u32 = 3; pub const XFS_DIR2_SPACE_SIZE: u64 = 1u64 << (32 + XFS_DIR2_DATA_ALIGN_LOG); pub const XFS_DIR2_DATA_SPACE: u32 = 0; pub const XFS_DIR2_DATA_OFFSET: u64 = 0;
#[repr(C)] pub struct xfs_dir2_data_free { pub offset: __be16, pub length: __be16 } pub type xfs_dir2_data_free_t = xfs_dir2_data_free;
#[repr(C)] pub struct xfs_dir2_data_hdr { pub magic: __be32, pub bestfree: [xfs_dir2_data_free; 3] } pub type xfs_dir2_data_hdr_t = xfs_dir2_data_hdr;
#[repr(C)] pub struct xfs_dir3_blk_hdr { pub magic: __be32, pub crc: __be32, pub blkno: __be64, pub lsn: __be64, pub uuid: uuid_t, pub owner: __be64 }
#[repr(C)] pub struct xfs_dir3_data_hdr { pub hdr: xfs_dir3_blk_hdr, pub best_free: [xfs_dir2_data_free; 3], pub pad: __be32 }
#[repr(C)] pub struct xfs_dir2_data_entry { pub inumber: __be64, pub namelen: u8, pub name: [u8; 0] } pub type xfs_dir2_data_entry_t = xfs_dir2_data_entry;
#[repr(C)] pub struct xfs_dir2_data_unused { pub freetag: __be16, pub length: __be16, pub tag: __be16 } pub type xfs_dir2_data_unused_t = xfs_dir2_data_unused;
pub unsafe fn xfs_dir2_data_unused_tag_p(dup: *mut xfs_dir2_data_unused) -> *mut __be16 { (dup as *mut u8).add(be16_to_cpu((*dup).length) as usize - 2) as _ }

pub const XFS_DIR2_LEAF_SPACE: u32 = 1; pub const XFS_DIR2_LEAF_OFFSET: u64 = XFS_DIR2_SPACE_SIZE;
#[repr(C)] pub struct xfs_dir2_leaf_hdr { pub info: xfs_da_blkinfo, pub count: __be16, pub stale: __be16 } pub type xfs_dir2_leaf_hdr_t = xfs_dir2_leaf_hdr;
#[repr(C)] pub struct xfs_dir3_leaf_hdr { pub info: xfs_da3_blkinfo, pub count: __be16, pub stale: __be16, pub pad: __be32 }
#[repr(C)] pub struct xfs_dir2_leaf_entry { pub hashval: __be32, pub address: __be32 } pub type xfs_dir2_leaf_entry_t = xfs_dir2_leaf_entry;
#[repr(C)] pub struct xfs_dir2_leaf_tail { pub bestcount: __be32 } pub type xfs_dir2_leaf_tail_t = xfs_dir2_leaf_tail;
#[repr(C)] pub struct xfs_dir2_leaf { pub hdr: xfs_dir2_leaf_hdr, pub __ents: [xfs_dir2_leaf_entry; 0] } pub type xfs_dir2_leaf_t = xfs_dir2_leaf;
#[repr(C)] pub struct xfs_dir3_leaf { pub hdr: xfs_dir3_leaf_hdr, pub __ents: [xfs_dir2_leaf_entry; 0] }
pub unsafe fn xfs_dir2_leaf_bests_p(ltp: *mut xfs_dir2_leaf_tail) -> *mut __be16 { (ltp as *mut __be16).sub(be32_to_cpu((*ltp).bestcount) as usize) }
pub const XFS_DIR2_FREE_SPACE: u32 = 2; pub const XFS_DIR2_FREE_OFFSET: u64 = 2 * XFS_DIR2_SPACE_SIZE;
#[repr(C)] pub struct xfs_dir2_free_hdr { pub magic: __be32, pub firstdb: __be32, pub nvalid: __be32, pub nused: __be32 } pub type xfs_dir2_free_hdr_t = xfs_dir2_free_hdr;
#[repr(C)] pub struct xfs_dir2_free { pub hdr: xfs_dir2_free_hdr, pub bests: [__be16; 0] } pub type xfs_dir2_free_t = xfs_dir2_free;
#[repr(C)] pub struct xfs_dir3_free_hdr { pub hdr: xfs_dir3_blk_hdr, pub firstdb: __be32, pub nvalid: __be32, pub nused: __be32, pub pad: __be32 }
#[repr(C)] pub struct xfs_dir3_free { pub hdr: xfs_dir3_free_hdr, pub bests: [__be16; 0] }
#[repr(C)] pub struct xfs_dir2_block_tail { pub count: __be32, pub stale: __be32 } pub type xfs_dir2_block_tail_t = xfs_dir2_block_tail;
pub unsafe fn xfs_dir2_block_leaf_p(btp: *mut xfs_dir2_block_tail) -> *mut xfs_dir2_leaf_entry { (btp as *mut xfs_dir2_leaf_entry).sub(be32_to_cpu((*btp).count) as usize) }

pub const XFS_ATTR_LEAF_MAPSIZE: usize = 3;
#[repr(C)] pub struct xfs_attr_sf_hdr { pub totsize: __be16, pub count: u8, pub padding: u8 }
#[repr(C)] pub struct xfs_attr_sf_entry { pub namelen: u8, pub valuelen: u8, pub flags: u8, pub nameval: [u8; 0] }
#[repr(C)] pub struct xfs_attr_leaf_map { pub base: __be16, pub size: __be16 } pub type xfs_attr_leaf_map_t = xfs_attr_leaf_map;
#[repr(C)] pub struct xfs_attr_leaf_hdr { pub info: xfs_da_blkinfo, pub count: __be16, pub usedbytes: __be16, pub firstused: __be16, pub holes: u8, pub pad1: u8, pub freemap: [xfs_attr_leaf_map; 3] } pub type xfs_attr_leaf_hdr_t = xfs_attr_leaf_hdr;
#[repr(C)] pub struct xfs_attr_leaf_entry { pub hashval: __be32, pub nameidx: __be16, pub flags: u8, pub pad2: u8 } pub type xfs_attr_leaf_entry_t = xfs_attr_leaf_entry;
#[repr(C)] pub struct xfs_attr_leaf_name_local { pub valuelen: __be16, pub namelen: u8, pub nameval: [u8; 0] } pub type xfs_attr_leaf_name_local_t = xfs_attr_leaf_name_local;
#[repr(C)] pub struct xfs_attr_leaf_name_remote { pub valueblk: __be32, pub valuelen: __be32, pub namelen: u8, pub name: [u8; 0] } pub type xfs_attr_leaf_name_remote_t = xfs_attr_leaf_name_remote;
#[repr(C)] pub struct xfs_attr_leafblock { pub hdr: xfs_attr_leaf_hdr, pub entries: [xfs_attr_leaf_entry; 0] } pub type xfs_attr_leafblock_t = xfs_attr_leafblock;
#[repr(C)] pub struct xfs_attr3_leaf_hdr { pub info: xfs_da3_blkinfo, pub count: __be16, pub usedbytes: __be16, pub firstused: __be16, pub holes: u8, pub pad1: u8, pub freemap: [xfs_attr_leaf_map; 3], pub pad2: __be32 }
#[repr(C)] pub struct xfs_attr3_leafblock { pub hdr: xfs_attr3_leaf_hdr, pub entries: [xfs_attr_leaf_entry; 0] }
pub const XFS_ATTR3_LEAF_NULLOFF: u16 = 0; pub const XFS_ATTR_LOCAL_BIT: u32 = 0; pub const XFS_ATTR_ROOT_BIT: u32 = 1; pub const XFS_ATTR_SECURE_BIT: u32 = 2; pub const XFS_ATTR_PARENT_BIT: u32 = 3; pub const XFS_ATTR_INCOMPLETE_BIT: u32 = 7;
pub const XFS_ATTR_LOCAL: u32 = 1 << 0; pub const XFS_ATTR_ROOT: u32 = 1 << 1; pub const XFS_ATTR_SECURE: u32 = 1 << 2; pub const XFS_ATTR_PARENT: u32 = 1 << 3; pub const XFS_ATTR_INCOMPLETE: u32 = 1 << 7;
pub const XFS_ATTR_NSP_ONDISK_MASK: u32 = XFS_ATTR_ROOT | XFS_ATTR_SECURE | XFS_ATTR_PARENT; pub const XFS_ATTR_PRIVATE_NSP_MASK: u32 = XFS_ATTR_PARENT; pub const XFS_ATTR_ONDISK_MASK: u32 = XFS_ATTR_NSP_ONDISK_MASK | XFS_ATTR_LOCAL | XFS_ATTR_INCOMPLETE;
pub const XFS_ATTR_LEAF_NAME_ALIGN: usize = core::mem::size_of::<xfs_dablk_t>();
pub unsafe fn xfs_attr3_leaf_entryp(leafp: *mut xfs_attr_leafblock) -> *mut xfs_attr_leaf_entry { (*leafp).entries.as_mut_ptr() }
pub unsafe fn xfs_attr3_leaf_name(leafp: *mut xfs_attr_leafblock, idx: i32) -> *mut i8 { (leafp as *mut u8).add(be16_to_cpu((*xfs_attr3_leaf_entryp(leafp).add(idx as usize)).nameidx) as usize) as _ }
pub unsafe fn xfs_attr3_leaf_name_remote(leafp: *mut xfs_attr_leafblock, idx: i32) -> *mut xfs_attr_leaf_name_remote { xfs_attr3_leaf_name(leafp, idx) as _ }
pub unsafe fn xfs_attr3_leaf_name_local(leafp: *mut xfs_attr_leafblock, idx: i32) -> *mut xfs_attr_leaf_name_local { xfs_attr3_leaf_name(leafp, idx) as _ }
pub unsafe fn xfs_attr_leaf_entsize_remote(nlen: i32) -> i32 { round_up(11 + nlen as usize, XFS_ATTR_LEAF_NAME_ALIGN) as _ }
pub unsafe fn xfs_attr_leaf_entsize_local(nlen: i32, vlen: i32) -> i32 { round_up(3 + nlen as usize + vlen as usize, XFS_ATTR_LEAF_NAME_ALIGN) as _ }
pub unsafe fn xfs_attr_leaf_entsize_local_max(bsize: i32) -> i32 { (bsize >> 1) + (bsize >> 2) }
pub const XFS_ATTR3_RMT_MAGIC: u32 = 0x5841524d;
#[repr(C)] pub struct xfs_attr3_rmt_hdr { pub rm_magic: __be32, pub rm_offset: __be32, pub rm_bytes: __be32, pub rm_crc: __be32, pub rm_uuid: uuid_t, pub rm_owner: __be64, pub rm_blkno: __be64, pub rm_lsn: __be64 }
extern "C" { pub fn xfs_attr3_rmt_buf_space(mp: *mut xfs_mount) -> uint; pub fn xfs_da3_blkinfo_verify(bp: *mut xfs_buf, hdr3: *mut xfs_da3_blkinfo) -> xfs_failaddr_t; }
pub unsafe fn xfs_dir2_dirblock_bytes(sbp: *mut xfs_sb) -> uint { 1 << ((*sbp).sb_blocklog + (*sbp).sb_dirblklog) }
#[repr(C, packed)] pub struct xfs_parent_rec { pub p_ino: __be64, pub p_gen: __be32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
