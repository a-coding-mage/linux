/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/fs/hpfs/hpfs.h. */

pub type secno = u32;
pub type dnode_secno = secno;
pub type fnode_secno = secno;
pub type anode_secno = secno;
pub type time32_t = u32;

pub const BB_MAGIC: u32 = 0xaa55;
pub const SB_MAGIC: u32 = 0xf995e849;
pub const SP_MAGIC: u32 = 0xf9911849;
pub const BAD_MAGIC: u32 = 0;
pub const CP_DIR_MAGIC: u32 = 0x494521f7;
pub const CP_DATA_MAGIC: u32 = 0x894521f7;
pub const DNODE_MAGIC: u32 = 0x77e40aae;
pub const FNODE_MAGIC: u32 = 0xf7e40aae;
pub const ANODE_MAGIC: u32 = 0x37e40aae;

#[repr(C)] pub struct hpfs_boot_block { pub jmp:[u8;3], pub oem_id:[u8;8], pub bytes_per_sector:[u8;2], pub sectors_per_cluster:u8, pub n_reserved_sectors:[u8;2], pub n_fats:u8, pub n_rootdir_entries:[u8;2], pub n_sectors_s:[u8;2], pub media_byte:u8, pub sectors_per_fat:__le16, pub sectors_per_track:__le16, pub heads_per_cyl:__le16, pub n_hidden_sectors:__le32, pub n_sectors_l:__le32, pub drive_number:u8, pub mbz:u8, pub sig_28h:u8, pub vol_serno:[u8;4], pub vol_label:[u8;11], pub sig_hpfs:[u8;8], pub pad:[u8;448], pub magic:__le16 }

#[repr(C)] pub struct hpfs_super_block { pub magic:__le32, pub magic1:__le32, pub version:u8, pub funcversion:u8, pub zero:__le16, pub root:__le32, pub n_sectors:__le32, pub n_badblocks:__le32, pub bitmaps:__le32, pub zero1:__le32, pub badblocks:__le32, pub zero3:__le32, pub last_chkdsk:__le32, pub last_optimize:__le32, pub n_dir_band:__le32, pub dir_band_start:__le32, pub dir_band_end:__le32, pub dir_band_bitmap:__le32, pub volume_name:[u8;32], pub user_id_table:__le32, pub zero6:[u32;103] }

#[repr(C)] pub struct hpfs_spare_block { pub magic:__le32, pub magic1:__le32, pub flags:u8, pub dasd_flags:u8, pub mm_contlgulty:u8, pub unused:u8, pub hotfix_map:__le32, pub n_spares_used:__le32, pub n_spares:__le32, pub n_dnode_spares_free:__le32, pub n_dnode_spares:__le32, pub code_page_dir:__le32, pub n_code_pages:__le32, pub super_crc:__le32, pub spare_crc:__le32, pub zero1:[__le32;15], pub spare_dnodes:[__le32;100], pub zero2:[__le32;1] }

#[repr(C)] pub struct code_page_directory { pub magic:__le32, pub n_code_pages:__le32, pub zero1:[__le32;2], pub array:[code_page_directory_entry;31] }
#[repr(C)] pub struct code_page_directory_entry { pub ix:__le16, pub code_page_number:__le16, pub bounds:__le32, pub code_page_data:__le32, pub index:__le16, pub unknown:__le16 }
#[repr(C)] pub struct code_page_data { pub magic:__le32, pub n_used:__le32, pub bounds:[__le32;3], pub offs:[__le16;3], pub code_page:[code_page_entry;3], pub incognita:[u8;78] }
#[repr(C)] pub struct code_page_entry { pub ix:__le16, pub code_page_number:__le16, pub unknown:__le16, pub map:[u8;128], pub zero2:__le16 }

#[repr(C)] pub struct dnode { pub magic:__le32, pub first_free:__le32, pub flags:u8, pub increment_me2:[u8;3], pub up:__le32, pub self_:__le32, pub dirent:[u8;2028] }
#[repr(C)] pub struct hpfs_dirent { pub length:__le16, pub flags:u8, pub attributes:u8, pub fnode:__le32, pub write_date:__le32, pub file_size:__le32, pub read_date:__le32, pub creation_date:__le32, pub ea_size:__le32, pub no_of_acls:u8, pub ix:u8, pub namelen:u8, pub name:[u8;0] }

#[repr(C)] pub struct bplus_leaf_node { pub file_secno:__le32, pub length:__le32, pub disk_secno:__le32 }
#[repr(C)] pub struct bplus_internal_node { pub file_secno:__le32, pub down:__le32 }
pub const BP_hbff:u8=1; pub const BP_fnode_parent:u8=0x20; pub const BP_binary_search:u8=0x40; pub const BP_internal:u8=0x80;
#[repr(C)] pub struct bplus_header_fixed { pub flags:u8, pub fill:[u8;3], pub n_free_nodes:u8, pub n_used_nodes:u8, pub first_free:__le16 }
#[repr(C)] pub union bplus_header_union { pub internal:[bplus_internal_node;0], pub external:[bplus_leaf_node;0] }
#[repr(C)] pub struct bplus_header { pub __hdr:bplus_header_fixed, pub u:bplus_header_union }
pub unsafe fn bp_internal(bp:*mut bplus_header)->bool { (*bp).__hdr.flags & BP_internal != 0 }
pub unsafe fn bp_fnode_parent(bp:*mut bplus_header)->bool { (*bp).__hdr.flags & BP_fnode_parent != 0 }

pub const FNODE_anode:__le16 = cpu_to_le16(2); pub const FNODE_dir:__le16 = cpu_to_le16(256);
#[repr(C)] pub union fnode_union { pub external:[bplus_leaf_node;8], pub internal:[bplus_internal_node;12] }
#[repr(C)] pub struct fnode { pub magic:__le32, pub zero1:[__le32;2], pub len:u8, pub name:[u8;15], pub up:__le32, pub acl_size_l:__le32, pub acl_secno:__le32, pub acl_size_s:__le16, pub acl_anode:u8, pub zero2:u8, pub ea_size_l:__le32, pub ea_secno:__le32, pub ea_size_s:__le16, pub flags:__le16, pub btree:bplus_header_fixed, pub u:fnode_union, pub file_size:__le32, pub n_needea:__le32, pub user_id:[u8;16], pub ea_offs:__le16, pub dasd_limit_treshhold:u8, pub dasd_limit_delta:u8, pub dasd_limit:__le32, pub dasd_usage:__le32, pub ea:[u8;316] }
pub unsafe fn fnode_in_anode(p:*mut fnode)->bool { ((*p).flags & FNODE_anode) != 0 }
pub unsafe fn fnode_is_dir(p:*mut fnode)->bool { ((*p).flags & FNODE_dir) != 0 }

#[repr(C)] pub struct anode { pub magic:__le32, pub self_:__le32, pub up:__le32, pub btree:bplus_header_fixed, pub u:anode_union, pub fill:[__le32;3] }
#[repr(C)] pub union anode_union { pub external:[bplus_leaf_node;40], pub internal:[bplus_internal_node;60] }
pub const EA_indirect:u8=1; pub const EA_anode:u8=2; pub const EA_needea:u8=128;
#[repr(C)] pub struct extended_attribute { pub flags:u8, pub namelen:u8, pub valuelen_lo:u8, pub valuelen_hi:u8, pub name:[u8;0] }
pub unsafe fn ea_indirect(ea:*mut extended_attribute)->bool { (*ea).flags & EA_indirect != 0 }
pub unsafe fn ea_in_anode(ea:*mut extended_attribute)->bool { (*ea).flags & EA_anode != 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
