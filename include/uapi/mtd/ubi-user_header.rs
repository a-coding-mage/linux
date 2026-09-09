/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Translation of the Linux UBI userspace ABI header. */

/* The __s* and __u* integer types and ioctl helpers are supplied externally. */

pub const UBI_VOL_NUM_AUTO: i32 = -1;
pub const UBI_DEV_NUM_AUTO: i32 = -1;
pub const UBI_MAX_VOLUME_NAME: usize = 127;

pub const UBI_IOC_MAGIC: u8 = b'o';
pub const UBI_IOCMKVOL: _ = _IOW(UBI_IOC_MAGIC, 0, ubi_mkvol_req);
pub const UBI_IOCRMVOL: _ = _IOW(UBI_IOC_MAGIC, 1, __s32);
pub const UBI_IOCRSVOL: _ = _IOW(UBI_IOC_MAGIC, 2, ubi_rsvol_req);
pub const UBI_IOCRNVOL: _ = _IOW(UBI_IOC_MAGIC, 3, ubi_rnvol_req);
pub const UBI_IOCRPEB: _ = _IOW(UBI_IOC_MAGIC, 4, __s32);
pub const UBI_IOCSPEB: _ = _IOW(UBI_IOC_MAGIC, 5, __s32);
pub const UBI_IOCECNFO: _ = _IOWR(UBI_IOC_MAGIC, 6, ubi_ecinfo_req);

pub const UBI_CTRL_IOC_MAGIC: u8 = b'o';
pub const UBI_IOCATT: _ = _IOW(UBI_CTRL_IOC_MAGIC, 64, ubi_attach_req);
pub const UBI_IOCDET: _ = _IOW(UBI_CTRL_IOC_MAGIC, 65, __s32);

pub const UBI_VOL_IOC_MAGIC: u8 = b'O';
pub const UBI_IOCVOLUP: _ = _IOW(UBI_VOL_IOC_MAGIC, 0, __s64);
pub const UBI_IOCEBER: _ = _IOW(UBI_VOL_IOC_MAGIC, 1, __s32);
pub const UBI_IOCEBCH: _ = _IOW(UBI_VOL_IOC_MAGIC, 2, __s32);
pub const UBI_IOCEBMAP: _ = _IOW(UBI_VOL_IOC_MAGIC, 3, ubi_map_req);
pub const UBI_IOCEBUNMAP: _ = _IOW(UBI_VOL_IOC_MAGIC, 4, __s32);
pub const UBI_IOCEBISMAP: _ = _IOR(UBI_VOL_IOC_MAGIC, 5, __s32);
pub const UBI_IOCSETVOLPROP: _ = _IOW(UBI_VOL_IOC_MAGIC, 6, ubi_set_vol_prop_req);
pub const UBI_IOCVOLCRBLK: _ = _IOW(UBI_VOL_IOC_MAGIC, 7, ubi_blkcreate_req);
pub const UBI_IOCVOLRMBLK: _ = _IO(UBI_VOL_IOC_MAGIC, 8);

pub const MAX_UBI_MTD_NAME_LEN: usize = 127;
pub const UBI_MAX_RNVOL: usize = 32;
pub const UBI_DYNAMIC_VOLUME: i32 = 3;
pub const UBI_STATIC_VOLUME: i32 = 4;
pub const UBI_VOL_PROP_DIRECT_WRITE: i32 = 1;
pub const UBI_VOL_SKIP_CRC_CHECK_FLG: u32 = 0x1;
pub const UBI_VOL_VALID_FLGS: u32 = UBI_VOL_SKIP_CRC_CHECK_FLG;

#[repr(C)]
pub struct ubi_attach_req {
    pub ubi_num: __s32,
    pub mtd_num: __s32,
    pub vid_hdr_offset: __s32,
    pub max_beb_per1024: __s16,
    pub disable_fm: __s8,
    pub need_resv_pool: __s8,
    pub wl_threshold: __s32,
    pub padding: [__s8; 4],
}

#[repr(C, packed)]
pub struct ubi_mkvol_req {
    pub vol_id: __s32,
    pub alignment: __s32,
    pub bytes: __s64,
    pub vol_type: __s8,
    pub flags: __u8,
    pub name_len: __s16,
    pub padding2: [__s8; 4],
    pub name: [core::ffi::c_char; UBI_MAX_VOLUME_NAME + 1],
}

#[repr(C, packed)]
pub struct ubi_rsvol_req {
    pub bytes: __s64,
    pub vol_id: __s32,
}

#[repr(C, packed)]
pub struct ubi_rnvol_req {
    pub count: __s32,
    pub padding1: [__s8; 12],
    pub ents: [ubi_rnvol_ent; UBI_MAX_RNVOL],
}

#[repr(C, packed)]
pub struct ubi_rnvol_ent {
    pub vol_id: __s32,
    pub name_len: __s16,
    pub padding2: [__s8; 2],
    pub name: [core::ffi::c_char; UBI_MAX_VOLUME_NAME + 1],
}

#[repr(C, packed)]
pub struct ubi_ecinfo_req {
    pub start: __s32,
    pub length: __s32,
    pub read_length: __s32,
    pub padding: [__s8; 16],
    pub erase_counters: [__s32; 0],
}

#[repr(C, packed)]
pub struct ubi_leb_change_req {
    pub lnum: __s32,
    pub bytes: __s32,
    pub dtype: __s8, /* obsolete, do not use! */
    pub padding: [__s8; 7],
}

#[repr(C, packed)]
pub struct ubi_map_req {
    pub lnum: __s32,
    pub dtype: __s8, /* obsolete, do not use! */
    pub padding: [__s8; 3],
}

#[repr(C, packed)]
pub struct ubi_set_vol_prop_req {
    pub property: __u8,
    pub padding: [__u8; 7],
    pub value: __u64,
}

#[repr(C, packed)]
pub struct ubi_blkcreate_req {
    pub padding: [__s8; 128],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
