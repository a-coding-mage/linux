/* SPDX-License-Identifier: MIT */
/* EROFS on-disk format definition, translated from erofs_fs.h. */

pub const EROFS_SUPER_OFFSET: u32 = 1024;
pub const EROFS_FEATURE_COMPAT_SB_CHKSUM: u32 = 0x00000001;
pub const EROFS_FEATURE_COMPAT_MTIME: u32 = 0x00000002;
pub const EROFS_FEATURE_COMPAT_XATTR_FILTER: u32 = 0x00000004;
pub const EROFS_FEATURE_COMPAT_SHARED_EA_IN_METABOX: u32 = 0x00000008;
pub const EROFS_FEATURE_COMPAT_PLAIN_XATTR_PFX: u32 = 0x00000010;
pub const EROFS_FEATURE_COMPAT_ISHARE_XATTRS: u32 = 0x00000020;
pub const EROFS_FEATURE_INCOMPAT_LZ4_0PADDING: u32 = 1;
pub const EROFS_FEATURE_INCOMPAT_COMPR_CFGS: u32 = 2;
pub const EROFS_FEATURE_INCOMPAT_BIG_PCLUSTER: u32 = 2;
pub const EROFS_FEATURE_INCOMPAT_CHUNKED_FILE: u32 = 4;
pub const EROFS_FEATURE_INCOMPAT_DEVICE_TABLE: u32 = 8;
pub const EROFS_FEATURE_INCOMPAT_COMPR_HEAD2: u32 = 8;
pub const EROFS_FEATURE_INCOMPAT_ZTAILPACKING: u32 = 0x10;
pub const EROFS_FEATURE_INCOMPAT_FRAGMENTS: u32 = 0x20;
pub const EROFS_FEATURE_INCOMPAT_DEDUPE: u32 = 0x20;
pub const EROFS_FEATURE_INCOMPAT_XATTR_PREFIXES: u32 = 0x40;
pub const EROFS_FEATURE_INCOMPAT_48BIT: u32 = 0x80;
pub const EROFS_FEATURE_INCOMPAT_METABOX: u32 = 0x100;
pub const EROFS_ALL_FEATURE_INCOMPAT: u32 = (EROFS_FEATURE_INCOMPAT_METABOX << 1) - 1;
pub const EROFS_SB_EXTSLOT_SIZE: usize = 16;

#[repr(C)] pub struct erofs_deviceslot { pub tag: [u8;64], pub blocks_lo: __le32, pub uniaddr_lo: __le32, pub blocks_hi: __le16, pub uniaddr_hi: __le16, pub reserved: [u8;52] }
pub const EROFS_DEVT_SLOT_SIZE: usize = core::mem::size_of::<erofs_deviceslot>();
#[repr(C)] pub union erofs_super_block_rb { pub rootnid_2b: __le16, pub blocks_hi: __le16 }
#[repr(C)] pub union erofs_super_block_u1 { pub available_compr_algs: __le16, pub lz4_max_distance: __le16 }
#[repr(C)] pub struct erofs_super_block { pub magic: __le32, pub checksum: __le32, pub feature_compat: __le32, pub blkszbits: u8, pub sb_extslots: u8, pub rb: erofs_super_block_rb, pub inos: __le64, pub epoch: __le64, pub fixed_nsec: __le32, pub blocks_lo: __le32, pub meta_blkaddr: __le32, pub xattr_blkaddr: __le32, pub uuid: [u8;16], pub volume_name: [u8;16], pub feature_incompat: __le32, pub u1: erofs_super_block_u1, pub extra_devices: __le16, pub devt_slotoff: __le16, pub dirblkbits: u8, pub xattr_prefix_count: u8, pub xattr_prefix_start: __le32, pub packed_nid: __le64, pub xattr_filter_reserved: u8, pub ishare_xattr_prefix_id: u8, pub reserved: [u8;2], pub build_time: __le32, pub rootnid_8b: __le64, pub reserved2: __le64, pub metabox_nid: __le64, pub reserved3: __le64 }

pub const EROFS_INODE_FLAT_PLAIN: u32=0; pub const EROFS_INODE_COMPRESSED_FULL: u32=1; pub const EROFS_INODE_FLAT_INLINE: u32=2; pub const EROFS_INODE_COMPRESSED_COMPACT: u32=3; pub const EROFS_INODE_CHUNK_BASED: u32=4; pub const EROFS_INODE_DATALAYOUT_MAX: u32=5;
#[inline] pub fn erofs_inode_is_data_compressed(datamode: u32)->bool { datamode==EROFS_INODE_COMPRESSED_COMPACT || datamode==EROFS_INODE_COMPRESSED_FULL }
pub const EROFS_I_VERSION_MASK:u32=1; pub const EROFS_I_DATALAYOUT_MASK:u32=7; pub const EROFS_I_VERSION_BIT:u32=0; pub const EROFS_I_DATALAYOUT_BIT:u32=1; pub const EROFS_I_NLINK_1_BIT:u32=4; pub const EROFS_I_DOT_OMITTED_BIT:u32=4; pub const EROFS_I_ALL:u32=(1<<(EROFS_I_NLINK_1_BIT+1))-1;
pub const EROFS_CHUNK_FORMAT_BLKBITS_MASK:u16=0x1f; pub const EROFS_CHUNK_FORMAT_INDEXES:u16=0x20; pub const EROFS_CHUNK_FORMAT_48BIT:u16=0x40; pub const EROFS_CHUNK_FORMAT_ALL:u16=(EROFS_CHUNK_FORMAT_48BIT<<1)-1;
pub const EROFS_INODE_LAYOUT_COMPACT:u32=0; pub const EROFS_INODE_LAYOUT_EXTENDED:u32=1;
#[repr(C)] pub struct erofs_inode_chunk_info { pub format: __le16, pub reserved: __le16 }
#[repr(C)] pub union erofs_inode_i_u { pub blocks_lo: __le32, pub startblk_lo: __le32, pub rdev: __le32, pub c: erofs_inode_chunk_info }
#[repr(C)] pub union erofs_inode_i_nb { pub nlink: __le16, pub blocks_hi: __le16, pub startblk_hi: __le16 }
#[repr(C)] pub struct erofs_inode_compact { pub i_format:__le16,pub i_xattr_icount:__le16,pub i_mode:__le16,pub i_nb:erofs_inode_i_nb,pub i_size:__le32,pub i_mtime:__le32,pub i_u:erofs_inode_i_u,pub i_ino:__le32,pub i_uid:__le16,pub i_gid:__le16,pub i_reserved:__le32 }
#[repr(C)] pub struct erofs_inode_extended { pub i_format:__le16,pub i_xattr_icount:__le16,pub i_mode:__le16,pub i_nb:erofs_inode_i_nb,pub i_size:__le64,pub i_u:erofs_inode_i_u,pub i_ino:__le32,pub i_uid:__le32,pub i_gid:__le32,pub i_mtime:__le64,pub i_mtime_nsec:__le32,pub i_nlink:__le32,pub i_reserved2:[u8;16] }
#[repr(C)] pub struct erofs_xattr_ibody_header { pub h_name_filter:__le32,pub h_shared_count:u8,pub h_reserved2:[u8;7],pub h_shared_xattrs:[__le32;0] }
pub const EROFS_XATTR_INDEX_USER:u32=1; pub const EROFS_XATTR_INDEX_POSIX_ACL_ACCESS:u32=2; pub const EROFS_XATTR_INDEX_POSIX_ACL_DEFAULT:u32=3; pub const EROFS_XATTR_INDEX_TRUSTED:u32=4; pub const EROFS_XATTR_INDEX_LUSTRE:u32=5; pub const EROFS_XATTR_INDEX_SECURITY:u32=6; pub const EROFS_XATTR_LONG_PREFIX:u8=0x80; pub const EROFS_XATTR_LONG_PREFIX_MASK:u8=0x7f; pub const EROFS_XATTR_FILTER_BITS:u32=32; pub const EROFS_XATTR_FILTER_DEFAULT:u32=u32::MAX; pub const EROFS_XATTR_FILTER_SEED:u32=0x25BBE08F;
#[repr(C)] pub struct erofs_xattr_entry { pub e_name_len:u8,pub e_name_index:u8,pub e_value_size:__le16,pub e_name:[core::ffi::c_char;0] }
#[repr(C)] pub struct erofs_xattr_long_prefix { pub base_index:u8,pub infix:[core::ffi::c_char;0] }
#[inline] pub fn erofs_xattr_ibody_size(i:__le16)->u32 { if i==0 {0} else {12 + 4 * (le16_to_cpu(i)-1) } }
#[inline] pub unsafe fn erofs_xattr_entry_size(e:*const erofs_xattr_entry)->u32 { EROFS_XATTR_ALIGN(core::mem::size_of::<erofs_xattr_entry>() as u32 + (*e).e_name_len as u32 + le16_to_cpu((*e).e_value_size) as u32) }
pub const EROFS_NULL_ADDR:i32=-1; pub const EROFS_BLOCK_MAP_ENTRY_SIZE:usize=core::mem::size_of::<__le32>();
#[repr(C)] pub struct erofs_inode_chunk_index { pub startblk_hi:__le16,pub device_id:__le16,pub startblk_lo:__le32 }
pub const EROFS_DIRENT_NID_METABOX_BIT:u32=63; pub const EROFS_DIRENT_NID_MASK:u64=(1u64<<63)-1;
#[repr(C)] pub struct erofs_dirent { pub nid:__le64,pub nameoff:__le16,pub file_type:u8,pub reserved:u8 }
pub const EROFS_NAME_LEN:usize=255; pub const Z_EROFS_PCLUSTER_MAX_SIZE:u32=1024*1024; pub const Z_EROFS_PCLUSTER_MAX_DSIZE:u32=12*1024*1024;
pub const Z_EROFS_COMPRESSION_LZ4:u32=0; pub const Z_EROFS_COMPRESSION_LZMA:u32=1; pub const Z_EROFS_COMPRESSION_DEFLATE:u32=2; pub const Z_EROFS_COMPRESSION_ZSTD:u32=3; pub const Z_EROFS_COMPRESSION_MAX:u32=4; pub const Z_EROFS_ALL_COMPR_ALGS:u32=(1<<4)-1;
#[repr(C)] pub struct z_erofs_lz4_cfgs { pub max_distance:__le16,pub max_pclusterblks:__le16,pub reserved:[u8;10] }
#[repr(C)] pub struct z_erofs_lzma_cfgs { pub dict_size:__le32,pub format:__le16,pub reserved:[u8;8] }
pub const Z_EROFS_LZMA_MAX_DICT_SIZE:u32=8*Z_EROFS_PCLUSTER_MAX_SIZE;
#[repr(C)] pub struct z_erofs_deflate_cfgs { pub windowbits:u8,pub reserved:[u8;5] }
#[repr(C)] pub struct z_erofs_zstd_cfgs { pub format:u8,pub windowlog:u8,pub reserved:[u8;4] }
pub const Z_EROFS_ZSTD_MAX_DICT_SIZE:u32=Z_EROFS_PCLUSTER_MAX_SIZE;
pub const Z_EROFS_ADVISE_COMPACTED_2B:u16=1; pub const Z_EROFS_ADVISE_EXTENTS:u16=1; pub const Z_EROFS_ADVISE_BIG_PCLUSTER_1:u16=2; pub const Z_EROFS_ADVISE_BIG_PCLUSTER_2:u16=4; pub const Z_EROFS_ADVISE_INLINE_PCLUSTER:u16=8; pub const Z_EROFS_ADVISE_INTERLACED_PCLUSTER:u16=0x10; pub const Z_EROFS_ADVISE_FRAGMENT_PCLUSTER:u16=0x20; pub const Z_EROFS_ADVISE_EXTRECSZ_BIT:u32=1; pub const Z_EROFS_ADVISE_EXTRECSZ_MASK:u32=3; pub const Z_EROFS_FRAGMENT_INODE_BIT:u32=7;
#[repr(C)] pub struct z_erofs_map_header { pub h_fragmentoff:__le32,pub h_advise:__le16,pub h_algorithmtype:u8,pub h_clusterbits:u8 }
pub const Z_EROFS_LCLUSTER_TYPE_PLAIN:u32=0; pub const Z_EROFS_LCLUSTER_TYPE_HEAD1:u32=1; pub const Z_EROFS_LCLUSTER_TYPE_NONHEAD:u32=2; pub const Z_EROFS_LCLUSTER_TYPE_HEAD2:u32=3; pub const Z_EROFS_LCLUSTER_TYPE_MAX:u32=4; pub const Z_EROFS_LI_LCLUSTER_TYPE_MASK:u32=3; pub const Z_EROFS_LI_PARTIAL_REF:u16=1<<15; pub const Z_EROFS_LI_HOLE:u16=1<<14; pub const Z_EROFS_LI_D0_CBLKCNT:u16=1<<11;
#[repr(C)] pub union z_erofs_lcluster_index_u { pub blkaddr:__le32,pub delta:[__le16;2] }
#[repr(C)] pub struct z_erofs_lcluster_index { pub di_advise:__le16,pub di_clusterofs:__le16,pub di_u:z_erofs_lcluster_index_u }
pub const Z_EROFS_EXTENT_PLEN_PARTIAL:u32=1<<27; pub const Z_EROFS_EXTENT_PLEN_FMT_BIT:u32=28; pub const Z_EROFS_EXTENT_PLEN_MASK:u32=(Z_EROFS_PCLUSTER_MAX_SIZE<<1)-1;
#[repr(C)] pub struct z_erofs_extent { pub plen:__le32,pub pstart_lo:__le32,pub pstart_hi:__le32,pub lstart_lo:__le32,pub lstart_hi:__le32,pub reserved:[u8;12] }
#[inline] pub fn z_erofs_extent_recsize(advise:u32)->i32 { 4 << ((advise>>Z_EROFS_ADVISE_EXTRECSZ_BIT)&Z_EROFS_ADVISE_EXTRECSZ_MASK) }
#[inline] pub const fn z_erofs_map_header_end(end: usize) -> usize { ((end + 7) & !7) + core::mem::size_of::<z_erofs_map_header>() }
#[inline] pub const fn z_erofs_full_index_start(end: usize) -> usize { z_erofs_map_header_end(end) + 8 }

#[inline] pub unsafe fn erofs_check_ondisk_layout_definitions() {
    assert!(core::mem::size_of::<erofs_super_block>() == 144);
    assert!(core::mem::size_of::<erofs_inode_compact>() == 32);
    assert!(core::mem::size_of::<erofs_inode_extended>() == 64);
    assert!(core::mem::size_of::<erofs_xattr_ibody_header>() == 12);
    assert!(core::mem::size_of::<erofs_xattr_entry>() == 4);
    assert!(core::mem::size_of::<erofs_inode_chunk_info>() == 4);
    assert!(core::mem::size_of::<erofs_inode_chunk_index>() == 8);
    assert!(core::mem::size_of::<z_erofs_map_header>() == 8);
    assert!(core::mem::size_of::<z_erofs_lcluster_index>() == 8);
    assert!(core::mem::size_of::<erofs_dirent>() == 12);
    assert!(core::mem::size_of::<erofs_inode_chunk_index>() == core::mem::size_of::<z_erofs_lcluster_index>());
    assert!(core::mem::size_of::<erofs_deviceslot>() == 128);
}
#[inline] pub const fn EROFS_XATTR_ALIGN(size: u32) -> u32 { (size + core::mem::size_of::<erofs_xattr_entry>() as u32 - 1) & !(core::mem::size_of::<erofs_xattr_entry>() as u32 - 1) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
