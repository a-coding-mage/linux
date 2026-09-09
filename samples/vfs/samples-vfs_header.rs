/* SPDX-License-Identifier: GPL-2.0 */

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct statmount {
    pub size: u32,
    pub mnt_opts: u32,
    pub mask: u64,
    pub sb_dev_major: u32,
    pub sb_dev_minor: u32,
    pub sb_magic: u64,
    pub sb_flags: u32,
    pub fs_type: u32,
    pub mnt_id: u64,
    pub mnt_parent_id: u64,
    pub mnt_id_old: u32,
    pub mnt_parent_id_old: u32,
    pub mnt_attr: u64,
    pub mnt_propagation: u64,
    pub mnt_peer_group: u64,
    pub mnt_master: u64,
    pub propagate_from: u64,
    pub mnt_root: u32,
    pub mnt_point: u32,
    pub mnt_ns_id: u64,
    pub fs_subtype: u32,
    pub sb_source: u32,
    pub opt_num: u32,
    pub opt_array: u32,
    pub opt_sec_num: u32,
    pub opt_sec_array: u32,
    pub mnt_uidmap_num: u32,
    pub mnt_uidmap: u32,
    pub mnt_gidmap_num: u32,
    pub mnt_gidmap: u32,
    pub __spare2: [u64; 44],
    pub str_: [std::ffi::c_char; 0],
}

#[repr(C)]
pub struct mnt_id_req {
    pub size: u32,
    pub spare: u32,
    pub mnt_id: u64,
    pub param: u64,
    pub mnt_ns_id: u64,
}

pub const MNT_ID_REQ_SIZE_VER0: u32 = 24;
pub const MNT_ID_REQ_SIZE_VER1: u32 = 32;

const fn _io(ty: u32, nr: u32) -> u32 { (ty << 8) | nr }
const fn _ior(ty: u32, nr: u32, size: u32) -> u32 { (2 << 30) | (size << 16) | (ty << 8) | nr }

pub const NS_GET_MNTNS_ID: u32 = _io(0xb7, 0x5);

#[repr(C)]
pub struct mnt_ns_info {
    pub size: u32,
    pub nr_mounts: u32,
    pub mnt_ns_id: u64,
}

pub const MNT_NS_INFO_SIZE_VER0: u32 = 16;
pub const NS_MNT_GET_INFO: u32 = _ior(0xb7, 10, 16);
pub const NS_MNT_GET_NEXT: u32 = _ior(0xb7, 11, 16);
pub const NS_MNT_GET_PREV: u32 = _ior(0xb7, 12, 16);
pub const PIDFD_GET_MNT_NAMESPACE: u32 = _io(0xff, 3);

pub const __NR_listmount: i32 = 458;
pub const __NR_statmount: i32 = 457;
pub const LSMT_ROOT: u64 = 0xffff_ffff_ffff_ffff;

pub const STATMOUNT_SB_BASIC: u32 = 0x0000_0001;
pub const STATMOUNT_MNT_BASIC: u32 = 0x0000_0002;
pub const STATMOUNT_PROPAGATE_FROM: u32 = 0x0000_0004;
pub const STATMOUNT_MNT_ROOT: u32 = 0x0000_0008;
pub const STATMOUNT_MNT_POINT: u32 = 0x0000_0010;
pub const STATMOUNT_FS_TYPE: u32 = 0x0000_0020;
pub const STATMOUNT_MNT_NS_ID: u32 = 0x0000_0040;
pub const STATMOUNT_MNT_OPTS: u32 = 0x0000_0080;
pub const STATMOUNT_FS_SUBTYPE: u32 = 0x0000_0100;
pub const STATMOUNT_SB_SOURCE: u32 = 0x0000_0200;
pub const STATMOUNT_OPT_ARRAY: u32 = 0x0000_0400;
pub const STATMOUNT_OPT_SEC_ARRAY: u32 = 0x0000_0800;
pub const STATX_MNT_ID_UNIQUE: u32 = 0x0000_4000;
pub const STATMOUNT_MNT_UIDMAP: u32 = 0x0000_2000;
pub const STATMOUNT_MNT_GIDMAP: u32 = 0x0000_4000;

pub const MOUNT_ATTR_RDONLY: u64 = 0x0000_0001;
pub const MOUNT_ATTR_NOSUID: u64 = 0x0000_0002;
pub const MOUNT_ATTR_NODEV: u64 = 0x0000_0004;
pub const MOUNT_ATTR_NOEXEC: u64 = 0x0000_0008;
pub const MOUNT_ATTR__ATIME: u64 = 0x0000_0070;
pub const MOUNT_ATTR_RELATIME: u64 = 0x0000_0000;
pub const MOUNT_ATTR_NOATIME: u64 = 0x0000_0010;
pub const MOUNT_ATTR_STRICTATIME: u64 = 0x0000_0020;
pub const MOUNT_ATTR_NODIRATIME: u64 = 0x0000_0080;
pub const MOUNT_ATTR_IDMAP: u64 = 0x0010_0000;
pub const MOUNT_ATTR_NOSYMFOLLOW: u64 = 0x0020_0000;

pub const MS_RDONLY: u64 = 1;
pub const MS_SYNCHRONOUS: u64 = 16;
pub const MS_MANDLOCK: u64 = 64;
pub const MS_DIRSYNC: u64 = 128;
pub const MS_UNBINDABLE: u64 = 1 << 17;
pub const MS_PRIVATE: u64 = 1 << 18;
pub const MS_SLAVE: u64 = 1 << 19;
pub const MS_SHARED: u64 = 1 << 20;
pub const MS_LAZYTIME: u64 = 1 << 25;

#[macro_export]
macro_rules! die_errno {
    ($($arg:tt)*) => {{
        eprintln!("{}:{}: {}: {}", file!(), line!(), module_path!(), format_args!($($arg)*));
        std::process::exit(1);
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
