/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2021-2024 Advanced Micro Devices, Inc.
 */

// Dependency supplied by the surrounding translation environment: linux/types.h

/* Mailbox data size for data_in and data_out */
pub const AMD_SBI_MB_DATA_SIZE: u32 = 4;

#[repr(C)]
pub struct apml_mbox_msg {
    /* Mailbox Message ID */
    pub cmd: u32,
    /* [0]...[3] mailbox 32bit input/output data */
    pub mb_in_out: u32,
    /* Error code is returned in case of soft mailbox error */
    pub fw_ret_code: u32,
}

#[repr(C)]
pub struct apml_cpuid_msg {
    /*
     * CPUID input
     * [0]...[3] cpuid func,
     * [4][5] cpuid: thread
     * [6] cpuid: ext function & read eax/ebx or ecx/edx
     * [7:0] -> bits [7:4] -> ext function &
     * bit [0] read eax/ebx or ecx/edx
     * CPUID output
     */
    pub cpu_in_out: u64,
    /* Status code for CPUID read */
    pub fw_ret_code: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct apml_mcamsr_msg {
    /*
     * MCAMSR input
     * [0]...[3] mca msr func,
     * [4][5] thread
     * MCAMSR output
     */
    pub mcamsr_in_out: u64,
    /* Status code for MCA/MSR access */
    pub fw_ret_code: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct apml_reg_xfer_msg {
    /* RMI register address offset */
    pub reg_addr: u16,
    /* Register data for read/write */
    pub data_in_out: u8,
    /* Register read or write */
    pub rflag: u8,
}

#[repr(C)]
pub struct apml_tsi_xfer_msg {
    pub reg_addr: u8,     /* TSI register address offset */
    pub data_in_out: u8,  /* Register data for read/write */
    pub rflag: u8,        /* Register read or write */
    pub pad: u8,          /* Explicit padding */
}

/* AMD sideband interface base IOCTL */
pub const SB_BASE_IOCTL_NR: u32 = 0xF9;

/*
 * IOCTL command for APML messages using generic _IOWR.
 * The _IOWR macro and its dependent definitions are supplied externally.
 */
pub const SBRMI_IOCTL_MBOX_CMD: _ = _IOWR(SB_BASE_IOCTL_NR, 0, apml_mbox_msg);
pub const SBRMI_IOCTL_CPUID_CMD: _ = _IOWR(SB_BASE_IOCTL_NR, 1, apml_cpuid_msg);
pub const SBRMI_IOCTL_MCAMSR_CMD: _ = _IOWR(SB_BASE_IOCTL_NR, 2, apml_mcamsr_msg);
pub const SBRMI_IOCTL_REG_XFER_CMD: _ = _IOWR(SB_BASE_IOCTL_NR, 3, apml_reg_xfer_msg);
pub const SBTSI_IOCTL_REG_XFER_CMD: _ = _IOWR(SB_BASE_IOCTL_NR, 4, apml_tsi_xfer_msg);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
