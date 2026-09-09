/* SPDX-License-Identifier: GPL-2.0
 *
 * CS40L50 Advanced Haptic Driver with waveform memory,
 * integrated DSP, and closed-loop algorithms
 *
 * Copyright 2024 Cirrus Logic, Inc.
 *
 * Author: James Ogletree <james.ogletree@cirrus.com>
 */

// C dependencies:
// linux/firmware/cirrus/cs_dsp.h, linux/gpio/consumer.h, linux/pm.h,
// and linux/regmap.h are supplied by other translated units.

/* Power Supply Configuration */
pub const CS40L50_BLOCK_ENABLES2: u32 = 0x201C;
pub const CS40L50_ERR_RLS: u32 = 0x2034;
pub const CS40L50_BST_LPMODE_SEL: u32 = 0x3810;
pub const CS40L50_DCM_LOW_POWER: u32 = 0x1;
pub const CS40L50_OVERTEMP_WARN: u32 = 0x4000010;

/* Interrupts */
pub const CS40L50_IRQ1_INT_1: u32 = 0xE010;
pub const CS40L50_IRQ1_BASE: u32 = CS40L50_IRQ1_INT_1;
pub const CS40L50_IRQ1_INT_2: u32 = 0xE014;
pub const CS40L50_IRQ1_INT_8: u32 = 0xE02C;
pub const CS40L50_IRQ1_INT_9: u32 = 0xE030;
pub const CS40L50_IRQ1_INT_10: u32 = 0xE034;
pub const CS40L50_IRQ1_INT_18: u32 = 0xE054;
pub const CS40L50_IRQ1_MASK_1: u32 = 0xE090;
pub const CS40L50_IRQ1_MASK_2: u32 = 0xE094;
pub const CS40L50_IRQ1_MASK_20: u32 = 0xE0DC;
pub const CS40L50_IRQ1_INT_1_OFFSET: u32 = CS40L50_IRQ1_INT_1 - CS40L50_IRQ1_BASE;
pub const CS40L50_IRQ1_INT_2_OFFSET: u32 = CS40L50_IRQ1_INT_2 - CS40L50_IRQ1_BASE;
pub const CS40L50_IRQ1_INT_8_OFFSET: u32 = CS40L50_IRQ1_INT_8 - CS40L50_IRQ1_BASE;
pub const CS40L50_IRQ1_INT_9_OFFSET: u32 = CS40L50_IRQ1_INT_9 - CS40L50_IRQ1_BASE;
pub const CS40L50_IRQ1_INT_10_OFFSET: u32 = CS40L50_IRQ1_INT_10 - CS40L50_IRQ1_BASE;
pub const CS40L50_IRQ1_INT_18_OFFSET: u32 = CS40L50_IRQ1_INT_18 - CS40L50_IRQ1_BASE;
pub const CS40L50_IRQ_MASK_2_OVERRIDE: u32 = 0xFFDF7FFF;
pub const CS40L50_IRQ_MASK_20_OVERRIDE: u32 = 0x15C01000;
pub const CS40L50_AMP_SHORT_MASK: u32 = 1u32 << 31;
pub const CS40L50_DSP_QUEUE_MASK: u32 = 1u32 << 21;
pub const CS40L50_TEMP_ERR_MASK: u32 = 1u32 << 31;
pub const CS40L50_BST_UVP_MASK: u32 = 1u32 << 6;
pub const CS40L50_BST_SHORT_MASK: u32 = 1u32 << 7;
pub const CS40L50_BST_ILIMIT_MASK: u32 = 1u32 << 18;
pub const CS40L50_UVLO_VDDBATT_MASK: u32 = 1u32 << 16;
pub const CS40L50_GLOBAL_ERROR_MASK: u32 = 1u32 << 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cs40l50_irq_list {
    CS40L50_DSP_QUEUE_IRQ,
    CS40L50_GLOBAL_ERROR_IRQ,
    CS40L50_UVLO_VDDBATT_IRQ,
    CS40L50_BST_ILIMIT_IRQ,
    CS40L50_BST_SHORT_IRQ,
    CS40L50_BST_UVP_IRQ,
    CS40L50_TEMP_ERR_IRQ,
    CS40L50_AMP_SHORT_IRQ,
}

/* DSP */
pub const CS40L50_XMEM_PACKED_0: u32 = 0x2000000;
pub const CS40L50_XMEM_UNPACKED24_0: u32 = 0x2800000;
pub const CS40L50_SYS_INFO_ID: u32 = 0x25E0000;
pub const CS40L50_DSP_QUEUE_WT: u32 = 0x28042C8;
pub const CS40L50_DSP_QUEUE_RD: u32 = 0x28042CC;
pub const CS40L50_NUM_WAVES: u32 = 0x2805C18;
pub const CS40L50_CORE_BASE: u32 = 0x2B80000;
pub const CS40L50_YMEM_PACKED_0: u32 = 0x2C00000;
pub const CS40L50_YMEM_UNPACKED24_0: u32 = 0x3400000;
pub const CS40L50_PMEM_0: u32 = 0x3800000;
pub const CS40L50_DSP_POLL_US: u32 = 1000;
pub const CS40L50_DSP_TIMEOUT_COUNT: u32 = 100;
pub const CS40L50_RESET_PULSE_US: u32 = 2200;
pub const CS40L50_CP_READY_US: u32 = 3100;
pub const CS40L50_AUTOSUSPEND_MS: u32 = 2000;
pub const CS40L50_PM_ALGO: u32 = 0x9F206;
pub const CS40L50_GLOBAL_ERR_RLS_SET: u32 = 1u32 << 11;
pub const CS40L50_GLOBAL_ERR_RLS_CLEAR: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cs40l50_wseqs {
    CS40L50_PWR_ON,
    CS40L50_STANDBY,
    CS40L50_ACTIVE,
    CS40L50_NUM_WSEQS,
}

/* DSP Queue */
pub const CS40L50_DSP_QUEUE_BASE: u32 = 0x11004;
pub const CS40L50_DSP_QUEUE_END: u32 = 0x1101C;
pub const CS40L50_DSP_QUEUE: u32 = 0x11020;
pub const CS40L50_PREVENT_HIBER: u32 = 0x2000003;
pub const CS40L50_ALLOW_HIBER: u32 = 0x2000004;
pub const CS40L50_SHUTDOWN: u32 = 0x2000005;
pub const CS40L50_SYSTEM_RESET: u32 = 0x2000007;
pub const CS40L50_START_I2S: u32 = 0x3000002;
pub const CS40L50_OWT_PUSH: u32 = 0x3000008;
pub const CS40L50_STOP_PLAYBACK: u32 = 0x5000000;
pub const CS40L50_OWT_DELETE: u32 = 0xD000000;

/* Firmware files */
pub const CS40L50_FW: &str = "cs40l50.wmfw";
pub const CS40L50_WT: &str = "cs40l50.bin";

/* Device */
pub const CS40L50_DEVID: u32 = 0x0;
pub const CS40L50_REVID: u32 = 0x4;
pub const CS40L50_DEVID_A: u32 = 0x40A50;
pub const CS40L50_REVID_B0: u32 = 0xB0;

#[repr(C)]
pub struct cs40l50 {
    pub dev: *mut crate::device,
    pub regmap: *mut crate::regmap,
    pub lock: crate::mutex,
    pub dsp: crate::cs_dsp,
    pub reset_gpio: *mut crate::gpio_desc,
    pub irq_data: *mut crate::regmap_irq_chip_data,
    pub fw: *const crate::firmware,
    pub bin: *const crate::firmware,
    pub wseqs: [crate::cs_dsp_wseq; CS40L50_NUM_WSEQS as usize],
    pub irq: core::ffi::c_int,
    pub devid: u32,
    pub revid: u32,
}

unsafe extern "C" {
    pub fn cs40l50_dsp_write(dev: *mut crate::device, regmap: *mut crate::regmap, val: u32) -> core::ffi::c_int;
    pub fn cs40l50_probe(cs40l50: *mut cs40l50) -> core::ffi::c_int;
    pub fn cs40l50_remove(cs40l50: *mut cs40l50) -> core::ffi::c_int;

    pub static cs40l50_regmap: crate::regmap_config;
    pub static cs40l50_pm_ops: crate::dev_pm_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
