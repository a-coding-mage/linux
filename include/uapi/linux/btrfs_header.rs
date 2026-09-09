/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Translated from linux/uapi/linux/btrfs.h.  ioctl encoding helpers and
// externally supplied Linux types are intentionally referenced symbolically.
pub const BTRFS_IOCTL_MAGIC: u32 = 0x94;
pub const BTRFS_VOL_NAME_MAX: usize = 255;
pub const BTRFS_LABEL_SIZE: usize = 256;
pub const BTRFS_PATH_NAME_MAX: usize = 4087;
pub const BTRFS_DEVICE_PATH_NAME_MAX: usize = 1024;
pub const BTRFS_SUBVOL_NAME_MAX: usize = 4039;
pub const BTRFS_FSID_SIZE: usize = 16;
pub const BTRFS_UUID_SIZE: usize = 16;
pub const BTRFS_UUID_UNPARSED_SIZE: usize = 37;

pub const BTRFS_SUBVOL_CREATE_ASYNC: u64 = 1 << 0;
pub const BTRFS_SUBVOL_RDONLY: u64 = 1 << 1;
pub const BTRFS_SUBVOL_QGROUP_INHERIT: u64 = 1 << 2;
pub const BTRFS_DEVICE_SPEC_BY_ID: u64 = 1 << 3;
pub const BTRFS_SUBVOL_SPEC_BY_ID: u64 = 1 << 4;
pub const BTRFS_VOL_ARG_V2_FLAGS_SUPPORTED: u64 = BTRFS_SUBVOL_RDONLY | BTRFS_SUBVOL_QGROUP_INHERIT | BTRFS_DEVICE_SPEC_BY_ID | BTRFS_SUBVOL_SPEC_BY_ID;

pub const BTRFS_QGROUP_LIMIT_MAX_RFER:u64=1<<0; pub const BTRFS_QGROUP_LIMIT_MAX_EXCL:u64=1<<1;
pub const BTRFS_QGROUP_LIMIT_RSV_RFER:u64=1<<2; pub const BTRFS_QGROUP_LIMIT_RSV_EXCL:u64=1<<3;
pub const BTRFS_QGROUP_LIMIT_RFER_CMPR:u64=1<<4; pub const BTRFS_QGROUP_LIMIT_EXCL_CMPR:u64=1<<5;
#[repr(C)] #[derive(Copy,Clone)] pub struct btrfs_qgroup_limit { pub flags:u64,pub max_rfer:u64,pub max_excl:u64,pub rsv_rfer:u64,pub rsv_excl:u64 }
pub const BTRFS_QGROUP_INHERIT_SET_LIMITS:u64=1; pub const BTRFS_QGROUP_INHERIT_FLAGS_SUPP:u64=BTRFS_QGROUP_INHERIT_SET_LIMITS;
#[repr(C)] pub struct btrfs_qgroup_inherit { pub flags:u64,pub num_qgroups:u64,pub num_ref_copies:u64,pub num_excl_copies:u64,pub lim:btrfs_qgroup_limit,pub qgroups:[u64;0] }
#[repr(C)] #[derive(Copy,Clone)] pub struct btrfs_ioctl_qgroup_limit_args { pub qgroupid:u64,pub lim:btrfs_qgroup_limit }
#[repr(C)] pub struct btrfs_ioctl_vol_args { pub fd:i64,pub name:[u8;BTRFS_PATH_NAME_MAX+1] }
#[repr(C)] pub union btrfs_ioctl_vol_args_v2__bindgen_ty_1 { pub size:u64,pub unused:[u64;4] }
#[repr(C)] pub union btrfs_ioctl_vol_args_v2__bindgen_ty_2 { pub name:[u8;BTRFS_SUBVOL_NAME_MAX+1],pub devid:u64,pub subvolid:u64 }
#[repr(C)] pub struct btrfs_ioctl_vol_args_v2 { pub fd:i64,pub transid:u64,pub flags:u64,pub x:btrfs_ioctl_vol_args_v2__bindgen_ty_1,pub y:btrfs_ioctl_vol_args_v2__bindgen_ty_2 }

#[repr(C)] #[derive(Copy,Clone)] pub struct btrfs_scrub_progress { pub data_extents_scrubbed:u64,pub tree_extents_scrubbed:u64,pub data_bytes_scrubbed:u64,pub tree_bytes_scrubbed:u64,pub read_errors:u64,pub csum_errors:u64,pub verify_errors:u64,pub no_csum:u64,pub csum_discards:u64,pub super_errors:u64,pub malloc_errors:u64,pub uncorrectable_errors:u64,pub corrected_errors:u64,pub last_physical:u64,pub unverified_errors:u64 }
pub const BTRFS_SCRUB_READONLY:u64=1; pub const BTRFS_SCRUB_SUPPORTED_FLAGS:u64=BTRFS_SCRUB_READONLY;
#[repr(C)] pub struct btrfs_ioctl_scrub_args { pub devid:u64,pub start:u64,pub end:u64,pub flags:u64,pub progress:btrfs_scrub_progress,pub unused:[u64;((1024-32-120)/8)] }
#[repr(C)] pub struct btrfs_ioctl_dev_replace_start_params { pub srcdevid:u64,pub cont_reading_from_srcdev_mode:u64,pub srcdev_name:[u8;BTRFS_DEVICE_PATH_NAME_MAX+1],pub tgtdev_name:[u8;BTRFS_DEVICE_PATH_NAME_MAX+1] }
#[repr(C)] pub struct btrfs_ioctl_dev_replace_status_params { pub replace_state:u64,pub progress_1000:u64,pub time_started:u64,pub time_stopped:u64,pub num_write_errors:u64,pub num_uncorrectable_read_errors:u64 }
#[repr(C)] pub union btrfs_ioctl_dev_replace_args__bindgen_ty_1 { pub start:btrfs_ioctl_dev_replace_start_params,pub status:btrfs_ioctl_dev_replace_status_params }
#[repr(C)] pub struct btrfs_ioctl_dev_replace_args { pub cmd:u64,pub result:u64,pub x:btrfs_ioctl_dev_replace_args__bindgen_ty_1,pub spare:[u64;64] }
pub const BTRFS_IOCTL_DEV_REPLACE_CMD_START:u64=0; pub const BTRFS_IOCTL_DEV_REPLACE_CMD_STATUS:u64=1; pub const BTRFS_IOCTL_DEV_REPLACE_CMD_CANCEL:u64=2;
pub const BTRFS_IOCTL_DEV_REPLACE_STATE_NEVER_STARTED:u64=0; pub const BTRFS_IOCTL_DEV_REPLACE_STATE_STARTED:u64=1; pub const BTRFS_IOCTL_DEV_REPLACE_STATE_FINISHED:u64=2; pub const BTRFS_IOCTL_DEV_REPLACE_STATE_CANCELED:u64=3; pub const BTRFS_IOCTL_DEV_REPLACE_STATE_SUSPENDED:u64=4;

#[repr(C)] pub struct btrfs_ioctl_dev_info_args { pub devid:u64,pub uuid:[u8;16],pub bytes_used:u64,pub total_bytes:u64,pub fsid:[u8;16],pub unused:[u64;377],pub path:[u8;1024] }
#[repr(C)] pub struct btrfs_ioctl_fs_info_args { pub max_id:u64,pub num_devices:u64,pub fsid:[u8;16],pub nodesize:u32,pub sectorsize:u32,pub clone_alignment:u32,pub csum_type:u16,pub csum_size:u16,pub flags:u64,pub generation:u64,pub metadata_uuid:[u8;16],pub reserved:[u8;944] }
#[repr(C)] #[derive(Copy,Clone)] pub struct btrfs_ioctl_feature_flags { pub compat_flags:u64,pub compat_ro_flags:u64,pub incompat_flags:u64 }
pub const BTRFS_FS_INFO_FLAG_CSUM_INFO:u64=1; pub const BTRFS_FS_INFO_FLAG_GENERATION:u64=2; pub const BTRFS_FS_INFO_FLAG_METADATA_UUID:u64=4;
pub const BTRFS_BALANCE_CTL_PAUSE:u64=1; pub const BTRFS_BALANCE_CTL_CANCEL:u64=2;
#[repr(C,packed)] pub struct btrfs_balance_args { pub profiles:u64,pub usage:u64,pub devid:u64,pub pstart:u64,pub pend:u64,pub vstart:u64,pub vend:u64,pub target:u64,pub flags:u64,pub limit:u64,pub stripes_min:u32,pub stripes_max:u32,pub unused:[u64;6] }
#[repr(C)] pub struct btrfs_balance_progress { pub expected:u64,pub considered:u64,pub completed:u64 }
#[repr(C)] pub struct btrfs_ioctl_balance_args { pub flags:u64,pub state:u64,pub data:btrfs_balance_args,pub meta:btrfs_balance_args,pub sys:btrfs_balance_args,pub stat:btrfs_balance_progress,pub unused:[u64;72] }
pub const BTRFS_BALANCE_DATA:u64=1;pub const BTRFS_BALANCE_SYSTEM:u64=2;pub const BTRFS_BALANCE_METADATA:u64=4;pub const BTRFS_BALANCE_FORCE:u64=8;pub const BTRFS_BALANCE_RESUME:u64=16;
pub const BTRFS_BALANCE_STATE_RUNNING:u64=1;pub const BTRFS_BALANCE_STATE_PAUSE_REQ:u64=2;pub const BTRFS_BALANCE_STATE_CANCEL_REQ:u64=4;
#[repr(C)] pub struct btrfs_ioctl_ino_lookup_args { pub treeid:u64,pub objectid:u64,pub name:[u8;4080] }
#[repr(C)] pub struct btrfs_ioctl_clone_range_args { pub src_fd:i64,pub src_offset:u64,pub src_length:u64,pub dest_offset:u64 }
#[repr(C)] pub struct btrfs_ioctl_timespec { pub sec:u64,pub nsec:u32 }
#[repr(C)] pub struct btrfs_ioctl_received_subvol_args { pub uuid:[u8;16],pub stransid:u64,pub rtransid:u64,pub stime:btrfs_ioctl_timespec,pub rtime:btrfs_ioctl_timespec,pub flags:u64,pub reserved:[u64;16] }
#[repr(C)] pub struct btrfs_ioctl_send_args { pub send_fd:i64,pub clone_sources_count:u64,pub clone_sources:*mut u64,pub parent_root:u64,pub flags:u64,pub version:u32,pub reserved:[u8;28] }
pub const BTRFS_SEND_FLAG_NO_FILE_DATA:u64=1;pub const BTRFS_SEND_FLAG_OMIT_STREAM_HEADER:u64=2;pub const BTRFS_SEND_FLAG_OMIT_END_CMD:u64=4;pub const BTRFS_SEND_FLAG_VERSION:u64=8;pub const BTRFS_SEND_FLAG_COMPRESSED:u64=16;
#[repr(C)] pub struct btrfs_ioctl_subvol_wait { pub subvolid:u64,pub mode:u32,pub count:u32 }
pub const BTRFS_SUBVOL_SYNC_WAIT_FOR_ONE:u32=0;pub const BTRFS_SUBVOL_SYNC_WAIT_FOR_QUEUED:u32=1;pub const BTRFS_SUBVOL_SYNC_COUNT:u32=2;pub const BTRFS_SUBVOL_SYNC_PEEK_FIRST:u32=3;pub const BTRFS_SUBVOL_SYNC_PEEK_LAST:u32=4;
#[repr(C)] pub struct btrfs_ioctl_get_csums_entry { pub offset:u64,pub length:u64,pub r#type:u32,pub reserved:u32 }
#[repr(C)] pub struct btrfs_ioctl_get_csums_args { pub offset:u64,pub length:u64,pub buf_size:u64,pub flags:u64,pub buf:[u8;0] }
pub const BTRFS_SHUTDOWN_FLAGS_DEFAULT:u32=0;pub const BTRFS_SHUTDOWN_FLAGS_LOGFLUSH:u32=1;pub const BTRFS_SHUTDOWN_FLAGS_NOLOGFLUSH:u32=2;pub const BTRFS_SHUTDOWN_FLAGS_LAST:u32=3;
// The remaining ioctl encodings depend on Linux's _IO* macros and external
// fs/ioctl definitions; preserve their public names for downstream bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
