/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) STMicroelectronics 2017
 *
 * Author: Fabrice Gasnier <fabrice.gasnier@st.com>
 */

// Dependencies supplied by the Linux IIO subsystem are intentionally not
// implemented here; this is the Rust equivalent of the C forward declaration.
#[repr(C)]
pub struct iio_trigger {
    _private: [u8; 0],
}

pub const LPTIM1_OUT: &str = "lptim1_out";
pub const LPTIM2_OUT: &str = "lptim2_out";
pub const LPTIM3_OUT: &str = "lptim3_out";
pub const LPTIM4_OUT: &str = "lptim4_out";
pub const LPTIM5_OUT: &str = "lptim5_out";

pub const LPTIM1_CH1: &str = "lptim1_ch1";
pub const LPTIM1_CH2: &str = "lptim1_ch2";
pub const LPTIM2_CH1: &str = "lptim2_ch1";
pub const LPTIM2_CH2: &str = "lptim2_ch2";
pub const LPTIM3_CH1: &str = "lptim3_ch1";
pub const LPTIM4_CH1: &str = "lptim4_ch1";

// When CONFIG_IIO_STM32_LPTIMER_TRIGGER is reachable, the implementation is
// provided by the linked Linux IIO trigger code.
#[cfg(feature = "CONFIG_IIO_STM32_LPTIMER_TRIGGER")]
unsafe extern "C" {
    pub fn is_stm32_lptim_trigger(trig: *mut iio_trigger) -> bool;
}

// Otherwise this is the C static-inline fallback.  The CONFIG_IIO... enabled
// warning is represented by the original intent in this dependency-free file.
#[cfg(not(feature = "CONFIG_IIO_STM32_LPTIMER_TRIGGER"))]
#[inline]
pub unsafe fn is_stm32_lptim_trigger(_trig: *mut iio_trigger) -> bool {
    // pr_warn_once("stm32 lptim_trigger not linked in\n") when the optional
    // CONFIG_IIO_STM32_LPTIMER_TRIGGER support is enabled.
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
