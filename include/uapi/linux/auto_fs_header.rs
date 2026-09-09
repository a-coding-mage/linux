/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Translated from the Linux UAPI auto_fs.h header. */

pub const AUTOFS_PROTO_VERSION: i32 = 5;
pub const AUTOFS_MIN_PROTO_VERSION: i32 = 3;
pub const AUTOFS_MAX_PROTO_VERSION: i32 = 5;
pub const AUTOFS_PROTO_SUBVERSION: i32 = 6;

/* On pure 64-bit architectures this is unsigned long; elsewhere unsigned int. */
pub type autofs_wqt_t = ::core::ffi::c_uint;

pub const autofs_ptype_missing: i32 = 0;
pub const autofs_ptype_expire: i32 = 1;

#[repr(C)]
pub struct autofs_packet_hdr {
    pub proto_version: ::core::ffi::c_int,
    pub type_: ::core::ffi::c_int,
}

#[repr(C)]
pub struct autofs_packet_missing {
    pub hdr: autofs_packet_hdr,
    pub wait_queue_token: autofs_wqt_t,
    pub len: ::core::ffi::c_int,
    pub name: [::core::ffi::c_char; 256 + 1],
}

#[repr(C)]
pub struct autofs_packet_expire {
    pub hdr: autofs_packet_hdr,
    pub len: ::core::ffi::c_int,
    pub name: [::core::ffi::c_char; 256 + 1],
}

pub const AUTOFS_IOCTL: u32 = 0x93;
pub const AUTOFS_IOC_READY_CMD: u32 = 0x60;
pub const AUTOFS_IOC_FAIL_CMD: u32 = 0x61;
pub const AUTOFS_IOC_CATATONIC_CMD: u32 = 0x62;
pub const AUTOFS_IOC_PROTOVER_CMD: u32 = 0x63;
pub const AUTOFS_IOC_SETTIMEOUT_CMD: u32 = 0x64;
pub const AUTOFS_IOC_EXPIRE_CMD: u32 = 0x65;

/* Linux ioctl encoding (the exact layout is supplied by the target ABI). */
const fn _ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << 30) | (size << 16) | (ty << 8) | nr
}
const fn _io(ty: u32, nr: u32) -> u32 { _ioc(0, ty, nr, 0) }
const fn _ior<T>(ty: u32, nr: u32) -> u32 { _ioc(2, ty, nr, core::mem::size_of::<T>() as u32) }
const fn _iow<T>(ty: u32, nr: u32) -> u32 { _ioc(1, ty, nr, core::mem::size_of::<T>() as u32) }
const fn _iowr<T>(ty: u32, nr: u32) -> u32 { _ioc(3, ty, nr, core::mem::size_of::<T>() as u32) }

pub const AUTOFS_IOC_READY: u32 = _io(AUTOFS_IOCTL, AUTOFS_IOC_READY_CMD);
pub const AUTOFS_IOC_FAIL: u32 = _io(AUTOFS_IOCTL, AUTOFS_IOC_FAIL_CMD);
pub const AUTOFS_IOC_CATATONIC: u32 = _io(AUTOFS_IOCTL, AUTOFS_IOC_CATATONIC_CMD);
pub const AUTOFS_IOC_PROTOVER: u32 = _ior::<::core::ffi::c_int>(AUTOFS_IOCTL, AUTOFS_IOC_PROTOVER_CMD);
pub const AUTOFS_IOC_SETTIMEOUT32: u32 = _iowr::<::core::ffi::c_ulong>(AUTOFS_IOCTL, AUTOFS_IOC_SETTIMEOUT_CMD);
pub const AUTOFS_IOC_SETTIMEOUT: u32 = _iowr::<::core::ffi::c_ulong>(AUTOFS_IOCTL, AUTOFS_IOC_SETTIMEOUT_CMD);
pub const AUTOFS_IOC_EXPIRE: u32 = _ior::<autofs_packet_expire>(AUTOFS_IOCTL, AUTOFS_IOC_EXPIRE_CMD);

pub const AUTOFS_EXP_NORMAL: u32 = 0x00;
pub const AUTOFS_EXP_IMMEDIATE: u32 = 0x01;
pub const AUTOFS_EXP_LEAVES: u32 = 0x02;
pub const AUTOFS_EXP_FORCED: u32 = 0x04;

pub const AUTOFS_TYPE_ANY: u32 = 0;
pub const AUTOFS_TYPE_INDIRECT: u32 = 1;
pub const AUTOFS_TYPE_DIRECT: u32 = 2;
pub const AUTOFS_TYPE_OFFSET: u32 = 4;

#[inline]
pub unsafe fn set_autofs_type_indirect(type_: *mut u32) { *type_ = AUTOFS_TYPE_INDIRECT; }
#[inline]
pub const fn autofs_type_indirect(type_: u32) -> u32 { (type_ == AUTOFS_TYPE_INDIRECT) as u32 }
#[inline]
pub unsafe fn set_autofs_type_direct(type_: *mut u32) { *type_ = AUTOFS_TYPE_DIRECT; }
#[inline]
pub const fn autofs_type_direct(type_: u32) -> u32 { (type_ == AUTOFS_TYPE_DIRECT) as u32 }
#[inline]
pub unsafe fn set_autofs_type_offset(type_: *mut u32) { *type_ = AUTOFS_TYPE_OFFSET; }
#[inline]
pub const fn autofs_type_offset(type_: u32) -> u32 { (type_ == AUTOFS_TYPE_OFFSET) as u32 }
#[inline]
pub const fn autofs_type_trigger(type_: u32) -> u32 { (type_ == AUTOFS_TYPE_DIRECT || type_ == AUTOFS_TYPE_OFFSET) as u32 }
#[inline]
pub unsafe fn set_autofs_type_any(type_: *mut u32) { *type_ = AUTOFS_TYPE_ANY; }
#[inline]
pub const fn autofs_type_any(type_: u32) -> u32 { (type_ == AUTOFS_TYPE_ANY) as u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum autofs_notify { NFY_NONE, NFY_MOUNT, NFY_EXPIRE }

pub const autofs_ptype_expire_multi: i32 = 2;
pub const autofs_ptype_missing_indirect: i32 = 3;
pub const autofs_ptype_expire_indirect: i32 = 4;
pub const autofs_ptype_missing_direct: i32 = 5;
pub const autofs_ptype_expire_direct: i32 = 6;

#[repr(C)]
pub struct autofs_packet_expire_multi {
    pub hdr: autofs_packet_hdr,
    pub wait_queue_token: autofs_wqt_t,
    pub len: ::core::ffi::c_int,
    pub name: [::core::ffi::c_char; 257],
}

#[repr(C)]
pub union autofs_packet_union {
    pub hdr: autofs_packet_hdr,
    pub missing: autofs_packet_missing,
    pub expire: autofs_packet_expire,
    pub expire_multi: autofs_packet_expire_multi,
}

#[repr(C)]
pub struct autofs_v5_packet {
    pub hdr: autofs_packet_hdr,
    pub wait_queue_token: autofs_wqt_t,
    pub dev: u32,
    pub ino: u64,
    pub uid: u32,
    pub gid: u32,
    pub pid: u32,
    pub tgid: u32,
    pub len: u32,
    pub name: [::core::ffi::c_char; 257],
}

pub type autofs_packet_missing_indirect_t = autofs_v5_packet;
pub type autofs_packet_expire_indirect_t = autofs_v5_packet;
pub type autofs_packet_missing_direct_t = autofs_v5_packet;
pub type autofs_packet_expire_direct_t = autofs_v5_packet;

#[repr(C)]
pub union autofs_v5_packet_union {
    pub hdr: autofs_packet_hdr,
    pub v5_packet: autofs_v5_packet,
    pub missing_indirect: autofs_packet_missing_indirect_t,
    pub expire_indirect: autofs_packet_expire_indirect_t,
    pub missing_direct: autofs_packet_missing_direct_t,
    pub expire_direct: autofs_packet_expire_direct_t,
}

pub const AUTOFS_IOC_EXPIRE_MULTI_CMD: u32 = 0x66;
pub const AUTOFS_IOC_PROTOSUBVER_CMD: u32 = 0x67;
pub const AUTOFS_IOC_ASKUMOUNT_CMD: u32 = 0x70;
pub const AUTOFS_IOC_EXPIRE_MULTI: u32 = _iow::<::core::ffi::c_int>(AUTOFS_IOCTL, AUTOFS_IOC_EXPIRE_MULTI_CMD);
pub const AUTOFS_IOC_PROTOSUBVER: u32 = _ior::<::core::ffi::c_int>(AUTOFS_IOCTL, AUTOFS_IOC_PROTOSUBVER_CMD);
pub const AUTOFS_IOC_ASKUMOUNT: u32 = _ior::<::core::ffi::c_int>(AUTOFS_IOCTL, AUTOFS_IOC_ASKUMOUNT_CMD);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
