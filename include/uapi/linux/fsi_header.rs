/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/*
 * /dev/scom "raw" ioctl interface
 *
 * The driver supports a high level "read/write" interface which
 * handles retries and converts the status to Linux error codes,
 * however low level tools an debugger need to access the "raw"
 * HW status information and interpret them themselves, so this
 * ioctl interface is also provided for their use case.
 */

/* Structure for SCOM read/write */
#[repr(C)]
pub struct scom_access {
    pub addr: u64,        /* SCOM address, supports indirect */
    pub data: u64,        /* SCOM data (in for write, out for read) */
    pub mask: u64,        /* Data mask for writes */
    pub intf_errors: u32, /* Interface error flags */
    pub pib_status: u8,   /* 3-bit PIB status */
    pub pad: u8,
}

pub const SCOM_INTF_ERR_PARITY: u32 = 0x00000001; /* Parity error */
pub const SCOM_INTF_ERR_PROTECTION: u32 = 0x00000002; /* Blocked by secure boot */
pub const SCOM_INTF_ERR_ABORT: u32 = 0x00000004; /* PIB reset during access */
pub const SCOM_INTF_ERR_UNKNOWN: u32 = 0x80000000; /* Unknown error */

/*
 * Note: Any other bit set in intf_errors need to be considered as an
 * error. Future implementations may define new error conditions. The
 * pib_status below is only valid if intf_errors is 0.
 */

pub const SCOM_PIB_SUCCESS: u8 = 0; /* Access successful */
pub const SCOM_PIB_BLOCKED: u8 = 1; /* PIB blocked, pls retry */
pub const SCOM_PIB_OFFLINE: u8 = 2; /* Chiplet offline */
pub const SCOM_PIB_PARTIAL: u8 = 3; /* Partial good */
pub const SCOM_PIB_BAD_ADDR: u8 = 4; /* Invalid address */
pub const SCOM_PIB_CLK_ERR: u8 = 5; /* Clock error */
pub const SCOM_PIB_PARITY_ERR: u8 = 6; /* Parity error on the PIB bus */
pub const SCOM_PIB_TIMEOUT: u8 = 7; /* Bus timeout */

/* Flags for SCOM check */
pub const SCOM_CHECK_SUPPORTED: u32 = 0x00000001; /* Interface supported */
pub const SCOM_CHECK_PROTECTED: u32 = 0x00000002; /* Interface blocked by secure boot */

/* Flags for SCOM reset */
pub const SCOM_RESET_INTF: u32 = 0x00000001; /* Reset interface */
pub const SCOM_RESET_PIB: u32 = 0x00000002; /* Reset PIB */

/* `_IOR`, `_IOWR`, and `_IOW` are supplied by the Linux ioctl dependency. */
pub const FSI_SCOM_CHECK: u32 = _IOR('s', 0x00, u32);
pub const FSI_SCOM_READ: u32 = _IOWR('s', 0x01, scom_access);
pub const FSI_SCOM_WRITE: u32 = _IOWR('s', 0x02, scom_access);
pub const FSI_SCOM_RESET: u32 = _IOW('s', 0x03, u32);

/*
 * /dev/sbefifo* ioctl interface
 */

/**
 * FSI_SBEFIFO_CMD_TIMEOUT sets the timeout for writing data to the SBEFIFO.
 *
 * The command timeout is specified in seconds.  The minimum value of command
 * timeout is 1 seconds (default) and the maximum value of command timeout is
 * 120 seconds.  A command timeout of 0 will reset the value to the default of
 * 1 seconds.
 */
pub const FSI_SBEFIFO_CMD_TIMEOUT_SECONDS: u32 = _IOW('s', 0x01, u32);

/**
 * FSI_SBEFIFO_READ_TIMEOUT sets the read timeout for response from SBE.
 *
 * The read timeout is specified in seconds.  The minimum value of read
 * timeout is 10 seconds (default) and the maximum value of read timeout is
 * 120 seconds.  A read timeout of 0 will reset the value to the default of
 * (10 seconds).
 */
pub const FSI_SBEFIFO_READ_TIMEOUT_SECONDS: u32 = _IOW('s', 0x00, u32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
