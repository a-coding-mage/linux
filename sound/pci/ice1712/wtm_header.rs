/* SPDX-License-Identifier: GPL-2.0 */

/* ID */
pub const WTM_DEVICE_DESC: &str = "{EGO SYS INC,WaveTerminal 192M},";
pub const VT1724_SUBDEVICE_WTM: u32 = 0x36495345; /* WT192M ver1.0 */

/*
 *chip addresses on I2C bus
 */

pub const AK4114_ADDR: u32 = 0x20; /*S/PDIF receiver*/
pub const STAC9460_I2C_ADDR: u32 = 0x54; /* ADC*2 | DAC*6 */
pub const STAC9460_2_I2C_ADDR: u32 = 0x56; /* ADC|DAC *2 */

unsafe extern "C" {
    pub static mut snd_vt1724_wtm_cards: [snd_ice1712_card_info; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
