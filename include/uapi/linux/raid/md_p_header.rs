/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Physical layout of Linux RAID devices. */

// Dependencies supplied by the surrounding kernel bindings.

pub const MD_RESERVED_BYTES: u64 = 64 * 1024;
pub const MD_RESERVED_SECTORS: u64 = MD_RESERVED_BYTES / 512;
pub const MD_SB_BYTES: u64 = 4096;
pub const MD_SB_WORDS: u64 = MD_SB_BYTES / 4;
pub const MD_SB_SECTORS: u64 = MD_SB_BYTES / 512;

pub const MD_SB_GENERIC_OFFSET: u32 = 0;
pub const MD_SB_PERSONALITY_OFFSET: u32 = 64;
pub const MD_SB_DISKS_OFFSET: u32 = 128;
pub const MD_SB_DESCRIPTOR_OFFSET: u32 = 992;
pub const MD_SB_GENERIC_CONSTANT_WORDS: usize = 32;
pub const MD_SB_GENERIC_STATE_WORDS: usize = 32;
pub const MD_SB_GENERIC_WORDS: usize = MD_SB_GENERIC_CONSTANT_WORDS + MD_SB_GENERIC_STATE_WORDS;
pub const MD_SB_PERSONALITY_WORDS: usize = 64;
pub const MD_SB_DESCRIPTOR_WORDS: usize = 32;
pub const MD_SB_DISKS: usize = 27;
pub const MD_SB_DISKS_WORDS: usize = MD_SB_DISKS * MD_SB_DESCRIPTOR_WORDS;
pub const MD_SB_RESERVED_WORDS: usize = 1024 - MD_SB_GENERIC_WORDS - MD_SB_PERSONALITY_WORDS - MD_SB_DISKS_WORDS - MD_SB_DESCRIPTOR_WORDS;
pub const MD_SB_EQUAL_WORDS: usize = MD_SB_GENERIC_WORDS + MD_SB_PERSONALITY_WORDS + MD_SB_DISKS_WORDS;

pub const MD_DISK_FAULTY: u32 = 0;
pub const MD_DISK_ACTIVE: u32 = 1;
pub const MD_DISK_SYNC: u32 = 2;
pub const MD_DISK_REMOVED: u32 = 3;
pub const MD_DISK_CLUSTER_ADD: u32 = 4;
pub const MD_DISK_CANDIDATE: u32 = 5;
pub const MD_DISK_FAILFAST: u32 = 10;
pub const MD_DISK_WRITEMOSTLY: u32 = 9;
pub const MD_DISK_JOURNAL: u32 = 18;
pub const MD_DISK_ROLE_SPARE: u32 = 0xffff;
pub const MD_DISK_ROLE_FAULTY: u32 = 0xfffe;
pub const MD_DISK_ROLE_JOURNAL: u32 = 0xfffd;
pub const MD_DISK_ROLE_MAX: u32 = 0xff00;

#[repr(C)]
pub struct mdp_disk_t {
    pub number: u32, pub major: u32, pub minor: u32, pub raid_disk: u32, pub state: u32,
    pub reserved: [u32; MD_SB_DESCRIPTOR_WORDS - 5],
}

pub const MD_SB_MAGIC: u32 = 0xa92b4efc;
pub const MD_SB_CLEAN: u32 = 0;
pub const MD_SB_ERRORS: u32 = 1;
pub const MD_SB_CLUSTERED: u32 = 5;
pub const MD_SB_BITMAP_PRESENT: u32 = 8;

#[repr(C)]
pub struct mdp_super_t {
    pub md_magic: u32, pub major_version: u32, pub minor_version: u32, pub patch_version: u32,
    pub gvalid_words: u32, pub set_uuid0: u32, pub ctime: u32, pub level: u32, pub size: u32,
    pub nr_disks: u32, pub raid_disks: u32, pub md_minor: u32, pub not_persistent: u32,
    pub set_uuid1: u32, pub set_uuid2: u32, pub set_uuid3: u32,
    pub gstate_creserved: [u32; MD_SB_GENERIC_CONSTANT_WORDS - 16],
    pub utime: u32, pub state: u32, pub active_disks: u32, pub working_disks: u32,
    pub failed_disks: u32, pub spare_disks: u32, pub sb_csum: u32,
    #[cfg(target_endian = "big")] pub events_hi: u32,
    #[cfg(target_endian = "big")] pub events_lo: u32,
    #[cfg(target_endian = "big")] pub cp_events_hi: u32,
    #[cfg(target_endian = "big")] pub cp_events_lo: u32,
    #[cfg(target_endian = "little")] pub events_lo: u32,
    #[cfg(target_endian = "little")] pub events_hi: u32,
    #[cfg(target_endian = "little")] pub cp_events_lo: u32,
    #[cfg(target_endian = "little")] pub cp_events_hi: u32,
    pub recovery_cp: u32, pub reshape_position: u64, pub new_level: u32, pub delta_disks: u32,
    pub new_layout: u32, pub new_chunk: u32,
    pub gstate_sreserved: [u32; MD_SB_GENERIC_STATE_WORDS - 18],
    pub layout: u32, pub chunk_size: u32, pub root_pv: u32, pub root_block: u32,
    pub pstate_reserved: [u32; MD_SB_PERSONALITY_WORDS - 4],
    pub disks: [mdp_disk_t; MD_SB_DISKS], pub reserved: [u32; MD_SB_RESERVED_WORDS],
    pub this_disk: mdp_disk_t,
}

#[inline]
pub unsafe fn md_event(sb: *mut mdp_super_t) -> u64 {
    let ev = (*sb).events_hi as u64;
    (ev << 32) | (*sb).events_lo as u64
}

pub const MD_SUPERBLOCK_1_TIME_SEC_MASK: u64 = (1u64 << 40) - 1;

#[repr(C)]
pub union mdp_superblock_1_bitmap {
    pub bitmap_offset: u32,
    pub ppl: mdp_superblock_1_ppl,
}
#[repr(C)] pub struct mdp_superblock_1_ppl { pub offset: u16, pub size: u16 }

#[repr(C)]
pub union mdp_superblock_1_recovery { pub recovery_offset: u64, pub journal_tail: u64 }

#[repr(C)]
pub struct mdp_superblock_1 {
    pub magic: u32, pub major_version: u32, pub feature_map: u32, pub pad0: u32,
    pub set_uuid: [u8; 16], pub set_name: [core::ffi::c_char; 32], pub ctime: u64,
    pub level: u32, pub layout: u32, pub size: u64, pub chunksize: u32, pub raid_disks: u32,
    pub bitmap: mdp_superblock_1_bitmap, pub new_level: u32, pub reshape_position: u64,
    pub delta_disks: u32, pub new_layout: u32, pub new_chunk: u32, pub new_offset: u32,
    pub data_offset: u64, pub data_size: u64, pub super_offset: u64,
    pub recovery: mdp_superblock_1_recovery, pub dev_number: u32, pub cnt_corrected_read: u32,
    pub device_uuid: [u8; 16], pub devflags: u8, pub bblog_shift: u8, pub bblog_size: u16,
    pub bblog_offset: u32, pub utime: u64, pub events: u64, pub resync_offset: u64,
    pub sb_csum: u32, pub max_dev: u32, pub logical_block_size: u32, pub pad3: [u8; 28],
    pub dev_roles: [u16; 0],
}

pub const WriteMostly1: u8 = 1;
pub const FailFast1: u8 = 2;
pub const MD_FEATURE_BITMAP_OFFSET: u32 = 1;
pub const MD_FEATURE_RECOVERY_OFFSET: u32 = 2;
pub const MD_FEATURE_RESHAPE_ACTIVE: u32 = 4;
pub const MD_FEATURE_BAD_BLOCKS: u32 = 8;
pub const MD_FEATURE_REPLACEMENT: u32 = 16;
pub const MD_FEATURE_RESHAPE_BACKWARDS: u32 = 32;
pub const MD_FEATURE_NEW_OFFSET: u32 = 64;
pub const MD_FEATURE_RECOVERY_BITMAP: u32 = 128;
pub const MD_FEATURE_CLUSTERED: u32 = 256;
pub const MD_FEATURE_JOURNAL: u32 = 512;
pub const MD_FEATURE_PPL: u32 = 1024;
pub const MD_FEATURE_MULTIPLE_PPLS: u32 = 2048;
pub const MD_FEATURE_RAID0_LAYOUT: u32 = 4096;
pub const MD_FEATURE_ALL: u32 = 8191;

#[repr(C, packed)] pub struct r5l_payload_header { pub type_: u16, pub flags: u16 }
pub const R5LOG_PAYLOAD_DATA: u32 = 0; pub const R5LOG_PAYLOAD_PARITY: u32 = 1; pub const R5LOG_PAYLOAD_FLUSH: u32 = 2;
#[repr(C, packed)] pub struct r5l_payload_data_parity { pub header: r5l_payload_header, pub size: u32, pub location: u64, pub checksum: [u32; 0] }
pub const R5LOG_PAYLOAD_FLAG_DISCARD: u32 = 1; pub const R5LOG_PAYLOAD_FLAG_RESHAPED: u32 = 2; pub const R5LOG_PAYLOAD_FLAG_RESHAPING: u32 = 3;
#[repr(C, packed)] pub struct r5l_payload_flush { pub header: r5l_payload_header, pub size: u32, pub flush_stripes: [u64; 0] }
pub const R5LOG_PAYLOAD_FLAG_FLUSH_STRIPE: u32 = 1;
#[repr(C, packed)] pub struct r5l_meta_block { pub magic: u32, pub checksum: u32, pub version: u8, pub __zero_pading_1: u8, pub __zero_pading_2: u16, pub meta_size: u32, pub seq: u64, pub position: u64, pub payloads: [r5l_payload_header; 0] }
pub const R5LOG_VERSION: u32 = 0x1; pub const R5LOG_MAGIC: u32 = 0x6433c509;
#[repr(C, packed)] pub struct ppl_header_entry { pub data_sector: u64, pub pp_size: u32, pub data_size: u32, pub parity_disk: u32, pub checksum: u32 }
pub const PPL_HEADER_SIZE: usize = 4096; pub const PPL_HDR_RESERVED: usize = 512; pub const PPL_HDR_ENTRY_SPACE: usize = PPL_HEADER_SIZE - PPL_HDR_RESERVED - 4 * 4 - 8; pub const PPL_HDR_MAX_ENTRIES: usize = PPL_HDR_ENTRY_SPACE / 24;
#[repr(C, packed)] pub struct ppl_header { pub reserved: [u8; PPL_HDR_RESERVED], pub signature: u32, pub padding: u32, pub generation: u64, pub entries_count: u32, pub checksum: u32, pub entries: [ppl_header_entry; PPL_HDR_MAX_ENTRIES] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
