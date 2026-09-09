/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* The MS-DOS filesystem constants/structures */

pub const SECTOR_SIZE: usize = 512;
pub const SECTOR_BITS: u32 = 9;
pub const MSDOS_DPB: usize = MSDOS_DPS;
pub const MSDOS_DPB_BITS: u32 = 4;
pub const MSDOS_DPS: usize = SECTOR_SIZE / core::mem::size_of::<msdos_dir_entry>();
pub const MSDOS_DPS_BITS: u32 = 4;
pub const MSDOS_LONGNAME: usize = 256;

pub const MSDOS_ROOT_INO: i32 = 1;
pub const MSDOS_FSINFO_INO: i32 = 2;
pub const MSDOS_DIR_BITS: u32 = 5;
pub const FAT_MAX_DIR_ENTRIES: usize = 65536;
pub const FAT_MAX_DIR_SIZE: usize = FAT_MAX_DIR_ENTRIES << MSDOS_DIR_BITS;

pub const ATTR_NONE: u8 = 0;
pub const ATTR_RO: u8 = 1;
pub const ATTR_HIDDEN: u8 = 2;
pub const ATTR_SYS: u8 = 4;
pub const ATTR_VOLUME: u8 = 8;
pub const ATTR_DIR: u8 = 16;
pub const ATTR_ARCH: u8 = 32;
pub const ATTR_UNUSED: u8 = ATTR_VOLUME | ATTR_ARCH | ATTR_SYS | ATTR_HIDDEN;
pub const ATTR_EXT: u8 = ATTR_RO | ATTR_HIDDEN | ATTR_SYS | ATTR_VOLUME;
pub const CASE_LOWER_BASE: u8 = 8;
pub const CASE_LOWER_EXT: u8 = 16;
pub const DELETED_FLAG: u8 = 0xe5;
pub const FAT_LFN_LEN: usize = 255;
pub const MSDOS_NAME: usize = 11;
pub const MSDOS_SLOTS: usize = 21;
pub const MSDOS_DOT: &str = ".          ";
pub const MSDOS_DOTDOT: &str = "..         ";
pub const FAT_START_ENT: u32 = 2;
pub const MAX_FAT12: u32 = 0xff4;
pub const MAX_FAT16: u32 = 0xfff4;
pub const MAX_FAT32: u32 = 0x0ffffff6;
pub const BAD_FAT12: u32 = 0xff7;
pub const BAD_FAT16: u32 = 0xfff7;
pub const BAD_FAT32: u32 = 0x0ffffff7;
pub const EOF_FAT12: u32 = 0xfff;
pub const EOF_FAT16: u32 = 0xffff;
pub const EOF_FAT32: u32 = 0x0fffffff;
pub const FAT_ENT_FREE: u32 = 0;
pub const FAT_ENT_BAD: u32 = BAD_FAT32;
pub const FAT_ENT_EOF: u32 = EOF_FAT32;
pub const FAT_FSINFO_SIG1: u32 = 0x41615252;
pub const FAT_FSINFO_SIG2: u32 = 0x61417272;
pub const FAT_STATE_DIRTY: u8 = 0x01;

#[inline]
pub fn cf_le_w(v: __le16) -> __u16 { le16_to_cpu(v) }
#[inline]
pub fn cf_le_l(v: __le32) -> __u32 { le32_to_cpu(v) }
#[inline]
pub fn ct_le_w(v: __u16) -> __le16 { cpu_to_le16(v) }
#[inline]
pub fn ct_le_l(v: __u32) -> __le32 { cpu_to_le32(v) }

#[inline]
pub unsafe fn is_fsinfo(x: *const fat_boot_fsinfo) -> bool {
    le32_to_cpu((*x).signature1) == FAT_FSINFO_SIG1 &&
    le32_to_cpu((*x).signature2) == FAT_FSINFO_SIG2
}

/* ioctl command constants; _IOR/_IOW and the integer aliases are supplied by
 * the platform headers translated alongside this file. */
pub const VFAT_IOCTL_READDIR_BOTH: u32 = _IOR('r', 1, [__fat_dirent; 2]);
pub const VFAT_IOCTL_READDIR_SHORT: u32 = _IOR('r', 2, [__fat_dirent; 2]);
pub const FAT_IOCTL_GET_ATTRIBUTES: u32 = _IOR('r', 0x10, __u32);
pub const FAT_IOCTL_SET_ATTRIBUTES: u32 = _IOW('r', 0x11, __u32);
pub const FAT_IOCTL_GET_VOLUME_ID: u32 = _IOR('r', 0x13, __u32);

#[inline]
pub unsafe fn is_free(n: *const u8) -> bool { *n == 0 || *n == DELETED_FLAG }

#[repr(C)]
pub struct __fat_dirent {
    pub d_ino: core::ffi::c_long,
    pub d_off: __kernel_off_t,
    pub d_reclen: u16,
    pub d_name: [core::ffi::c_char; 256],
}

#[repr(C)]
pub struct fat_boot_sector {
    pub ignored: [__u8; 3], pub system_id: [__u8; 8], pub sector_size: [__u8; 2],
    pub sec_per_clus: __u8, pub reserved: __le16, pub fats: __u8,
    pub dir_entries: [__u8; 2], pub sectors: [__u8; 2], pub media: __u8,
    pub fat_length: __le16, pub secs_track: __le16, pub heads: __le16,
    pub hidden: __le32, pub total_sect: __le32,
    pub ext: fat_boot_sector_ext,
}

#[repr(C)]
pub union fat_boot_sector_ext { pub fat16: fat_boot_sector_fat16, pub fat32: fat_boot_sector_fat32 }

#[repr(C)]
pub struct fat_boot_sector_fat16 {
    pub drive_number: __u8, pub state: __u8, pub signature: __u8, pub vol_id: [__u8; 4],
    pub vol_label: [__u8; MSDOS_NAME], pub fs_type: [__u8; 8],
}

#[repr(C)]
pub struct fat_boot_sector_fat32 {
    pub length: __le32, pub flags: __le16, pub version: [__u8; 2], pub root_cluster: __le32,
    pub info_sector: __le16, pub backup_boot: __le16, pub reserved2: [__le16; 6],
    pub drive_number: __u8, pub state: __u8, pub signature: __u8, pub vol_id: [__u8; 4],
    pub vol_label: [__u8; MSDOS_NAME], pub fs_type: [__u8; 8],
}

#[repr(C)]
pub struct fat_boot_fsinfo {
    pub signature1: __le32, pub reserved1: [__le32; 120], pub signature2: __le32,
    pub free_clusters: __le32, pub next_cluster: __le32, pub reserved2: [__le32; 4],
}

#[repr(C)]
pub struct msdos_dir_entry {
    pub name: [__u8; MSDOS_NAME], pub attr: __u8, pub lcase: __u8, pub ctime_cs: __u8,
    pub ctime: __le16, pub cdate: __le16, pub adate: __le16, pub starthi: __le16,
    pub time: __le16, pub date: __le16, pub start: __le16, pub size: __le32,
}

#[repr(C)]
pub struct msdos_dir_slot {
    pub id: __u8, pub name0_4: [__u8; 10], pub attr: __u8, pub reserved: __u8,
    pub alias_checksum: __u8, pub name5_10: [__u8; 12], pub start: __le16,
    pub name11_12: [__u8; 4],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
