/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2019 Intel Corporation
 *
 * Author: Keyon Jie <yang.jie@linux.intel.com>
 */

/*
 * Primary register, mapped to
 * - DIPCTDR (HIPCIDR) in sideband IPC (cAVS 1.8+)
 * - DIPCT in cAVS 1.5 IPC
 *
 * Secondary register, mapped to:
 * - DIPCTDD (HIPCIDD) in sideband IPC (cAVS 1.8+)
 * - DIPCTE in cAVS 1.5 IPC
 */

/* Common bits in primary register */

/* Reserved for doorbell */
pub const HDA_IPC_RSVD_31: u32 = 1u32 << 31;
/* Target, 0 - normal message, 1 - compact message(cAVS compatible) */
pub const HDA_IPC_MSG_COMPACT: u32 = 1u32 << 30;
/* Direction, 0 - request, 1 - response */
pub const HDA_IPC_RSP: u32 = 1u32 << 29;

pub const HDA_IPC_TYPE_SHIFT: u32 = 24;
pub const HDA_IPC_TYPE_MASK: u32 = 0x1f000000;
pub const fn HDA_IPC_TYPE(x: u32) -> u32 {
    x << HDA_IPC_TYPE_SHIFT
}

pub const HDA_IPC_PM_GATE: u32 = HDA_IPC_TYPE(0x8u32);

/* Command specific payload bits in secondary register */

/* Disable DMA tracing (0 - keep tracing, 1 - to disable DMA trace) */
pub const HDA_PM_NO_DMA_TRACE: u32 = 1u32 << 4;
/* Prevent clock gating (0 - cg allowed, 1 - DSP clock always on) */
pub const HDA_PM_PCG: u32 = 1u32 << 3;
/* Prevent power gating (0 - deep power state transitions allowed) */
pub const HDA_PM_PPG: u32 = 1u32 << 2;
/* Indicates whether streaming is active */
pub const HDA_PM_PG_STREAMING: u32 = 1u32 << 1;
pub const HDA_PM_PG_RSVD: u32 = 1u32 << 0;

unsafe extern "C" {
    pub fn cnl_ipc_irq_thread(irq: ::core::ffi::c_int, context: *mut ::core::ffi::c_void) -> irqreturn_t;
    pub fn cnl_ipc_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> ::core::ffi::c_int;
    pub fn cnl_ipc_dump(sdev: *mut snd_sof_dev);
    pub fn cnl_ipc4_dump(sdev: *mut snd_sof_dev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
