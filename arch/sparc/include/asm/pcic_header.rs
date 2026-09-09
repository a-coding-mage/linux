/* SPDX-License-Identifier: GPL-2.0 */
/*
 * pcic.h: JavaEngine 1 specific PCI definitions.
 *
 * Copyright (C) 1998 V. Roganov and G. Raiko
 */

/* C dependencies: linux/types.h, linux/smp.h, linux/pci.h,
 * linux/ioport.h, and asm/pbm.h. */

#[repr(C)]
pub struct linux_pcic {
    pub pcic_regs: *mut core::ffi::c_void,
    pub pcic_io: c_ulong,
    pub pcic_config_space_addr: *mut core::ffi::c_void,
    pub pcic_config_space_data: *mut core::ffi::c_void,
    pub pcic_res_regs: resource,
    pub pcic_res_io: resource,
    pub pcic_res_cfg_addr: resource,
    pub pcic_res_cfg_data: resource,
    pub pbm: linux_pbm_info,
    pub pcic_imap: *mut pcic_ca2irq,
    pub pcic_imdim: c_int,
}

#[cfg(CONFIG_PCIC_PCI)]
extern "C" {
    pub fn pcic_present() -> c_int;
    pub fn pcic_probe() -> c_int;
    pub fn pci_time_init();
    pub fn sun4m_pci_init_IRQ();
}

#[cfg(not(CONFIG_PCIC_PCI))]
#[inline]
pub const fn pcic_present() -> c_int { 0 }

#[cfg(not(CONFIG_PCIC_PCI))]
#[inline]
pub const fn pcic_probe() -> c_int { 0 }

#[cfg(not(CONFIG_PCIC_PCI))]
#[inline]
pub const fn pci_time_init() {}

#[cfg(not(CONFIG_PCIC_PCI))]
#[inline]
pub const fn sun4m_pci_init_IRQ() {}

/* Size of PCI I/O space which we relocate. */
pub const PCI_SPACE_SIZE: u32 = 0x1000000; /* 16 MB */

/* PCIC Register Set. */
pub const PCI_DIAGNOSTIC_0: u32 = 0x40; /* 32 bits */
pub const PCI_SIZE_0: u32 = 0x44; /* 32 bits */
pub const PCI_SIZE_1: u32 = 0x48; /* 32 bits */
pub const PCI_SIZE_2: u32 = 0x4c; /* 32 bits */
pub const PCI_SIZE_3: u32 = 0x50; /* 32 bits */
pub const PCI_SIZE_4: u32 = 0x54; /* 32 bits */
pub const PCI_SIZE_5: u32 = 0x58; /* 32 bits */
pub const PCI_PIO_CONTROL: u32 = 0x60; /* 8 bits */
pub const PCI_DVMA_CONTROL: u32 = 0x62; /* 8 bits */
pub const PCI_DVMA_CONTROL_INACTIVITY_REQ: u32 = 1 << 0;
pub const PCI_DVMA_CONTROL_IOTLB_ENABLE: u32 = 1 << 0;
pub const PCI_DVMA_CONTROL_IOTLB_DISABLE: u32 = 0;
pub const PCI_DVMA_CONTROL_INACTIVITY_ACK: u32 = 1 << 4;
pub const PCI_INTERRUPT_CONTROL: u32 = 0x63; /* 8 bits */
pub const PCI_CPU_INTERRUPT_PENDING: u32 = 0x64; /* 32 bits */
pub const PCI_DIAGNOSTIC_1: u32 = 0x68; /* 16 bits */
pub const PCI_SOFTWARE_INT_CLEAR: u32 = 0x6a; /* 16 bits */
pub const PCI_SOFTWARE_INT_SET: u32 = 0x6e; /* 16 bits */
pub const PCI_SYS_INT_PENDING: u32 = 0x70; /* 32 bits */
pub const PCI_SYS_INT_PENDING_PIO: u32 = 0x40000000;
pub const PCI_SYS_INT_PENDING_DMA: u32 = 0x20000000;
pub const PCI_SYS_INT_PENDING_PCI: u32 = 0x10000000;
pub const PCI_SYS_INT_PENDING_APSR: u32 = 0x08000000;
pub const PCI_SYS_INT_TARGET_MASK: u32 = 0x74; /* 32 bits */
pub const PCI_SYS_INT_TARGET_MASK_CLEAR: u32 = 0x78; /* 32 bits */
pub const PCI_SYS_INT_TARGET_MASK_SET: u32 = 0x7c; /* 32 bits */
pub const PCI_SYS_INT_PENDING_CLEAR: u32 = 0x83; /* 8 bits */
pub const PCI_SYS_INT_PENDING_CLEAR_ALL: u32 = 0x80;
pub const PCI_SYS_INT_PENDING_CLEAR_PIO: u32 = 0x40;
pub const PCI_SYS_INT_PENDING_CLEAR_DMA: u32 = 0x20;
pub const PCI_SYS_INT_PENDING_CLEAR_PCI: u32 = 0x10;
pub const PCI_IOTLB_CONTROL: u32 = 0x84; /* 8 bits */
pub const PCI_INT_SELECT_LO: u32 = 0x88; /* 16 bits */
pub const PCI_ARBITRATION_SELECT: u32 = 0x8a; /* 16 bits */
pub const PCI_INT_SELECT_HI: u32 = 0x8c; /* 16 bits */
pub const PCI_HW_INT_OUTPUT: u32 = 0x8e; /* 16 bits */
pub const PCI_IOTLB_RAM_INPUT: u32 = 0x90; /* 32 bits */
pub const PCI_IOTLB_CAM_INPUT: u32 = 0x94; /* 32 bits */
pub const PCI_IOTLB_RAM_OUTPUT: u32 = 0x98; /* 32 bits */
pub const PCI_IOTLB_CAM_OUTPUT: u32 = 0x9c; /* 32 bits */
pub const PCI_SMBAR0: u32 = 0xa0; /* 8 bits */
pub const PCI_MSIZE0: u32 = 0xa1; /* 8 bits */
pub const PCI_PMBAR0: u32 = 0xa2; /* 8 bits */
pub const PCI_SMBAR1: u32 = 0xa4; /* 8 bits */
pub const PCI_MSIZE1: u32 = 0xa5; /* 8 bits */
pub const PCI_PMBAR1: u32 = 0xa6; /* 8 bits */
pub const PCI_SIBAR: u32 = 0xa8; /* 8 bits */
pub const PCI_SIBAR_ADDRESS_MASK: u32 = 0xf;
pub const PCI_ISIZE: u32 = 0xa9; /* 8 bits */
pub const PCI_ISIZE_16M: u32 = 0xf;
pub const PCI_ISIZE_32M: u32 = 0xe;
pub const PCI_ISIZE_64M: u32 = 0xc;
pub const PCI_ISIZE_128M: u32 = 0x8;
pub const PCI_ISIZE_256M: u32 = 0x0;
pub const PCI_PIBAR: u32 = 0xaa; /* 8 bits */
pub const PCI_CPU_COUNTER_LIMIT_HI: u32 = 0xac; /* 32 bits */
pub const PCI_CPU_COUNTER_LIMIT_LO: u32 = 0xb0; /* 32 bits */
pub const PCI_CPU_COUNTER_LIMIT: u32 = 0xb4; /* 32 bits */
pub const PCI_SYS_LIMIT: u32 = 0xb8; /* 32 bits */
pub const PCI_SYS_COUNTER: u32 = 0xbc; /* 32 bits */
pub const PCI_SYS_COUNTER_OVERFLOW: u32 = 1 << 31; /* Limit reached */
pub const PCI_SYS_LIMIT_PSEUDO: u32 = 0xc0; /* 32 bits */
pub const PCI_USER_TIMER_CONTROL: u32 = 0xc4; /* 8 bits */
pub const PCI_USER_TIMER_CONFIG: u32 = 0xc5; /* 8 bits */
pub const PCI_COUNTER_IRQ: u32 = 0xc6; /* 8 bits */

#[inline]
pub const fn PCI_COUNTER_IRQ_SET(sys_irq: u32, cpu_irq: u32) -> u32 {
    ((sys_irq & 0xf) << 4) | (cpu_irq & 0xf)
}
#[inline]
pub const fn PCI_COUNTER_IRQ_SYS(v: u32) -> u32 { (v >> 4) & 0xf }
#[inline]
pub const fn PCI_COUNTER_IRQ_CPU(v: u32) -> u32 { v & 0xf }

pub const PCI_PIO_ERROR_COMMAND: u32 = 0xc7; /* 8 bits */
pub const PCI_PIO_ERROR_ADDRESS: u32 = 0xc8; /* 32 bits */
pub const PCI_IOTLB_ERROR_ADDRESS: u32 = 0xcc; /* 32 bits */
pub const PCI_SYS_STATUS: u32 = 0xd0; /* 8 bits */
pub const PCI_SYS_STATUS_RESET_ENABLE: u32 = 1 << 0;
pub const PCI_SYS_STATUS_RESET: u32 = 1 << 1;
pub const PCI_SYS_STATUS_WATCHDOG_RESET: u32 = 1 << 4;
pub const PCI_SYS_STATUS_PCI_RESET: u32 = 1 << 5;
pub const PCI_SYS_STATUS_PCI_RESET_ENABLE: u32 = 1 << 6;
pub const PCI_SYS_STATUS_PCI_SATTELITE_MODE: u32 = 1 << 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
