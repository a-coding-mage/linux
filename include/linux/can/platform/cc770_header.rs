/* SPDX-License-Identifier: GPL-2.0 */

/* CPU Interface Register (0x02) */
pub const CPUIF_CEN: u8 = 0x01; /* Clock Out Enable */
pub const CPUIF_MUX: u8 = 0x04; /* Multiplex */
pub const CPUIF_SLP: u8 = 0x08; /* Sleep */
pub const CPUIF_PWD: u8 = 0x10; /* Power Down Mode */
pub const CPUIF_DMC: u8 = 0x20; /* Divide Memory Clock */
pub const CPUIF_DSC: u8 = 0x40; /* Divide System Clock */
pub const CPUIF_RST: u8 = 0x80; /* Hardware Reset Status */

/* Clock Out Register (0x1f) */
pub const CLKOUT_CD_MASK: u8 = 0x0f; /* Clock Divider mask */
pub const CLKOUT_SL_MASK: u8 = 0x30; /* Slew Rate mask */
pub const CLKOUT_SL_SHIFT: u32 = 4;

/* Bus Configuration Register (0x2f) */
pub const BUSCFG_DR0: u8 = 0x01; /* Disconnect RX0 Input / Select RX input */
pub const BUSCFG_DR1: u8 = 0x02; /* Disconnect RX1 Input / Silent mode */
pub const BUSCFG_DT1: u8 = 0x08; /* Disconnect TX1 Output */
pub const BUSCFG_POL: u8 = 0x20; /* Polarity dominant or recessive */
pub const BUSCFG_CBY: u8 = 0x40; /* Input Comparator Bypass */

#[repr(C)]
pub struct cc770_platform_data {
    pub osc_freq: u32, /* CAN bus oscillator frequency in Hz */

    pub cir: u8, /* CPU Interface Register */
    pub cor: u8, /* Clock Out Register */
    pub bcr: u8, /* Bus Configuration Register */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
