/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2009-2010 Pengutronix
 * Uwe Kleine-Koenig <u.kleine-koenig@pengutronix.de>
 */

// Dependency supplied externally: Linux interrupt types, device_node, and
// regulator_init_data declarations.

use core::ffi::{c_char, c_int, c_uint, c_ushort, c_void};

#[repr(C)]
pub struct mc13xxx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator_init_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

pub type irq_handler_t = unsafe extern "C" fn(c_int, *mut c_void) -> c_int;

extern "C" {
    pub fn mc13xxx_lock(mc13xxx: *mut mc13xxx);
    pub fn mc13xxx_unlock(mc13xxx: *mut mc13xxx);
    pub fn mc13xxx_reg_read(mc13xxx: *mut mc13xxx, offset: c_uint, val: *mut u32) -> c_int;
    pub fn mc13xxx_reg_write(mc13xxx: *mut mc13xxx, offset: c_uint, val: u32) -> c_int;
    pub fn mc13xxx_reg_rmw(mc13xxx: *mut mc13xxx, offset: c_uint, mask: u32, val: u32) -> c_int;
    pub fn mc13xxx_irq_request(mc13xxx: *mut mc13xxx, irq: c_int, handler: irq_handler_t, name: *const c_char, dev: *mut c_void) -> c_int;
    pub fn mc13xxx_irq_free(mc13xxx: *mut mc13xxx, irq: c_int, dev: *mut c_void) -> c_int;
    pub fn mc13xxx_irq_status(mc13xxx: *mut mc13xxx, irq: c_int, enabled: *mut c_int, pending: *mut c_int) -> c_int;
    pub fn mc13xxx_get_flags(mc13xxx: *mut mc13xxx) -> c_int;
    pub fn mc13xxx_adc_do_conversion(mc13xxx: *mut mc13xxx, mode: c_uint, channel: c_uint, ato: u8, atox: bool, sample: *mut c_uint) -> c_int;
    pub fn mc13xxx_irq_mask(mc13xxx: *mut mc13xxx, irq: c_int) -> c_int;
    pub fn mc13xxx_irq_unmask(mc13xxx: *mut mc13xxx, irq: c_int) -> c_int;
}

#[inline]
pub unsafe fn mc13xxx_irq_request_nounmask(mc13xxx: *mut mc13xxx, irq: c_int, handler: irq_handler_t, name: *const c_char, dev: *mut c_void) -> c_int {
    mc13xxx_irq_request(mc13xxx, irq, handler, name, dev)
}

pub const MC13783_AUDIO_RX0: c_int = 36;
pub const MC13783_AUDIO_RX1: c_int = 37;
pub const MC13783_AUDIO_TX: c_int = 38;
pub const MC13783_SSI_NETWORK: c_int = 39;
pub const MC13783_AUDIO_CODEC: c_int = 40;
pub const MC13783_AUDIO_DAC: c_int = 41;

pub const MC13XXX_IRQ_ADCDONE: c_int = 0;
pub const MC13XXX_IRQ_ADCBISDONE: c_int = 1;
pub const MC13XXX_IRQ_TS: c_int = 2;
pub const MC13XXX_IRQ_CHGDET: c_int = 6;
pub const MC13XXX_IRQ_CHGREV: c_int = 8;
pub const MC13XXX_IRQ_CHGSHORT: c_int = 9;
pub const MC13XXX_IRQ_CCCV: c_int = 10;
pub const MC13XXX_IRQ_CHGCURR: c_int = 11;
pub const MC13XXX_IRQ_BPON: c_int = 12;
pub const MC13XXX_IRQ_LOBATL: c_int = 13;
pub const MC13XXX_IRQ_LOBATH: c_int = 14;
pub const MC13XXX_IRQ_1HZ: c_int = 24;
pub const MC13XXX_IRQ_TODA: c_int = 25;
pub const MC13XXX_IRQ_SYSRST: c_int = 30;
pub const MC13XXX_IRQ_RTCRST: c_int = 31;
pub const MC13XXX_IRQ_PC: c_int = 32;
pub const MC13XXX_IRQ_WARM: c_int = 33;
pub const MC13XXX_IRQ_MEMHLD: c_int = 34;
pub const MC13XXX_IRQ_THWARNL: c_int = 36;
pub const MC13XXX_IRQ_THWARNH: c_int = 37;
pub const MC13XXX_IRQ_CLK: c_int = 38;

#[repr(C)]
pub struct mc13xxx_regulator_init_data { pub id: c_int, pub init_data: *mut regulator_init_data, pub node: *mut device_node }
#[repr(C)]
pub struct mc13xxx_regulator_platform_data { pub num_regulators: c_int, pub regulators: *mut mc13xxx_regulator_init_data }

pub const MC13783_LED_MD: c_int = 0; pub const MC13783_LED_AD: c_int = 1; pub const MC13783_LED_KP: c_int = 2;
pub const MC13783_LED_R1: c_int = 3; pub const MC13783_LED_G1: c_int = 4; pub const MC13783_LED_B1: c_int = 5;
pub const MC13783_LED_R2: c_int = 6; pub const MC13783_LED_G2: c_int = 7; pub const MC13783_LED_B2: c_int = 8;
pub const MC13783_LED_R3: c_int = 9; pub const MC13783_LED_G3: c_int = 10; pub const MC13783_LED_B3: c_int = 11;
pub const MC13892_LED_MD: c_int = 12; pub const MC13892_LED_AD: c_int = 13; pub const MC13892_LED_KP: c_int = 14;
pub const MC13892_LED_R: c_int = 15; pub const MC13892_LED_G: c_int = 16; pub const MC13892_LED_B: c_int = 17;
pub const MC34708_LED_R: c_int = 18; pub const MC34708_LED_G: c_int = 19;

#[repr(C)]
pub struct mc13xxx_led_platform_data { pub id: c_int, pub name: *const c_char, pub default_trigger: *const c_char }
pub const MAX_LED_CONTROL_REGS: usize = 6;

macro_rules! bit { ($n:expr) => { 1u32 << $n }; }
macro_rules! fld { ($x:expr, $mask:expr, $shift:expr) => { (($x & $mask) << $shift) }; }
pub const MC13783_LED_C0_ENABLE: u32 = bit!(0); pub const MC13783_LED_C0_TRIODE_MD: u32 = bit!(7); pub const MC13783_LED_C0_TRIODE_AD: u32 = bit!(8); pub const MC13783_LED_C0_TRIODE_KP: u32 = bit!(9); pub const MC13783_LED_C0_BOOST: u32 = bit!(10);
pub const MC13783_LED_C0_ABMODE: fn(u32)->u32 = |x| fld!(x,0x7,11); pub const MC13783_LED_C0_ABREF: fn(u32)->u32 = |x| fld!(x,0x3,14);
pub const MC13783_LED_C1_TC1HALF: u32 = bit!(18); pub const MC13783_LED_C1_SLEWLIM: u32 = bit!(23);
pub const MC13783_LED_C2_CURRENT_MD: fn(u32)->u32 = |x| fld!(x,0x7,0); pub const MC13783_LED_C2_CURRENT_AD: fn(u32)->u32 = |x| fld!(x,0x7,3); pub const MC13783_LED_C2_CURRENT_KP: fn(u32)->u32 = |x| fld!(x,0x7,6); pub const MC13783_LED_C2_PERIOD: fn(u32)->u32 = |x| fld!(x,0x3,21);
pub const MC13783_LED_C3_CURRENT_R1: fn(u32)->u32 = |x| fld!(x,0x3,0); pub const MC13783_LED_C3_CURRENT_G1: fn(u32)->u32 = |x| fld!(x,0x3,2); pub const MC13783_LED_C3_CURRENT_B1: fn(u32)->u32 = |x| fld!(x,0x3,4); pub const MC13783_LED_C3_PERIOD: fn(u32)->u32 = |x| fld!(x,0x3,21);
pub const MC13783_LED_C4_CURRENT_R2: fn(u32)->u32 = |x| fld!(x,0x3,0); pub const MC13783_LED_C4_CURRENT_G2: fn(u32)->u32 = |x| fld!(x,0x3,2); pub const MC13783_LED_C4_CURRENT_B2: fn(u32)->u32 = |x| fld!(x,0x3,4); pub const MC13783_LED_C4_PERIOD: fn(u32)->u32 = |x| fld!(x,0x3,21);
pub const MC13783_LED_C5_CURRENT_R3: fn(u32)->u32 = |x| fld!(x,0x3,0); pub const MC13783_LED_C5_CURRENT_G3: fn(u32)->u32 = |x| fld!(x,0x3,2); pub const MC13783_LED_C5_CURRENT_B3: fn(u32)->u32 = |x| fld!(x,0x3,4); pub const MC13783_LED_C5_PERIOD: fn(u32)->u32 = |x| fld!(x,0x3,21);
pub const MC13783_LED_C2_SLEWLIM: u32 = bit!(23); pub const MC13783_LED_C3_TRIODE_TC1: u32 = bit!(23); pub const MC13783_LED_C4_TRIODE_TC2: u32 = bit!(23); pub const MC13783_LED_C5_TRIODE_TC3: u32 = bit!(23);
pub const MC13892_LED_C0_CURRENT_MD: fn(u32)->u32 = |x| fld!(x,0x7,9); pub const MC13892_LED_C0_CURRENT_AD: fn(u32)->u32 = |x| fld!(x,0x7,21);
pub const MC13892_LED_C1_CURRENT_KP: fn(u32)->u32 = |x| fld!(x,0x7,9); pub const MC13892_LED_C2_CURRENT_R: fn(u32)->u32 = |x| fld!(x,0x7,9); pub const MC13892_LED_C2_CURRENT_G: fn(u32)->u32 = |x| fld!(x,0x7,21); pub const MC13892_LED_C3_CURRENT_B: fn(u32)->u32 = |x| fld!(x,0x7,9);
pub const MC34708_LED_C0_CURRENT_R: fn(u32)->u32 = |x| fld!(x,0x3,9); pub const MC34708_LED_C0_CURRENT_G: fn(u32)->u32 = |x| fld!(x,0x3,21);

#[repr(C)]
pub struct mc13xxx_leds_platform_data { pub led: *mut mc13xxx_led_platform_data, pub num_leds: c_int, pub led_control: [u32; MAX_LED_CONTROL_REGS] }
pub const MC13783_BUTTON_DBNC_0MS: c_int = 0; pub const MC13783_BUTTON_DBNC_30MS: c_int = 1; pub const MC13783_BUTTON_DBNC_150MS: c_int = 2; pub const MC13783_BUTTON_DBNC_750MS: c_int = 3;
pub const MC13783_BUTTON_ENABLE: u32 = bit!(2); pub const MC13783_BUTTON_POL_INVERT: u32 = bit!(3); pub const MC13783_BUTTON_RESET_EN: u32 = bit!(4);
#[repr(C)]
pub struct mc13xxx_buttons_platform_data { pub b1on_flags: c_int, pub b1on_key: c_ushort, pub b2on_flags: c_int, pub b2on_key: c_ushort, pub b3on_flags: c_int, pub b3on_key: c_ushort }
pub const MC13783_TS_ATO_FIRST: bool = false; pub const MC13783_TS_ATO_EACH: bool = true;
#[repr(C)]
pub struct mc13xxx_ts_platform_data { pub ato: u8, pub atox: bool }
#[repr(C)]
pub enum mc13783_ssi_port { MC13783_SSI1_PORT, MC13783_SSI2_PORT }
#[repr(C)]
pub struct mc13xxx_codec_platform_data { pub adc_ssi_port: mc13783_ssi_port, pub dac_ssi_port: mc13783_ssi_port }
pub const MC13XXX_USE_TOUCHSCREEN: u32 = bit!(0); pub const MC13XXX_USE_CODEC: u32 = bit!(1); pub const MC13XXX_USE_ADC: u32 = bit!(2); pub const MC13XXX_USE_RTC: u32 = bit!(3);
#[repr(C)]
pub struct mc13xxx_platform_data { pub flags: c_uint, pub regulators: mc13xxx_regulator_platform_data, pub leds: *mut mc13xxx_leds_platform_data, pub buttons: *mut mc13xxx_buttons_platform_data, pub touch: mc13xxx_ts_platform_data, pub codec: *mut mc13xxx_codec_platform_data }
pub const MC13XXX_ADC_MODE_TS: c_uint = 1; pub const MC13XXX_ADC_MODE_SINGLE_CHAN: c_uint = 2; pub const MC13XXX_ADC_MODE_MULT_CHAN: c_uint = 3;
pub const MC13XXX_ADC0: c_uint = 43;
pub const MC13XXX_ADC0_LICELLCON: u32 = bit!(0); pub const MC13XXX_ADC0_CHRGICON: u32 = bit!(1); pub const MC13XXX_ADC0_BATICON: u32 = bit!(2); pub const MC13XXX_ADC0_ADIN7SEL_DIE: u32 = bit!(4); pub const MC13XXX_ADC0_ADIN7SEL_UID: u32 = 2u32 << 4; pub const MC13XXX_ADC0_ADREFEN: u32 = bit!(10); pub const MC13XXX_ADC0_TSMOD0: u32 = bit!(12); pub const MC13XXX_ADC0_TSMOD1: u32 = bit!(13); pub const MC13XXX_ADC0_TSMOD2: u32 = bit!(14); pub const MC13XXX_ADC0_CHRGRAWDIV: u32 = bit!(15); pub const MC13XXX_ADC0_ADINC1: u32 = bit!(16); pub const MC13XXX_ADC0_ADINC2: u32 = bit!(17);
pub const MC13XXX_ADC0_TSMOD_MASK: u32 = MC13XXX_ADC0_TSMOD0 | MC13XXX_ADC0_TSMOD1 | MC13XXX_ADC0_TSMOD2;
pub const MC13XXX_ADC0_CONFIG_MASK: u32 = MC13XXX_ADC0_TSMOD_MASK | MC13XXX_ADC0_LICELLCON | MC13XXX_ADC0_CHRGICON | MC13XXX_ADC0_BATICON;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
