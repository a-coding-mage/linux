/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * module/mite.h
 * Hardware driver for NI Mite PCI interface chip
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1999 David A. Schleef <ds@schleef.org>
 */

// Dependency supplied by the surrounding kernel/Rust translation.

pub const MAX_MITE_DMA_CHANNELS: usize = 8;

#[repr(C)]
pub struct comedi_device;
#[repr(C)]
pub struct comedi_subdevice;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct pci_dev;

pub type __le32 = u32;
pub type u32 = std::ffi::c_uint;
pub type dma_addr_t = usize;
pub type spinlock_t = usize;

#[repr(C)]
pub struct mite_dma_desc {
    pub count: __le32,
    pub addr: __le32,
    pub next: __le32,
    pub dar: u32,
}

#[repr(C)]
pub struct mite_ring {
    pub hw_dev: *mut device,
    pub n_links: std::ffi::c_uint,
    pub descs: *mut mite_dma_desc,
    pub dma_addr: dma_addr_t,
}

#[repr(C)]
pub struct mite_channel {
    pub mite: *mut mite,
    pub channel: std::ffi::c_uint,
    pub dir: std::ffi::c_int,
    pub done: std::ffi::c_int,
    pub ring: *mut mite_ring,
}

#[repr(C)]
pub struct mite {
    pub pcidev: *mut pci_dev,
    pub mmio: *mut std::ffi::c_void,
    pub channels: [mite_channel; MAX_MITE_DMA_CHANNELS],
    pub num_channels: std::ffi::c_int,
    pub fifo_size: std::ffi::c_uint,
    /* protects mite_channel from being released by the driver */
    pub lock: spinlock_t,
}

extern "C" {
    pub fn mite_bytes_in_transit(mite_chan: *mut mite_channel) -> u32;

    pub fn mite_sync_dma(mite_chan: *mut mite_channel, s: *mut comedi_subdevice);
    pub fn mite_ack_linkc(
        mite_chan: *mut mite_channel,
        s: *mut comedi_subdevice,
        sync: bool,
    );
    pub fn mite_done(mite_chan: *mut mite_channel) -> std::ffi::c_int;

    pub fn mite_dma_arm(mite_chan: *mut mite_channel);
    pub fn mite_dma_disarm(mite_chan: *mut mite_channel);

    pub fn mite_prep_dma(
        mite_chan: *mut mite_channel,
        num_device_bits: std::ffi::c_uint,
        num_memory_bits: std::ffi::c_uint,
    );

    pub fn mite_request_channel_in_range(
        mite: *mut mite,
        ring: *mut mite_ring,
        min_channel: std::ffi::c_uint,
        max_channel: std::ffi::c_uint,
    ) -> *mut mite_channel;
    pub fn mite_request_channel(mite: *mut mite, ring: *mut mite_ring) -> *mut mite_channel;
    pub fn mite_release_channel(mite_chan: *mut mite_channel);

    pub fn mite_init_ring_descriptors(
        ring: *mut mite_ring,
        s: *mut comedi_subdevice,
        nbytes: std::ffi::c_uint,
    ) -> std::ffi::c_int;
    pub fn mite_buf_change(ring: *mut mite_ring, s: *mut comedi_subdevice) -> std::ffi::c_int;

    pub fn mite_alloc_ring(mite: *mut mite) -> *mut mite_ring;
    pub fn mite_free_ring(ring: *mut mite_ring);

    pub fn mite_attach(dev: *mut comedi_device, use_win1: bool) -> *mut mite;
    pub fn mite_detach(mite: *mut mite);
}

/* Mite registers (used outside of the mite driver) */
pub const MITE_IODWBSR: u32 = 0xc0;
pub const MITE_IODWBSR_1: u32 = 0xc4;
pub const WENAB: u32 = 1u32 << 7; /* window enable */
pub const MITE_IODWCR_1: u32 = 0xf4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
