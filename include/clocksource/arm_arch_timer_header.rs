/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by other translation units/headers:
// `timecounter`, `phys_addr_t`, `size_t`, `u32`, `u64`, `bool`, and `USEC_PER_SEC`.

pub const ARCH_TIMER_CTRL_ENABLE: i32 = 1 << 0;
pub const ARCH_TIMER_CTRL_IT_MASK: i32 = 1 << 1;
pub const ARCH_TIMER_CTRL_IT_STAT: i32 = 1 << 2;

pub const CNTHCTL_EL1PCTEN: i32 = 1 << 0;
pub const CNTHCTL_EL1PCEN: i32 = 1 << 1;
pub const CNTHCTL_EVNTEN: i32 = 1 << 2;
pub const CNTHCTL_EVNTDIR: i32 = 1 << 3;
pub const CNTHCTL_EVNTI: i32 = 0xF << 4;
pub const CNTHCTL_ECV: i32 = 1 << 12;
pub const CNTHCTL_EL1TVT: i32 = 1 << 13;
pub const CNTHCTL_EL1TVCT: i32 = 1 << 14;
pub const CNTHCTL_EL1NVPCT: i32 = 1 << 15;
pub const CNTHCTL_EL1NVVCT: i32 = 1 << 16;
pub const CNTHCTL_CNTVMASK: i32 = 1 << 18;
pub const CNTHCTL_CNTPMASK: i32 = 1 << 19;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum arch_timer_reg {
    ARCH_TIMER_REG_CTRL,
    ARCH_TIMER_REG_CVAL,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum arch_timer_ppi_nr {
    ARCH_TIMER_PHYS_SECURE_PPI,
    ARCH_TIMER_PHYS_NONSECURE_PPI,
    ARCH_TIMER_VIRT_PPI,
    ARCH_TIMER_HYP_PPI,
    ARCH_TIMER_HYP_VIRT_PPI,
    ARCH_TIMER_MAX_TIMER_PPI,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum arch_timer_spi_nr {
    ARCH_TIMER_PHYS_SPI,
    ARCH_TIMER_VIRT_SPI,
    ARCH_TIMER_MAX_TIMER_SPI,
}

pub const ARCH_TIMER_PHYS_ACCESS: i32 = 0;
pub const ARCH_TIMER_VIRT_ACCESS: i32 = 1;
pub const ARCH_TIMER_MEM_MAX_FRAMES: usize = 8;

pub const ARCH_TIMER_USR_PCT_ACCESS_EN: i32 = 1 << 0; // physical counter
pub const ARCH_TIMER_USR_VCT_ACCESS_EN: i32 = 1 << 1; // virtual counter
pub const ARCH_TIMER_VIRT_EVT_EN: i32 = 1 << 2;
pub const ARCH_TIMER_EVT_TRIGGER_SHIFT: i32 = 4;
pub const ARCH_TIMER_EVT_TRIGGER_MASK: i32 = 0xF << ARCH_TIMER_EVT_TRIGGER_SHIFT;
pub const ARCH_TIMER_USR_VT_ACCESS_EN: i32 = 1 << 8; // virtual timer registers
pub const ARCH_TIMER_USR_PT_ACCESS_EN: i32 = 1 << 9; // physical timer registers
pub const ARCH_TIMER_EVT_INTERVAL_SCALE: i32 = 1 << 17; // EVNTIS in the ARMv8 ARM

pub const ARCH_TIMER_EVT_STREAM_PERIOD_US: i32 = 100;
pub const ARCH_TIMER_EVT_STREAM_FREQ: _ = USEC_PER_SEC / ARCH_TIMER_EVT_STREAM_PERIOD_US;

#[repr(C)]
pub struct arch_timer_kvm_info {
    pub timecounter: timecounter,
    pub virtual_irq: i32,
    pub physical_irq: i32,
}

#[repr(C)]
pub struct arch_timer_mem_frame {
    pub valid: bool,
    pub cntbase: phys_addr_t,
    pub size: size_t,
    pub phys_irq: i32,
    pub virt_irq: i32,
}

#[repr(C)]
pub struct arch_timer_mem {
    pub cntctlbase: phys_addr_t,
    pub size: size_t,
    pub frame: [arch_timer_mem_frame; ARCH_TIMER_MEM_MAX_FRAMES],
}

// CONFIG_ARM_ARCH_TIMER
#[cfg(CONFIG_ARM_ARCH_TIMER)]
extern "C" {
    pub fn arch_timer_get_rate() -> u32;
    pub static mut arch_timer_read_counter: unsafe extern "C" fn() -> u64;
    pub fn arch_timer_get_kvm_info() -> *mut arch_timer_kvm_info;
    pub fn arch_timer_evtstrm_available() -> bool;
}

// Fallback definitions when CONFIG_ARM_ARCH_TIMER is not enabled.
#[cfg(not(CONFIG_ARM_ARCH_TIMER))]
#[inline]
pub fn arch_timer_get_rate() -> u32 {
    0
}

#[cfg(not(CONFIG_ARM_ARCH_TIMER))]
#[inline]
pub unsafe fn arch_timer_read_counter() -> u64 {
    0
}

#[cfg(not(CONFIG_ARM_ARCH_TIMER))]
#[inline]
pub fn arch_timer_evtstrm_available() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
