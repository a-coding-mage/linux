/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies from:
// <linux/regmap.h>
// <linux/platform_data/adau17x1.h>
// "sigmadsp.h"

#[repr(C)]
pub enum adau17x1_type {
    ADAU1361,
    ADAU1761,
    ADAU1761_AS_1361,
    ADAU1381,
    ADAU1781,
}

#[repr(C)]
pub enum adau17x1_pll {
    ADAU17X1_PLL,
}

#[repr(C)]
pub enum adau17x1_pll_src {
    ADAU17X1_PLL_SRC_MCLK,
}

#[repr(C)]
pub enum adau17x1_clk_src {
    /* Automatically configure PLL based on the sample rate */
    ADAU17X1_CLK_SRC_PLL_AUTO,
    ADAU17X1_CLK_SRC_MCLK,
    ADAU17X1_CLK_SRC_PLL,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
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
pub struct sigmadsp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub enum adau17x1_micbias_voltage {
    _bindgen_placeholder = 0,
}

#[repr(C)]
pub struct adau {
    pub sysclk: ::std::os::raw::c_uint,
    pub pll_freq: ::std::os::raw::c_uint,
    pub mclk: *mut clk,

    pub clk_src: adau17x1_clk_src,
    pub type_: adau17x1_type,
    pub switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,

    pub dai_fmt: ::std::os::raw::c_uint,

    pub pll_regs: [u8; 6],

    pub master: bool,

    pub tdm_slot: [::std::os::raw::c_uint; 2],
    pub dsp_bypass: [bool; 2],

    pub regmap: *mut regmap,
    pub sigmadsp: *mut sigmadsp,
}

unsafe extern "C" {
    pub fn adau17x1_add_widgets(component: *mut snd_soc_component) -> ::std::os::raw::c_int;
    pub fn adau17x1_add_routes(component: *mut snd_soc_component) -> ::std::os::raw::c_int;
    pub fn adau17x1_probe(
        dev: *mut device,
        regmap: *mut regmap,
        type_: adau17x1_type,
        switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,
        firmware_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn adau17x1_remove(dev: *mut device);
    pub fn adau17x1_set_micbias_voltage(
        component: *mut snd_soc_component,
        micbias: adau17x1_micbias_voltage,
    ) -> ::std::os::raw::c_int;
    pub fn adau17x1_readable_register(dev: *mut device, reg: ::std::os::raw::c_uint) -> bool;
    pub fn adau17x1_volatile_register(dev: *mut device, reg: ::std::os::raw::c_uint) -> bool;
    pub fn adau17x1_precious_register(dev: *mut device, reg: ::std::os::raw::c_uint) -> bool;
    pub fn adau17x1_resume(component: *mut snd_soc_component) -> ::std::os::raw::c_int;

    pub static adau17x1_dai_ops: snd_soc_dai_ops;
}

pub const ADAU17X1_CLOCK_CONTROL: ::std::os::raw::c_uint = 0x4000;
pub const ADAU17X1_PLL_CONTROL: ::std::os::raw::c_uint = 0x4002;
pub const ADAU17X1_REC_POWER_MGMT: ::std::os::raw::c_uint = 0x4009;
pub const ADAU17X1_MICBIAS: ::std::os::raw::c_uint = 0x4010;
pub const ADAU17X1_SERIAL_PORT0: ::std::os::raw::c_uint = 0x4015;
pub const ADAU17X1_SERIAL_PORT1: ::std::os::raw::c_uint = 0x4016;
pub const ADAU17X1_CONVERTER0: ::std::os::raw::c_uint = 0x4017;
pub const ADAU17X1_CONVERTER1: ::std::os::raw::c_uint = 0x4018;
pub const ADAU17X1_LEFT_INPUT_DIGITAL_VOL: ::std::os::raw::c_uint = 0x401a;
pub const ADAU17X1_RIGHT_INPUT_DIGITAL_VOL: ::std::os::raw::c_uint = 0x401b;
pub const ADAU17X1_ADC_CONTROL: ::std::os::raw::c_uint = 0x4019;
pub const ADAU17X1_PLAY_POWER_MGMT: ::std::os::raw::c_uint = 0x4029;
pub const ADAU17X1_DAC_CONTROL0: ::std::os::raw::c_uint = 0x402a;
pub const ADAU17X1_DAC_CONTROL1: ::std::os::raw::c_uint = 0x402b;
pub const ADAU17X1_DAC_CONTROL2: ::std::os::raw::c_uint = 0x402c;
pub const ADAU17X1_SERIAL_PORT_PAD: ::std::os::raw::c_uint = 0x402d;
pub const ADAU17X1_CONTROL_PORT_PAD0: ::std::os::raw::c_uint = 0x402f;
pub const ADAU17X1_CONTROL_PORT_PAD1: ::std::os::raw::c_uint = 0x4030;
pub const ADAU17X1_DSP_SAMPLING_RATE: ::std::os::raw::c_uint = 0x40eb;
pub const ADAU17X1_SERIAL_INPUT_ROUTE: ::std::os::raw::c_uint = 0x40f2;
pub const ADAU17X1_SERIAL_OUTPUT_ROUTE: ::std::os::raw::c_uint = 0x40f3;
pub const ADAU17X1_DSP_ENABLE: ::std::os::raw::c_uint = 0x40f5;
pub const ADAU17X1_DSP_RUN: ::std::os::raw::c_uint = 0x40f6;
pub const ADAU17X1_SERIAL_SAMPLING_RATE: ::std::os::raw::c_uint = 0x40f8;

pub const ADAU17X1_SERIAL_PORT0_BCLK_POL: ::std::os::raw::c_uint = 1 << 4;
pub const ADAU17X1_SERIAL_PORT0_LRCLK_POL: ::std::os::raw::c_uint = 1 << 3;
pub const ADAU17X1_SERIAL_PORT0_MASTER: ::std::os::raw::c_uint = 1 << 0;

pub const ADAU17X1_SERIAL_PORT1_DELAY1: ::std::os::raw::c_uint = 0x00;
pub const ADAU17X1_SERIAL_PORT1_DELAY0: ::std::os::raw::c_uint = 0x01;
pub const ADAU17X1_SERIAL_PORT1_DELAY8: ::std::os::raw::c_uint = 0x02;
pub const ADAU17X1_SERIAL_PORT1_DELAY16: ::std::os::raw::c_uint = 0x03;
pub const ADAU17X1_SERIAL_PORT1_DELAY_MASK: ::std::os::raw::c_uint = 0x03;

pub const ADAU17X1_CLOCK_CONTROL_INFREQ_MASK: ::std::os::raw::c_uint = 0x6;
pub const ADAU17X1_CLOCK_CONTROL_CORECLK_SRC_PLL: ::std::os::raw::c_uint = 1 << 3;
pub const ADAU17X1_CLOCK_CONTROL_SYSCLK_EN: ::std::os::raw::c_uint = 1 << 0;

pub const ADAU17X1_SERIAL_PORT1_BCLK64: ::std::os::raw::c_uint = 0x0 << 5;
pub const ADAU17X1_SERIAL_PORT1_BCLK32: ::std::os::raw::c_uint = 0x1 << 5;
pub const ADAU17X1_SERIAL_PORT1_BCLK48: ::std::os::raw::c_uint = 0x2 << 5;
pub const ADAU17X1_SERIAL_PORT1_BCLK128: ::std::os::raw::c_uint = 0x3 << 5;
pub const ADAU17X1_SERIAL_PORT1_BCLK256: ::std::os::raw::c_uint = 0x4 << 5;
pub const ADAU17X1_SERIAL_PORT1_BCLK_MASK: ::std::os::raw::c_uint = 0x7 << 5;

pub const ADAU17X1_SERIAL_PORT0_STEREO: ::std::os::raw::c_uint = 0x0 << 1;
pub const ADAU17X1_SERIAL_PORT0_TDM4: ::std::os::raw::c_uint = 0x1 << 1;
pub const ADAU17X1_SERIAL_PORT0_TDM8: ::std::os::raw::c_uint = 0x2 << 1;
pub const ADAU17X1_SERIAL_PORT0_TDM_MASK: ::std::os::raw::c_uint = 0x3 << 1;
pub const ADAU17X1_SERIAL_PORT0_PULSE_MODE: ::std::os::raw::c_uint = 1 << 5;

pub const fn ADAU17X1_CONVERTER0_DAC_PAIR(x: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint {
    (x.wrapping_sub(1)) << 5
}
pub const ADAU17X1_CONVERTER0_DAC_PAIR_MASK: ::std::os::raw::c_uint = 0x3 << 5;
pub const fn ADAU17X1_CONVERTER1_ADC_PAIR(x: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint {
    x.wrapping_sub(1)
}
pub const ADAU17X1_CONVERTER1_ADC_PAIR_MASK: ::std::os::raw::c_uint = 0x3;

pub const ADAU17X1_CONVERTER0_CONVSR_MASK: ::std::os::raw::c_uint = 0x7;

pub const ADAU17X1_CONVERTER0_ADOSR: ::std::os::raw::c_uint = 1 << 3;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
