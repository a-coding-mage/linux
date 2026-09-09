/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies: __u8 and inode/superblock types are supplied by other headers. */

/*
 * The isofs filesystem constants/structures
 */

/* This part borrowed from the bsd386 isofs */
macro_rules! ISODCL { ($from:expr, $to:expr) => { $to - $from + 1 }; }

#[repr(C)]
pub struct iso_volume_descriptor {
    pub r#type: [__u8; ISODCL!(1, 1)], /* 711 */
    pub id: [i8; ISODCL!(2, 6)],
    pub version: [__u8; ISODCL!(7, 7)],
    pub data: [__u8; ISODCL!(8, 2048)],
}

/* volume descriptor types */
pub const ISO_VD_PRIMARY: i32 = 1;
pub const ISO_VD_SUPPLEMENTARY: i32 = 2;
pub const ISO_VD_END: i32 = 255;

pub const ISO_STANDARD_ID: &[u8; 5] = b"CD001";

#[repr(C)]
pub struct iso_primary_descriptor {
    pub r#type: [__u8; ISODCL!(1, 1)], /* 711 */
    pub id: [i8; ISODCL!(2, 6)],
    pub version: [__u8; ISODCL!(7, 7)], /* 711 */
    pub unused1: [__u8; ISODCL!(8, 8)],
    pub system_id: [i8; ISODCL!(9, 40)], /* achars */
    pub volume_id: [i8; ISODCL!(41, 72)], /* dchars */
    pub unused2: [__u8; ISODCL!(73, 80)],
    pub volume_space_size: [__u8; ISODCL!(81, 88)], /* 733 */
    pub unused3: [__u8; ISODCL!(89, 120)],
    pub volume_set_size: [__u8; ISODCL!(121, 124)], /* 723 */
    pub volume_sequence_number: [__u8; ISODCL!(125, 128)], /* 723 */
    pub logical_block_size: [__u8; ISODCL!(129, 132)], /* 723 */
    pub path_table_size: [__u8; ISODCL!(133, 140)], /* 733 */
    pub type_l_path_table: [__u8; ISODCL!(141, 144)], /* 731 */
    pub opt_type_l_path_table: [__u8; ISODCL!(145, 148)], /* 731 */
    pub type_m_path_table: [__u8; ISODCL!(149, 152)], /* 732 */
    pub opt_type_m_path_table: [__u8; ISODCL!(153, 156)], /* 732 */
    pub root_directory_record: [__u8; ISODCL!(157, 190)], /* 9.1 */
    pub volume_set_id: [i8; ISODCL!(191, 318)], /* dchars */
    pub publisher_id: [i8; ISODCL!(319, 446)], /* achars */
    pub preparer_id: [i8; ISODCL!(447, 574)], /* achars */
    pub application_id: [i8; ISODCL!(575, 702)], /* achars */
    pub copyright_file_id: [i8; ISODCL!(703, 739)], /* 7.5 dchars */
    pub abstract_file_id: [i8; ISODCL!(740, 776)], /* 7.5 dchars */
    pub bibliographic_file_id: [i8; ISODCL!(777, 813)], /* 7.5 dchars */
    pub creation_date: [__u8; ISODCL!(814, 830)], /* 8.4.26.1 */
    pub modification_date: [__u8; ISODCL!(831, 847)], /* 8.4.26.1 */
    pub expiration_date: [__u8; ISODCL!(848, 864)], /* 8.4.26.1 */
    pub effective_date: [__u8; ISODCL!(865, 881)], /* 8.4.26.1 */
    pub file_structure_version: [__u8; ISODCL!(882, 882)], /* 711 */
    pub unused4: [__u8; ISODCL!(883, 883)],
    pub application_data: [__u8; ISODCL!(884, 1395)],
    pub unused5: [__u8; ISODCL!(1396, 2048)],
}

/* Almost the same as the primary descriptor but two fields are specified */
#[repr(C)]
pub struct iso_supplementary_descriptor {
    pub r#type: [__u8; 1], pub id: [i8; 5], pub version: [__u8; 1], pub flags: [__u8; 1],
    pub system_id: [i8; 32], pub volume_id: [i8; 32], pub unused2: [__u8; 8],
    pub volume_space_size: [__u8; 8], pub escape: [__u8; 32], pub volume_set_size: [__u8; 4],
    pub volume_sequence_number: [__u8; 4], pub logical_block_size: [__u8; 4], pub path_table_size: [__u8; 8],
    pub type_l_path_table: [__u8; 4], pub opt_type_l_path_table: [__u8; 4], pub type_m_path_table: [__u8; 4],
    pub opt_type_m_path_table: [__u8; 4], pub root_directory_record: [__u8; 34],
    pub volume_set_id: [i8; 128], pub publisher_id: [i8; 128], pub preparer_id: [i8; 128],
    pub application_id: [i8; 128], pub copyright_file_id: [i8; 37], pub abstract_file_id: [i8; 37],
    pub bibliographic_file_id: [i8; 37], pub creation_date: [__u8; 17], pub modification_date: [__u8; 17],
    pub expiration_date: [__u8; 17], pub effective_date: [__u8; 17], pub file_structure_version: [__u8; 1],
    pub unused4: [__u8; 1], pub application_data: [__u8; 512], pub unused5: [__u8; 653],
}

pub const HS_STANDARD_ID: &[u8; 5] = b"CDROM";

#[repr(C)]
pub struct hs_volume_descriptor { pub foo: [__u8; 8], pub r#type: [__u8; 1], pub id: [i8; 5], pub version: [__u8; 1], pub data: [__u8; 2033] }

#[repr(C)]
pub struct hs_primary_descriptor {
    pub foo: [__u8; 8], pub r#type: [__u8; 1], pub id: [__u8; 5], pub version: [__u8; 1], pub unused1: [__u8; 1],
    pub system_id: [i8; 32], pub volume_id: [i8; 32], pub unused2: [__u8; 8], pub volume_space_size: [__u8; 8],
    pub unused3: [__u8; 32], pub volume_set_size: [__u8; 4], pub volume_sequence_number: [__u8; 4],
    pub logical_block_size: [__u8; 4], pub path_table_size: [__u8; 8], pub type_l_path_table: [__u8; 4],
    pub unused4: [__u8; 28], pub root_directory_record: [__u8; 34],
}

/* We use this to help us look up the parent inode numbers. */
#[repr(C, packed)]
pub struct iso_path_table { pub name_len: [__u8; 2], pub extent: [__u8; 4], pub parent: [__u8; 2], pub name: [i8; 0] }

/* high sierra is identical to iso, except that the date is only 6 bytes, and
   there is an extra reserved byte after the flags */
#[repr(C, packed)]
pub struct iso_directory_record {
    pub length: [__u8; 1], pub ext_attr_length: [__u8; 1], pub extent: [__u8; 8], pub size: [__u8; 8],
    pub date: [__u8; 7], pub flags: [__u8; 1], pub file_unit_size: [__u8; 1], pub interleave: [__u8; 1],
    pub volume_sequence_number: [__u8; 4], pub name_len: [__u8; 1], pub name: [i8; 0],
}

pub const ISOFS_BLOCK_BITS: i32 = 11;
pub const ISOFS_BLOCK_SIZE: i32 = 2048;

/* These macros require the supplied inode/superblock definitions. */
macro_rules! ISOFS_BUFFER_SIZE { ($inode:expr) => { ($inode).i_sb.s_blocksize }; }
macro_rules! ISOFS_BUFFER_BITS { ($inode:expr) => { ($inode).i_sb.s_blocksize_bits }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
