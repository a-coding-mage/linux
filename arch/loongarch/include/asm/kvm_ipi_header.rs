/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2024 Loongson Technology Corporation Limited
 */

// Dependency supplied by the surrounding kernel translation.

pub const LARCH_INT_IPI: u32 = 12;

#[repr(C)]
pub struct loongarch_ipi {
    pub lock: spinlock_t,
    pub kvm: *mut kvm,
    pub device: kvm_io_device,
}

#[repr(C)]
pub struct ipi_state {
    pub lock: spinlock_t,
    pub status: u32,
    pub en: u32,
    pub set: u32,
    pub clear: u32,
    pub buf: [u64; 4],
}

pub const IOCSR_IPI_BASE: u32 = 0x1000;
pub const IOCSR_IPI_SIZE: u32 = 0x160;

pub const IOCSR_IPI_STATUS: u32 = 0x000;
pub const IOCSR_IPI_EN: u32 = 0x004;
pub const IOCSR_IPI_SET: u32 = 0x008;
pub const IOCSR_IPI_CLEAR: u32 = 0x00c;
pub const IOCSR_IPI_BUF_20: u32 = 0x020;
pub const IOCSR_IPI_BUF_28: u32 = 0x028;
pub const IOCSR_IPI_BUF_30: u32 = 0x030;
pub const IOCSR_IPI_BUF_38: u32 = 0x038;
pub const IOCSR_IPI_SEND: u32 = 0x040;
pub const IOCSR_MAIL_SEND: u32 = 0x048;
pub const IOCSR_ANY_SEND: u32 = 0x158;

unsafe extern "C" {
    pub fn kvm_loongarch_register_ipi_device() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
