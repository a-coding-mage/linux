/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2023 Intel Corporation
 *
 * telemetry data in debug windows
 */

/* C dependency: ../ipc4-telemetry.h */

#[repr(C, packed)]
pub struct xtensa_arch_block {
    pub soc: u8, /* should be equal to XTENSA_SOC_INTEL_ADSP */
    pub version: u16,
    pub toolchain: u8, /* ZEPHYR or XCC */

    pub pc: u32,
    pub exccause: u32,
    pub excvaddr: u32,
    pub sar: u32,
    pub ps: u32,
    pub scompare1: u32,
    pub ar: [u32; XTENSA_CORE_AR_REGS_COUNT],
    pub lbeg: u32,
    pub lend: u32,
    pub lcount: u32,
}

unsafe extern "C" {
    pub fn sof_ipc4_intel_dump_telemetry_state(sdev: *mut snd_sof_dev, flags: u32);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
