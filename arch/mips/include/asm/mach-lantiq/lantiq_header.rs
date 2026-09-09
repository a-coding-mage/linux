/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 *  Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/irq.h, linux/device.h, and linux/clk.h.

/* generic reg access functions */
macro_rules! ltq_r32 {
    ($reg:expr) => {
        __raw_readl($reg)
    };
}

macro_rules! ltq_w32 {
    ($val:expr, $reg:expr) => {
        __raw_writel($val, $reg)
    };
}

macro_rules! ltq_w32_mask {
    ($clear:expr, $set:expr, $reg:expr) => {
        ltq_w32!((ltq_r32!($reg) & !($clear)) | ($set), $reg)
    };
}

macro_rules! ltq_r8 {
    ($reg:expr) => {
        __raw_readb($reg)
    };
}

macro_rules! ltq_w8 {
    ($val:expr, $reg:expr) => {
        __raw_writeb($val, $reg)
    };
}

/* register access macros for EBU and CGU */
macro_rules! ltq_ebu_w32 {
    ($x:expr, $y:expr) => {
        ltq_w32!($x, ltq_ebu_membase + ($y))
    };
}

macro_rules! ltq_ebu_r32 {
    ($x:expr) => {
        ltq_r32!(ltq_ebu_membase + ($x))
    };
}

macro_rules! ltq_ebu_w32_mask {
    ($x:expr, $y:expr, $z:expr) => {
        ltq_w32_mask!($x, $y, ltq_ebu_membase + ($z))
    };
}

extern "C" {
    pub static mut ltq_ebu_membase: *mut core::ffi::c_void;

    /* spinlock all ebu i/o */
    pub static mut ebu_lock: spinlock_t;

    /* some irq helpers */
    pub fn ltq_disable_irq(data: *mut irq_data);
    pub fn ltq_mask_and_ack_irq(data: *mut irq_data);
    pub fn ltq_enable_irq(data: *mut irq_data);
    pub fn ltq_eiu_get_irq(exin: core::ffi::c_int) -> core::ffi::c_int;

    /* clock handling */
    pub fn clk_activate(clk: *mut clk) -> core::ffi::c_int;
    pub fn clk_deactivate(clk: *mut clk);
    pub fn clk_get_cpu() -> *mut clk;
    pub fn clk_get_fpi() -> *mut clk;
    pub fn clk_get_io() -> *mut clk;
    pub fn clk_get_ppe() -> *mut clk;

    /* find out what bootsource we have */
    pub fn ltq_boot_select() -> u8;
    /* find out the soc type */
    pub fn ltq_soc_type() -> core::ffi::c_int;
}

pub const IOPORT_RESOURCE_START: u32 = 0x10000000;
pub const IOPORT_RESOURCE_END: u32 = 0xffff_ffff;
pub const IOMEM_RESOURCE_START: u32 = 0x10000000;
pub const IOMEM_RESOURCE_END: u32 = 0xffff_ffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
