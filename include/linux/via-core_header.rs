/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 1998-2009 VIA Technologies, Inc. All Rights Reserved.
 * Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
 * Copyright 2009-2010 Jonathan Corbet <corbet@lwn.net>
 * Copyright 2010 Florian Tobias Schandinat <FlorianSchandinat@gmx.de>
 */

// Translated from via-core.h. Kernel types, I/O functions, and dependent
// structures are supplied by the surrounding build.

use core::ffi::c_void;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct pci_dev;

pub type spinlock_t = usize;

extern "C" {
    pub fn outb(value: u8, port: u16);
    pub fn inb(port: u16) -> u8;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum via_port_type {
    VIA_PORT_NONE = 0,
    VIA_PORT_I2C,
    VIA_PORT_GPIO,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum via_port_mode {
    VIA_MODE_OFF = 0,
    VIA_MODE_I2C, // Used as I2C port
    VIA_MODE_GPIO, // Two GPIO ports
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum viafb_i2c_adap {
    VIA_PORT_26 = 0,
    VIA_PORT_31,
    VIA_PORT_25,
    VIA_PORT_2C,
    VIA_PORT_3D,
}

pub const VIAFB_NUM_PORTS: usize = 5;

#[repr(C)]
pub struct via_port_cfg {
    pub r#type: via_port_type,
    pub mode: via_port_mode,
    pub io_port: u16,
    pub ioport_index: u8,
}

/* Allow subdevs to register suspend/resume hooks. */
#[repr(C)]
pub struct viafb_pm_hooks {
    pub list: list_head,
    pub suspend: Option<unsafe extern "C" fn(private: *mut c_void) -> i32>,
    pub resume: Option<unsafe extern "C" fn(private: *mut c_void) -> i32>,
    pub private: *mut c_void,
}

extern "C" {
    pub fn viafb_pm_register(hooks: *mut viafb_pm_hooks);
    pub fn viafb_pm_unregister(hooks: *mut viafb_pm_hooks);
}

/* This is the global viafb "device" containing stuff needed by all subdevs. */
#[repr(C)]
pub struct viafb_dev {
    pub pdev: *mut pci_dev,
    pub chip_type: i32,
    pub port_cfg: *mut via_port_cfg,
    pub reg_lock: spinlock_t,
    pub fbmem_start: usize,
    pub fbmem_len: isize,
    pub fbmem: *mut c_void,
    // Preserved build-time condition: CONFIG_VIDEO_VIA_CAMERA or
    // CONFIG_VIDEO_VIA_CAMERA_MODULE.
    #[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
    pub camera_fbmem_offset: isize,
    #[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
    pub camera_fbmem_size: isize,
    pub engine_start: usize,
    pub engine_len: usize,
    pub engine_mmio: *mut c_void,
}

extern "C" {
    pub fn viafb_irq_enable(mask: u32);
    pub fn viafb_irq_disable(mask: u32);
}

pub const VDE_INTERRUPT: u32 = 0x200;
pub const VDE_I_DVISENSE: u32 = 0x00000001;
pub const VDE_I_VBLANK: u32 = 0x00000002;
pub const VDE_I_MCCFI: u32 = 0x00000004;
pub const VDE_I_VSYNC: u32 = 0x00000008;
pub const VDE_I_DMA0DDONE: u32 = 0x00000010;
pub const VDE_I_DMA0TDONE: u32 = 0x00000020;
pub const VDE_I_DMA1DDONE: u32 = 0x00000040;
pub const VDE_I_DMA1TDONE: u32 = 0x00000080;
pub const VDE_I_C1AV: u32 = 0x00000100;
pub const VDE_I_HQV0: u32 = 0x00000200;
pub const VDE_I_HQV1: u32 = 0x00000400;
pub const VDE_I_HQV1EN: u32 = 0x00000800;
pub const VDE_I_C0AV: u32 = 0x00001000;
pub const VDE_I_C0VBI: u32 = 0x00002000;
pub const VDE_I_C1VBI: u32 = 0x00004000;
pub const VDE_I_VSYNC2: u32 = 0x00008000;
pub const VDE_I_DVISNSEN: u32 = 0x00010000;
pub const VDE_I_VSYNC2EN: u32 = 0x00020000;
pub const VDE_I_MCCFIEN: u32 = 0x00040000;
pub const VDE_I_VSYNCEN: u32 = 0x00080000;
pub const VDE_I_DMA0DDEN: u32 = 0x00100000;
pub const VDE_I_DMA0TDEN: u32 = 0x00200000;
pub const VDE_I_DMA1DDEN: u32 = 0x00400000;
pub const VDE_I_DMA1TDEN: u32 = 0x00800000;
pub const VDE_I_C1AVEN: u32 = 0x01000000;
pub const VDE_I_HQV0EN: u32 = 0x02000000;
pub const VDE_I_C1VBIEN: u32 = 0x04000000;
pub const VDE_I_LVDSSI: u32 = 0x08000000;
pub const VDE_I_C0AVEN: u32 = 0x10000000;
pub const VDE_I_C0VBIEN: u32 = 0x20000000;
pub const VDE_I_LVDSSIEN: u32 = 0x40000000;
pub const VDE_I_ENABLE: u32 = 0x80000000;

// Preserved build-time condition: CONFIG_VIDEO_VIA_CAMERA or
// CONFIG_VIDEO_VIA_CAMERA_MODULE.
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
extern "C" {
    pub fn viafb_request_dma() -> i32;
    pub fn viafb_release_dma();
    pub fn viafb_dma_copy_out_sg(offset: u32, sg: *mut scatterlist, nsg: i32) -> i32;
}

#[repr(C)]
pub struct scatterlist;

#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_MR0: u32 = 0xe00;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_MR_CHAIN: u32 = 0x01;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_MR_TDIE: u32 = 0x02;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_CSR0: u32 = 0xe04;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_C_ENABLE: u32 = 0x01;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_C_START: u32 = 0x02;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_C_ABORT: u32 = 0x04;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_C_DONE: u32 = 0x08;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_MARL0: u32 = 0xe20;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_MARH0: u32 = 0xe24;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_DAR0: u32 = 0xe28;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_DQWCR0: u32 = 0xe2c;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_TMR0: u32 = 0xe30;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_DPRL0: u32 = 0xe34;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_DPR_IN: u32 = 0x08;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_DPRH0: u32 = 0xe38;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VDMA_PMR0: u32 = 0xe00 + 0x134;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VGA_WIDTH: u32 = 640;
#[cfg(any(feature = "CONFIG_VIDEO_VIA_CAMERA", feature = "CONFIG_VIDEO_VIA_CAMERA_MODULE"))]
pub const VGA_HEIGHT: u32 = 480;

pub const VIAStatus: u16 = 0x3DA;
pub const VIACR: u16 = 0x3D4;
pub const VIASR: u16 = 0x3C4;
pub const VIAGR: u16 = 0x3CE;
pub const VIAAR: u16 = 0x3C0;

pub unsafe fn via_read_reg(port: u16, index: u8) -> u8 {
    outb(index, port);
    inb(port.wrapping_add(1))
}

pub unsafe fn via_write_reg(port: u16, index: u8, data: u8) {
    outb(index, port);
    outb(data, port.wrapping_add(1));
}

pub unsafe fn via_write_reg_mask(port: u16, index: u8, data: u8, mask: u8) {
    outb(index, port);
    let old = inb(port.wrapping_add(1));
    outb((data & mask) | (old & !mask), port.wrapping_add(1));
}

pub const VIA_MISC_REG_READ: u16 = 0x03CC;
pub const VIA_MISC_REG_WRITE: u16 = 0x03C2;

pub unsafe fn via_write_misc_reg_mask(data: u8, mask: u8) {
    let old = inb(VIA_MISC_REG_READ);
    outb((data & mask) | (old & !mask), VIA_MISC_REG_WRITE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
