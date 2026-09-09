/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ni_labpc register definitions.
 */

/*
 * Register map (all registers are 8-bit)
 */

#[inline]
pub const fn bit(n: u8) -> u8 {
    1u8 << n
}

pub const STAT1_REG: u8 = 0x00; /* R: Status 1 reg */
pub const STAT1_DAVAIL: u8 = bit(0);
pub const STAT1_OVERRUN: u8 = bit(1);
pub const STAT1_OVERFLOW: u8 = bit(2);
pub const STAT1_CNTINT: u8 = bit(3);
pub const STAT1_GATA0: u8 = bit(5);
pub const STAT1_EXTGATA0: u8 = bit(6);
pub const CMD1_REG: u8 = 0x00; /* W: Command 1 reg */
pub const fn cmd1_ma(x: u8) -> u8 { (x & 0x7) << 0 }
pub const CMD1_TWOSCMP: u8 = bit(3);
pub const fn cmd1_gain(x: u8) -> u8 { (x & 0x7) << 4 }
pub const CMD1_SCANEN: u8 = bit(7);
pub const CMD2_REG: u8 = 0x01; /* W: Command 2 reg */
pub const CMD2_PRETRIG: u8 = bit(0);
pub const CMD2_HWTRIG: u8 = bit(1);
pub const CMD2_SWTRIG: u8 = bit(2);
pub const CMD2_TBSEL: u8 = bit(3);
pub const CMD2_2SDAC0: u8 = bit(4);
pub const CMD2_2SDAC1: u8 = bit(5);
pub const fn cmd2_ldac(x: u8) -> u8 { bit(6 + (x & 0x1)) }
pub const CMD3_REG: u8 = 0x02; /* W: Command 3 reg */
pub const CMD3_DMAEN: u8 = bit(0);
pub const CMD3_DIOINTEN: u8 = bit(1);
pub const CMD3_DMATCINTEN: u8 = bit(2);
pub const CMD3_CNTINTEN: u8 = bit(3);
pub const CMD3_ERRINTEN: u8 = bit(4);
pub const CMD3_FIFOINTEN: u8 = bit(5);
pub const ADC_START_CONVERT_REG: u8 = 0x03; /* W: Start Convert reg */
pub const fn dac_lsb_reg(x: u8) -> u8 { 0x04 + 2 * x } /* W: DAC0/1 LSB reg */
pub const fn dac_msb_reg(x: u8) -> u8 { 0x05 + 2 * x } /* W: DAC0/1 MSB reg */
pub const ADC_FIFO_CLEAR_REG: u8 = 0x08; /* W: A/D FIFO Clear reg */
pub const ADC_FIFO_REG: u8 = 0x0a; /* R: A/D FIFO reg */
pub const DMATC_CLEAR_REG: u8 = 0x0a; /* W: DMA Interrupt Clear reg */
pub const TIMER_CLEAR_REG: u8 = 0x0c; /* W: Timer Interrupt Clear reg */
pub const CMD6_REG: u8 = 0x0e; /* W: Command 6 reg */
pub const CMD6_NRSE: u8 = bit(0);
pub const CMD6_ADCUNI: u8 = bit(1);
pub const fn cmd6_dacuni(x: u8) -> u8 { bit(2 + (x & 0x1)) }
pub const CMD6_HFINTEN: u8 = bit(5);
pub const CMD6_DQINTEN: u8 = bit(6);
pub const CMD6_SCANUP: u8 = bit(7);
pub const CMD4_REG: u8 = 0x0f; /* W: Command 3 reg */
pub const CMD4_INTSCAN: u8 = bit(0);
pub const CMD4_EOIRCV: u8 = bit(1);
pub const CMD4_ECLKDRV: u8 = bit(2);
pub const CMD4_SEDIFF: u8 = bit(3);
pub const CMD4_ECLKRCV: u8 = bit(4);
pub const DIO_BASE_REG: u8 = 0x10; /* R/W: 8255 DIO base reg */
pub const COUNTER_A_BASE_REG: u8 = 0x14; /* R/W: 8253 Counter A base reg */
pub const COUNTER_B_BASE_REG: u8 = 0x18; /* R/W: 8253 Counter B base reg */
pub const CMD5_REG: u8 = 0x1c; /* W: Command 5 reg */
pub const CMD5_WRTPRT: u8 = bit(2);
pub const CMD5_DITHEREN: u8 = bit(3);
pub const CMD5_CALDACLD: u8 = bit(4);
pub const CMD5_SCLK: u8 = bit(5);
pub const CMD5_SDATA: u8 = bit(6);
pub const CMD5_EEPROMCS: u8 = bit(7);
pub const STAT2_REG: u8 = 0x1d; /* R: Status 2 reg */
pub const STAT2_PROMOUT: u8 = bit(0);
pub const STAT2_OUTA1: u8 = bit(1);
pub const STAT2_FIFONHF: u8 = bit(2);
pub const INTERVAL_COUNT_REG: u8 = 0x1e; /* W: Interval Counter Data reg */
pub const INTERVAL_STROBE_REG: u8 = 0x1f; /* W: Interval Counter Strobe reg */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
