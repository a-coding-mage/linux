/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Renesas SuperH DMA Engine support
 *
 * Copyright (C) 2009 Nobuhiro Iwamatsu <iwamatsu.nobuhiro@renesas.com>
 * Copyright (C) 2009 Renesas Solutions, Inc. All rights reserved.
 *
 */

// C dependencies supplied by the surrounding kernel translation:
// linux/sh_dma.h, linux/shdma-base.h, linux/dmaengine.h,
// linux/interrupt.h, and linux/list.h.

pub const SH_DMAE_MAX_CHANNELS: usize = 20;
pub const SH_DMAE_TCR_MAX: u32 = 0x00FF_FFFF; // 16MB

pub struct device;

#[repr(C)]
pub struct sh_dmae_chan {
    pub shdma_chan: shdma_chan,
    pub config: *const sh_dmae_slave_config, // Slave DMA configuration
    pub xmit_shift: ::core::ffi::c_int, // log_2(bytes_per_xfer)
    pub base: *mut core::ffi::c_void,
    pub dev_id: [core::ffi::c_char; 32], // unique name per DMAC of channel
    pub pm_error: ::core::ffi::c_int,
    pub slave_addr: dma_addr_t,
}

#[repr(C)]
pub struct sh_dmae_device {
    pub shdma_dev: shdma_dev,
    pub chan: [*mut sh_dmae_chan; SH_DMAE_MAX_CHANNELS],
    pub pdata: *const sh_dmae_pdata,
    pub node: list_head,
    pub chan_reg: *mut core::ffi::c_void,
    pub dmars: *mut core::ffi::c_void,
    pub chcr_offset: ::core::ffi::c_uint,
    pub chcr_ie_bit: u32,
}

#[repr(C)]
pub struct sh_dmae_regs {
    pub sar: u32, // SAR / source address
    pub dar: u32, // DAR / destination address
    pub tcr: u32, // TCR / transfer count
}

#[repr(C)]
pub struct sh_dmae_desc {
    pub hw: sh_dmae_regs,
    pub shdma_desc: shdma_desc,
}

// Direct translations of the C container_of macros. The container_of!
// primitive is supplied by the surrounding kernel translation.
#[macro_export]
macro_rules! to_sh_chan {
    ($chan:expr) => {
        container_of!($chan, sh_dmae_chan, shdma_chan)
    };
}

#[macro_export]
macro_rules! to_sh_desc {
    ($lh:expr) => {
        container_of!($lh, sh_desc, node)
    };
}

#[macro_export]
macro_rules! tx_to_sh_desc {
    ($tx:expr) => {
        container_of!($tx, sh_desc, async_tx)
    };
}

#[macro_export]
macro_rules! to_sh_dev {
    ($chan:expr) => {
        container_of!(
            ($chan).shdma_chan.dma_chan.device,
            sh_dmae_device,
            shdma_dev.dma_dev
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
