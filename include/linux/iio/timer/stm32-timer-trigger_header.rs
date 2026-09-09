/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) STMicroelectronics 2016
 *
 * Author: Benjamin Gaignard <benjamin.gaignard@st.com>
 */

// Dependency intent: the C header's IS_REACHABLE/IS_ENABLED configuration
// macros and kernel declarations are supplied by the surrounding build.

pub const TIM1_TRGO: &str = "tim1_trgo";
pub const TIM1_TRGO2: &str = "tim1_trgo2";
pub const TIM1_CH1: &str = "tim1_ch1";
pub const TIM1_CH2: &str = "tim1_ch2";
pub const TIM1_CH3: &str = "tim1_ch3";
pub const TIM1_CH4: &str = "tim1_ch4";

pub const TIM2_TRGO: &str = "tim2_trgo";
pub const TIM2_CH1: &str = "tim2_ch1";
pub const TIM2_CH2: &str = "tim2_ch2";
pub const TIM2_CH3: &str = "tim2_ch3";
pub const TIM2_CH4: &str = "tim2_ch4";

pub const TIM3_TRGO: &str = "tim3_trgo";
pub const TIM3_CH1: &str = "tim3_ch1";
pub const TIM3_CH2: &str = "tim3_ch2";
pub const TIM3_CH3: &str = "tim3_ch3";
pub const TIM3_CH4: &str = "tim3_ch4";

pub const TIM4_TRGO: &str = "tim4_trgo";
pub const TIM4_CH1: &str = "tim4_ch1";
pub const TIM4_CH2: &str = "tim4_ch2";
pub const TIM4_CH3: &str = "tim4_ch3";
pub const TIM4_CH4: &str = "tim4_ch4";

pub const TIM5_TRGO: &str = "tim5_trgo";
pub const TIM5_CH1: &str = "tim5_ch1";
pub const TIM5_CH2: &str = "tim5_ch2";
pub const TIM5_CH3: &str = "tim5_ch3";
pub const TIM5_CH4: &str = "tim5_ch4";

pub const TIM6_TRGO: &str = "tim6_trgo";
pub const TIM7_TRGO: &str = "tim7_trgo";

pub const TIM8_TRGO: &str = "tim8_trgo";
pub const TIM8_TRGO2: &str = "tim8_trgo2";
pub const TIM8_CH1: &str = "tim8_ch1";
pub const TIM8_CH2: &str = "tim8_ch2";
pub const TIM8_CH3: &str = "tim8_ch3";
pub const TIM8_CH4: &str = "tim8_ch4";

pub const TIM9_TRGO: &str = "tim9_trgo";
pub const TIM9_CH1: &str = "tim9_ch1";
pub const TIM9_CH2: &str = "tim9_ch2";

pub const TIM10_OC1: &str = "tim10_oc1";
pub const TIM11_OC1: &str = "tim11_oc1";

pub const TIM12_TRGO: &str = "tim12_trgo";
pub const TIM12_CH1: &str = "tim12_ch1";
pub const TIM12_CH2: &str = "tim12_ch2";

pub const TIM13_OC1: &str = "tim13_oc1";
pub const TIM14_OC1: &str = "tim14_oc1";
pub const TIM15_TRGO: &str = "tim15_trgo";
pub const TIM16_OC1: &str = "tim16_oc1";
pub const TIM17_OC1: &str = "tim17_oc1";

pub const TIM20_OC1: &str = "tim20_oc1";
pub const TIM20_OC2: &str = "tim20_oc2";
pub const TIM20_OC3: &str = "tim20_oc3";
pub const TIM20_TRGO: &str = "tim20_trgo";
pub const TIM20_TRGO2: &str = "tim20_trgo2";

// When CONFIG_IIO_STM32_TIMER_TRIGGER is reachable, this is an external
// function supplied by the STM32 timer trigger implementation.
extern "C" {
    pub fn is_stm32_timer_trigger(trig: *mut iio_trigger) -> bool;
}

// When CONFIG_IIO_STM32_TIMER_TRIGGER is not reachable, the C header provides
// this inline fallback. If CONFIG_IIO_STM32_TIMER_TRIGGER is enabled, the
// original implementation also emits pr_warn_once; that kernel macro is an
// external dependency and is preserved here as a comment.
#[inline]
pub fn is_stm32_timer_trigger_unreachable(_trig: *mut iio_trigger) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
