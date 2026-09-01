/* SPDX-License-Identifier: GPL-2.0 */

pub const CM9780_JACK: u16 = 0x62;
pub const CM9780_MIXER: u16 = 0x64;
pub const CM9780_GPIO_SETUP: u16 = 0x70;
pub const CM9780_GPIO_STATUS: u16 = 0x72;

/* jack control */
pub const CM9780_RSOE: u16 = 0x0001;
pub const CM9780_CBOE: u16 = 0x0002;
pub const CM9780_SSOE: u16 = 0x0004;
pub const CM9780_FROE: u16 = 0x0008;
pub const CM9780_HP2FMICOE: u16 = 0x0010;
pub const CM9780_CB2MICOE: u16 = 0x0020;
pub const CM9780_FMIC2LI: u16 = 0x0040;
pub const CM9780_FMIC2MIC: u16 = 0x0080;
pub const CM9780_HP2LI: u16 = 0x0100;
pub const CM9780_HP2MIC: u16 = 0x0200;
pub const CM9780_MIC2LI: u16 = 0x0400;
pub const CM9780_MIC2MIC: u16 = 0x0800;
pub const CM9780_LI2LI: u16 = 0x1000;
pub const CM9780_LI2MIC: u16 = 0x2000;
pub const CM9780_LO2LI: u16 = 0x4000;
pub const CM9780_LO2MIC: u16 = 0x8000;

/* mixer control */
pub const CM9780_BSTSEL: u16 = 0x0001;
pub const CM9780_STRO_MIC: u16 = 0x0002;
pub const CM9780_SPDI_FREX: u16 = 0x0004;
pub const CM9780_SPDI_SSEX: u16 = 0x0008;
pub const CM9780_SPDI_CBEX: u16 = 0x0010;
pub const CM9780_SPDI_RSEX: u16 = 0x0020;
pub const CM9780_MIX2FR: u16 = 0x0040;
pub const CM9780_MIX2SS: u16 = 0x0080;
pub const CM9780_MIX2CB: u16 = 0x0100;
pub const CM9780_MIX2RS: u16 = 0x0200;
pub const CM9780_MIX2FR_EX: u16 = 0x0400;
pub const CM9780_MIX2SS_EX: u16 = 0x0800;
pub const CM9780_MIX2CB_EX: u16 = 0x1000;
pub const CM9780_MIX2RS_EX: u16 = 0x2000;
pub const CM9780_P47_IO: u16 = 0x4000;
pub const CM9780_PCBSW: u16 = 0x8000;

/* GPIO setup */
pub const CM9780_GPI0EN: u16 = 0x0001;
pub const CM9780_GPI1EN: u16 = 0x0002;
pub const CM9780_SENSE_P: u16 = 0x0004;
pub const CM9780_LOCK_P: u16 = 0x0008;
pub const CM9780_GPIO0P: u16 = 0x0010;
pub const CM9780_GPIO1P: u16 = 0x0020;
pub const CM9780_GPIO0IO: u16 = 0x0100;
pub const CM9780_GPIO1IO: u16 = 0x0200;

/* GPIO status */
pub const CM9780_GPO0: u16 = 0x0001;
pub const CM9780_GPO1: u16 = 0x0002;
pub const CM9780_GPIO0S: u16 = 0x0010;
pub const CM9780_GPIO1S: u16 = 0x0020;
pub const CM9780_GPII0S: u16 = 0x0100;
pub const CM9780_GPII1S: u16 = 0x0200;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
