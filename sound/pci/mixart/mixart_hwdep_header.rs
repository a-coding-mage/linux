/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for Digigram miXart soundcards
 *
 * definitions and makros for basic card access
 *
 * Copyright (c) 2003 by Digigram <alsa@digigram.com>
 */

use core::ffi::c_void;

// C dependency: <sound/hwdep.h>

#[repr(C)]
pub struct mixart_mgr {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn __raw_readl(addr: *const c_void) -> u32;
    fn __raw_writel(data: u32, addr: *mut c_void);
}

#[inline]
pub unsafe fn readl_be(x: *const c_void) -> u32 {
    u32::from_be(unsafe { __raw_readl(x) })
}

#[inline]
pub unsafe fn writel_be(data: u32, addr: *mut c_void) {
    unsafe { __raw_writel(data.to_be(), addr) };
}

#[inline]
pub unsafe fn readl_le(x: *const c_void) -> u32 {
    u32::from_le(unsafe { __raw_readl(x) })
}

#[inline]
pub unsafe fn writel_le(data: u32, addr: *mut c_void) {
    unsafe { __raw_writel(data.to_le(), addr) };
}

// C macros requiring the external layout of struct mixart_mgr:
// #define MIXART_MEM(mgr,x) ((mgr)->mem[0].virt + (x))
// #define MIXART_REG(mgr,x) ((mgr)->mem[1].virt + (x))

/* Daughter board Type */
pub const DAUGHTER_TYPE_MASK: u32 = 0x0F;
pub const DAUGHTER_VER_MASK: u32 = 0xF0;
pub const DAUGHTER_TYPEVER_MASK: u32 = DAUGHTER_TYPE_MASK | DAUGHTER_VER_MASK;

pub const MIXART_DAUGHTER_TYPE_NONE: u32 = 0x00;
pub const MIXART_DAUGHTER_TYPE_COBRANET: u32 = 0x08;
pub const MIXART_DAUGHTER_TYPE_AES: u32 = 0x0E;

pub const MIXART_BA0_SIZE: u32 = 16 * 1024 * 1024; /* 16M */
pub const MIXART_BA1_SIZE: u32 = 4 * 1024; /* 4k */

/*
 * -----------BAR 0 --------------------------------------------------------------------------------------------------------
 */
pub const MIXART_PSEUDOREG: u32 = 0x2000; /* base address for pseudoregister */

pub const MIXART_PSEUDOREG_BOARDNUMBER: u32 = MIXART_PSEUDOREG + 0; /* board number */

/* perfmeter (available when elf loaded)*/
pub const MIXART_PSEUDOREG_PERF_STREAM_LOAD_OFFSET: u32 = MIXART_PSEUDOREG + 0x70; /* streaming load */
pub const MIXART_PSEUDOREG_PERF_SYSTEM_LOAD_OFFSET: u32 = MIXART_PSEUDOREG + 0x78; /* system load (reference)*/
pub const MIXART_PSEUDOREG_PERF_MAILBX_LOAD_OFFSET: u32 = MIXART_PSEUDOREG + 0x7C; /* mailbox load */
pub const MIXART_PSEUDOREG_PERF_INTERR_LOAD_OFFSET: u32 = MIXART_PSEUDOREG + 0x74; /* interrupt handling  load */

/* motherboard xilinx loader info */
pub const MIXART_PSEUDOREG_MXLX_BASE_ADDR_OFFSET: u32 = MIXART_PSEUDOREG + 0x9C; /* 0x00600000 */
pub const MIXART_PSEUDOREG_MXLX_SIZE_OFFSET: u32 = MIXART_PSEUDOREG + 0xA0; /* xilinx size in bytes */
pub const MIXART_PSEUDOREG_MXLX_STATUS_OFFSET: u32 = MIXART_PSEUDOREG + 0xA4; /* status = EMBEBBED_STAT_XXX */

/* elf loader info */
pub const MIXART_PSEUDOREG_ELF_STATUS_OFFSET: u32 = MIXART_PSEUDOREG + 0xB0; /* status = EMBEBBED_STAT_XXX */

/*
*  after the elf code is loaded, and the flowtable info was passed to it,
*  the driver polls on this address, until it shows 1 (presence) or 2 (absence)
*  once it is non-zero, the daughter board type may be read
*/
pub const MIXART_PSEUDOREG_DBRD_PRESENCE_OFFSET: u32 = MIXART_PSEUDOREG + 0x990;

/* Global info structure */
pub const MIXART_PSEUDOREG_DBRD_TYPE_OFFSET: u32 = MIXART_PSEUDOREG + 0x994; /* Type and version of daughterboard  */

/* daughterboard xilinx loader info */
pub const MIXART_PSEUDOREG_DXLX_BASE_ADDR_OFFSET: u32 = MIXART_PSEUDOREG + 0x998; /* get the address here where to write the file */
pub const MIXART_PSEUDOREG_DXLX_SIZE_OFFSET: u32 = MIXART_PSEUDOREG + 0x99C; /* xilinx size in bytes */
pub const MIXART_PSEUDOREG_DXLX_STATUS_OFFSET: u32 = MIXART_PSEUDOREG + 0x9A0; /* status = EMBEBBED_STAT_XXX */

/*  */
pub const MIXART_FLOWTABLE_PTR: u32 = 0x3000; /* pointer to flow table */

/* mailbox addresses  */

/* message DRV -> EMB */
pub const MSG_INBOUND_POST_HEAD: u32 = 0x010008; /* DRV posts MF + increment4 */
pub const MSG_INBOUND_POST_TAIL: u32 = 0x01000C; /* EMB gets MF + increment4 */
/* message EMB -> DRV */
pub const MSG_OUTBOUND_POST_TAIL: u32 = 0x01001C; /* DRV gets MF + increment4 */
pub const MSG_OUTBOUND_POST_HEAD: u32 = 0x010018; /* EMB posts MF + increment4 */
/* Get Free Frames */
pub const MSG_INBOUND_FREE_TAIL: u32 = 0x010004; /* DRV gets MFA + increment4 */
pub const MSG_OUTBOUND_FREE_TAIL: u32 = 0x010014; /* EMB gets MFA + increment4 */
/* Put Free Frames */
pub const MSG_OUTBOUND_FREE_HEAD: u32 = 0x010010; /* DRV puts MFA + increment4 */
pub const MSG_INBOUND_FREE_HEAD: u32 = 0x010000; /* EMB puts MFA + increment4 */

/* firmware addresses of the message fifos */
pub const MSG_BOUND_STACK_SIZE: u32 = 0x004000; /* size of each following stack */
/* posted messages */
pub const MSG_OUTBOUND_POST_STACK: u32 = 0x108000; /* stack of messages to the DRV */
pub const MSG_INBOUND_POST_STACK: u32 = 0x104000; /* stack of messages to the EMB */
/* available empty messages */
pub const MSG_OUTBOUND_FREE_STACK: u32 = 0x10C000; /* stack of free enveloped for EMB */
pub const MSG_INBOUND_FREE_STACK: u32 = 0x100000; /* stack of free enveloped for DRV */

/* defines for mailbox message frames */
pub const MSG_FRAME_OFFSET: u32 = 0x64;
pub const MSG_FRAME_SIZE: u32 = 0x6400;
pub const MSG_FRAME_NUMBER: u32 = 32;
pub const MSG_FROM_AGENT_ITMF_OFFSET: u32 = MSG_FRAME_OFFSET + (MSG_FRAME_SIZE * MSG_FRAME_NUMBER);
pub const MSG_TO_AGENT_ITMF_OFFSET: u32 = MSG_FROM_AGENT_ITMF_OFFSET + MSG_FRAME_SIZE;
pub const MSG_HOST_RSC_PROTECTION: u32 = MSG_TO_AGENT_ITMF_OFFSET + MSG_FRAME_SIZE;
pub const MSG_AGENT_RSC_PROTECTION: u32 = MSG_HOST_RSC_PROTECTION + 4;

/*
 * -----------BAR 1 --------------------------------------------------------------------------------------------------------
 */

/* interrupt addresses and constants */
pub const MIXART_PCI_OMIMR_OFFSET: u32 = 0x34; /* outbound message interrupt mask register */
pub const MIXART_PCI_OMISR_OFFSET: u32 = 0x30; /* outbound message interrupt status register */
pub const MIXART_PCI_ODBR_OFFSET: u32 = 0x60; /* outbound doorbell register */

pub const MIXART_BA1_BRUTAL_RESET_OFFSET: u32 = 0x68; /* write 1 in LSBit to reset board */

pub const MIXART_HOST_ALL_INTERRUPT_MASKED: u32 = 0x02B; /* 0000 0010 1011 */
pub const MIXART_ALLOW_OUTBOUND_DOORBELL: u32 = 0x023; /* 0000 0010 0011 */
pub const MIXART_OIDI: u32 = 0x008; /* 0000 0000 1000 */

unsafe extern "C" {
    pub fn snd_mixart_setup_firmware(mgr: *mut mixart_mgr) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
