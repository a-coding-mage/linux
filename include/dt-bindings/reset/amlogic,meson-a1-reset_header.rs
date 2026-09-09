/* SPDX-License-Identifier: (GPL-2.0+ OR MIT)
 *
 * Copyright (c) 2019 Amlogic, Inc. All rights reserved.
 * Author: Xingyu Chen <xingyu.chen@amlogic.com>
 *
 */

/*	RESET0					*/
/*					0	*/
pub const RESET_AM2AXI_VAD: u32 = 1;
/*					2-3	*/
pub const RESET_PSRAM: u32 = 4;
pub const RESET_PAD_CTRL: u32 = 5;
/*					6	*/
pub const RESET_TEMP_SENSOR: u32 = 7;
pub const RESET_AM2AXI_DEV: u32 = 8;
/*					9	*/
pub const RESET_SPICC_A: u32 = 10;
pub const RESET_MSR_CLK: u32 = 11;
pub const RESET_AUDIO: u32 = 12;
pub const RESET_ANALOG_CTRL: u32 = 13;
pub const RESET_SAR_ADC: u32 = 14;
pub const RESET_AUDIO_VAD: u32 = 15;
pub const RESET_CEC: u32 = 16;
pub const RESET_PWM_EF: u32 = 17;
pub const RESET_PWM_CD: u32 = 18;
pub const RESET_PWM_AB: u32 = 19;
/*					20	*/
pub const RESET_IR_CTRL: u32 = 21;
pub const RESET_I2C_S_A: u32 = 22;
/*					23	*/
pub const RESET_I2C_M_D: u32 = 24;
pub const RESET_I2C_M_C: u32 = 25;
pub const RESET_I2C_M_B: u32 = 26;
pub const RESET_I2C_M_A: u32 = 27;
pub const RESET_I2C_PROD_AHB: u32 = 28;
pub const RESET_I2C_PROD: u32 = 29;
/*					30-31	*/

/*	RESET1					*/
pub const RESET_ACODEC: u32 = 32;
pub const RESET_DMA: u32 = 33;
pub const RESET_SD_EMMC_A: u32 = 34;
/*					35	*/
pub const RESET_USBCTRL: u32 = 36;
/*					37	*/
pub const RESET_USBPHY: u32 = 38;
/*					39-41	*/
pub const RESET_RSA: u32 = 42;
pub const RESET_DMC: u32 = 43;
/*					44	*/
pub const RESET_IRQ_CTRL: u32 = 45;
/*					46	*/
pub const RESET_NIC_VAD: u32 = 47;
pub const RESET_NIC_AXI: u32 = 48;
pub const RESET_RAMA: u32 = 49;
pub const RESET_RAMB: u32 = 50;
/*					51-52	*/
pub const RESET_ROM: u32 = 53;
pub const RESET_SPIFC: u32 = 54;
pub const RESET_GIC: u32 = 55;
pub const RESET_UART_C: u32 = 56;
pub const RESET_UART_B: u32 = 57;
pub const RESET_UART_A: u32 = 58;
pub const RESET_OSC_RING: u32 = 59;
/*					60-63	*/

/*	RESET2					*/
/*					64-95	*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
