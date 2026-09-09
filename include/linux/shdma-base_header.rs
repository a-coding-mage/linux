/* SPDX-License-Identifier: GPL-2.0
 *
 * Dmaengine driver base library for DMA controllers, found on SH-based SoCs
 *
 * extracted from shdma.c and headers
 *
 * Copyright (C) 2011-2012 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 * Copyright (C) 2009 Nobuhiro Iwamatsu <iwamatsu.nobuhiro@renesas.com>
 * Copyright (C) 2009 Renesas Solutions, Inc. All rights reserved.
 * Copyright (C) 2007 Freescale Semiconductor, Inc. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/dmaengine.h, linux/interrupt.h, linux/list.h, linux/types.h

/**
 * enum shdma_pm_state - DMA channel PM state
 * @SHDMA_PM_ESTABLISHED: either idle or during data transfer
 * @SHDMA_PM_BUSY: during the transfer preparation, when we have to
 *                 drop the lock temporarily
 * @SHDMA_PM_PENDING: transfers pending
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum shdma_pm_state {
    SHDMA_PM_ESTABLISHED,
    SHDMA_PM_BUSY,
    SHDMA_PM_PENDING,
}

pub struct device;

/*
 * Drivers, using this library are expected to embed struct shdma_dev,
 * struct shdma_chan, struct shdma_desc, and struct shdma_slave
 * in their respective device, channel, descriptor and slave objects.
 */

#[repr(C)]
pub struct shdma_slave {
    pub slave_id: ::core::ffi::c_int,
}

#[repr(C)]
pub struct shdma_desc {
    pub node: list_head,
    pub async_tx: dma_async_tx_descriptor,
    pub direction: dma_transfer_direction,
    pub partial: usize,
    pub cookie: dma_cookie_t,
    pub chunks: ::core::ffi::c_int,
    pub mark: ::core::ffi::c_int,
    pub cyclic: bool, /* used as cyclic transfer */
}

#[repr(C)]
pub struct shdma_chan {
    pub chan_lock: spinlock_t, /* Channel operation lock */
    pub ld_queue: list_head,   /* Link descriptors queue */
    pub ld_free: list_head,    /* Free link descriptors */
    pub dma_chan: dma_chan,    /* DMA channel */
    pub dev: *mut device,      /* Channel device */
    pub desc: *mut ::core::ffi::c_void, /* buffer for descriptor array */
    pub desc_num: ::core::ffi::c_int, /* desc count */
    pub max_xfer_len: usize,    /* max transfer length */
    pub id: ::core::ffi::c_int, /* Raw id of this channel */
    pub irq: ::core::ffi::c_int, /* Channel IRQ */
    pub slave_id: ::core::ffi::c_int, /* Client ID for slave DMA */
    pub real_slave_id: ::core::ffi::c_int, /* argument passed to filter function */
    pub hw_req: ::core::ffi::c_int, /* DMA request line for slave DMA - same
                                      * as MID/RID, used with DT */
    pub pm_state: shdma_pm_state,
}

/**
 * struct shdma_ops - simple DMA driver operations
 * @desc_completed: return true, if this is the descriptor, that just has
 *                  completed (atomic)
 * @halt_channel: stop DMA channel operation (atomic)
 * @channel_busy: return true, if the channel is busy (atomic)
 * @slave_addr: return slave DMA address
 * @desc_setup: set up the hardware specific descriptor portion (atomic)
 * @set_slave: bind channel to a slave
 * @setup_xfer: configure channel hardware for operation (atomic)
 * @start_xfer: start the DMA transfer (atomic)
 * @embedded_desc: return Nth struct shdma_desc pointer from the descriptor array
 * @chan_irq: process channel IRQ, return true if a transfer has completed (atomic)
 */
#[repr(C)]
pub struct shdma_ops {
    pub desc_completed: Option<unsafe extern "C" fn(*mut shdma_chan, *mut shdma_desc) -> bool>,
    pub halt_channel: Option<unsafe extern "C" fn(*mut shdma_chan)>,
    pub channel_busy: Option<unsafe extern "C" fn(*mut shdma_chan) -> bool>,
    pub slave_addr: Option<unsafe extern "C" fn(*mut shdma_chan) -> dma_addr_t>,
    pub desc_setup: Option<unsafe extern "C" fn(*mut shdma_chan, *mut shdma_desc, dma_addr_t, dma_addr_t, *mut usize) -> ::core::ffi::c_int>,
    pub set_slave: Option<unsafe extern "C" fn(*mut shdma_chan, ::core::ffi::c_int, dma_addr_t, bool) -> ::core::ffi::c_int>,
    pub setup_xfer: Option<unsafe extern "C" fn(*mut shdma_chan, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub start_xfer: Option<unsafe extern "C" fn(*mut shdma_chan, *mut shdma_desc)>,
    pub embedded_desc: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> *mut shdma_desc>,
    pub chan_irq: Option<unsafe extern "C" fn(*mut shdma_chan, ::core::ffi::c_int) -> bool>,
    pub get_partial: Option<unsafe extern "C" fn(*mut shdma_chan, *mut shdma_desc) -> usize>,
}

#[repr(C)]
pub struct shdma_dev {
    pub dma_dev: dma_device,
    pub schan: *mut *mut shdma_chan,
    pub ops: *const shdma_ops,
    pub desc_size: usize,
}

#[macro_export]
macro_rules! shdma_for_each_chan {
    ($c:ident, $d:ident, $i:ident) => {
        for $i in 0..unsafe { (*$d).dma_dev.chancnt } {
            $c = unsafe { (*$d).schan.add($i).read() };
        }
    };
}

extern "C" {
    pub fn shdma_request_irq(chan: *mut shdma_chan, irq: ::core::ffi::c_int,
                              flags: ::core::ffi::c_ulong, name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn shdma_reset(sdev: *mut shdma_dev) -> bool;
    pub fn shdma_chan_probe(sdev: *mut shdma_dev, schan: *mut shdma_chan, id: ::core::ffi::c_int);
    pub fn shdma_chan_remove(schan: *mut shdma_chan);
    pub fn shdma_init(dev: *mut device, sdev: *mut shdma_dev, chan_num: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn shdma_cleanup(sdev: *mut shdma_dev);
}

// CONFIG_SH_DMAE_BASE conditional: when enabled this is supplied externally.
extern "C" {
    pub fn shdma_chan_filter(chan: *mut dma_chan, arg: *mut ::core::ffi::c_void) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
