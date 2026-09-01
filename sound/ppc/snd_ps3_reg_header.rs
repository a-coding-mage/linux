/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Audio support for PS3
 * Copyright (C) 2007 Sony Computer Entertainment Inc.
 * Copyright 2006, 2007 Sony Corporation
 * All rights reserved.
 */

/*
 * interrupt / configure registers
 */

pub const PS3_AUDIO_INTR_0: u32 = 0x00000100;
pub const PS3_AUDIO_INTR_EN_0: u32 = 0x00000140;
pub const PS3_AUDIO_CONFIG: u32 = 0x00000200;

/*
 * DMAC registers
 * n:0..9
 */
pub const fn PS3_AUDIO_DMAC_REGBASE(x: u32) -> u32 {
    0x00000210u32.wrapping_add(0x20u32.wrapping_mul(x))
}

pub const fn PS3_AUDIO_KICK(n: u32) -> u32 {
    PS3_AUDIO_DMAC_REGBASE(n).wrapping_add(0x00)
}
pub const fn PS3_AUDIO_SOURCE(n: u32) -> u32 {
    PS3_AUDIO_DMAC_REGBASE(n).wrapping_add(0x04)
}
pub const fn PS3_AUDIO_DEST(n: u32) -> u32 {
    PS3_AUDIO_DMAC_REGBASE(n).wrapping_add(0x08)
}
pub const fn PS3_AUDIO_DMASIZE(n: u32) -> u32 {
    PS3_AUDIO_DMAC_REGBASE(n).wrapping_add(0x0C)
}

/*
 * mute control
 */
pub const PS3_AUDIO_AX_MCTRL: u32 = 0x00004000;
pub const PS3_AUDIO_AX_ISBP: u32 = 0x00004004;
pub const PS3_AUDIO_AX_AOBP: u32 = 0x00004008;
pub const PS3_AUDIO_AX_IC: u32 = 0x00004010;
pub const PS3_AUDIO_AX_IE: u32 = 0x00004014;
pub const PS3_AUDIO_AX_IS: u32 = 0x00004018;

/*
 * three wire serial
 * n:0..3
 */
pub const PS3_AUDIO_AO_MCTRL: u32 = 0x00006000;
pub const PS3_AUDIO_AO_3WMCTRL: u32 = 0x00006004;

pub const fn PS3_AUDIO_AO_3WCTRL(n: u32) -> u32 {
    0x00006200u32.wrapping_add(0x200u32.wrapping_mul(n))
}

/*
 * S/PDIF
 * n:0..1
 * x:0..11
 * y:0..5
 */
pub const fn PS3_AUDIO_AO_SPD_REGBASE(n: u32) -> u32 {
    0x00007200u32.wrapping_add(0x200u32.wrapping_mul(n))
}

pub const fn PS3_AUDIO_AO_SPDCTRL(n: u32) -> u32 {
    PS3_AUDIO_AO_SPD_REGBASE(n).wrapping_add(0x00)
}
pub const fn PS3_AUDIO_AO_SPDUB(n: u32, x: u32) -> u32 {
    PS3_AUDIO_AO_SPD_REGBASE(n)
        .wrapping_add(0x04)
        .wrapping_add(0x04u32.wrapping_mul(x))
}
pub const fn PS3_AUDIO_AO_SPDCS(n: u32, y: u32) -> u32 {
    PS3_AUDIO_AO_SPD_REGBASE(n)
        .wrapping_add(0x34)
        .wrapping_add(0x04u32.wrapping_mul(y))
}

/*
  PS3_AUDIO_INTR_0 register tells an interrupt handler which audio
  DMA channel triggered the interrupt.  The interrupt status for a channel
  can be cleared by writing a '1' to the corresponding bit.  A new interrupt
  cannot be generated until the previous interrupt has been cleared.

  Note that the status reported by PS3_AUDIO_INTR_0 is independent of the
  value of PS3_AUDIO_INTR_EN_0.

 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0 0 0 0 0 0 0 0 0 0 0 0 0|C|0|C|0|C|0|C|0|C|0|C|0|C|0|C|0|C|0|C| INTR_0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/
pub const fn PS3_AUDIO_INTR_0_CHAN(n: u32) -> u32 {
    1u32 << n.wrapping_mul(2)
}
pub const PS3_AUDIO_INTR_0_CHAN9: u32 = PS3_AUDIO_INTR_0_CHAN(9);
pub const PS3_AUDIO_INTR_0_CHAN8: u32 = PS3_AUDIO_INTR_0_CHAN(8);
pub const PS3_AUDIO_INTR_0_CHAN7: u32 = PS3_AUDIO_INTR_0_CHAN(7);
pub const PS3_AUDIO_INTR_0_CHAN6: u32 = PS3_AUDIO_INTR_0_CHAN(6);
pub const PS3_AUDIO_INTR_0_CHAN5: u32 = PS3_AUDIO_INTR_0_CHAN(5);
pub const PS3_AUDIO_INTR_0_CHAN4: u32 = PS3_AUDIO_INTR_0_CHAN(4);
pub const PS3_AUDIO_INTR_0_CHAN3: u32 = PS3_AUDIO_INTR_0_CHAN(3);
pub const PS3_AUDIO_INTR_0_CHAN2: u32 = PS3_AUDIO_INTR_0_CHAN(2);
pub const PS3_AUDIO_INTR_0_CHAN1: u32 = PS3_AUDIO_INTR_0_CHAN(1);
pub const PS3_AUDIO_INTR_0_CHAN0: u32 = PS3_AUDIO_INTR_0_CHAN(0);

/*
  The PS3_AUDIO_INTR_EN_0 register specifies which DMA channels can generate
  an interrupt to the PU.  Each bit of PS3_AUDIO_INTR_EN_0 is ANDed with the
  corresponding bit in PS3_AUDIO_INTR_0.  The resulting bits are OR'd together
  to generate the Audio interrupt.

 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0 0 0 0 0 0 0 0 0 0 0 0 0|C|0|C|0|C|0|C|0|C|0|C|0|C|0|C|0|C|0|C| INTR_EN_0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+

  Bit assignments are same as PS3_AUDIO_INTR_0
*/

/*
  PS3_AUDIO_CONFIG
  31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0 0 0 0 0 0 0 0|0 0 0 0 0 0 0 0|0 0 0 0 0 0 0 C|0 0 0 0 0 0 0 0| CONFIG
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+

*/

/* The CLEAR field cancels all pending transfers, and stops any running DMA
   transfers.  Any interrupts associated with the canceled transfers
   will occur as if the transfer had finished.
   Since this bit is designed to recover from DMA related issues
   which are caused by unpredictable situations, it is preferred to wait
   for normal DMA transfer end without using this bit.
*/
pub const PS3_AUDIO_CONFIG_CLEAR: u32 = 1u32 << 8; /* RWIVF */

/*
  PS3_AUDIO_AX_MCTRL: Audio Port Mute Control Register

 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0|A|A|A|0 0 0 0 0 0 0|S|S|A|A|A|A| AX_MCTRL
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/

/* 3 Wire Audio Serial Output Channel Mutes (0..3)  */
pub const fn PS3_AUDIO_AX_MCTRL_ASOMT(n: u32) -> u32 {
    1u32 << 3u32.wrapping_sub(n)
}
pub const PS3_AUDIO_AX_MCTRL_ASO3MT: u32 = 1u32 << 0;
pub const PS3_AUDIO_AX_MCTRL_ASO2MT: u32 = 1u32 << 1;
pub const PS3_AUDIO_AX_MCTRL_ASO1MT: u32 = 1u32 << 2;
pub const PS3_AUDIO_AX_MCTRL_ASO0MT: u32 = 1u32 << 3;

/* S/PDIF mutes (0,1)*/
pub const fn PS3_AUDIO_AX_MCTRL_SPOMT(n: u32) -> u32 {
    1u32 << 5u32.wrapping_sub(n)
}
pub const PS3_AUDIO_AX_MCTRL_SPO1MT: u32 = 1u32 << 4;
pub const PS3_AUDIO_AX_MCTRL_SPO0MT: u32 = 1u32 << 5;

/* All 3 Wire Serial Outputs Mute */
pub const PS3_AUDIO_AX_MCTRL_AASOMT: u32 = 1u32 << 13;

/* All S/PDIF Mute */
pub const PS3_AUDIO_AX_MCTRL_ASPOMT: u32 = 1u32 << 14;

/* All Audio Outputs Mute */
pub const PS3_AUDIO_AX_MCTRL_AAOMT: u32 = 1u32 << 15;

/*
  S/PDIF Outputs Buffer Read/Write Pointer Register

 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0 0 0 0 0 0 0 0|0|SPO0B|0|SPO1B|0 0 0 0 0 0 0 0|0|SPO0B|0|SPO1B| AX_ISBP
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+

*/
/*
 S/PDIF Output Channel Read Buffer Numbers
 Buffer number is  value of field.
 Indicates current read access buffer ID from Audio Data
 Transfer controller of S/PDIF Output
*/

pub const fn PS3_AUDIO_AX_ISBP_SPOBRN_MASK(n: u32) -> u32 {
    0x7u32 << 4u32.wrapping_mul(1u32.wrapping_sub(n))
}
pub const PS3_AUDIO_AX_ISBP_SPO1BRN_MASK: u32 = 0x7u32 << 0;
pub const PS3_AUDIO_AX_ISBP_SPO0BRN_MASK: u32 = 0x7u32 << 4;

/*
S/PDIF Output Channel Buffer Write Numbers
Indicates current write access buffer ID from bus master.
*/
pub const fn PS3_AUDIO_AX_ISBP_SPOBWN_MASK(n: u32) -> u32 {
    0x7u32 << 4u32.wrapping_mul(5u32.wrapping_sub(n))
}
pub const PS3_AUDIO_AX_ISBP_SPO1BWN_MASK: u32 = 0x7u32 << 16;
pub const PS3_AUDIO_AX_ISBP_SPO0BWN_MASK: u32 = 0x7u32 << 20;

/*
  3 Wire Audio Serial Outputs Buffer Read/Write
  Pointer Register
  Buffer number is  value of field

 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0|ASO0B|0|ASO1B|0|ASO2B|0|ASO3B|0|ASO0B|0|ASO1B|0|ASO2B|0|ASO3B| AX_AOBP
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/

/*
3 Wire Audio Serial Output Channel Buffer Read Numbers
Indicates current read access buffer Id from Audio Data Transfer
Controller of 3 Wire Audio Serial Output Channels
*/
pub const fn PS3_AUDIO_AX_AOBP_ASOBRN_MASK(n: u32) -> u32 {
    0x7u32 << 4u32.wrapping_mul(3u32.wrapping_sub(n))
}

pub const PS3_AUDIO_AX_AOBP_ASO3BRN_MASK: u32 = 0x7u32 << 0;
pub const PS3_AUDIO_AX_AOBP_ASO2BRN_MASK: u32 = 0x7u32 << 4;
pub const PS3_AUDIO_AX_AOBP_ASO1BRN_MASK: u32 = 0x7u32 << 8;
pub const PS3_AUDIO_AX_AOBP_ASO0BRN_MASK: u32 = 0x7u32 << 12;

/*
3 Wire Audio Serial Output Channel Buffer Write Numbers
Indicates current write access buffer ID from bus master.
*/
pub const fn PS3_AUDIO_AX_AOBP_ASOBWN_MASK(n: u32) -> u32 {
    0x7u32 << 4u32.wrapping_mul(7u32.wrapping_sub(n))
}

pub const PS3_AUDIO_AX_AOBP_ASO3BWN_MASK: u32 = 0x7u32 << 16;
pub const PS3_AUDIO_AX_AOBP_ASO2BWN_MASK: u32 = 0x7u32 << 20;
pub const PS3_AUDIO_AX_AOBP_ASO1BWN_MASK: u32 = 0x7u32 << 24;
pub const PS3_AUDIO_AX_AOBP_ASO0BWN_MASK: u32 = 0x7u32 << 28;

/*
Audio Port Interrupt Condition Register
For the fields in this register, the following values apply:
0 = Interrupt is generated every interrupt event.
1 = Interrupt is generated every 2 interrupt events.
2 = Interrupt is generated every 4 interrupt events.
3 = Reserved


 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0 0 0 0 0 0 0 0|0 0|SPO|0 0|SPO|0 0|AAS|0 0 0 0 0 0 0 0 0 0 0 0| AX_IC
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/
/*
All 3-Wire Audio Serial Outputs Interrupt Mode
Configures the Interrupt and Signal Notification
condition of all 3-wire Audio Serial Outputs.
*/
pub const PS3_AUDIO_AX_IC_AASOIMD_MASK: u32 = 0x3u32 << 12;
pub const PS3_AUDIO_AX_IC_AASOIMD_EVERY1: u32 = 0x0u32 << 12;
pub const PS3_AUDIO_AX_IC_AASOIMD_EVERY2: u32 = 0x1u32 << 12;
pub const PS3_AUDIO_AX_IC_AASOIMD_EVERY4: u32 = 0x2u32 << 12;

/*
S/PDIF Output Channel Interrupt Modes
Configures the Interrupt and signal Notification
conditions of S/PDIF output channels.
*/
pub const PS3_AUDIO_AX_IC_SPO1IMD_MASK: u32 = 0x3u32 << 16;
pub const PS3_AUDIO_AX_IC_SPO1IMD_EVERY1: u32 = 0x0u32 << 16;
pub const PS3_AUDIO_AX_IC_SPO1IMD_EVERY2: u32 = 0x1u32 << 16;
pub const PS3_AUDIO_AX_IC_SPO1IMD_EVERY4: u32 = 0x2u32 << 16;

pub const PS3_AUDIO_AX_IC_SPO0IMD_MASK: u32 = 0x3u32 << 20;
pub const PS3_AUDIO_AX_IC_SPO0IMD_EVERY1: u32 = 0x0u32 << 20;
pub const PS3_AUDIO_AX_IC_SPO0IMD_EVERY2: u32 = 0x1u32 << 20;
pub const PS3_AUDIO_AX_IC_SPO0IMD_EVERY4: u32 = 0x2u32 << 20;

/*
Audio Port interrupt Enable Register
Configures whether to enable or disable each Interrupt Generation.


 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0 0 0 0 0 0 0 0|S|S|0 0|A|A|A|A|0 0 0 0|S|S|0 0|S|S|0 0|A|A|A|A| AX_IE
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+

*/

/*
3 Wire Audio Serial Output Channel Buffer Underflow
Interrupt Enables
Select enable/disable of Buffer Underflow Interrupts for
3-Wire Audio Serial Output Channels
DISABLED=Interrupt generation disabled.
*/
pub const fn PS3_AUDIO_AX_IE_ASOBUIE(n: u32) -> u32 {
    1u32 << 3u32.wrapping_sub(n)
}
pub const PS3_AUDIO_AX_IE_ASO3BUIE: u32 = 1u32 << 0;
pub const PS3_AUDIO_AX_IE_ASO2BUIE: u32 = 1u32 << 1;
pub const PS3_AUDIO_AX_IE_ASO1BUIE: u32 = 1u32 << 2;
pub const PS3_AUDIO_AX_IE_ASO0BUIE: u32 = 1u32 << 3;

/* S/PDIF Output Channel Buffer Underflow Interrupt Enables */

pub const fn PS3_AUDIO_AX_IE_SPOBUIE(n: u32) -> u32 {
    1u32 << 7u32.wrapping_sub(n)
}
pub const PS3_AUDIO_AX_IE_SPO1BUIE: u32 = 1u32 << 6;
pub const PS3_AUDIO_AX_IE_SPO0BUIE: u32 = 1u32 << 7;

/* S/PDIF Output Channel One Block Transfer Completion Interrupt Enables */

pub const fn PS3_AUDIO_AX_IE_SPOBTCIE(n: u32) -> u32 {
    1u32 << 11u32.wrapping_sub(n)
}
pub const PS3_AUDIO_AX_IE_SPO1BTCIE: u32 = 1u32 << 10;
pub const PS3_AUDIO_AX_IE_SPO0BTCIE: u32 = 1u32 << 11;

/* 3-Wire Audio Serial Output Channel Buffer Empty Interrupt Enables */

pub const fn PS3_AUDIO_AX_IE_ASOBEIE(n: u32) -> u32 {
    1u32 << 19u32.wrapping_sub(n)
}
pub const PS3_AUDIO_AX_IE_ASO3BEIE: u32 = 1u32 << 16;
pub const PS3_AUDIO_AX_IE_ASO2BEIE: u32 = 1u32 << 17;
pub const PS3_AUDIO_AX_IE_ASO1BEIE: u32 = 1u32 << 18;
pub const PS3_AUDIO_AX_IE_ASO0BEIE: u32 = 1u32 << 19;

/* S/PDIF Output Channel Buffer Empty Interrupt Enables */

pub const fn PS3_AUDIO_AX_IE_SPOBEIE(n: u32) -> u32 {
    1u32 << 23u32.wrapping_sub(n)
}
pub const PS3_AUDIO_AX_IE_SPO1BEIE: u32 = 1u32 << 22;
pub const PS3_AUDIO_AX_IE_SPO0BEIE: u32 = 1u32 << 23;

/*
Audio Port Interrupt Status Register
Indicates Interrupt status, which interrupt has occurred, and can clear
each interrupt in this register.
Writing 1b to a field containing 1b clears field and de-asserts interrupt.
Writing 0b to a field has no effect.
Field values are the following:
0 - Interrupt hasn't occurred.
1 - Interrupt has occurred.


 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0 0 0 0 0 0 0 0|S|S|0 0|A|A|A|A|0 0 0 0|S|S|0 0|S|S|0 0|A|A|A|A| AX_IS
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+

 Bit assignment are same as AX_IE
*/

/*
Audio Output Master Control Register
Configures Master Clock and other master Audio Output Settings


 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0|SCKSE|0|SCKSE|  MR0  |  MR1  |MCL|MCL|0 0 0 0|0 0 0 0 0 0 0 0| AO_MCTRL
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/

/*
MCLK Output Control
Controls mclko[1] output.
0 - Disable output (fixed at High)
1 - Output clock produced by clock selected
with scksel1 by mr1
2 - Reserved
3 - Reserved
*/

pub const PS3_AUDIO_AO_MCTRL_MCLKC1_MASK: u32 = 0x3u32 << 12;
pub const PS3_AUDIO_AO_MCTRL_MCLKC1_DISABLED: u32 = 0x0u32 << 12;
pub const PS3_AUDIO_AO_MCTRL_MCLKC1_ENABLED: u32 = 0x1u32 << 12;
pub const PS3_AUDIO_AO_MCTRL_MCLKC1_RESVD2: u32 = 0x2u32 << 12;
pub const PS3_AUDIO_AO_MCTRL_MCLKC1_RESVD3: u32 = 0x3u32 << 12;

/*
MCLK Output Control
Controls mclko[0] output.
0 - Disable output (fixed at High)
1 - Output clock produced by clock selected
with SCKSEL0 by MR0
2 - Reserved
3 - Reserved
*/
pub const PS3_AUDIO_AO_MCTRL_MCLKC0_MASK: u32 = 0x3u32 << 14;
pub const PS3_AUDIO_AO_MCTRL_MCLKC0_DISABLED: u32 = 0x0u32 << 14;
pub const PS3_AUDIO_AO_MCTRL_MCLKC0_ENABLED: u32 = 0x1u32 << 14;
pub const PS3_AUDIO_AO_MCTRL_MCLKC0_RESVD2: u32 = 0x2u32 << 14;
pub const PS3_AUDIO_AO_MCTRL_MCLKC0_RESVD3: u32 = 0x3u32 << 14;
/*
Master Clock Rate 1
Sets the divide ration of Master Clock1 (clock output from
mclko[1] for the input clock selected by scksel1.
*/
pub const PS3_AUDIO_AO_MCTRL_MR1_MASK: u32 = 0xfu32 << 16;
pub const PS3_AUDIO_AO_MCTRL_MR1_DEFAULT: u32 = 0x0u32 << 16;
/*
Master Clock Rate 0
Sets the divide ratio of Master Clock0 (clock output from
mclko[0] for the input clock selected by scksel0).
*/
pub const PS3_AUDIO_AO_MCTRL_MR0_MASK: u32 = 0xfu32 << 20;
pub const PS3_AUDIO_AO_MCTRL_MR0_DEFAULT: u32 = 0x0u32 << 20;
/*
System Clock Select 0/1
Selects the system clock to be used as Master Clock 0/1
Input the system clock that is appropriate for the sampling
rate.
*/
pub const PS3_AUDIO_AO_MCTRL_SCKSEL1_MASK: u32 = 0x7u32 << 24;
pub const PS3_AUDIO_AO_MCTRL_SCKSEL1_DEFAULT: u32 = 0x2u32 << 24;

pub const PS3_AUDIO_AO_MCTRL_SCKSEL0_MASK: u32 = 0x7u32 << 28;
pub const PS3_AUDIO_AO_MCTRL_SCKSEL0_DEFAULT: u32 = 0x2u32 << 28;

/*
3-Wire Audio Output Master Control Register
Configures clock, 3-Wire Audio Serial Output Enable, and
other 3-Wire Audio Serial Output Master Settings


 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |A|A|A|A|0 0 0|A| ASOSR |0 0 0 0|A|A|A|A|A|A|0|1|0 0 0 0 0 0 0 0| AO_3WMCTRL
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/

/*
LRCKO Polarity
0 - Reserved
1 - default
*/
pub const PS3_AUDIO_AO_3WMCTRL_ASOPLRCK: u32 = 1u32 << 8;
pub const PS3_AUDIO_AO_3WMCTRL_ASOPLRCK_DEFAULT: u32 = 1u32 << 8;

/* LRCK Output Disable */

pub const PS3_AUDIO_AO_3WMCTRL_ASOLRCKD: u32 = 1u32 << 10;
pub const PS3_AUDIO_AO_3WMCTRL_ASOLRCKD_ENABLED: u32 = 0u32 << 10;
pub const PS3_AUDIO_AO_3WMCTRL_ASOLRCKD_DISABLED: u32 = 1u32 << 10;

/* Bit Clock Output Disable */

pub const PS3_AUDIO_AO_3WMCTRL_ASOBCLKD: u32 = 1u32 << 11;
pub const PS3_AUDIO_AO_3WMCTRL_ASOBCLKD_ENABLED: u32 = 0u32 << 11;
pub const PS3_AUDIO_AO_3WMCTRL_ASOBCLKD_DISABLED: u32 = 1u32 << 11;

/*
3-Wire Audio Serial Output Channel 0-3 Operational
Status.  Each bit becomes 1 after each 3-Wire Audio
Serial Output Channel N is in action by setting 1 to
asoen.
Each bit becomes 0 after each 3-Wire Audio Serial Output
Channel N is out of action by setting 0 to asoen.
*/
pub const fn PS3_AUDIO_AO_3WMCTRL_ASORUN(n: u32) -> u32 {
    1u32 << 15u32.wrapping_sub(n)
}
pub const fn PS3_AUDIO_AO_3WMCTRL_ASORUN_STOPPED(n: u32) -> u32 {
    0u32 << 15u32.wrapping_sub(n)
}
pub const fn PS3_AUDIO_AO_3WMCTRL_ASORUN_RUNNING(n: u32) -> u32 {
    1u32 << 15u32.wrapping_sub(n)
}
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN0: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN(0);
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN0_STOPPED: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN_STOPPED(0);
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN0_RUNNING: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN_RUNNING(0);
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN1: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN(1);
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN1_STOPPED: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN_STOPPED(1);
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN1_RUNNING: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN_RUNNING(1);
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN2: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN(2);
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN2_STOPPED: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN_STOPPED(2);
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN2_RUNNING: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN_RUNNING(2);
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN3: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN(3);
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN3_STOPPED: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN_STOPPED(3);
pub const PS3_AUDIO_AO_3WMCTRL_ASORUN3_RUNNING: u32 = PS3_AUDIO_AO_3WMCTRL_ASORUN_RUNNING(3);

/*
Sampling Rate
Specifies the divide ratio of the bit clock (clock output
from bclko) used by the 3-wire Audio Output Clock, which
is applied to the master clock selected by mcksel.
Data output is synchronized with this clock.
*/
pub const PS3_AUDIO_AO_3WMCTRL_ASOSR_MASK: u32 = 0xfu32 << 20;
pub const PS3_AUDIO_AO_3WMCTRL_ASOSR_DIV2: u32 = 0x1u32 << 20;
pub const PS3_AUDIO_AO_3WMCTRL_ASOSR_DIV4: u32 = 0x2u32 << 20;
pub const PS3_AUDIO_AO_3WMCTRL_ASOSR_DIV8: u32 = 0x4u32 << 20;
pub const PS3_AUDIO_AO_3WMCTRL_ASOSR_DIV12: u32 = 0x6u32 << 20;

/*
Master Clock Select
0 - Master Clock 0
1 - Master Clock 1
*/
pub const PS3_AUDIO_AO_3WMCTRL_ASOMCKSEL: u32 = 1u32 << 24;
pub const PS3_AUDIO_AO_3WMCTRL_ASOMCKSEL_CLK0: u32 = 0u32 << 24;
pub const PS3_AUDIO_AO_3WMCTRL_ASOMCKSEL_CLK1: u32 = 1u32 << 24;

/*
Enables and disables 4ch 3-Wire Audio Serial Output
operation.  Each Bit from 0 to 3 corresponds to an
output channel, which means that each output channel
can be enabled or disabled individually.  When
multiple channels are enabled at the same time, output
operations are performed in synchronization.
Bit 0 - Output Channel 0 (SDOUT[0])
Bit 1 - Output Channel 1 (SDOUT[1])
Bit 2 - Output Channel 2 (SDOUT[2])
Bit 3 - Output Channel 3 (SDOUT[3])
*/
pub const fn PS3_AUDIO_AO_3WMCTRL_ASOEN(n: u32) -> u32 {
    1u32 << 31u32.wrapping_sub(n)
}
pub const fn PS3_AUDIO_AO_3WMCTRL_ASOEN_DISABLED(n: u32) -> u32 {
    0u32 << 31u32.wrapping_sub(n)
}
pub const fn PS3_AUDIO_AO_3WMCTRL_ASOEN_ENABLED(n: u32) -> u32 {
    1u32 << 31u32.wrapping_sub(n)
}

pub const PS3_AUDIO_AO_3WMCTRL_ASOEN0: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN(0);
pub const PS3_AUDIO_AO_3WMCTRL_ASOEN0_DISABLED: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN_DISABLED(0);
pub const PS3_AUDIO_AO_3WMCTRL_ASOEN0_ENABLED: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN_ENABLED(0);
pub const PS3_AUDIO_A1_3WMCTRL_ASOEN0: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN(1);
pub const PS3_AUDIO_A1_3WMCTRL_ASOEN0_DISABLED: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN_DISABLED(1);
pub const PS3_AUDIO_A1_3WMCTRL_ASOEN0_ENABLED: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN_ENABLED(1);
pub const PS3_AUDIO_A2_3WMCTRL_ASOEN0: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN(2);
pub const PS3_AUDIO_A2_3WMCTRL_ASOEN0_DISABLED: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN_DISABLED(2);
pub const PS3_AUDIO_A2_3WMCTRL_ASOEN0_ENABLED: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN_ENABLED(2);
pub const PS3_AUDIO_A3_3WMCTRL_ASOEN0: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN(3);
pub const PS3_AUDIO_A3_3WMCTRL_ASOEN0_DISABLED: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN_DISABLED(3);
pub const PS3_AUDIO_A3_3WMCTRL_ASOEN0_ENABLED: u32 = PS3_AUDIO_AO_3WMCTRL_ASOEN_ENABLED(3);

/*
3-Wire Audio Serial output Channel 0-3 Control Register
Configures settings for 3-Wire Serial Audio Output Channel 0-3


 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0 0 0 0 0 0 0 0 0 0 0 0 0 0 0|A|0 0 0 0|A|0|ASO|0 0 0|0|0|0|0|0| AO_3WCTRL
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+

*/
/*
Data Bit Mode
Specifies the number of data bits
0 - 16 bits
1 - reserved
2 - 20 bits
3 - 24 bits
*/
pub const PS3_AUDIO_AO_3WCTRL_ASODB_MASK: u32 = 0x3u32 << 8;
pub const PS3_AUDIO_AO_3WCTRL_ASODB_16BIT: u32 = 0x0u32 << 8;
pub const PS3_AUDIO_AO_3WCTRL_ASODB_RESVD: u32 = 0x1u32 << 8;
pub const PS3_AUDIO_AO_3WCTRL_ASODB_20BIT: u32 = 0x2u32 << 8;
pub const PS3_AUDIO_AO_3WCTRL_ASODB_24BIT: u32 = 0x3u32 << 8;
/*
Data Format Mode
Specifies the data format where (LSB side or MSB) the data(in 20 bit
or 24 bit resolution mode) is put in a 32 bit field.
0 - Data put on LSB side
1 - Data put on MSB side
*/
pub const PS3_AUDIO_AO_3WCTRL_ASODF: u32 = 1u32 << 11;
pub const PS3_AUDIO_AO_3WCTRL_ASODF_LSB: u32 = 0u32 << 11;
pub const PS3_AUDIO_AO_3WCTRL_ASODF_MSB: u32 = 1u32 << 11;
/*
Buffer Reset
Performs buffer reset.  Writing 1 to this bit initializes the
corresponding 3-Wire Audio Output buffers(both L and R).
*/
pub const PS3_AUDIO_AO_3WCTRL_ASOBRST: u32 = 1u32 << 16;
pub const PS3_AUDIO_AO_3WCTRL_ASOBRST_IDLE: u32 = 0u32 << 16;
pub const PS3_AUDIO_AO_3WCTRL_ASOBRST_RESET: u32 = 1u32 << 16;

/*
S/PDIF Audio Output Channel 0/1 Control Register
Configures settings for S/PDIF Audio Output Channel 0/1.

 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |S|0 0 0|S|0 0|S| SPOSR |0 0|SPO|0 0 0 0|S|0|SPO|0 0 0 0 0 0 0|S| AO_SPDCTRL
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/
/*
Buffer reset.  Writing 1 to this bit initializes the
corresponding S/PDIF output buffer pointer.
*/
pub const PS3_AUDIO_AO_SPDCTRL_SPOBRST: u32 = 1u32 << 0;
pub const PS3_AUDIO_AO_SPDCTRL_SPOBRST_IDLE: u32 = 0u32 << 0;
pub const PS3_AUDIO_AO_SPDCTRL_SPOBRST_RESET: u32 = 1u32 << 0;

/*
Data Bit Mode
Specifies number of data bits
0 - 16 bits
1 - Reserved
2 - 20 bits
3 - 24 bits
*/
pub const PS3_AUDIO_AO_SPDCTRL_SPODB_MASK: u32 = 0x3u32 << 8;
pub const PS3_AUDIO_AO_SPDCTRL_SPODB_16BIT: u32 = 0x0u32 << 8;
pub const PS3_AUDIO_AO_SPDCTRL_SPODB_RESVD: u32 = 0x1u32 << 8;
pub const PS3_AUDIO_AO_SPDCTRL_SPODB_20BIT: u32 = 0x2u32 << 8;
pub const PS3_AUDIO_AO_SPDCTRL_SPODB_24BIT: u32 = 0x3u32 << 8;
/*
Data format Mode
Specifies the data format, where (LSB side or MSB)
the data(in 20 or 24 bit resolution) is put in the
32 bit field.
0 - LSB Side
1 - MSB Side
*/
pub const PS3_AUDIO_AO_SPDCTRL_SPODF: u32 = 1u32 << 11;
pub const PS3_AUDIO_AO_SPDCTRL_SPODF_LSB: u32 = 0u32 << 11;
pub const PS3_AUDIO_AO_SPDCTRL_SPODF_MSB: u32 = 1u32 << 11;
/*
Source Select
Specifies the source of the S/PDIF output.  When 0, output
operation is controlled by 3wen[0] of AO_3WMCTRL register.
The SR must have the same setting as the a0_3wmctrl reg.
0 - 3-Wire Audio OUT Ch0 Buffer
1 - S/PDIF buffer
*/
pub const PS3_AUDIO_AO_SPDCTRL_SPOSS_MASK: u32 = 0x3u32 << 16;
pub const PS3_AUDIO_AO_SPDCTRL_SPOSS_3WEN: u32 = 0x0u32 << 16;
pub const PS3_AUDIO_AO_SPDCTRL_SPOSS_SPDIF: u32 = 0x1u32 << 16;
/*
Sampling Rate
Specifies the divide ratio of the bit clock (clock output
from bclko) used by the S/PDIF Output Clock, which
is applied to the master clock selected by mcksel.
*/
pub const PS3_AUDIO_AO_SPDCTRL_SPOSR: u32 = 0xfu32 << 20;
pub const PS3_AUDIO_AO_SPDCTRL_SPOSR_DIV2: u32 = 0x1u32 << 20;
pub const PS3_AUDIO_AO_SPDCTRL_SPOSR_DIV4: u32 = 0x2u32 << 20;
pub const PS3_AUDIO_AO_SPDCTRL_SPOSR_DIV8: u32 = 0x4u32 << 20;
pub const PS3_AUDIO_AO_SPDCTRL_SPOSR_DIV12: u32 = 0x6u32 << 20;
/*
Master Clock Select
0 - Master Clock 0
1 - Master Clock 1
*/
pub const PS3_AUDIO_AO_SPDCTRL_SPOMCKSEL: u32 = 1u32 << 24;
pub const PS3_AUDIO_AO_SPDCTRL_SPOMCKSEL_CLK0: u32 = 0u32 << 24;
pub const PS3_AUDIO_AO_SPDCTRL_SPOMCKSEL_CLK1: u32 = 1u32 << 24;

/*
S/PDIF Output Channel Operational Status
This bit becomes 1 after S/PDIF Output Channel is in
action by setting 1 to spoen.  This bit becomes 0
after S/PDIF Output Channel is out of action by setting
0 to spoen.
*/
pub const PS3_AUDIO_AO_SPDCTRL_SPORUN: u32 = 1u32 << 27;
pub const PS3_AUDIO_AO_SPDCTRL_SPORUN_STOPPED: u32 = 0u32 << 27;
pub const PS3_AUDIO_AO_SPDCTRL_SPORUN_RUNNING: u32 = 1u32 << 27;

/*
S/PDIF Audio Output Channel Output Enable
Enables and disables output operation.  This bit is used
only when sposs = 1
*/
pub const PS3_AUDIO_AO_SPDCTRL_SPOEN: u32 = 1u32 << 31;
pub const PS3_AUDIO_AO_SPDCTRL_SPOEN_DISABLED: u32 = 0u32 << 31;
pub const PS3_AUDIO_AO_SPDCTRL_SPOEN_ENABLED: u32 = 1u32 << 31;

/*
S/PDIF Audio Output Channel Channel Status
Setting Registers.
Configures channel status bit settings for each block
(192 bits).
Output is performed from the MSB(AO_SPDCS0 register bit 31).
The same value is added for subframes within the same frame.
 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |                             SPOCS                             | AO_SPDCS
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+

S/PDIF Audio Output Channel User Bit Setting
Configures user bit settings for each block (384 bits).
Output is performed from the MSB(ao_spdub0 register bit 31).


 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |                             SPOUB                             | AO_SPDUB
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/
/*****************************************************************************
 *
 * DMAC register
 *
 *****************************************************************************/
/*
The PS3_AUDIO_KICK register is used to initiate a DMA transfer and monitor
its status

 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0 0 0 0 0|STATU|0 0 0|  EVENT  |0 0 0 0 0 0 0 0 0 0 0 0 0 0 0|R| KICK
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/
/*
The REQUEST field is written to ACTIVE to initiate a DMA request when EVENT
occurs.
It will return to the DONE state when the request is completed.
The registers for a DMA channel should only be written if REQUEST is IDLE.
*/

pub const PS3_AUDIO_KICK_REQUEST: u32 = 1u32 << 0;
pub const PS3_AUDIO_KICK_REQUEST_IDLE: u32 = 0u32 << 0;
pub const PS3_AUDIO_KICK_REQUEST_ACTIVE: u32 = 1u32 << 0;

/*
 *The EVENT field is used to set the event in which
 *the DMA request becomes active.
 */
pub const PS3_AUDIO_KICK_EVENT_MASK: u32 = 0x1fu32 << 16;
pub const PS3_AUDIO_KICK_EVENT_ALWAYS: u32 = 0x00u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SERIALOUT0_EMPTY: u32 = 0x01u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SERIALOUT0_UNDERFLOW: u32 = 0x02u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SERIALOUT1_EMPTY: u32 = 0x03u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SERIALOUT1_UNDERFLOW: u32 = 0x04u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SERIALOUT2_EMPTY: u32 = 0x05u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SERIALOUT2_UNDERFLOW: u32 = 0x06u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SERIALOUT3_EMPTY: u32 = 0x07u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SERIALOUT3_UNDERFLOW: u32 = 0x08u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SPDIF0_BLOCKTRANSFERCOMPLETE: u32 = 0x09u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SPDIF0_UNDERFLOW: u32 = 0x0Au32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SPDIF0_EMPTY: u32 = 0x0Bu32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SPDIF1_BLOCKTRANSFERCOMPLETE: u32 = 0x0Cu32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SPDIF1_UNDERFLOW: u32 = 0x0Du32 << 16;
pub const PS3_AUDIO_KICK_EVENT_SPDIF1_EMPTY: u32 = 0x0Eu32 << 16;

pub const fn PS3_AUDIO_KICK_EVENT_AUDIO_DMA(n: u32) -> u32 {
    0x13u32.wrapping_add(n) << 16
}
pub const PS3_AUDIO_KICK_EVENT_AUDIO_DMA0: u32 = 0x13u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_AUDIO_DMA1: u32 = 0x14u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_AUDIO_DMA2: u32 = 0x15u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_AUDIO_DMA3: u32 = 0x16u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_AUDIO_DMA4: u32 = 0x17u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_AUDIO_DMA5: u32 = 0x18u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_AUDIO_DMA6: u32 = 0x19u32 << 16;
pub const PS3_AUDIO_KICK_EVENT_AUDIO_DMA7: u32 = 0x1Au32 << 16;
pub const PS3_AUDIO_KICK_EVENT_AUDIO_DMA8: u32 = 0x1Bu32 << 16;
pub const PS3_AUDIO_KICK_EVENT_AUDIO_DMA9: u32 = 0x1Cu32 << 16;

/*
The STATUS field can be used to monitor the progress of a DMA request.
DONE indicates the previous request has completed.
EVENT indicates that the DMA engine is waiting for the EVENT to occur.
PENDING indicates that the DMA engine has not started processing this
request, but the EVENT has occurred.
DMA indicates that the data transfer is in progress.
NOTIFY indicates that the notifier signalling end of transfer is being written.
CLEAR indicated that the previous transfer was cleared.
ERROR indicates the previous transfer requested an unsupported
source/destination combination.
*/

pub const PS3_AUDIO_KICK_STATUS_MASK: u32 = 0x7u32 << 24;
pub const PS3_AUDIO_KICK_STATUS_DONE: u32 = 0x0u32 << 24;
pub const PS3_AUDIO_KICK_STATUS_EVENT: u32 = 0x1u32 << 24;
pub const PS3_AUDIO_KICK_STATUS_PENDING: u32 = 0x2u32 << 24;
pub const PS3_AUDIO_KICK_STATUS_DMA: u32 = 0x3u32 << 24;
pub const PS3_AUDIO_KICK_STATUS_NOTIFY: u32 = 0x4u32 << 24;
pub const PS3_AUDIO_KICK_STATUS_CLEAR: u32 = 0x5u32 << 24;
pub const PS3_AUDIO_KICK_STATUS_ERROR: u32 = 0x6u32 << 24;

/*
The PS3_AUDIO_SOURCE register specifies the source address for transfers.


 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |                      START                      |0 0 0 0 0|TAR| SOURCE
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/

/*
The Audio DMA engine uses 128-byte transfers, thus the address must be aligned
to a 128 byte boundary.  The low seven bits are assumed to be 0.
*/

pub const PS3_AUDIO_SOURCE_START_MASK: u32 = 0x01FFFFFFu32 << 7;

/*
The TARGET field specifies the memory space containing the source address.
*/

pub const PS3_AUDIO_SOURCE_TARGET_MASK: u32 = 3u32 << 0;
pub const PS3_AUDIO_SOURCE_TARGET_SYSTEM_MEMORY: u32 = 2u32 << 0;

/*
The PS3_AUDIO_DEST register specifies the destination address for transfers.


 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |                      START                      |0 0 0 0 0|TAR| DEST
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/

/*
The Audio DMA engine uses 128-byte transfers, thus the address must be aligned
to a 128 byte boundary.  The low seven bits are assumed to be 0.
*/

pub const PS3_AUDIO_DEST_START_MASK: u32 = 0x01FFFFFFu32 << 7;

/*
The TARGET field specifies the memory space containing the destination address
AUDIOFIFO = Audio WriteData FIFO,
*/

pub const PS3_AUDIO_DEST_TARGET_MASK: u32 = 3u32 << 0;
pub const PS3_AUDIO_DEST_TARGET_AUDIOFIFO: u32 = 1u32 << 0;

/*
PS3_AUDIO_DMASIZE specifies the number of 128-byte blocks + 1 to transfer.
So a value of 0 means 128-bytes will get transferred.


 31            24 23           16 15            8 7             0
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
 |0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0|   BLOCKS    | DMASIZE
 +-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-|-+-+-+-+-+-+-+-+
*/

pub const PS3_AUDIO_DMASIZE_BLOCKS_MASK: u32 = 0x7fu32 << 0;

/*
 * source/destination address for internal fifos
 */
pub const fn PS3_AUDIO_AO_3W_LDATA(n: u32) -> u32 {
    0x1000u32.wrapping_add(0x100u32.wrapping_mul(n))
}
pub const fn PS3_AUDIO_AO_3W_RDATA(n: u32) -> u32 {
    0x1080u32.wrapping_add(0x100u32.wrapping_mul(n))
}

pub const fn PS3_AUDIO_AO_SPD_DATA(n: u32) -> u32 {
    0x2000u32.wrapping_add(0x400u32.wrapping_mul(n))
}

/*
 * field attiribute
 *
 *	Read
 *	  ' ' = Other Information
 *	  '-' = Field is part of a write-only register
 *	  'C' = Value read is always the same, constant value line follows (C)
 *	  'R' = Value is read
 *
 *	Write
 *	  ' ' = Other Information
 *	  '-' = Must not be written (D), value ignored when written (R,A,F)
 *	  'W' = Can be written
 *
 *	Internal State
 *	  ' ' = Other Information
 *	  '-' = No internal state
 *	  'X' = Internal state, initial value is unknown
 *	  'I' = Internal state, initial value is known and follows (I)
 *
 *	Declaration/Size
 *	  ' ' = Other Information
 *	  '-' = Does Not Apply
 *	  'V' = Type is void
 *	  'U' = Type is unsigned integer
 *	  'S' = Type is signed integer
 *	  'F' = Type is IEEE floating point
 *	  '1' = Byte size (008)
 *	  '2' = Short size (016)
 *	  '3' = Three byte size (024)
 *	  '4' = Word size (032)
 *	  '8' = Double size (064)
 *
 *	Define Indicator
 *	  ' ' = Other Information
 *	  'D' = Device
 *	  'M' = Memory
 *	  'R' = Register
 *	  'A' = Array of Registers
 *	  'F' = Field
 *	  'V' = Value
 *	  'T' = Task
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
