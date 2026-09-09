/*
 * Copyright (c) 2015 Joachim Eastwood <manabian@gmail.com>
 *
 * This code is released using a dual license strategy: BSD/GPL
 * You can choose the licence that better fits your requirements.
 *
 * Released under the terms of 3-clause BSD License
 * Released under the terms of GNU General Public License Version 2.0
 *
 */

/* Clock Control Unit 1 (CCU1) clock offsets */
pub const CLK_APB3_BUS: u32 = 0x100;
pub const CLK_APB3_I2C1: u32 = 0x108;
pub const CLK_APB3_DAC: u32 = 0x110;
pub const CLK_APB3_ADC0: u32 = 0x118;
pub const CLK_APB3_ADC1: u32 = 0x120;
pub const CLK_APB3_CAN0: u32 = 0x128;
pub const CLK_APB1_BUS: u32 = 0x200;
pub const CLK_APB1_MOTOCON_PWM: u32 = 0x208;
pub const CLK_APB1_I2C0: u32 = 0x210;
pub const CLK_APB1_I2S: u32 = 0x218;
pub const CLK_APB1_CAN1: u32 = 0x220;
pub const CLK_SPIFI: u32 = 0x300;
pub const CLK_CPU_BUS: u32 = 0x400;
pub const CLK_CPU_SPIFI: u32 = 0x408;
pub const CLK_CPU_GPIO: u32 = 0x410;
pub const CLK_CPU_LCD: u32 = 0x418;
pub const CLK_CPU_ETHERNET: u32 = 0x420;
pub const CLK_CPU_USB0: u32 = 0x428;
pub const CLK_CPU_EMC: u32 = 0x430;
pub const CLK_CPU_SDIO: u32 = 0x438;
pub const CLK_CPU_DMA: u32 = 0x440;
pub const CLK_CPU_CORE: u32 = 0x448;
pub const CLK_CPU_SCT: u32 = 0x468;
pub const CLK_CPU_USB1: u32 = 0x470;
pub const CLK_CPU_EMCDIV: u32 = 0x478;
pub const CLK_CPU_FLASHA: u32 = 0x480;
pub const CLK_CPU_FLASHB: u32 = 0x488;
pub const CLK_CPU_M0APP: u32 = 0x490;
pub const CLK_CPU_ADCHS: u32 = 0x498;
pub const CLK_CPU_EEPROM: u32 = 0x4a0;
pub const CLK_CPU_WWDT: u32 = 0x500;
pub const CLK_CPU_UART0: u32 = 0x508;
pub const CLK_CPU_UART1: u32 = 0x510;
pub const CLK_CPU_SSP0: u32 = 0x518;
pub const CLK_CPU_TIMER0: u32 = 0x520;
pub const CLK_CPU_TIMER1: u32 = 0x528;
pub const CLK_CPU_SCU: u32 = 0x530;
pub const CLK_CPU_CREG: u32 = 0x538;
pub const CLK_CPU_RITIMER: u32 = 0x600;
pub const CLK_CPU_UART2: u32 = 0x608;
pub const CLK_CPU_UART3: u32 = 0x610;
pub const CLK_CPU_TIMER2: u32 = 0x618;
pub const CLK_CPU_TIMER3: u32 = 0x620;
pub const CLK_CPU_SSP1: u32 = 0x628;
pub const CLK_CPU_QEI: u32 = 0x630;
pub const CLK_PERIPH_BUS: u32 = 0x700;
pub const CLK_PERIPH_CORE: u32 = 0x710;
pub const CLK_PERIPH_SGPIO: u32 = 0x718;
pub const CLK_USB0: u32 = 0x800;
pub const CLK_USB1: u32 = 0x900;
pub const CLK_SPI: u32 = 0xA00;
pub const CLK_ADCHS: u32 = 0xB00;

/* Clock Control Unit 2 (CCU2) clock offsets */
pub const CLK_AUDIO: u32 = 0x100;
pub const CLK_APB2_UART3: u32 = 0x200;
pub const CLK_APB2_UART2: u32 = 0x300;
pub const CLK_APB0_UART1: u32 = 0x400;
pub const CLK_APB0_UART0: u32 = 0x500;
pub const CLK_APB2_SSP1: u32 = 0x600;
pub const CLK_APB0_SSP0: u32 = 0x700;
pub const CLK_SDIO: u32 = 0x800;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
