/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent from the original header:
// linux/types.h, asm/ioctl.h, and asm/papr-miscdev.h.

pub const PAPR_PHYATTEST_MAX_INPUT: usize = 4084; /* Max 4K buffer: 4K-12 */

/*
 * Defined in PAPR 2.13+ 21.6 Attestation Command Structures.
 * User space pass this struct and the max size should be 4K.
 */
#[repr(C)]
pub struct papr_phy_attest_io_block {
    pub version: u8,
    pub command: u8,
    pub TCG_major_ver: u8,
    pub TCG_minor_ver: u8,
    pub length: u32,
    pub correlator: u32,
    pub payload: [u8; PAPR_PHYATTEST_MAX_INPUT],
}

/*
 * ioctl for /dev/papr-physical-attestation. Returns a attestation
 * command fd handle.
 *
 * `_IOW` and `PAPR_MISCDEV_IOC_ID` are supplied by the corresponding
 * external Rust dependencies.
 */
pub const PAPR_PHY_ATTEST_IOC_HANDLE: usize =
    crate::_IOW!(PAPR_MISCDEV_IOC_ID, 8, papr_phy_attest_io_block);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
