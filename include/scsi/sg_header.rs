/* SPDX-License-Identifier: GPL-2.0 */

/* Linux kernel/user-space dependency: __user, compat types, and ioctl ABI. */

use core::ffi::c_void;

#[repr(C)]
pub struct sg_iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}
pub type sg_iovec_t = sg_iovec;

#[repr(C)]
pub struct sg_io_hdr {
    pub interface_id: i32,
    pub dxfer_direction: i32,
    pub cmd_len: u8,
    pub mx_sb_len: u8,
    pub iovec_count: u16,
    pub dxfer_len: u32,
    pub dxferp: *mut c_void,
    pub cmdp: *mut u8,
    pub sbp: *mut c_void,
    pub timeout: u32,
    pub flags: u32,
    pub pack_id: i32,
    pub usr_ptr: *mut c_void,
    pub status: u8,
    pub masked_status: u8,
    pub msg_status: u8,
    pub sb_len_wr: u8,
    pub host_status: u16,
    pub driver_status: u16,
    pub resid: i32,
    pub duration: u32,
    pub info: u32,
}
pub type sg_io_hdr_t = sg_io_hdr;

/* Defined only when compiling for the kernel; compat_int_t/compat_uint_t and
 * compat_uptr_t are supplied by linux/compat.h. */
#[cfg(feature = "kernel")]
#[repr(C)]
pub struct compat_sg_io_hdr {
    pub interface_id: i32,
    pub dxfer_direction: i32,
    pub cmd_len: u8,
    pub mx_sb_len: u8,
    pub iovec_count: u16,
    pub dxfer_len: u32,
    pub dxferp: u32,
    pub cmdp: u32,
    pub sbp: u32,
    pub timeout: u32,
    pub flags: u32,
    pub pack_id: i32,
    pub usr_ptr: u32,
    pub status: u8,
    pub masked_status: u8,
    pub msg_status: u8,
    pub sb_len_wr: u8,
    pub host_status: u16,
    pub driver_status: u16,
    pub resid: i32,
    pub duration: u32,
    pub info: u32,
}

pub const SG_INTERFACE_ID_ORIG: i32 = 'S' as i32;
pub const SG_DXFER_NONE: i32 = -1;
pub const SG_DXFER_TO_DEV: i32 = -2;
pub const SG_DXFER_FROM_DEV: i32 = -3;
pub const SG_DXFER_TO_FROM_DEV: i32 = -4;
pub const SG_DXFER_UNKNOWN: i32 = -5;

pub const SG_FLAG_DIRECT_IO: u32 = 1;
pub const SG_FLAG_UNUSED_LUN_INHIBIT: u32 = 2;
pub const SG_FLAG_MMAP_IO: u32 = 4;
pub const SG_FLAG_NO_DXFER: u32 = 0x10000;
pub const SG_FLAG_Q_AT_TAIL: u32 = 0x10;
pub const SG_FLAG_Q_AT_HEAD: u32 = 0x20;

pub const SG_INFO_OK_MASK: u32 = 0x1;
pub const SG_INFO_OK: u32 = 0x0;
pub const SG_INFO_CHECK: u32 = 0x1;
pub const SG_INFO_DIRECT_IO_MASK: u32 = 0x6;
pub const SG_INFO_INDIRECT_IO: u32 = 0x0;
pub const SG_INFO_DIRECT_IO: u32 = 0x2;
pub const SG_INFO_MIXED_IO: u32 = 0x4;
pub const DRIVER_SENSE: u32 = 0x08;

#[inline]
pub const fn driver_byte(result: u32) -> u32 { (result >> 24) & 0xff }

pub const GOOD: u32 = 0x00;
pub const CHECK_CONDITION: u32 = 0x01;
pub const CONDITION_GOOD: u32 = 0x02;
pub const BUSY: u32 = 0x04;
pub const INTERMEDIATE_GOOD: u32 = 0x08;
pub const INTERMEDIATE_C_GOOD: u32 = 0x0a;
pub const RESERVATION_CONFLICT: u32 = 0x0c;
pub const COMMAND_TERMINATED: u32 = 0x11;
pub const QUEUE_FULL: u32 = 0x14;
pub const ACA_ACTIVE: u32 = 0x18;
pub const TASK_ABORTED: u32 = 0x20;

#[inline]
pub const fn sg_status_byte(result: u32) -> u32 { (result >> 1) & 0x7f }

#[repr(C)]
pub struct sg_scsi_id {
    pub host_no: i32, pub channel: i32, pub scsi_id: i32, pub lun: i32,
    pub scsi_type: i32, pub h_cmd_per_lun: i16, pub d_queue_depth: i16,
    pub unused: [i32; 2],
}
pub type sg_scsi_id_t = sg_scsi_id;

#[repr(C)]
pub struct sg_req_info {
    pub req_state: i8, pub orphan: i8, pub sg_io_owned: i8, pub problem: i8,
    pub pack_id: i32, pub usr_ptr: *mut c_void, pub duration: u32, pub unused: i32,
}
pub type sg_req_info_t = sg_req_info;

pub const SG_EMULATED_HOST: u32 = 0x2203;
pub const SG_SET_TRANSFORM: u32 = 0x2204;
pub const SG_GET_TRANSFORM: u32 = 0x2205;
pub const SG_SET_RESERVED_SIZE: u32 = 0x2275;
pub const SG_GET_RESERVED_SIZE: u32 = 0x2272;
pub const SG_GET_SCSI_ID: u32 = 0x2276;
pub const SG_SET_FORCE_LOW_DMA: u32 = 0x2279;
pub const SG_GET_LOW_DMA: u32 = 0x227a;
pub const SG_SET_FORCE_PACK_ID: u32 = 0x227b;
pub const SG_GET_PACK_ID: u32 = 0x227c;
pub const SG_GET_NUM_WAITING: u32 = 0x227d;
pub const SG_GET_SG_TABLESIZE: u32 = 0x227f;
pub const SG_GET_VERSION_NUM: u32 = 0x2282;
pub const SG_SCSI_RESET: u32 = 0x2284;
pub const SG_SCSI_RESET_NOTHING: u32 = 0;
pub const SG_SCSI_RESET_DEVICE: u32 = 1;
pub const SG_SCSI_RESET_BUS: u32 = 2;
pub const SG_SCSI_RESET_HOST: u32 = 3;
pub const SG_SCSI_RESET_TARGET: u32 = 4;
pub const SG_SCSI_RESET_NO_ESCALATE: u32 = 0x100;
pub const SG_IO: u32 = 0x2285;
pub const SG_GET_REQUEST_TABLE: u32 = 0x2286;
pub const SG_SET_KEEP_ORPHAN: u32 = 0x2287;
pub const SG_GET_KEEP_ORPHAN: u32 = 0x2288;
pub const SG_GET_ACCESS_COUNT: u32 = 0x2289;

pub const SG_SCATTER_SZ: usize = 8 * 4096;
pub const SG_DEFAULT_RETRIES: i32 = 0;
pub const SG_DEF_FORCE_PACK_ID: i32 = 0;
pub const SG_DEF_KEEP_ORPHAN: i32 = 0;
pub const SG_DEF_RESERVED_SIZE: usize = SG_SCATTER_SZ;
pub const SG_MAX_QUEUE: i32 = 16;
pub const SG_BIG_BUFF: usize = SG_DEF_RESERVED_SIZE;

pub type Sg_io_hdr = sg_io_hdr;
/* The source names sg_io_vec although no such structure is defined here. */
pub type Sg_io_vec = sg_io_vec;
pub type Sg_scsi_id = sg_scsi_id;
pub type Sg_req_info = sg_req_info;

pub const SG_MAX_SENSE: usize = 16;

#[repr(C)]
pub struct sg_header {
    pub pack_len: i32,
    pub reply_len: i32,
    pub pack_id: i32,
    pub result: i32,
    /* C bit-fields: twelve_byte:1, target_status:5, host_status:8,
     * driver_status:8, other_flags:10. */
    pub status_flags: u32,
    pub sense_buffer: [u8; SG_MAX_SENSE],
}

pub const SG_SET_TIMEOUT: u32 = 0x2201;
pub const SG_GET_TIMEOUT: u32 = 0x2202;
pub const SG_GET_COMMAND_Q: u32 = 0x2270;
pub const SG_SET_COMMAND_Q: u32 = 0x2271;
pub const SG_SET_DEBUG: u32 = 0x227e;
pub const SG_NEXT_CMD_LEN: u32 = 0x2283;
/* HZ/USER_HZ are supplied by the target kernel/user environment. */
#[cfg(feature = "kernel")]
pub const SG_DEFAULT_TIMEOUT_USER: u32 = 60 * USER_HZ;
#[cfg(not(feature = "kernel"))]
pub const SG_DEFAULT_TIMEOUT: u32 = 60 * HZ;
pub const SG_DEF_COMMAND_Q: i32 = 0;
pub const SG_DEF_UNDERRUN_FLAG: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
