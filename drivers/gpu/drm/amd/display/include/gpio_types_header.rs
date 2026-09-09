/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

pub const BUNDLE_A_MASK: u32 = 0x00FFF000;
pub const BUNDLE_B_MASK: u32 = 0x00000FFF;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_result {
    GPIO_RESULT_OK,
    GPIO_RESULT_NULL_HANDLE,
    GPIO_RESULT_INVALID_DATA,
    GPIO_RESULT_DEVICE_BUSY,
    GPIO_RESULT_OPEN_FAILED,
    GPIO_RESULT_ALREADY_OPENED,
    GPIO_RESULT_NON_SPECIFIC_ERROR,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_id {
    GPIO_ID_UNKNOWN = -1,
    GPIO_ID_DDC_DATA,
    GPIO_ID_DDC_CLOCK,
    GPIO_ID_GENERIC,
    GPIO_ID_HPD,
    GPIO_ID_GPIO_PAD,
    GPIO_ID_VIP_PAD,
    GPIO_ID_SYNC,
    GPIO_ID_GSL,
    GPIO_ID_COUNT,
    GPIO_ID_MIN = GPIO_ID_DDC_DATA as isize,
    GPIO_ID_MAX = GPIO_ID_GSL as isize,
}

pub const GPIO_ENUM_UNKNOWN: i32 = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_pin_info {
    pub offset: u32,
    pub offset_y: u32,
    pub offset_en: u32,
    pub offset_mask: u32,
    pub mask: u32,
    pub mask_y: u32,
    pub mask_en: u32,
    pub mask_mask: u32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_pin_output_state {
    GPIO_PIN_OUTPUT_STATE_ACTIVE_LOW,
    GPIO_PIN_OUTPUT_STATE_ACTIVE_HIGH,
    GPIO_PIN_OUTPUT_STATE_DEFAULT = GPIO_PIN_OUTPUT_STATE_ACTIVE_LOW as isize,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_generic { GPIO_GENERIC_UNKNOWN = -1, GPIO_GENERIC_A, GPIO_GENERIC_B, GPIO_GENERIC_C, GPIO_GENERIC_D, GPIO_GENERIC_E, GPIO_GENERIC_F, GPIO_GENERIC_G, GPIO_GENERIC_COUNT, GPIO_GENERIC_MIN = GPIO_GENERIC_A as isize, GPIO_GENERIC_MAX = GPIO_GENERIC_B as isize }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_hpd { GPIO_HPD_UNKNOWN = -1, GPIO_HPD_1, GPIO_HPD_2, GPIO_HPD_3, GPIO_HPD_4, GPIO_HPD_5, GPIO_HPD_6, GPIO_HPD_COUNT, GPIO_HPD_MIN = GPIO_HPD_1 as isize, GPIO_HPD_MAX = GPIO_HPD_6 as isize }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_gpio_pad {
    GPIO_GPIO_PAD_UNKNOWN = -1,
    GPIO_GPIO_PAD_0, GPIO_GPIO_PAD_1, GPIO_GPIO_PAD_2, GPIO_GPIO_PAD_3, GPIO_GPIO_PAD_4, GPIO_GPIO_PAD_5, GPIO_GPIO_PAD_6, GPIO_GPIO_PAD_7, GPIO_GPIO_PAD_8, GPIO_GPIO_PAD_9, GPIO_GPIO_PAD_10, GPIO_GPIO_PAD_11, GPIO_GPIO_PAD_12, GPIO_GPIO_PAD_13, GPIO_GPIO_PAD_14, GPIO_GPIO_PAD_15, GPIO_GPIO_PAD_16, GPIO_GPIO_PAD_17, GPIO_GPIO_PAD_18, GPIO_GPIO_PAD_19, GPIO_GPIO_PAD_20, GPIO_GPIO_PAD_21, GPIO_GPIO_PAD_22, GPIO_GPIO_PAD_23, GPIO_GPIO_PAD_24, GPIO_GPIO_PAD_25, GPIO_GPIO_PAD_26, GPIO_GPIO_PAD_27, GPIO_GPIO_PAD_28, GPIO_GPIO_PAD_29, GPIO_GPIO_PAD_30,
    GPIO_GPIO_PAD_COUNT, GPIO_GPIO_PAD_MIN = GPIO_GPIO_PAD_0 as isize, GPIO_GPIO_PAD_MAX = GPIO_GPIO_PAD_30 as isize,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_vip_pad { GPIO_VIP_PAD_UNKNOWN = -1, GPIO_VIP_PAD_SCL, GPIO_VIP_PAD_SDA, GPIO_VIP_PAD_VHAD, GPIO_VIP_PAD_VPHCTL, GPIO_VIP_PAD_VIPCLK, GPIO_VIP_PAD_VID, GPIO_VIP_PAD_VPCLK0, GPIO_VIP_PAD_DVALID, GPIO_VIP_PAD_PSYNC, GPIO_VIP_PAD_COUNT, GPIO_VIP_PAD_MIN = GPIO_VIP_PAD_SCL as isize, GPIO_VIP_PAD_MAX = GPIO_VIP_PAD_PSYNC as isize }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_sync { GPIO_SYNC_UNKNOWN = -1, GPIO_SYNC_HSYNC_A, GPIO_SYNC_VSYNC_A, GPIO_SYNC_HSYNC_B, GPIO_SYNC_VSYNC_B, GPIO_SYNC_COUNT, GPIO_SYNC_MIN = GPIO_SYNC_HSYNC_A as isize, GPIO_SYNC_MAX = GPIO_SYNC_VSYNC_B as isize }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_gsl { GPIO_GSL_UNKNOWN = -1, GPIO_GSL_GENLOCK_CLOCK, GPIO_GSL_GENLOCK_VSYNC, GPIO_GSL_SWAPLOCK_A, GPIO_GSL_SWAPLOCK_B, GPIO_GSL_COUNT, GPIO_GSL_MIN = GPIO_GSL_GENLOCK_CLOCK as isize, GPIO_GSL_MAX = GPIO_GSL_SWAPLOCK_B as isize }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_ddc_line { GPIO_DDC_LINE_UNKNOWN = -1, GPIO_DDC_LINE_DDC1, GPIO_DDC_LINE_DDC2, GPIO_DDC_LINE_DDC3, GPIO_DDC_LINE_DDC4, GPIO_DDC_LINE_DDC5, GPIO_DDC_LINE_DDC6, GPIO_DDC_LINE_DDC_VGA, GPIO_DDC_LINE_VIP_PAD, GPIO_DDC_LINE_I2C_PAD = GPIO_DDC_LINE_VIP_PAD as isize, GPIO_DDC_LINE_COUNT, GPIO_DDC_LINE_MIN = GPIO_DDC_LINE_DDC1 as isize, GPIO_DDC_LINE_MAX = GPIO_DDC_LINE_I2C_PAD as isize }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_mode { GPIO_MODE_UNKNOWN = -1, GPIO_MODE_INPUT, GPIO_MODE_OUTPUT, GPIO_MODE_FAST_OUTPUT, GPIO_MODE_HARDWARE, GPIO_MODE_INTERRUPT }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_signal_source { GPIO_SIGNAL_SOURCE_UNKNOWN = -1, GPIO_SIGNAL_SOURCE_DACA_STEREO_SYNC, GPIO_SIGNAL_SOURCE_PASS_THROUGH_STEREO_SYNC, GPIO_SIGNAL_SOURCE_DACB_STEREO_SYNC, GPIO_SIGNAL_SOURCE_DACA_HSYNC, GPIO_SIGNAL_SOURCE_DACB_HSYNC, GPIO_SIGNAL_SOURCE_DACA_VSYNC, GPIO_SIGNAL_SOURCE_DACB_VSYNC }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_stereo_source { GPIO_STEREO_SOURCE_UNKNOWN = -1, GPIO_STEREO_SOURCE_D1, GPIO_STEREO_SOURCE_D2, GPIO_STEREO_SOURCE_D3, GPIO_STEREO_SOURCE_D4, GPIO_STEREO_SOURCE_D5, GPIO_STEREO_SOURCE_D6 }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_config_type { GPIO_CONFIG_TYPE_NONE, GPIO_CONFIG_TYPE_DDC, GPIO_CONFIG_TYPE_HPD, GPIO_CONFIG_TYPE_GENERIC_MUX, GPIO_CONFIG_TYPE_GSL_MUX, GPIO_CONFIG_TYPE_I2C_AUX_DUAL_MODE }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_id_offset_entry { pub offset: u32, pub mask: u32, pub check_mask: bool, pub id: gpio_id, pub en: u32 }

#[macro_export]
macro_rules! GPIO_ENTRY { ($offset:expr, $id:expr, $en:expr) => { gpio_id_offset_entry { offset: REG!($offset), mask: 0, check_mask: false, id: $id, en: $en } }; }
#[macro_export]
macro_rules! GPIO_MASK_ENTRY { ($offset:expr, $mask:expr, $id:expr, $en:expr) => { gpio_id_offset_entry { offset: REG!($offset), mask: $mask, check_mask: true, id: $id, en: $en } }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_pin_entry { pub id: gpio_id, pub en: u32, pub offset: u32, pub mask: u32 }

#[macro_export]
macro_rules! GPIO_PIN_ENTRY { ($id:expr, $en:expr, $offset:expr, $mask:expr) => { gpio_pin_entry { id: $id, en: $en, offset: REG!($offset), mask: $mask } }; }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_ddc_config_type { GPIO_DDC_CONFIG_TYPE_MODE_AUX, GPIO_DDC_CONFIG_TYPE_MODE_I2C, GPIO_DDC_CONFIG_TYPE_POLL_FOR_CONNECT, GPIO_DDC_CONFIG_TYPE_POLL_FOR_DISCONNECT, GPIO_DDC_CONFIG_TYPE_DISABLE_POLLING }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_ddc_config { pub type_: gpio_ddc_config_type, pub data_en_bit_present: bool, pub clock_en_bit_present: bool }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_ddc_offset_entry { pub offset: u32, pub en: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_hpd_config { pub delay_on_connect: u32, pub delay_on_disconnect: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_generic_mux_config { pub enable_output_from_mux: bool, pub mux_select: gpio_signal_source, pub stereo_select: gpio_stereo_source }
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_gsl_mux_config_type { GPIO_GSL_MUX_CONFIG_TYPE_DISABLE, GPIO_GSL_MUX_CONFIG_TYPE_TIMING_SYNC, GPIO_GSL_MUX_CONFIG_TYPE_FLIP_SYNC }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_gsl_mux_config { pub type_: gpio_gsl_mux_config_type, pub gsl_group: u32 }
#[repr(C)]
pub union gpio_config_union { pub ddc: gpio_ddc_config, pub hpd: gpio_hpd_config, pub generic_mux: gpio_generic_mux_config, pub gsl_mux: gpio_gsl_mux_config }
#[repr(C)]
pub struct gpio_config_data { pub type_: gpio_config_type, pub config: gpio_config_union }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
