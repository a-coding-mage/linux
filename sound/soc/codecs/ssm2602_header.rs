/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * File:         sound/soc/codecs/ssm2602.h
 * Author:       Cliff Cai <Cliff.Cai@analog.com>
 *
 * Created:      Tue June 06 2008
 *
 * Modified:
 *               Copyright 2008 Analog Devices Inc.
 *
 * Bugs:         Enter bugs at http://blackfin.uclinux.org/
 */

/* C header guard and include omitted: <linux/regmap.h>. */

use core::ffi::c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ssm2602_type {
    SSM2602,
    SSM2604,
}

unsafe extern "C" {
    pub static ssm2602_regmap_config: regmap_config;

    pub fn ssm2602_probe(
        dev: *mut device,
        type_: ssm2602_type,
        regmap: *mut regmap,
    ) -> c_int;
}

/* SSM2602 Codec Register definitions */

pub const SSM2602_LINVOL: u32 = 0x00;
pub const SSM2602_RINVOL: u32 = 0x01;
pub const SSM2602_LOUT1V: u32 = 0x02;
pub const SSM2602_ROUT1V: u32 = 0x03;
pub const SSM2602_APANA: u32 = 0x04;
pub const SSM2602_APDIGI: u32 = 0x05;
pub const SSM2602_PWR: u32 = 0x06;
pub const SSM2602_IFACE: u32 = 0x07;
pub const SSM2602_SRATE: u32 = 0x08;
pub const SSM2602_ACTIVE: u32 = 0x09;
pub const SSM2602_RESET: u32 = 0x0f;

/*SSM2602 Codec Register Field definitions
 *(Mask value to extract the corresponding Register field)
 */

/*Left ADC Volume Control (SSM2602_REG_LEFT_ADC_VOL)*/
pub const LINVOL_LIN_VOL: u32 = 0x01F; /* Left Channel PGA Volume control                      */
pub const LINVOL_LIN_ENABLE_MUTE: u32 = 0x080; /* Left Channel Input Mute                              */
pub const LINVOL_LRIN_BOTH: u32 = 0x100; /* Left Channel Line Input Volume update                */

/*Right ADC Volume Control (SSM2602_REG_RIGHT_ADC_VOL)*/
pub const RINVOL_RIN_VOL: u32 = 0x01F; /* Right Channel PGA Volume control                     */
pub const RINVOL_RIN_ENABLE_MUTE: u32 = 0x080; /* Right Channel Input Mute                             */
pub const RINVOL_RLIN_BOTH: u32 = 0x100; /* Right Channel Line Input Volume update               */

/*Left DAC Volume Control (SSM2602_REG_LEFT_DAC_VOL)*/
pub const LOUT1V_LHP_VOL: u32 = 0x07F; /* Left Channel Headphone volume control                */
pub const LOUT1V_ENABLE_LZC: u32 = 0x080; /* Left Channel Zero cross detect enable                */
pub const LOUT1V_LRHP_BOTH: u32 = 0x100; /* Left Channel Headphone volume update                 */

/*Right DAC Volume Control (SSM2602_REG_RIGHT_DAC_VOL)*/
pub const ROUT1V_RHP_VOL: u32 = 0x07F; /* Right Channel Headphone volume control               */
pub const ROUT1V_ENABLE_RZC: u32 = 0x080; /* Right Channel Zero cross detect enable               */
pub const ROUT1V_RLHP_BOTH: u32 = 0x100; /* Right Channel Headphone volume update                */

/*Analogue Audio Path Control (SSM2602_REG_ANALOGUE_PATH)*/
pub const APANA_ENABLE_MIC_BOOST: u32 = 0x001; /* Primary Microphone Amplifier gain booster control    */
pub const APANA_ENABLE_MIC_MUTE: u32 = 0x002; /* Microphone Mute Control                              */
pub const APANA_ADC_IN_SELECT: u32 = 0x004; /* Microphone/Line IN select to ADC (1=MIC, 0=Line In)  */
pub const APANA_ENABLE_BYPASS: u32 = 0x008; /* Line input bypass to line output                     */
pub const APANA_SELECT_DAC: u32 = 0x010; /* Select DAC (1=Select DAC, 0=Don't Select DAC)        */
pub const APANA_ENABLE_SIDETONE: u32 = 0x020; /* Enable/Disable Side Tone                             */
pub const APANA_SIDETONE_ATTN: u32 = 0x0C0; /* Side Tone Attenuation                                */
pub const APANA_ENABLE_MIC_BOOST2: u32 = 0x100; /* Secondary Microphone Amplifier gain booster control  */

/*Digital Audio Path Control (SSM2602_REG_DIGITAL_PATH)*/
pub const APDIGI_ENABLE_ADC_HPF: u32 = 0x001; /* Enable/Disable ADC Highpass Filter                   */
pub const APDIGI_DE_EMPHASIS: u32 = 0x006; /* De-Emphasis Control                                  */
pub const APDIGI_ENABLE_DAC_MUTE: u32 = 0x008; /* DAC Mute Control                                     */
pub const APDIGI_STORE_OFFSET: u32 = 0x010; /* Store/Clear DC offset when HPF is disabled           */

/*Power Down Control (SSM2602_REG_POWER)
 *(1=Enable PowerDown, 0=Disable PowerDown)
 */
pub const PWR_LINE_IN_PDN: u32 = 0x001; /* Line Input Power Down                                */
pub const PWR_MIC_PDN: u32 = 0x002; /* Microphone Input & Bias Power Down                   */
pub const PWR_ADC_PDN: u32 = 0x004; /* ADC Power Down                                       */
pub const PWR_DAC_PDN: u32 = 0x008; /* DAC Power Down                                       */
pub const PWR_OUT_PDN: u32 = 0x010; /* Outputs Power Down                                   */
pub const PWR_OSC_PDN: u32 = 0x020; /* Oscillator Power Down                                */
pub const PWR_CLK_OUT_PDN: u32 = 0x040; /* CLKOUT Power Down                                    */
pub const PWR_POWER_OFF: u32 = 0x080; /* POWEROFF Mode                                        */

/*Digital Audio Interface Format (SSM2602_REG_DIGITAL_IFACE)*/
pub const IFACE_IFACE_FORMAT: u32 = 0x003; /* Digital Audio input format control                   */
pub const IFACE_AUDIO_DATA_LEN: u32 = 0x00C; /* Audio Data word length control                       */
pub const IFACE_DAC_LR_POLARITY: u32 = 0x010; /* Polarity Control for clocks in RJ,LJ and I2S modes   */
pub const IFACE_DAC_LR_SWAP: u32 = 0x020; /* Swap DAC data control                                */
pub const IFACE_ENABLE_MASTER: u32 = 0x040; /* Enable/Disable Master Mode                           */
pub const IFACE_BCLK_INVERT: u32 = 0x080; /* Bit Clock Inversion control                          */

/*Sampling Control (SSM2602_REG_SAMPLING_CTRL)*/
pub const SRATE_ENABLE_USB_MODE: u32 = 0x001; /* Enable/Disable USB Mode                              */
pub const SRATE_BOS_RATE: u32 = 0x002; /* Base Over-Sampling rate                              */
pub const SRATE_SAMPLE_RATE: u32 = 0x03C; /* Clock setting condition (Sampling rate control)      */
pub const SRATE_CORECLK_DIV2: u32 = 0x040; /* Core Clock divider select                            */
pub const SRATE_CLKOUT_DIV2: u32 = 0x080; /* Clock Out divider select                             */

/*Active Control (SSM2602_REG_ACTIVE_CTRL)*/
pub const ACTIVE_ACTIVATE_CODEC: u32 = 0x001; /* Activate Codec Digital Audio Interface               */

/*********************************************************************/

pub const SSM2602_CACHEREGNUM: u32 = 10;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ssm2602_clk {
    SSM2602_SYSCLK,
    SSM2602_CLK_CLKOUT,
    SSM2602_CLK_XTO,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
