/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: Flora Fu, MediaTek
 */

// C dependencies supplied by other headers/modules.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chip_id {
    MT6323_CHIP_ID = 0x23,
    MT6328_CHIP_ID = 0x28,
    MT6331_CHIP_ID = 0x31,
    MT6332_CHIP_ID = 0x32,
    MT6357_CHIP_ID = 0x57,
    MT6358_CHIP_ID = 0x58,
    MT6359_CHIP_ID = 0x59,
    MT6366_CHIP_ID = 0x66,
    MT6391_CHIP_ID = 0x91,
    MT6397_CHIP_ID = 0x97,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt6397_irq_numbers {
    MT6397_IRQ_SPKL_AB = 0,
    MT6397_IRQ_SPKR_AB,
    MT6397_IRQ_SPKL,
    MT6397_IRQ_SPKR,
    MT6397_IRQ_BAT_L,
    MT6397_IRQ_BAT_H,
    MT6397_IRQ_FG_BAT_L,
    MT6397_IRQ_FG_BAT_H,
    MT6397_IRQ_WATCHDOG,
    MT6397_IRQ_PWRKEY,
    MT6397_IRQ_THR_L,
    MT6397_IRQ_THR_H,
    MT6397_IRQ_VBATON_UNDET,
    MT6397_IRQ_BVALID_DET,
    MT6397_IRQ_CHRDET,
    MT6397_IRQ_OV,
    MT6397_IRQ_LDO,
    MT6397_IRQ_HOMEKEY,
    MT6397_IRQ_ACCDET,
    MT6397_IRQ_AUDIO,
    MT6397_IRQ_RTC,
    MT6397_IRQ_PWRKEY_RSTB,
    MT6397_IRQ_HDMI_SIFM,
    MT6397_IRQ_HDMI_CEC,
    MT6397_IRQ_VCA15,
    MT6397_IRQ_VSRMCA15,
    MT6397_IRQ_VCORE,
    MT6397_IRQ_VGPU,
    MT6397_IRQ_VIO18,
    MT6397_IRQ_VPCA7,
    MT6397_IRQ_VSRMCA7,
    MT6397_IRQ_VDRM,
    MT6397_IRQ_NR,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mt6397_chip {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pm_nb: notifier_block,
    pub irq: core::ffi::c_int,
    pub irq_domain: *mut irq_domain,
    pub irqlock: mutex,
    pub wake_mask: [u16; 3],
    pub irq_masks_cur: [u16; 3],
    pub irq_masks_cache: [u16; 3],
    pub int_con: [u16; 3],
    pub int_status: [u16; 3],
    pub chip_id: u16,
    pub irq_data: *mut core::ffi::c_void,
}

unsafe extern "C" {
    pub fn mt6358_irq_init(chip: *mut mt6397_chip) -> core::ffi::c_int;
    pub fn mt6397_irq_init(chip: *mut mt6397_chip) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
