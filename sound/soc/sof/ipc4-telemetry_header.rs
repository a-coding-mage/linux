/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2023 Intel Corporation
 */

/* Target code */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sof_ipc4_coredump_tgt_code {
    COREDUMP_TGT_UNKNOWN = 0,
    COREDUMP_TGT_X86,
    COREDUMP_TGT_X86_64,
    COREDUMP_TGT_ARM_CORTEX_M,
    COREDUMP_TGT_RISC_V,
    COREDUMP_TGT_XTENSA,
}

pub const COREDUMP_ARCH_HDR_ID: u8 = b'A';
pub const COREDUMP_HDR_ID0: u8 = b'Z';
pub const COREDUMP_HDR_ID1: u8 = b'E';

pub const XTENSA_BLOCK_HDR_VER: u32 = 2;
pub const XTENSA_CORE_DUMP_SEPARATOR: u32 = 0x0DEC0DEB;
pub const XTENSA_CORE_AR_REGS_COUNT: u32 = 16;
pub const XTENSA_SOC_INTEL_ADSP: u32 = 3;
pub const XTENSA_TOOL_CHAIN_ZEPHYR: u32 = 1;
pub const XTENSA_TOOL_CHAIN_XCC: u32 = 2;

/* Coredump header */
#[repr(C, packed)]
pub struct sof_ipc4_coredump_hdr {
    /* 'Z', 'E' as identifier of file */
    pub id: [::core::ffi::c_char; 2],

    /* Identify the version of the header */
    pub hdr_version: u16,

    /* Indicate which target (e.g. architecture or SoC) */
    pub tgt_code: u16,

    /* Size of uintptr_t in power of 2. (e.g. 5 for 32-bit, 6 for 64-bit) */
    pub ptr_size_bits: u8,

    pub flag: u8,

    /* Reason for the fatal error */
    pub reason: u32,
}

/* Architecture-specific block header */
#[repr(C, packed)]
pub struct sof_ipc4_coredump_arch_hdr {
    /* COREDUMP_ARCH_HDR_ID to indicate this is a architecture-specific block */
    pub id: ::core::ffi::c_char,

    /* Identify the version of this block */
    pub hdr_version: u16,

    /* Number of bytes following the header */
    pub num_bytes: u16,
}

#[repr(C, packed)]
pub struct sof_ipc4_telemetry_slot_data {
    pub separator: u32,
    pub hdr: sof_ipc4_coredump_hdr,
    pub arch_hdr: sof_ipc4_coredump_arch_hdr,
    pub arch_data: [u32; 0],
}

unsafe extern "C" {
    pub fn sof_ipc4_create_exception_debugfs_node(sdev: *mut snd_sof_dev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
