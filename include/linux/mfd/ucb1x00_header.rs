/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/include/mfd/ucb1x00.h. */

use core::ffi::c_void;

pub const UCB_IO_DATA: u32 = 0x00;
pub const UCB_IO_DIR: u32 = 0x01;
pub const UCB_IO_0: u32 = 1 << 0;
pub const UCB_IO_1: u32 = 1 << 1;
pub const UCB_IO_2: u32 = 1 << 2;
pub const UCB_IO_3: u32 = 1 << 3;
pub const UCB_IO_4: u32 = 1 << 4;
pub const UCB_IO_5: u32 = 1 << 5;
pub const UCB_IO_6: u32 = 1 << 6;
pub const UCB_IO_7: u32 = 1 << 7;
pub const UCB_IO_8: u32 = 1 << 8;
pub const UCB_IO_9: u32 = 1 << 9;
pub const UCB_IE_RIS: u32 = 0x02;
pub const UCB_IE_FAL: u32 = 0x03;
pub const UCB_IE_STATUS: u32 = 0x04;
pub const UCB_IE_CLEAR: u32 = 0x04;
pub const UCB_IE_ADC: u32 = 1 << 11;
pub const UCB_IE_TSPX: u32 = 1 << 12;
pub const UCB_IE_TSMX: u32 = 1 << 13;
pub const UCB_IE_TCLIP: u32 = 1 << 14;
pub const UCB_IE_ACLIP: u32 = 1 << 15;
pub const UCB_IRQ_TSPX: u32 = 12;
pub const UCB_TC_A: u32 = 0x05;
pub const UCB_TC_A_LOOP: u32 = 1 << 7; // UCB1200
pub const UCB_TC_A_AMPL: u32 = 1 << 7; // UCB1300
pub const UCB_TC_B: u32 = 0x06;
pub const UCB_TC_B_VOICE_ENA: u32 = 1 << 3;
pub const UCB_TC_B_CLIP: u32 = 1 << 4;
pub const UCB_TC_B_ATT: u32 = 1 << 6;
pub const UCB_TC_B_SIDE_ENA: u32 = 1 << 11;
pub const UCB_TC_B_MUTE: u32 = 1 << 13;
pub const UCB_TC_B_IN_ENA: u32 = 1 << 14;
pub const UCB_TC_B_OUT_ENA: u32 = 1 << 15;
pub const UCB_AC_A: u32 = 0x07;
pub const UCB_AC_B: u32 = 0x08;
pub const UCB_AC_B_LOOP: u32 = 1 << 8;
pub const UCB_AC_B_MUTE: u32 = 1 << 13;
pub const UCB_AC_B_IN_ENA: u32 = 1 << 14;
pub const UCB_AC_B_OUT_ENA: u32 = 1 << 15;
pub const UCB_TS_CR: u32 = 0x09;
pub const UCB_TS_CR_TSMX_POW: u32 = 1 << 0;
pub const UCB_TS_CR_TSPX_POW: u32 = 1 << 1;
pub const UCB_TS_CR_TSMY_POW: u32 = 1 << 2;
pub const UCB_TS_CR_TSPY_POW: u32 = 1 << 3;
pub const UCB_TS_CR_TSMX_GND: u32 = 1 << 4;
pub const UCB_TS_CR_TSPX_GND: u32 = 1 << 5;
pub const UCB_TS_CR_TSMY_GND: u32 = 1 << 6;
pub const UCB_TS_CR_TSPY_GND: u32 = 1 << 7;
pub const UCB_TS_CR_MODE_INT: u32 = 0 << 8;
pub const UCB_TS_CR_MODE_PRES: u32 = 1 << 8;
pub const UCB_TS_CR_MODE_POS: u32 = 2 << 8;
pub const UCB_TS_CR_BIAS_ENA: u32 = 1 << 11;
pub const UCB_TS_CR_TSPX_LOW: u32 = 1 << 12;
pub const UCB_TS_CR_TSMX_LOW: u32 = 1 << 13;
pub const UCB_ADC_CR: u32 = 0x0a;
pub const UCB_ADC_SYNC_ENA: u32 = 1 << 0;
pub const UCB_ADC_VREFBYP_CON: u32 = 1 << 1;
pub const UCB_ADC_INP_TSPX: u32 = 0 << 2;
pub const UCB_ADC_INP_TSMX: u32 = 1 << 2;
pub const UCB_ADC_INP_TSPY: u32 = 2 << 2;
pub const UCB_ADC_INP_TSMY: u32 = 3 << 2;
pub const UCB_ADC_INP_AD0: u32 = 4 << 2;
pub const UCB_ADC_INP_AD1: u32 = 5 << 2;
pub const UCB_ADC_INP_AD2: u32 = 6 << 2;
pub const UCB_ADC_INP_AD3: u32 = 7 << 2;
pub const UCB_ADC_EXT_REF: u32 = 1 << 5;
pub const UCB_ADC_START: u32 = 1 << 7;
pub const UCB_ADC_ENA: u32 = 1 << 15;
pub const UCB_ADC_DATA: u32 = 0x0b;
pub const UCB_ADC_DAT_VAL: u32 = 1 << 15;
pub const UCB_ID: u32 = 0x0c;
pub const UCB_ID_1200: u32 = 0x1004;
pub const UCB_ID_1300: u32 = 0x1005;
pub const UCB_ID_TC35143: u32 = 0x9712;
pub const UCB_MODE: u32 = 0x0d;
pub const UCB_MODE_DYN_VFLAG_ENA: u32 = 1 << 12;
pub const UCB_MODE_AUD_OFF_CAN: u32 = 1 << 13;

#[inline]
pub const fn UCB_ADC_DAT(x: u32) -> u32 { (x & 0x7fe0) >> 5 }

#[repr(C)] pub struct software_node { _private: [u8; 0] }
#[repr(C)] pub struct mcp { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct gpio_chip { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

extern "C" {
    pub static ucb1x00_gpiochip_node: software_node;
    pub fn ucb1x00_register_driver(driver: *mut ucb1x00_driver) -> i32;
    pub fn ucb1x00_unregister_driver(driver: *mut ucb1x00_driver);
    pub fn ucb1x00_io_set_dir(ucb: *mut ucb1x00, in_: u32, out: u32);
    pub fn ucb1x00_io_write(ucb: *mut ucb1x00, in_: u32, out: u32);
    pub fn ucb1x00_io_read(ucb: *mut ucb1x00) -> u32;
    pub fn ucb1x00_adc_read(ucb: *mut ucb1x00, adc_channel: i32, sync: i32) -> u32;
    pub fn ucb1x00_adc_enable(ucb: *mut ucb1x00);
    pub fn ucb1x00_adc_disable(ucb: *mut ucb1x00);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ucb1x00_reset { UCB_RST_PROBE, UCB_RST_RESUME, UCB_RST_SUSPEND, UCB_RST_REMOVE, UCB_RST_PROBE_FAIL }

#[repr(C)] pub struct ucb1x00_plat_data { pub reset: Option<unsafe extern "C" fn(ucb1x00_reset)>, pub irq_base: u32, pub gpio_base: i32, pub can_wakeup: u32 }
#[repr(C)] pub struct ucb1x00 { pub irq_lock: raw_spinlock_t, pub mcp: *mut mcp, pub irq: u32, pub irq_base: i32, pub adc_mutex: mutex, pub io_lock: spinlock_t, pub id: u16, pub io_dir: u16, pub io_out: u16, pub adc_cr: u16, pub irq_fal_enbl: u16, pub irq_ris_enbl: u16, pub irq_mask: u16, pub irq_wake: u16, pub dev: device, pub node: list_head, pub devs: list_head, pub gpio: gpio_chip }
#[repr(C)] pub struct ucb1x00_dev { pub dev_node: list_head, pub drv_node: list_head, pub ucb: *mut ucb1x00, pub drv: *mut ucb1x00_driver, pub priv_: *mut c_void }
#[repr(C)] pub struct ucb1x00_driver { pub node: list_head, pub devs: list_head, pub add: Option<unsafe extern "C" fn(*mut ucb1x00_dev) -> i32>, pub remove: Option<unsafe extern "C" fn(*mut ucb1x00_dev)>, pub suspend: Option<unsafe extern "C" fn(*mut ucb1x00_dev) -> i32>, pub resume: Option<unsafe extern "C" fn(*mut ucb1x00_dev) -> i32> }

extern "C" { pub fn mcp_get_sclk_rate(mcp: *mut mcp) -> u32; pub fn mcp_enable(mcp: *mut mcp); pub fn mcp_disable(mcp: *mut mcp); pub fn mcp_reg_write(mcp: *mut mcp, reg: u32, val: u32); pub fn mcp_reg_read(mcp: *mut mcp, reg: u32) -> u32; pub fn mcp_set_audio_divisor(mcp: *mut mcp, div: u32); pub fn mcp_set_telecom_divisor(mcp: *mut mcp, div: u32); }
#[inline] pub unsafe fn ucb1x00_clkrate(ucb: *mut ucb1x00) -> u32 { mcp_get_sclk_rate((*ucb).mcp) }
#[inline] pub unsafe fn ucb1x00_enable(ucb: *mut ucb1x00) { mcp_enable((*ucb).mcp) }
#[inline] pub unsafe fn ucb1x00_disable(ucb: *mut ucb1x00) { mcp_disable((*ucb).mcp) }
#[inline] pub unsafe fn ucb1x00_reg_write(ucb: *mut ucb1x00, reg: u32, val: u32) { mcp_reg_write((*ucb).mcp, reg, val) }
#[inline] pub unsafe fn ucb1x00_reg_read(ucb: *mut ucb1x00, reg: u32) -> u32 { mcp_reg_read((*ucb).mcp, reg) }
#[inline] pub unsafe fn ucb1x00_set_audio_divisor(ucb: *mut ucb1x00, div: u32) { mcp_set_audio_divisor((*ucb).mcp, div) }
#[inline] pub unsafe fn ucb1x00_set_telecom_divisor(ucb: *mut ucb1x00, div: u32) { mcp_set_telecom_divisor((*ucb).mcp, div) }
pub const UCB_NOSYNC: u32 = 0;
pub const UCB_SYNC: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
