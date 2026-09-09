/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2024 Loongson Technology Corporation Limited
 */

// Dependency supplied by kvm/iodev.h in the original source.

pub const PCH_PIC_SIZE: u32 = 0x3e8;

pub const PCH_PIC_INT_ID_START: u32 = 0x0;
pub const PCH_PIC_INT_ID_END: u32 = 0x7;
pub const PCH_PIC_MASK_START: u32 = 0x20;
pub const PCH_PIC_MASK_END: u32 = 0x27;
pub const PCH_PIC_HTMSI_EN_START: u32 = 0x40;
pub const PCH_PIC_HTMSI_EN_END: u32 = 0x47;
pub const PCH_PIC_EDGE_START: u32 = 0x60;
pub const PCH_PIC_EDGE_END: u32 = 0x67;
pub const PCH_PIC_CLEAR_START: u32 = 0x80;
pub const PCH_PIC_CLEAR_END: u32 = 0x87;
pub const PCH_PIC_AUTO_CTRL0_START: u32 = 0xc0;
pub const PCH_PIC_AUTO_CTRL0_END: u32 = 0xc7;
pub const PCH_PIC_AUTO_CTRL1_START: u32 = 0xe0;
pub const PCH_PIC_AUTO_CTRL1_END: u32 = 0xe7;
pub const PCH_PIC_ROUTE_ENTRY_START: u32 = 0x100;
pub const PCH_PIC_ROUTE_ENTRY_END: u32 = 0x13f;
pub const PCH_PIC_HTMSI_VEC_START: u32 = 0x200;
pub const PCH_PIC_HTMSI_VEC_END: u32 = 0x23f;
pub const PCH_PIC_INT_IRR_START: u32 = 0x380;
pub const PCH_PIC_INT_IRR_END: u32 = 0x38f;
pub const PCH_PIC_INT_ISR_START: u32 = 0x3a0;
pub const PCH_PIC_INT_ISR_END: u32 = 0x3af;
pub const PCH_PIC_POLARITY_START: u32 = 0x3e0;
pub const PCH_PIC_POLARITY_END: u32 = 0x3e7;
pub const PCH_PIC_INT_ID_VAL: u64 = 0x7;
pub const PCH_PIC_INT_ID_VER: u64 = 0x1;

#[repr(C)]
pub struct PchPicIdDesc {
    pub reserved_0: [u8; 3],
    pub id: u8,
    pub version: u8,
    pub reserved_1: u8,
    pub irq_num: u8,
    pub reserved_2: u8,
}

#[repr(C)]
pub union PchPicId {
    pub desc: PchPicIdDesc,
    pub data: u64,
}

#[repr(C)]
pub struct LoongarchPchPic {
    pub lock: spinlock_t,
    pub kvm: *mut kvm,
    pub device: kvm_io_device,
    pub id: PchPicId,
    pub mask: u64, /* 1:disable irq, 0:enable irq */
    pub htmsi_en: u64, /* 1:msi */
    pub edge: u64, /* 1:edge triggered, 0:level triggered */
    pub auto_ctrl0: u64, /* only use default value 00b */
    pub auto_ctrl1: u64, /* only use default value 00b */
    pub last_intirr: u64, /* edge detection */
    pub irr: u64, /* interrupt request register */
    pub isr: u64, /* interrupt service register */
    pub polarity: u64, /* 0: high level trigger, 1: low level trigger */
    pub route_entry: [u8; 64], /* default value 0, route to int0: eiointc */
    pub htmsi_vector: [u8; 64], /* irq route table for routing to eiointc */
    pub pch_pic_base: u64,
}

pub struct KvmKernelIrqRoutingEntry;

extern "C" {
    pub fn kvm_loongarch_register_pch_pic_device() -> i32;
    pub fn pch_pic_set_irq(s: *mut LoongarchPchPic, irq: i32, level: i32);
    pub fn pch_msi_set_irq(
        kvm: *mut kvm,
        e: *mut KvmKernelIrqRoutingEntry,
        level: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
