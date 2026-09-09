/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  Copyright IBM Corp. 2022, 2024
 *  Author(s): Steffen Eiden <seiden@linux.ibm.com>
 */

#[repr(C)]
pub struct uvio_ioctl_cb {
    pub flags: u32,
    pub uv_rc: u16, /* UV header rc value */
    pub uv_rrc: u16, /* UV header rrc value */
    pub argument_addr: u64, /* Userspace address of uvio argument */
    pub argument_len: u32,
    pub reserved14: [u8; 0x40 - 0x14], /* must be zero */
}

pub const UVIO_ATT_USER_DATA_LEN: usize = 0x100;
pub const UVIO_ATT_UID_LEN: usize = 0x10;

#[repr(C)]
pub struct uvio_attest {
    pub arcb_addr: u64, /* 0x0000 */
    pub meas_addr: u64, /* 0x0008 */
    pub add_data_addr: u64, /* 0x0010 */
    pub user_data: [u8; UVIO_ATT_USER_DATA_LEN], /* 0x0018 */
    pub config_uid: [u8; UVIO_ATT_UID_LEN], /* 0x0118 */
    pub arcb_len: u32, /* 0x0128 */
    pub meas_len: u32, /* 0x012c */
    pub add_data_len: u32, /* 0x0130 */
    pub user_data_len: u16, /* 0x0134 */
    pub reserved136: u16, /* 0x0136 */
}

/**
 * uvio_uvdev_info - Information of supported functions
 * @supp_uvio_cmds - supported IOCTLs by this device
 * @supp_uv_cmds - supported UVCs corresponding to the IOCTL
 *
 * UVIO request to get information about supported request types by this
 * uvdevice and the Ultravisor.  Everything is output. Bits are in LSB0
 * ordering.  If the bit is set in both, @supp_uvio_cmds and @supp_uv_cmds, the
 * uvdevice and the Ultravisor support that call.
 *
 * Note that bit 0 (UVIO_IOCTL_UVDEV_INFO_NR) is always zero for `supp_uv_cmds`
 * as there is no corresponding UV-call.
 */
#[repr(C)]
pub struct uvio_uvdev_info {
    /* If bit `n` is set, this device supports the IOCTL with nr `n`. */
    pub supp_uvio_cmds: u64,
    /*
     * If bit `n` is set, the Ultravisor(UV) supports the UV-call
     * corresponding to the IOCTL with nr `n` in the calling context (host
     * or guest).  The value is only valid if the corresponding bit in
     * @supp_uvio_cmds is set as well.
     */
    pub supp_uv_cmds: u64,
}

pub const UVIO_ATT_ARCB_MAX_LEN: usize = 0x100000;
pub const UVIO_ATT_MEASUREMENT_MAX_LEN: usize = 0x8000;
pub const UVIO_ATT_ADDITIONAL_MAX_LEN: usize = 0x8000;
pub const UVIO_ADD_SECRET_MAX_LEN: usize = 0x100000;
pub const UVIO_LIST_SECRETS_LEN: usize = 0x1000;
pub const UVIO_RETR_SECRET_MAX_LEN: usize = 0x2000;

pub const UVIO_DEVICE_NAME: &str = "uv";
pub const UVIO_TYPE_UVC: u8 = b'u';

#[repr(u32)]
pub enum UVIO_IOCTL_NR {
    UVIO_IOCTL_UVDEV_INFO_NR = 0x00,
    UVIO_IOCTL_ATT_NR,
    UVIO_IOCTL_ADD_SECRET_NR,
    UVIO_IOCTL_LIST_SECRETS_NR,
    UVIO_IOCTL_LOCK_SECRETS_NR,
    UVIO_IOCTL_RETR_SECRET_NR,
    /* must be the last entry */
    UVIO_IOCTL_NUM_IOCTLS,
}

/* `_IOWR` is supplied by the platform ioctl definitions. */
#[macro_export]
macro_rules! UVIO_IOCTL {
    ($nr:expr) => { _IOWR!(UVIO_TYPE_UVC, $nr, uvio_ioctl_cb) };
}
pub const UVIO_IOCTL_UVDEV_INFO: u32 = UVIO_IOCTL!(UVIO_IOCTL_UVDEV_INFO_NR as u32);
pub const UVIO_IOCTL_ATT: u32 = UVIO_IOCTL!(UVIO_IOCTL_ATT_NR as u32);
pub const UVIO_IOCTL_ADD_SECRET: u32 = UVIO_IOCTL!(UVIO_IOCTL_ADD_SECRET_NR as u32);
pub const UVIO_IOCTL_LIST_SECRETS: u32 = UVIO_IOCTL!(UVIO_IOCTL_LIST_SECRETS_NR as u32);
pub const UVIO_IOCTL_LOCK_SECRETS: u32 = UVIO_IOCTL!(UVIO_IOCTL_LOCK_SECRETS_NR as u32);
pub const UVIO_IOCTL_RETR_SECRET: u32 = UVIO_IOCTL!(UVIO_IOCTL_RETR_SECRET_NR as u32);

pub const fn UVIO_SUPP_CALL(nr: u32) -> u64 {
    1u64 << nr
}
/* The source uses UVIO_IOCTL_UDEV_INFO_NR (without "V"); preserve that name. */
pub const UVIO_SUPP_UDEV_INFO: u64 = UVIO_SUPP_CALL(UVIO_IOCTL_UDEV_INFO_NR as u32);
pub const UVIO_SUPP_ATT: u64 = UVIO_SUPP_CALL(UVIO_IOCTL_ATT_NR as u32);
pub const UVIO_SUPP_ADD_SECRET: u64 = UVIO_SUPP_CALL(UVIO_IOCTL_ADD_SECRET_NR as u32);
pub const UVIO_SUPP_LIST_SECRETS: u64 = UVIO_SUPP_CALL(UVIO_IOCTL_LIST_SECRETS_NR as u32);
pub const UVIO_SUPP_LOCK_SECRETS: u64 = UVIO_SUPP_CALL(UVIO_IOCTL_LOCK_SECRETS_NR as u32);
pub const UVIO_SUPP_RETR_SECRET: u64 = UVIO_SUPP_CALL(UVIO_IOCTL_RETR_SECRET_NR as u32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
