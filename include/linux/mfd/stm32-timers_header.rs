/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) STMicroelectronics 2016
 * Author: Benjamin Gaignard <benjamin.gaignard@st.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left unresolved here: linux/clk.h, linux/dmaengine.h, linux/dma-mapping.h,
// and linux/regmap.h.

pub const TIM_CR1: u32 = 0x00;
pub const TIM_CR2: u32 = 0x04;
pub const TIM_SMCR: u32 = 0x08;
pub const TIM_DIER: u32 = 0x0C;
pub const TIM_SR: u32 = 0x10;
pub const TIM_EGR: u32 = 0x14;
pub const TIM_CCMR1: u32 = 0x18;
pub const TIM_CCMR2: u32 = 0x1C;
pub const TIM_CCER: u32 = 0x20;
pub const TIM_CNT: u32 = 0x24;
pub const TIM_PSC: u32 = 0x28;
pub const TIM_ARR: u32 = 0x2c;
macro_rules! TIM_CCRx { ($x:expr) => { 0x34 + 4 * (($x) - 1) }; }
pub const TIM_CCR1: u32 = TIM_CCRx!(1);
pub const TIM_CCR2: u32 = TIM_CCRx!(2);
pub const TIM_CCR3: u32 = TIM_CCRx!(3);
pub const TIM_CCR4: u32 = TIM_CCRx!(4);
pub const TIM_BDTR: u32 = 0x44;
pub const TIM_DCR: u32 = 0x48;
pub const TIM_DMAR: u32 = 0x4C;
pub const TIM_TISEL: u32 = 0x68;
pub const TIM_HWCFGR2: u32 = 0x3EC;
pub const TIM_HWCFGR1: u32 = 0x3F0;
pub const TIM_IPIDR: u32 = 0x3F8;

pub const TIM_CR1_CEN: u32 = BIT(0);
pub const TIM_CR1_DIR: u32 = BIT(4);
pub const TIM_CR1_ARPE: u32 = BIT(7);
pub const TIM_CR2_MMS: u32 = BIT(4) | BIT(5) | BIT(6);
pub const TIM_CR2_MMS2: u32 = GENMASK(23, 20);
pub const TIM_SMCR_SMS: u32 = BIT(0) | BIT(1) | BIT(2);
pub const TIM_SMCR_TS: u32 = BIT(4) | BIT(5) | BIT(6);
pub const TIM_DIER_UIE: u32 = BIT(0);
macro_rules! TIM_DIER_CCxIE { ($x:expr) => { BIT(1 + (($x) - 1)) }; }
pub const TIM_DIER_CC1IE: u32 = TIM_DIER_CCxIE!(1);
pub const TIM_DIER_CC2IE: u32 = TIM_DIER_CCxIE!(2);
pub const TIM_DIER_CC3IE: u32 = TIM_DIER_CCxIE!(3);
pub const TIM_DIER_CC4IE: u32 = TIM_DIER_CCxIE!(4);
pub const TIM_DIER_UDE: u32 = BIT(8);
macro_rules! TIM_DIER_CCxDE { ($x:expr) => { BIT(9 + (($x) - 1)) }; }
pub const TIM_DIER_CC1DE: u32 = TIM_DIER_CCxDE!(1);
pub const TIM_DIER_CC2DE: u32 = TIM_DIER_CCxDE!(2);
pub const TIM_DIER_CC3DE: u32 = TIM_DIER_CCxDE!(3);
pub const TIM_DIER_CC4DE: u32 = TIM_DIER_CCxDE!(4);
pub const TIM_DIER_COMDE: u32 = BIT(13);
pub const TIM_DIER_TDE: u32 = BIT(14);
pub const TIM_SR_UIF: u32 = BIT(0);
macro_rules! TIM_SR_CC_IF { ($x:expr) => { BIT(($x) + 1) }; }
pub const TIM_EGR_UG: u32 = BIT(0);
pub const TIM_CCMR_PE: u32 = BIT(3);
pub const TIM_CCMR_M1: u32 = BIT(6) | BIT(5);
pub const TIM_CCMR_CC1S: u32 = BIT(0) | BIT(1);
pub const TIM_CCMR_IC1PSC: u32 = GENMASK(3, 2);
pub const TIM_CCMR_CC2S: u32 = BIT(8) | BIT(9);
pub const TIM_CCMR_IC2PSC: u32 = GENMASK(11, 10);
pub const TIM_CCMR_CC1S_TI1: u32 = BIT(0);
pub const TIM_CCMR_CC1S_TI2: u32 = BIT(1);
pub const TIM_CCMR_CC2S_TI2: u32 = BIT(8);
pub const TIM_CCMR_CC2S_TI1: u32 = BIT(9);
pub const TIM_CCMR_CC3S: u32 = BIT(0) | BIT(1);
pub const TIM_CCMR_CC4S: u32 = BIT(8) | BIT(9);
pub const TIM_CCMR_CC3S_TI3: u32 = BIT(0);
pub const TIM_CCMR_CC4S_TI4: u32 = BIT(8);
macro_rules! TIM_CCER_CCxE { ($x:expr) => { BIT(4 * (($x) - 1)) }; }
macro_rules! TIM_CCER_CCxP { ($x:expr) => { BIT(1 + 4 * (($x) - 1)) }; }
macro_rules! TIM_CCER_CCxNE { ($x:expr) => { BIT(2 + 4 * (($x) - 1)) }; }
macro_rules! TIM_CCER_CCxNP { ($x:expr) => { BIT(3 + 4 * (($x) - 1)) }; }
pub const TIM_CCER_CC1E: u32 = TIM_CCER_CCxE!(1);
pub const TIM_CCER_CC1P: u32 = TIM_CCER_CCxP!(1);
pub const TIM_CCER_CC1NE: u32 = TIM_CCER_CCxNE!(1);
pub const TIM_CCER_CC1NP: u32 = TIM_CCER_CCxNP!(1);
pub const TIM_CCER_CC2E: u32 = TIM_CCER_CCxE!(2);
pub const TIM_CCER_CC2P: u32 = TIM_CCER_CCxP!(2);
pub const TIM_CCER_CC2NE: u32 = TIM_CCER_CCxNE!(2);
pub const TIM_CCER_CC2NP: u32 = TIM_CCER_CCxNP!(2);
pub const TIM_CCER_CC3E: u32 = TIM_CCER_CCxE!(3);
pub const TIM_CCER_CC3P: u32 = TIM_CCER_CCxP!(3);
pub const TIM_CCER_CC3NE: u32 = TIM_CCER_CCxNE!(3);
pub const TIM_CCER_CC3NP: u32 = TIM_CCER_CCxNP!(3);
pub const TIM_CCER_CC4E: u32 = TIM_CCER_CCxE!(4);
pub const TIM_CCER_CC4P: u32 = TIM_CCER_CCxP!(4);
pub const TIM_CCER_CC4NE: u32 = TIM_CCER_CCxNE!(4);
pub const TIM_CCER_CC4NP: u32 = TIM_CCER_CCxNP!(4);
pub const TIM_CCER_CCXE: u32 = BIT(0) | BIT(4) | BIT(8) | BIT(12);
macro_rules! TIM_BDTR_BKE { ($x:expr) => { BIT(12 + ($x) * 12) }; }
macro_rules! TIM_BDTR_BKP { ($x:expr) => { BIT(13 + ($x) * 12) }; }
pub const TIM_BDTR_AOE: u32 = BIT(14);
pub const TIM_BDTR_MOE: u32 = BIT(15);
macro_rules! TIM_BDTR_BKF { ($x:expr) => { 0xf << (16 + ($x) * 4) }; }
pub const TIM_DCR_DBA: u32 = GENMASK(4, 0);
pub const TIM_DCR_DBL: u32 = GENMASK(12, 8);
pub const TIM_HWCFGR1_NB_OF_CC: u32 = GENMASK(3, 0);
pub const TIM_HWCFGR1_NB_OF_DT: u32 = GENMASK(7, 4);
pub const TIM_HWCFGR2_CNT_WIDTH: u32 = GENMASK(15, 8);

pub const MAX_TIM_PSC: u32 = 0xFFFF;
pub const MAX_TIM_ICPSC: u32 = 0x3;
pub const TIM_CR2_MMS_SHIFT: u32 = 4;
pub const TIM_CR2_MMS2_SHIFT: u32 = 20;
pub const TIM_SMCR_SMS_SLAVE_MODE_DISABLED: u32 = 0;
pub const TIM_SMCR_SMS_ENCODER_MODE_1: u32 = 1;
pub const TIM_SMCR_SMS_ENCODER_MODE_2: u32 = 2;
pub const TIM_SMCR_SMS_ENCODER_MODE_3: u32 = 3;
pub const TIM_SMCR_TS_SHIFT: u32 = 4;
pub const TIM_BDTR_BKF_MASK: u32 = 0xF;
macro_rules! TIM_BDTR_BKF_SHIFT { ($x:expr) => { 16 + ($x) * 4 }; }
pub const STM32MP25_TIM_IPIDR: u32 = 0x00120002;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum stm32_timers_dmas {
    STM32_TIMERS_DMA_CH1,
    STM32_TIMERS_DMA_CH2,
    STM32_TIMERS_DMA_CH3,
    STM32_TIMERS_DMA_CH4,
    STM32_TIMERS_DMA_UP,
    STM32_TIMERS_DMA_TRIG,
    STM32_TIMERS_DMA_COM,
    STM32_TIMERS_MAX_DMAS,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum stm32_timers_irqs {
    STM32_TIMERS_IRQ_GLOBAL_BRK,
    STM32_TIMERS_IRQ_UP,
    STM32_TIMERS_IRQ_TRG_COM,
    STM32_TIMERS_IRQ_CC,
    STM32_TIMERS_MAX_IRQS,
}

#[repr(C)]
pub struct stm32_timers_dma {
    pub completion: completion,
    pub phys_base: phys_addr_t,
    pub lock: mutex,
    pub chan: *mut dma_chan,
    pub chans: [*mut dma_chan; STM32_TIMERS_MAX_DMAS as usize],
}

#[repr(C)]
pub struct stm32_timers {
    pub clk: *mut clk,
    pub ipidr: u32,
    pub regmap: *mut regmap,
    pub max_arr: u32,
    pub dma: stm32_timers_dma,
    pub nr_irqs: core::ffi::c_uint,
    pub irq: [core::ffi::c_int; STM32_TIMERS_MAX_IRQS as usize],
}

// IS_REACHABLE(CONFIG_MFD_STM32_TIMERS) is a build-time kernel condition.
#[cfg(feature = "CONFIG_MFD_STM32_TIMERS")]
extern "C" {
    pub fn stm32_timers_dma_burst_read(
        dev: *mut device,
        buf: *mut u32,
        id: stm32_timers_dmas,
        reg: u32,
        num_reg: core::ffi::c_uint,
        bursts: core::ffi::c_uint,
        tmo_ms: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_MFD_STM32_TIMERS"))]
#[inline]
pub unsafe fn stm32_timers_dma_burst_read(
    _dev: *mut device,
    _buf: *mut u32,
    _id: stm32_timers_dmas,
    _reg: u32,
    _num_reg: core::ffi::c_uint,
    _bursts: core::ffi::c_uint,
    _tmo_ms: core::ffi::c_ulong,
) -> core::ffi::c_int {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
