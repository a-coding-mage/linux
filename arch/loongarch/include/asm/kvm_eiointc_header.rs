/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2024 Loongson Technology Corporation Limited
 */

// Dependency supplied by the surrounding kernel translation: kvm/iodev.h

pub const EIOINTC_IRQS: usize = 256;
pub const EIOINTC_ROUTE_MAX_VCPUS: usize = 256;
pub const EIOINTC_IRQS_U64_NUMS: usize = EIOINTC_IRQS / 64;
/* map to ipnum per 32 irqs */
pub const EIOINTC_IRQS_NODETYPE_COUNT: usize = 16;

pub const EIOINTC_BASE: usize = 0x1400;
pub const EIOINTC_SIZE: usize = 0x900;

pub const EIOINTC_NODETYPE_START: usize = 0xa0;
pub const EIOINTC_NODETYPE_END: usize = 0xbf;
pub const EIOINTC_IPMAP_START: usize = 0xc0;
pub const EIOINTC_IPMAP_END: usize = 0xc7;
pub const EIOINTC_ENABLE_START: usize = 0x200;
pub const EIOINTC_ENABLE_END: usize = 0x21f;
pub const EIOINTC_BOUNCE_START: usize = 0x280;
pub const EIOINTC_BOUNCE_END: usize = 0x29f;
pub const EIOINTC_ISR_START: usize = 0x300;
pub const EIOINTC_ISR_END: usize = 0x31f;
pub const EIOINTC_COREISR_START: usize = 0x400;
pub const EIOINTC_COREISR_END: usize = 0x41f;
pub const EIOINTC_COREMAP_START: usize = 0x800;
pub const EIOINTC_COREMAP_END: usize = 0x8ff;

pub const EIOINTC_VIRT_BASE: usize = 0x40000000;
pub const EIOINTC_VIRT_SIZE: usize = 0x1000;

pub const EIOINTC_VIRT_FEATURES: u32 = 0x0;
pub const EIOINTC_HAS_VIRT_EXTENSION: u32 = 0;
pub const EIOINTC_HAS_ENABLE_OPTION: u32 = 1;
pub const EIOINTC_HAS_INT_ENCODE: u32 = 2;
pub const EIOINTC_HAS_CPU_ENCODE: u32 = 3;
pub const EIOINTC_VIRT_HAS_FEATURES: u32 =
    (1u32 << EIOINTC_HAS_VIRT_EXTENSION)
        | (1u32 << EIOINTC_HAS_ENABLE_OPTION)
        | (1u32 << EIOINTC_HAS_INT_ENCODE)
        | (1u32 << EIOINTC_HAS_CPU_ENCODE);
pub const EIOINTC_VIRT_CONFIG: u32 = 0x4;
pub const EIOINTC_ENABLE: u32 = 1;
pub const EIOINTC_ENABLE_INT_ENCODE: u32 = 2;
pub const EIOINTC_ENABLE_CPU_ENCODE: u32 = 3;

pub const LOONGSON_IP_NUM: usize = 8;

#[repr(C)]
pub struct loongarch_eiointc {
    pub lock: spinlock_t,
    pub kvm: *mut kvm,
    pub device: kvm_io_device,
    pub device_vext: kvm_io_device,
    pub num_cpu: u32,
    pub features: u32,
    pub status: u32,

    /* hardware state */
    pub nodetype: [u64; EIOINTC_IRQS_NODETYPE_COUNT / 4],

    /* one bit shows the state of one irq */
    pub bounce: [u64; EIOINTC_IRQS_U64_NUMS],
    pub isr: [u64; EIOINTC_IRQS_U64_NUMS],
    pub coreisr: [[u64; EIOINTC_IRQS_U64_NUMS]; EIOINTC_ROUTE_MAX_VCPUS],
    pub enable: [u64; EIOINTC_IRQS_U64_NUMS],

    /* use one byte to config ipmap for 32 irqs at once */
    pub ipmap: u64,
    /* use one byte to config coremap for one irq */
    pub coremap: [u64; EIOINTC_IRQS / 8],

    pub sw_coreisr:
        [[[core::ffi::c_ulong; (EIOINTC_IRQS + (core::mem::size_of::<core::ffi::c_ulong>() * 8) - 1) / (core::mem::size_of::<core::ffi::c_ulong>() * 8)]; LOONGSON_IP_NUM]; EIOINTC_ROUTE_MAX_VCPUS],
    pub sw_coremap: [u8; EIOINTC_IRQS],
}

extern "C" {
    pub fn kvm_loongarch_register_eiointc_device() -> core::ffi::c_int;
    pub fn eiointc_set_irq(s: *mut loongarch_eiointc, irq: core::ffi::c_int, level: core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
