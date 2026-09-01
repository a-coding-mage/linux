/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2017-2021 Intel Corporation
 *
 * Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
 */

use core::ffi::{c_int, c_void};

/* DSP memories */
pub const IRAM_OFFSET: u32 = 0x0C0000;
pub const IRAM_SIZE: u32 = 80 * 1024;
pub const DRAM_OFFSET: u32 = 0x100000;
pub const DRAM_SIZE: u32 = 160 * 1024;
pub const SHIM_OFFSET: u32 = 0x140000;
pub const SHIM_SIZE_BYT: u32 = 0x100;
pub const SHIM_SIZE_CHT: u32 = 0x118;
pub const MBOX_OFFSET: u32 = 0x144000;
pub const MBOX_SIZE: u32 = 0x1000;
pub const EXCEPT_OFFSET: u32 = 0x800;
pub const EXCEPT_MAX_HDR_SIZE: u32 = 0x400;

/* DSP peripherals */
pub const DMAC0_OFFSET: u32 = 0x098000;
pub const DMAC1_OFFSET: u32 = 0x09c000;
pub const DMAC2_OFFSET: u32 = 0x094000;
pub const DMAC_SIZE: u32 = 0x420;
pub const SSP0_OFFSET: u32 = 0x0a0000;
pub const SSP1_OFFSET: u32 = 0x0a1000;
pub const SSP2_OFFSET: u32 = 0x0a2000;
pub const SSP3_OFFSET: u32 = 0x0a4000;
pub const SSP4_OFFSET: u32 = 0x0a5000;
pub const SSP5_OFFSET: u32 = 0x0a6000;
pub const SSP_SIZE: u32 = 0x100;

pub const STACK_DUMP_SIZE: u32 = 32;

pub const PCI_BAR_SIZE: u32 = 0x200000;

pub const fn PANIC_OFFSET(x: u64) -> u64 {
    (x & 0x0000ffff00000000) >> 32
}

/*
 * Debug
 */

pub const MBOX_DUMP_SIZE: u32 = 0x30;

/* BARs */
pub const DSP_BAR: u32 = 0;
pub const PCI_BAR: u32 = 1;
pub const IMR_BAR: u32 = 2;

unsafe extern "C" {
    pub fn atom_irq_handler(irq: c_int, context: *mut c_void) -> irqreturn_t;
    pub fn atom_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t;

    pub fn atom_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
    pub fn atom_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int;
    pub fn atom_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> c_int;

    pub fn atom_run(sdev: *mut snd_sof_dev) -> c_int;
    pub fn atom_reset(sdev: *mut snd_sof_dev) -> c_int;
    pub fn atom_dump(sdev: *mut snd_sof_dev, flags: u32);

    pub fn atom_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach;
    pub fn atom_set_mach_params(mach: *mut snd_soc_acpi_mach, sdev: *mut snd_sof_dev);

    pub static mut atom_dai: [snd_soc_dai_driver; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
