/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/*
 * The C header includes linux/types.h and linux/ioctl.h. Their declarations
 * are external dependencies of this translation.
 */

/*
 * UACCE_CMD_START_Q: Start queue
 */
pub const UACCE_CMD_START_Q: u32 = (('W' as u32) << 8) | 0;

/*
 * UACCE_CMD_PUT_Q:
 * User actively stop queue and free queue resource immediately
 * Optimization method since close fd may delay
 */
pub const UACCE_CMD_PUT_Q: u32 = (('W' as u32) << 8) | 1;

/*
 * UACCE Device flags:
 * UACCE_DEV_SVA: Shared Virtual Addresses
 *                 Support PASID
 *                 Support device page faults (PCI PRI or SMMU Stall)
 */
pub const UACCE_DEV_SVA: u32 = 1u32 << 0;

/**
 * enum uacce_qfrt: queue file region type
 * @UACCE_QFRT_MMIO: device mmio region
 * @UACCE_QFRT_DUS: device user share region
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum uacce_qfrt {
    UACCE_QFRT_MMIO = 0,
    UACCE_QFRT_DUS = 1,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
