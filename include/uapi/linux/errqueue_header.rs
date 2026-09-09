/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Types supplied by linux/types.h, linux/time_types.h, and related headers are
 * intentionally referenced here rather than redefined. */

/* RFC 4884: return offset to extension struct + validation */
#[repr(C)]
pub struct sock_ee_data_rfc4884 {
    pub len: __u16,
    pub flags: __u8,
    pub reserved: __u8,
}

#[repr(C)]
pub union sock_extended_err__bindgen_ty_1 {
    pub ee_data: __u32,
    pub ee_rfc4884: sock_ee_data_rfc4884,
}

#[repr(C)]
pub struct sock_extended_err {
    pub ee_errno: __u32,
    pub ee_origin: __u8,
    pub ee_type: __u8,
    pub ee_code: __u8,
    pub ee_pad: __u8,
    pub ee_info: __u32,
    pub __bindgen_anon_1: sock_extended_err__bindgen_ty_1,
}

pub const SO_EE_ORIGIN_NONE: u32 = 0;
pub const SO_EE_ORIGIN_LOCAL: u32 = 1;
pub const SO_EE_ORIGIN_ICMP: u32 = 2;
pub const SO_EE_ORIGIN_ICMP6: u32 = 3;
pub const SO_EE_ORIGIN_TXSTATUS: u32 = 4;
pub const SO_EE_ORIGIN_ZEROCOPY: u32 = 5;
pub const SO_EE_ORIGIN_TXTIME: u32 = 6;
pub const SO_EE_ORIGIN_TIMESTAMPING: u32 = SO_EE_ORIGIN_TXSTATUS;

#[inline]
pub unsafe fn SO_EE_OFFENDER(ee: *mut sock_extended_err) -> *mut sockaddr {
    (ee.add(1)) as *mut sockaddr
}

pub const SO_EE_CODE_ZEROCOPY_COPIED: u32 = 1;

pub const SO_EE_CODE_TXTIME_INVALID_PARAM: u32 = 1;
pub const SO_EE_CODE_TXTIME_MISSED: u32 = 2;

pub const SO_EE_RFC4884_FLAG_INVALID: u32 = 1;

/**
 * struct scm_timestamping - timestamps exposed through cmsg
 *
 * The timestamping interfaces SO_TIMESTAMPING, MSG_TSTAMP_* communicate
 * network timestamps by passing this struct in a cmsg with recvmsg().
 * See Documentation/networking/timestamping.rst for details.
 * User space sees a timespec definition that matches either __kernel_timespec
 * or __kernel_old_timespec; in the kernel both structure definitions are
 * provided as required.
 */
#[repr(C)]
pub struct scm_timestamping {
    /* When building for the kernel, use __kernel_old_timespec; otherwise use
     * the userspace timespec definition. */
    #[cfg(feature = "__KERNEL__")]
    pub ts: [__kernel_old_timespec; 3],
    #[cfg(not(feature = "__KERNEL__"))]
    pub ts: [timespec; 3],
}

#[repr(C)]
pub struct scm_timestamping64 {
    pub ts: [__kernel_timespec; 3],
}

/* The type of scm_timestamping, passed in sock_extended_err ee_info.
 * This defines the type of ts[0]. For SCM_TSTAMP_SND only, if ts[0]
 * is zero, then this is a hardware timestamp and recorded in ts[2].
 */
pub const SCM_TSTAMP_SND: i32 = 0;
pub const SCM_TSTAMP_SCHED: i32 = 1;
pub const SCM_TSTAMP_ACK: i32 = 2;
pub const SCM_TSTAMP_COMPLETION: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
