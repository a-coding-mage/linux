// SPDX-License-Identifier: GPL-2.0
// Socionext UniPhier AIO ALSA driver.
// Copyright (c) 2016-2018 Socionext Inc.

// Depends on: linux/bitops.h (GENMASK, BIT macros)

// soc-glue
pub const SG_AOUTEN: u32 = 0x1c04;

// SW view
pub const fn A2CHNMAPCTR0(n: u32) -> u32 {
    0x00000 + 0x40 * n
}

pub const fn A2RBNMAPCTR0(n: u32) -> u32 {
    0x01000 + 0x40 * n
}

pub const fn A2IPORTNMAPCTR0(n: u32) -> u32 {
    0x02000 + 0x40 * n
}

pub const fn A2IPORTNMAPCTR1(n: u32) -> u32 {
    0x02004 + 0x40 * n
}

pub const fn A2IIFNMAPCTR0(n: u32) -> u32 {
    0x03000 + 0x40 * n
}

pub const fn A2OPORTNMAPCTR0(n: u32) -> u32 {
    0x04000 + 0x40 * n
}

pub const fn A2OPORTNMAPCTR1(n: u32) -> u32 {
    0x04004 + 0x40 * n
}

pub const fn A2OPORTNMAPCTR2(n: u32) -> u32 {
    0x04008 + 0x40 * n
}

pub const fn A2OIFNMAPCTR0(n: u32) -> u32 {
    0x05000 + 0x40 * n
}

pub const fn A2ATNMAPCTR0(n: u32) -> u32 {
    0x06000 + 0x40 * n
}

pub const MAPCTR0_EN: u32 = 0x80000000;

// CTL
pub const A2APLLCTR0: u32 = 0x07000;
pub const A2APLLCTR0_APLLXPOW_MASK: u32 = 0x0f;
pub const A2APLLCTR0_APLLXPOW_PWOFF: u32 = 0x0 << 0;
pub const A2APLLCTR0_APLLXPOW_PWON: u32 = 0xf << 0;
pub const A2APLLCTR1: u32 = 0x07004;
pub const A2APLLCTR1_APLLX_MASK: u32 = 0x00010101;
pub const A2APLLCTR1_APLLX_36MHZ: u32 = 0x00000000;
pub const A2APLLCTR1_APLLX_33MHZ: u32 = 0x00000001;
pub const A2EXMCLKSEL0: u32 = 0x07030;
pub const A2EXMCLKSEL0_EXMCLK_MASK: u32 = 0x07;
pub const A2EXMCLKSEL0_EXMCLK_OUTPUT: u32 = 0x0 << 0;
pub const A2EXMCLKSEL0_EXMCLK_INPUT: u32 = 0x7 << 0;
pub const A2SSIFSW: u32 = 0x07050;
pub const A2CH22_2CTR: u32 = 0x07054;
pub const A2AIOINPUTSEL: u32 = 0x070e0;
pub const A2AIOINPUTSEL_RXSEL_PCMI1_MASK: u32 = 0x07;
pub const A2AIOINPUTSEL_RXSEL_PCMI1_HDMIRX1: u32 = 0x2 << 0;
pub const A2AIOINPUTSEL_RXSEL_PCMI2_MASK: u32 = 0x70;
pub const A2AIOINPUTSEL_RXSEL_PCMI2_SIF: u32 = 0x7 << 4;
pub const A2AIOINPUTSEL_RXSEL_PCMI3_MASK: u32 = 0x700;
pub const A2AIOINPUTSEL_RXSEL_PCMI3_EVEA: u32 = 0x1 << 8;
pub const A2AIOINPUTSEL_RXSEL_IECI1_MASK: u32 = 0x7000;
pub const A2AIOINPUTSEL_RXSEL_IECI1_HDMIRX1: u32 = 0x2 << 12;
pub const A2AIOINPUTSEL_RXSEL_MASK: u32 = A2AIOINPUTSEL_RXSEL_PCMI1_MASK | A2AIOINPUTSEL_RXSEL_PCMI2_MASK | A2AIOINPUTSEL_RXSEL_PCMI3_MASK | A2AIOINPUTSEL_RXSEL_IECI1_HDMIRX1;

// INTC
pub const fn INTCHIM(m: u32) -> u32 {
    0x9028 + 0x80 * m
}

pub const fn INTRBIM(m: u32) -> u32 {
    0x9030 + 0x80 * m
}

pub const fn INTCHID(m: u32) -> u32 {
    0xa028 + 0x80 * m
}

pub const fn INTRBID(m: u32) -> u32 {
    0xa030 + 0x80 * m
}

// AIN(PCMINN)
pub const fn IPORTMXCTR1(n: u32) -> u32 {
    0x22000 + 0x400 * n
}

pub const IPORTMXCTR1_LRSEL_MASK: u32 = 0x0c00;
pub const IPORTMXCTR1_LRSEL_RIGHT: u32 = 0x0 << 10;
pub const IPORTMXCTR1_LRSEL_LEFT: u32 = 0x1 << 10;
pub const IPORTMXCTR1_LRSEL_I2S: u32 = 0x2 << 10;
pub const IPORTMXCTR1_OUTBITSEL_MASK: u32 = (0x800003u32 << 8);
pub const IPORTMXCTR1_OUTBITSEL_32: u32 = (0x800000u32 << 8);
pub const IPORTMXCTR1_OUTBITSEL_24: u32 = (0x000000u32 << 8);
pub const IPORTMXCTR1_OUTBITSEL_20: u32 = (0x000001u32 << 8);
pub const IPORTMXCTR1_OUTBITSEL_16: u32 = (0x000002u32 << 8);
pub const IPORTMXCTR1_CHSEL_MASK: u32 = 0x70;
pub const IPORTMXCTR1_CHSEL_ALL: u32 = 0x0 << 4;
pub const IPORTMXCTR1_CHSEL_D0_D2: u32 = 0x1 << 4;
pub const IPORTMXCTR1_CHSEL_D0: u32 = 0x2 << 4;
pub const IPORTMXCTR1_CHSEL_D1: u32 = 0x3 << 4;
pub const IPORTMXCTR1_CHSEL_D2: u32 = 0x4 << 4;
pub const IPORTMXCTR1_CHSEL_DMIX: u32 = 0x5 << 4;
pub const IPORTMXCTR1_FSSEL_MASK: u32 = 0x0f;
pub const IPORTMXCTR1_FSSEL_48: u32 = 0x0 << 0;
pub const IPORTMXCTR1_FSSEL_96: u32 = 0x1 << 0;
pub const IPORTMXCTR1_FSSEL_192: u32 = 0x2 << 0;
pub const IPORTMXCTR1_FSSEL_32: u32 = 0x3 << 0;
pub const IPORTMXCTR1_FSSEL_44_1: u32 = 0x4 << 0;
pub const IPORTMXCTR1_FSSEL_88_2: u32 = 0x5 << 0;
pub const IPORTMXCTR1_FSSEL_176_4: u32 = 0x6 << 0;
pub const IPORTMXCTR1_FSSEL_16: u32 = 0x8 << 0;
pub const IPORTMXCTR1_FSSEL_22_05: u32 = 0x9 << 0;
pub const IPORTMXCTR1_FSSEL_24: u32 = 0xa << 0;
pub const IPORTMXCTR1_FSSEL_8: u32 = 0xb << 0;
pub const IPORTMXCTR1_FSSEL_11_025: u32 = 0xc << 0;
pub const IPORTMXCTR1_FSSEL_12: u32 = 0xd << 0;

pub const fn IPORTMXCTR2(n: u32) -> u32 {
    0x22004 + 0x400 * n
}

pub const IPORTMXCTR2_ACLKSEL_MASK: u32 = 0x000f0000;
pub const IPORTMXCTR2_ACLKSEL_A1: u32 = 0x0 << 16;
pub const IPORTMXCTR2_ACLKSEL_F1: u32 = 0x1 << 16;
pub const IPORTMXCTR2_ACLKSEL_A2: u32 = 0x2 << 16;
pub const IPORTMXCTR2_ACLKSEL_F2: u32 = 0x3 << 16;
pub const IPORTMXCTR2_ACLKSEL_A2PLL: u32 = 0x4 << 16;
pub const IPORTMXCTR2_ACLKSEL_RX1: u32 = 0x5 << 16;
pub const IPORTMXCTR2_ACLKSEL_RX2: u32 = 0x6 << 16;
pub const IPORTMXCTR2_MSSEL_MASK: u32 = 0x8000;
pub const IPORTMXCTR2_MSSEL_SLAVE: u32 = 0x0 << 15;
pub const IPORTMXCTR2_MSSEL_MASTER: u32 = 0x1 << 15;
pub const IPORTMXCTR2_EXTLSIFSSEL_MASK: u32 = 0x4000;
pub const IPORTMXCTR2_EXTLSIFSSEL_36: u32 = 0x0 << 14;
pub const IPORTMXCTR2_EXTLSIFSSEL_24: u32 = 0x1 << 14;
pub const IPORTMXCTR2_DACCKSEL_MASK: u32 = 0x0300;
pub const IPORTMXCTR2_DACCKSEL_1_2: u32 = 0x0 << 8;
pub const IPORTMXCTR2_DACCKSEL_1_3: u32 = 0x1 << 8;
pub const IPORTMXCTR2_DACCKSEL_1_1: u32 = 0x2 << 8;
pub const IPORTMXCTR2_DACCKSEL_2_3: u32 = 0x3 << 8;
pub const IPORTMXCTR2_REQEN_MASK: u32 = 0x0001;
pub const IPORTMXCTR2_REQEN_DISABLE: u32 = 0x0 << 0;
pub const IPORTMXCTR2_REQEN_ENABLE: u32 = 0x1 << 0;

pub const fn IPORTMXCNTCTR(n: u32) -> u32 {
    0x22010 + 0x400 * n
}

pub const fn IPORTMXCOUNTER(n: u32) -> u32 {
    0x22014 + 0x400 * n
}

pub const fn IPORTMXCNTMONI(n: u32) -> u32 {
    0x22018 + 0x400 * n
}

pub const fn IPORTMXACLKSEL0EX(n: u32) -> u32 {
    0x22020 + 0x400 * n
}

pub const IPORTMXACLKSEL0EX_ACLKSEL0EX_MASK: u32 = 0x0f;
pub const IPORTMXACLKSEL0EX_ACLKSEL0EX_INTERNAL: u32 = 0x0 << 0;
pub const IPORTMXACLKSEL0EX_ACLKSEL0EX_EXTERNAL: u32 = 0xf << 0;

pub const fn IPORTMXEXNOE(n: u32) -> u32 {
    0x22070 + 0x400 * n
}

pub const IPORTMXEXNOE_PCMINOE_MASK: u32 = 0x0001;
pub const IPORTMXEXNOE_PCMINOE_OUTPUT: u32 = 0x0 << 0;
pub const IPORTMXEXNOE_PCMINOE_INPUT: u32 = 0x1 << 0;

pub const fn IPORTMXMASK(n: u32) -> u32 {
    0x22078 + 0x400 * n
}

pub const IPORTMXMASK_IUXCKMSK_MASK: u32 = 0x70000;
pub const IPORTMXMASK_IUXCKMSK_ON: u32 = 0x0 << 16;
pub const IPORTMXMASK_IUXCKMSK_OFF: u32 = 0x7 << 16;
pub const IPORTMXMASK_XCKMSK_MASK: u32 = 0x0007;
pub const IPORTMXMASK_XCKMSK_ON: u32 = 0x0 << 0;
pub const IPORTMXMASK_XCKMSK_OFF: u32 = 0x7 << 0;

pub const fn IPORTMXRSTCTR(n: u32) -> u32 {
    0x2207c + 0x400 * n
}

pub const IPORTMXRSTCTR_RSTPI_MASK: u32 = 0x0080;
pub const IPORTMXRSTCTR_RSTPI_RELEASE: u32 = 0x0 << 7;
pub const IPORTMXRSTCTR_RSTPI_RESET: u32 = 0x1 << 7;

// AIN(PBinMX)
pub const fn PBINMXCTR(n: u32) -> u32 {
    0x20200 + 0x40 * n
}

pub const PBINMXCTR_NCONNECT_MASK: u32 = 0x8000;
pub const PBINMXCTR_NCONNECT_CONNECT: u32 = 0x0 << 15;
pub const PBINMXCTR_NCONNECT_DISCONNECT: u32 = 0x1 << 15;
pub const PBINMXCTR_INOUTSEL_MASK: u32 = 0x4000;
pub const PBINMXCTR_INOUTSEL_IN: u32 = 0x0 << 14;
pub const PBINMXCTR_INOUTSEL_OUT: u32 = 0x1 << 14;
pub const PBINMXCTR_PBINSEL_SHIFT: u32 = 8;
pub const PBINMXCTR_ENDIAN_MASK: u32 = 0x0030;
pub const PBINMXCTR_ENDIAN_3210: u32 = 0x0 << 4;
pub const PBINMXCTR_ENDIAN_0123: u32 = 0x1 << 4;
pub const PBINMXCTR_ENDIAN_1032: u32 = 0x2 << 4;
pub const PBINMXCTR_ENDIAN_2301: u32 = 0x3 << 4;
pub const PBINMXCTR_MEMFMT_MASK: u32 = 0x000f;
pub const PBINMXCTR_MEMFMT_D0: u32 = 0x0 << 0;
pub const PBINMXCTR_MEMFMT_5_1CH_DMIX: u32 = 0x1 << 0;
pub const PBINMXCTR_MEMFMT_6CH: u32 = 0x2 << 0;
pub const PBINMXCTR_MEMFMT_4CH: u32 = 0x3 << 0;
pub const PBINMXCTR_MEMFMT_DMIX: u32 = 0x4 << 0;
pub const PBINMXCTR_MEMFMT_1CH: u32 = 0x5 << 0;
pub const PBINMXCTR_MEMFMT_16LR: u32 = 0x6 << 0;
pub const PBINMXCTR_MEMFMT_7_1CH: u32 = 0x7 << 0;
pub const PBINMXCTR_MEMFMT_7_1CH_DMIX: u32 = 0x8 << 0;
pub const PBINMXCTR_MEMFMT_STREAM: u32 = 0xf << 0;

pub const fn PBINMXPAUSECTR0(n: u32) -> u32 {
    0x20204 + 0x40 * n
}

pub const fn PBINMXPAUSECTR1(n: u32) -> u32 {
    0x20208 + 0x40 * n
}

// AOUT
pub const AOUTFADECTR0: u32 = 0x40020;
pub const AOUTENCTR0: u32 = 0x40040;
pub const AOUTENCTR1: u32 = 0x40044;
pub const AOUTENCTR2: u32 = 0x40048;
pub const AOUTRSTCTR0: u32 = 0x40060;
pub const AOUTRSTCTR1: u32 = 0x40064;
pub const AOUTRSTCTR2: u32 = 0x40068;
pub const AOUTSRCRSTCTR0: u32 = 0x400c0;
pub const AOUTSRCRSTCTR1: u32 = 0x400c4;
pub const AOUTSRCRSTCTR2: u32 = 0x400c8;

// AOUT PCMOUT has 5 slots, slot0-3: D0-3, slot4: DMIX
pub const OPORT_SLOT_MAX: u32 = 5;

// AOUT(PCMOUTN)
pub const fn OPORTMXCTR1(n: u32) -> u32 {
    0x42000 + 0x400 * n
}

pub const OPORTMXCTR1_I2SLRSEL_MASK: u32 = 0x1100;
pub const OPORTMXCTR1_I2SLRSEL_RIGHT: u32 = 0x00 << 10;
pub const OPORTMXCTR1_I2SLRSEL_LEFT: u32 = 0x01 << 10;
pub const OPORTMXCTR1_I2SLRSEL_I2S: u32 = 0x11 << 10;
pub const OPORTMXCTR1_OUTBITSEL_MASK: u32 = (0x800003u32 << 8);
pub const OPORTMXCTR1_OUTBITSEL_32: u32 = (0x800000u32 << 8);
pub const OPORTMXCTR1_OUTBITSEL_24: u32 = (0x000000u32 << 8);
pub const OPORTMXCTR1_OUTBITSEL_20: u32 = (0x000001u32 << 8);
pub const OPORTMXCTR1_OUTBITSEL_16: u32 = (0x000002u32 << 8);
pub const OPORTMXCTR1_FSSEL_MASK: u32 = 0x0f;
pub const OPORTMXCTR1_FSSEL_48: u32 = 0x0 << 0;
pub const OPORTMXCTR1_FSSEL_96: u32 = 0x1 << 0;
pub const OPORTMXCTR1_FSSEL_192: u32 = 0x2 << 0;
pub const OPORTMXCTR1_FSSEL_32: u32 = 0x3 << 0;
pub const OPORTMXCTR1_FSSEL_44_1: u32 = 0x4 << 0;
pub const OPORTMXCTR1_FSSEL_88_2: u32 = 0x5 << 0;
pub const OPORTMXCTR1_FSSEL_176_4: u32 = 0x6 << 0;
pub const OPORTMXCTR1_FSSEL_16: u32 = 0x8 << 0;
pub const OPORTMXCTR1_FSSEL_22_05: u32 = 0x9 << 0;
pub const OPORTMXCTR1_FSSEL_24: u32 = 0xa << 0;
pub const OPORTMXCTR1_FSSEL_8: u32 = 0xb << 0;
pub const OPORTMXCTR1_FSSEL_11_025: u32 = 0xc << 0;
pub const OPORTMXCTR1_FSSEL_12: u32 = 0xd << 0;

pub const fn OPORTMXCTR2(n: u32) -> u32 {
    0x42004 + 0x400 * n
}

pub const OPORTMXCTR2_ACLKSEL_MASK: u32 = 0x000f0000;
pub const OPORTMXCTR2_ACLKSEL_A1: u32 = 0x0 << 16;
pub const OPORTMXCTR2_ACLKSEL_F1: u32 = 0x1 << 16;
pub const OPORTMXCTR2_ACLKSEL_A2: u32 = 0x2 << 16;
pub const OPORTMXCTR2_ACLKSEL_F2: u32 = 0x3 << 16;
pub const OPORTMXCTR2_ACLKSEL_A2PLL: u32 = 0x4 << 16;
pub const OPORTMXCTR2_ACLKSEL_RX1: u32 = 0x5 << 16;
pub const OPORTMXCTR2_ACLKSEL_RX2: u32 = 0x6 << 16;
pub const OPORTMXCTR2_MSSEL_MASK: u32 = 0x8000;
pub const OPORTMXCTR2_MSSEL_SLAVE: u32 = 0x0 << 15;
pub const OPORTMXCTR2_MSSEL_MASTER: u32 = 0x1 << 15;
pub const OPORTMXCTR2_EXTLSIFSSEL_MASK: u32 = 0x4000;
pub const OPORTMXCTR2_EXTLSIFSSEL_36: u32 = 0x0 << 14;
pub const OPORTMXCTR2_EXTLSIFSSEL_24: u32 = 0x1 << 14;
pub const OPORTMXCTR2_DACCKSEL_MASK: u32 = 0x0300;
pub const OPORTMXCTR2_DACCKSEL_1_2: u32 = 0x0 << 8;
pub const OPORTMXCTR2_DACCKSEL_1_3: u32 = 0x1 << 8;
pub const OPORTMXCTR2_DACCKSEL_1_1: u32 = 0x2 << 8;
pub const OPORTMXCTR2_DACCKSEL_2_3: u32 = 0x3 << 8;

pub const fn OPORTMXCTR3(n: u32) -> u32 {
    0x42008 + 0x400 * n
}

pub const OPORTMXCTR3_IECTHUR_MASK: u32 = 0x080000;
pub const OPORTMXCTR3_IECTHUR_IECOUT: u32 = 0x0 << 19;
pub const OPORTMXCTR3_IECTHUR_IECIN: u32 = 0x1 << 19;
pub const OPORTMXCTR3_SRCSEL_MASK: u32 = 0x070000;
pub const OPORTMXCTR3_SRCSEL_PCM: u32 = 0x0 << 16;
pub const OPORTMXCTR3_SRCSEL_STREAM: u32 = 0x1 << 16;
pub const OPORTMXCTR3_SRCSEL_CDDTS: u32 = 0x2 << 16;
pub const OPORTMXCTR3_VALID_MASK: u32 = 0x1000;
pub const OPORTMXCTR3_VALID_PCM: u32 = 0x0 << 12;
pub const OPORTMXCTR3_VALID_STREAM: u32 = 0x1 << 12;
pub const OPORTMXCTR3_PMSEL_MASK: u32 = 0x0008;
pub const OPORTMXCTR3_PMSEL_MUTE: u32 = 0x0 << 3;
pub const OPORTMXCTR3_PMSEL_PAUSE: u32 = 0x1 << 3;
pub const OPORTMXCTR3_PMSW_MASK: u32 = 0x0004;
pub const OPORTMXCTR3_PMSW_MUTE_OFF: u32 = 0x0 << 2;
pub const OPORTMXCTR3_PMSW_MUTE_ON: u32 = 0x1 << 2;

pub const fn OPORTMXSRC1CTR(n: u32) -> u32 {
    0x4200c + 0x400 * n
}

pub const OPORTMXSRC1CTR_FSIIPNUM_SHIFT: u32 = 24;
pub const OPORTMXSRC1CTR_THMODE_MASK: u32 = 0x800000;
pub const OPORTMXSRC1CTR_THMODE_SRC: u32 = 0x0 << 23;
pub const OPORTMXSRC1CTR_THMODE_BYPASS: u32 = 0x1 << 23;
pub const OPORTMXSRC1CTR_LOCK_MASK: u32 = 0x010000;
pub const OPORTMXSRC1CTR_LOCK_UNLOCK: u32 = 0x0 << 16;
pub const OPORTMXSRC1CTR_LOCK_LOCK: u32 = 0x1 << 16;
pub const OPORTMXSRC1CTR_SRCPATH_MASK: u32 = 0x008000;
pub const OPORTMXSRC1CTR_SRCPATH_BYPASS: u32 = 0x0 << 15;
pub const OPORTMXSRC1CTR_SRCPATH_CALC: u32 = 0x1 << 15;
pub const OPORTMXSRC1CTR_SYNC_MASK: u32 = 0x004000;
pub const OPORTMXSRC1CTR_SYNC_ASYNC: u32 = 0x0 << 14;
pub const OPORTMXSRC1CTR_SYNC_SYNC: u32 = 0x1 << 14;
pub const OPORTMXSRC1CTR_FSOCK_MASK: u32 = 0x0c00;
pub const OPORTMXSRC1CTR_FSOCK_44_1: u32 = 0x0 << 10;
pub const OPORTMXSRC1CTR_FSOCK_48: u32 = 0x1 << 10;
pub const OPORTMXSRC1CTR_FSOCK_32: u32 = 0x2 << 10;
pub const OPORTMXSRC1CTR_FSICK_MASK: u32 = 0x0300;
pub const OPORTMXSRC1CTR_FSICK_44_1: u32 = 0x0 << 8;
pub const OPORTMXSRC1CTR_FSICK_48: u32 = 0x1 << 8;
pub const OPORTMXSRC1CTR_FSICK_32: u32 = 0x2 << 8;
pub const OPORTMXSRC1CTR_FSIIPSEL_MASK: u32 = 0x0030;
pub const OPORTMXSRC1CTR_FSIIPSEL_INNER: u32 = 0x0 << 4;
pub const OPORTMXSRC1CTR_FSIIPSEL_OUTER: u32 = 0x1 << 4;
pub const OPORTMXSRC1CTR_FSISEL_MASK: u32 = 0x000f;
pub const OPORTMXSRC1CTR_FSISEL_ACLK: u32 = 0x0 << 0;
pub const OPORTMXSRC1CTR_FSISEL_DD: u32 = 0x1 << 0;

pub const fn OPORTMXDSDMUTEDAT(n: u32) -> u32 {
    0x42020 + 0x400 * n
}

pub const fn OPORTMXDXDFREQMODE(n: u32) -> u32 {
    0x42024 + 0x400 * n
}

pub const fn OPORTMXDSDSEL(n: u32) -> u32 {
    0x42028 + 0x400 * n
}

pub const fn OPORTMXDSDPORT(n: u32) -> u32 {
    0x4202c + 0x400 * n
}

pub const fn OPORTMXACLKSEL0EX(n: u32) -> u32 {
    0x42030 + 0x400 * n
}

pub const fn OPORTMXPATH(n: u32) -> u32 {
    0x42040 + 0x400 * n
}

pub const fn OPORTMXSYNC(n: u32) -> u32 {
    0x42044 + 0x400 * n
}

pub const fn OPORTMXREPET(n: u32) -> u32 {
    0x42050 + 0x400 * n
}

// SBF_(frame, shift) macro: (((frame) * 2 - 1) << shift)
pub const fn SBF_(frame: u32, shift: u32) -> u32 {
    ((frame * 2 - 1) << shift)
}

pub const OPORTMXREPET_STRLENGTH_AC3: u32 = SBF_(2, 16);
pub const OPORTMXREPET_STRLENGTH_MPA: u32 = SBF_(2, 16);
pub const OPORTMXREPET_STRLENGTH_MP3: u32 = SBF_(2, 16);
pub const OPORTMXREPET_STRLENGTH_DTS1: u32 = SBF_(2, 16);
pub const OPORTMXREPET_STRLENGTH_DTS2: u32 = SBF_(2, 16);
pub const OPORTMXREPET_STRLENGTH_DTS3: u32 = SBF_(2, 16);
pub const OPORTMXREPET_STRLENGTH_AAC: u32 = SBF_(2, 16);
pub const OPORTMXREPET_PMLENGTH_AC3: u32 = SBF_(2, 0);
pub const OPORTMXREPET_PMLENGTH_MPA: u32 = SBF_(2, 0);
pub const OPORTMXREPET_PMLENGTH_MP3: u32 = SBF_(2, 0);
pub const OPORTMXREPET_PMLENGTH_DTS1: u32 = SBF_(2, 0);
pub const OPORTMXREPET_PMLENGTH_DTS2: u32 = SBF_(2, 0);
pub const OPORTMXREPET_PMLENGTH_DTS3: u32 = SBF_(2, 0);
pub const OPORTMXREPET_PMLENGTH_AAC: u32 = SBF_(2, 0);

pub const fn OPORTMXPAUDAT(n: u32) -> u32 {
    0x42054 + 0x400 * n
}

// Depends on: IEC61937_PC_PAUSE, IEC61937_FRM_PAU_* constants from external headers
pub const OPORTMXPAUDAT_PAUSEPC_CMN: u32 = 0x00050000;
pub const OPORTMXPAUDAT_PAUSEPD_AC3: u32 = 0x00000048;
pub const OPORTMXPAUDAT_PAUSEPD_MPA: u32 = 0x00000048;
pub const OPORTMXPAUDAT_PAUSEPD_MP3: u32 = 0x00000048;
pub const OPORTMXPAUDAT_PAUSEPD_DTS1: u32 = 0x00000048;
pub const OPORTMXPAUDAT_PAUSEPD_DTS2: u32 = 0x00000048;
pub const OPORTMXPAUDAT_PAUSEPD_DTS3: u32 = 0x00000048;
pub const OPORTMXPAUDAT_PAUSEPD_AAC: u32 = 0x00000048;

pub const fn OPORTMXRATE_I(n: u32) -> u32 {
    0x420e4 + 0x400 * n
}

pub const OPORTMXRATE_I_EQU_MASK: u32 = 0x80000000;
pub const OPORTMXRATE_I_EQU_NOTEQUAL: u32 = 0x0 << 31;
pub const OPORTMXRATE_I_EQU_EQUAL: u32 = 0x1 << 31;
pub const OPORTMXRATE_I_SRCBPMD_MASK: u32 = 0x20000000;
pub const OPORTMXRATE_I_SRCBPMD_BYPASS: u32 = 0x0 << 29;
pub const OPORTMXRATE_I_SRCBPMD_SRC: u32 = 0x1 << 29;
pub const OPORTMXRATE_I_LRCKSTP_MASK: u32 = 0x01000000;
pub const OPORTMXRATE_I_LRCKSTP_START: u32 = 0x0 << 24;
pub const OPORTMXRATE_I_LRCKSTP_STOP: u32 = 0x1 << 24;
pub const OPORTMXRATE_I_ACLKSRC_MASK: u32 = 0x0000f000;
pub const OPORTMXRATE_I_ACLKSRC_APLL: u32 = 0x0 << 12;
pub const OPORTMXRATE_I_ACLKSRC_USB: u32 = 0x1 << 12;
pub const OPORTMXRATE_I_ACLKSRC_HSC: u32 = 0x3 << 12;
// if OPORTMXRATE_I_ACLKSRC_APLL
pub const OPORTMXRATE_I_ACLKSEL_MASK: u32 = 0x00000f00;
pub const OPORTMXRATE_I_ACLKSEL_APLLA1: u32 = 0x0 << 8;
pub const OPORTMXRATE_I_ACLKSEL_APLLF1: u32 = 0x1 << 8;
pub const OPORTMXRATE_I_ACLKSEL_APLLA2: u32 = 0x2 << 8;
pub const OPORTMXRATE_I_ACLKSEL_APLLF2: u32 = 0x3 << 8;
pub const OPORTMXRATE_I_ACLKSEL_APLL: u32 = 0x4 << 8;
pub const OPORTMXRATE_I_ACLKSEL_HDMI1: u32 = 0x5 << 8;
pub const OPORTMXRATE_I_ACLKSEL_HDMI2: u32 = 0x6 << 8;
pub const OPORTMXRATE_I_ACLKSEL_AI1ADCCK: u32 = 0xc << 8;
pub const OPORTMXRATE_I_ACLKSEL_AI2ADCCK: u32 = 0xd << 8;
pub const OPORTMXRATE_I_ACLKSEL_AI3ADCCK: u32 = 0xe << 8;
pub const OPORTMXRATE_I_MCKSEL_MASK: u32 = 0x000000f0;
pub const OPORTMXRATE_I_MCKSEL_36: u32 = 0x0 << 4;
pub const OPORTMXRATE_I_MCKSEL_33: u32 = 0x1 << 4;
pub const OPORTMXRATE_I_MCKSEL_HSC27: u32 = 0xb << 4;
pub const OPORTMXRATE_I_FSSEL_MASK: u32 = 0x0000000f;
pub const OPORTMXRATE_I_FSSEL_48: u32 = 0x0 << 0;
pub const OPORTMXRATE_I_FSSEL_96: u32 = 0x1 << 0;
pub const OPORTMXRATE_I_FSSEL_192: u32 = 0x2 << 0;
pub const OPORTMXRATE_I_FSSEL_32: u32 = 0x3 << 0;
pub const OPORTMXRATE_I_FSSEL_44_1: u32 = 0x4 << 0;
pub const OPORTMXRATE_I_FSSEL_88_2: u32 = 0x5 << 0;
pub const OPORTMXRATE_I_FSSEL_176_4: u32 = 0x6 << 0;
pub const OPORTMXRATE_I_FSSEL_16: u32 = 0x8 << 0;
pub const OPORTMXRATE_I_FSSEL_22_05: u32 = 0x9 << 0;
pub const OPORTMXRATE_I_FSSEL_24: u32 = 0xa << 0;
pub const OPORTMXRATE_I_FSSEL_8: u32 = 0xb << 0;
pub const OPORTMXRATE_I_FSSEL_11_025: u32 = 0xc << 0;
pub const OPORTMXRATE_I_FSSEL_12: u32 = 0xd << 0;

pub const fn OPORTMXEXNOE(n: u32) -> u32 {
    0x420f0 + 0x400 * n
}

pub const fn OPORTMXMASK(n: u32) -> u32 {
    0x420f8 + 0x400 * n
}

pub const OPORTMXMASK_IUDXMSK_MASK: u32 = 0x1f000000;
pub const OPORTMXMASK_IUDXMSK_ON: u32 = 0x00 << 24;
pub const OPORTMXMASK_IUDXMSK_OFF: u32 = 0x1f << 24;
pub const OPORTMXMASK_IUXCKMSK_MASK: u32 = 0x00070000;
pub const OPORTMXMASK_IUXCKMSK_ON: u32 = 0x0 << 16;
pub const OPORTMXMASK_IUXCKMSK_OFF: u32 = 0x7 << 16;
pub const OPORTMXMASK_DXMSK_MASK: u32 = 0x00001f00;
pub const OPORTMXMASK_DXMSK_ON: u32 = 0x00 << 8;
pub const OPORTMXMASK_DXMSK_OFF: u32 = 0x1f << 8;
pub const OPORTMXMASK_XCKMSK_MASK: u32 = 0x00000007;
pub const OPORTMXMASK_XCKMSK_ON: u32 = 0x0 << 0;
pub const OPORTMXMASK_XCKMSK_OFF: u32 = 0x7 << 0;

pub const fn OPORTMXDEBUG(n: u32) -> u32 {
    0x420fc + 0x400 * n
}

pub const fn OPORTMXTYVOLPARA1(n: u32, m: u32) -> u32 {
    0x42100 + 0x400 * n + 0x20 * m
}

pub const OPORTMXTYVOLPARA1_SLOPEU_MASK: u32 = 0xffff0000;

pub const fn OPORTMXTYVOLPARA2(n: u32, m: u32) -> u32 {
    0x42104 + 0x400 * n + 0x20 * m
}

pub const OPORTMXTYVOLPARA2_FADE_MASK: u32 = 0x00030000;
pub const OPORTMXTYVOLPARA2_FADE_NOOP: u32 = 0x0 << 16;
pub const OPORTMXTYVOLPARA2_FADE_FADEOUT: u32 = 0x1 << 16;
pub const OPORTMXTYVOLPARA2_FADE_FADEIN: u32 = 0x2 << 16;
pub const OPORTMXTYVOLPARA2_TARGET_MASK: u32 = 0x0000ffff;

pub const fn OPORTMXTYVOLGAINSTATUS(n: u32, m: u32) -> u32 {
    0x42108 + 0x400 * n + 0x20 * m
}

pub const OPORTMXTYVOLGAINSTATUS_CUR_MASK: u32 = 0x0000ffff;

pub const fn OPORTMXTYSLOTCTR(n: u32, m: u32) -> u32 {
    0x42114 + 0x400 * n + 0x20 * m
}

pub const OPORTMXTYSLOTCTR_MODE: u32 = 0x8000;
pub const OPORTMXTYSLOTCTR_SLOTSEL_MASK: u32 = 0x0f00;
pub const OPORTMXTYSLOTCTR_SLOTSEL_SLOT0: u32 = 0x8 << 8;
pub const OPORTMXTYSLOTCTR_SLOTSEL_SLOT1: u32 = 0x9 << 8;
pub const OPORTMXTYSLOTCTR_SLOTSEL_SLOT2: u32 = 0xa << 8;
pub const OPORTMXTYSLOTCTR_SLOTSEL_SLOT3: u32 = 0xb << 8;
pub const OPORTMXTYSLOTCTR_SLOTSEL_SLOT4: u32 = 0xc << 8;
pub const OPORTMXT0SLOTCTR_MUTEOFF_MASK: u32 = 0x0002;
pub const OPORTMXT0SLOTCTR_MUTEOFF_MUTE: u32 = 0x0 << 1;
pub const OPORTMXT0SLOTCTR_MUTEOFF_UNMUTE: u32 = 0x1 << 1;

pub const fn OPORTMXTYRSTCTR(n: u32, m: u32) -> u32 {
    0x4211c + 0x400 * n + 0x20 * m
}

pub const OPORTMXT0RSTCTR_RST_MASK: u32 = 0x0002;
pub const OPORTMXT0RSTCTR_RST_OFF: u32 = 0x0 << 1;
pub const OPORTMXT0RSTCTR_RST_ON: u32 = 0x1 << 1;

// AOUT(PBoutMX)
pub const fn PBOUTMXCTR0(n: u32) -> u32 {
    0x40200 + 0x40 * n
}

pub const PBOUTMXCTR0_ENDIAN_MASK: u32 = 0x0030;
pub const PBOUTMXCTR0_ENDIAN_3210: u32 = 0x0 << 4;
pub const PBOUTMXCTR0_ENDIAN_0123: u32 = 0x1 << 4;
pub const PBOUTMXCTR0_ENDIAN_1032: u32 = 0x2 << 4;
pub const PBOUTMXCTR0_ENDIAN_2301: u32 = 0x3 << 4;
pub const PBOUTMXCTR0_MEMFMT_MASK: u32 = 0x000f;
pub const PBOUTMXCTR0_MEMFMT_10CH: u32 = 0x0 << 0;
pub const PBOUTMXCTR0_MEMFMT_8CH: u32 = 0x1 << 0;
pub const PBOUTMXCTR0_MEMFMT_6CH: u32 = 0x2 << 0;
pub const PBOUTMXCTR0_MEMFMT_4CH: u32 = 0x3 << 0;
pub const PBOUTMXCTR0_MEMFMT_2CH: u32 = 0x4 << 0;
pub const PBOUTMXCTR0_MEMFMT_STREAM: u32 = 0x5 << 0;
pub const PBOUTMXCTR0_MEMFMT_1CH: u32 = 0x6 << 0;

pub const fn PBOUTMXCTR1(n: u32) -> u32 {
    0x40204 + 0x40 * n
}

pub const fn PBOUTMXINTCTR(n: u32) -> u32 {
    0x40208 + 0x40 * n
}

// A2D(subsystem)
pub const CDA2D_STRT0: u32 = 0x10000;
pub const CDA2D_STRT0_STOP_MASK: u32 = 0x80000000;
pub const CDA2D_STRT0_STOP_START: u32 = 0x0 << 31;
pub const CDA2D_STRT0_STOP_STOP: u32 = 0x1 << 31;
pub const CDA2D_STAT0: u32 = 0x10020;
pub const CDA2D_TEST: u32 = 0x100a0;
pub const CDA2D_TEST_DDR_MODE_MASK: u32 = 0x0000000c;
pub const CDA2D_TEST_DDR_MODE_EXTON0: u32 = 0x0 << 2;
pub const CDA2D_TEST_DDR_MODE_EXTOFF1: u32 = 0x3 << 2;
pub const CDA2D_STRTADRSLOAD: u32 = 0x100b0;

pub const fn CDA2D_CHMXCTRL1(n: u32) -> u32 {
    0x12000 + 0x80 * n
}

pub const CDA2D_CHMXCTRL1_INDSIZE_MASK: u32 = 0x0001;
pub const CDA2D_CHMXCTRL1_INDSIZE_FINITE: u32 = 0x0 << 0;
pub const CDA2D_CHMXCTRL1_INDSIZE_INFINITE: u32 = 0x1 << 0;

pub const fn CDA2D_CHMXCTRL2(n: u32) -> u32 {
    0x12004 + 0x80 * n
}

pub const fn CDA2D_CHMXSRCAMODE(n: u32) -> u32 {
    0x12020 + 0x80 * n
}

pub const fn CDA2D_CHMXDSTAMODE(n: u32) -> u32 {
    0x12024 + 0x80 * n
}

pub const CDA2D_CHMXAMODE_ENDIAN_MASK: u32 = 0x00030000;
pub const CDA2D_CHMXAMODE_ENDIAN_3210: u32 = 0x0 << 16;
pub const CDA2D_CHMXAMODE_ENDIAN_0123: u32 = 0x1 << 16;
pub const CDA2D_CHMXAMODE_ENDIAN_1032: u32 = 0x2 << 16;
pub const CDA2D_CHMXAMODE_ENDIAN_2301: u32 = 0x3 << 16;
pub const CDA2D_CHMXAMODE_RSSEL_SHIFT: u32 = 8;
pub const CDA2D_CHMXAMODE_AUPDT_MASK: u32 = 0x00000030;
pub const CDA2D_CHMXAMODE_AUPDT_INC: u32 = 0x0 << 4;
pub const CDA2D_CHMXAMODE_AUPDT_FIX: u32 = 0x2 << 4;
pub const CDA2D_CHMXAMODE_TYPE_MASK: u32 = 0x0000000c;
pub const CDA2D_CHMXAMODE_TYPE_NORMAL: u32 = 0x0 << 2;
pub const CDA2D_CHMXAMODE_TYPE_RING: u32 = 0x1 << 2;

pub const fn CDA2D_CHMXSRCSTRTADRS(n: u32) -> u32 {
    0x12030 + 0x80 * n
}

pub const fn CDA2D_CHMXSRCSTRTADRSU(n: u32) -> u32 {
    0x12034 + 0x80 * n
}

pub const fn CDA2D_CHMXDSTSTRTADRS(n: u32) -> u32 {
    0x12038 + 0x80 * n
}

pub const fn CDA2D_CHMXDSTSTRTADRSU(n: u32) -> u32 {
    0x1203c + 0x80 * n
}

// A2D(ring buffer)
pub const CDA2D_RBFLUSH0: u32 = 0x10040;
pub const CDA2D_RBADRSLOAD: u32 = 0x100b4;
pub const CDA2D_RDPTRLOAD: u32 = 0x100b8;
pub const CDA2D_RDPTRLOAD_LSFLAG_LOAD: u32 = 0x0 << 31;
pub const CDA2D_RDPTRLOAD_LSFLAG_STORE: u32 = 0x1 << 31;
pub const CDA2D_WRPTRLOAD: u32 = 0x100bc;
pub const CDA2D_WRPTRLOAD_LSFLAG_LOAD: u32 = 0x0 << 31;
pub const CDA2D_WRPTRLOAD_LSFLAG_STORE: u32 = 0x1 << 31;

pub const fn CDA2D_RBMXBGNADRS(n: u32) -> u32 {
    0x14000 + 0x80 * n
}

pub const fn CDA2D_RBMXBGNADRSU(n: u32) -> u32 {
    0x14004 + 0x80 * n
}

pub const fn CDA2D_RBMXENDADRS(n: u32) -> u32 {
    0x14008 + 0x80 * n
}

pub const fn CDA2D_RBMXENDADRSU(n: u32) -> u32 {
    0x1400c + 0x80 * n
}

pub const fn CDA2D_RBMXBTH(n: u32) -> u32 {
    0x14038 + 0x80 * n
}

pub const fn CDA2D_RBMXRTH(n: u32) -> u32 {
    0x1403c + 0x80 * n
}

pub const fn CDA2D_RBMXRDPTR(n: u32) -> u32 {
    0x14020 + 0x80 * n
}

pub const fn CDA2D_RBMXRDPTRU(n: u32) -> u32 {
    0x14024 + 0x80 * n
}

pub const fn CDA2D_RBMXWRPTR(n: u32) -> u32 {
    0x14028 + 0x80 * n
}

pub const fn CDA2D_RBMXWRPTRU(n: u32) -> u32 {
    0x1402c + 0x80 * n
}

pub const CDA2D_RBMXPTRU_PTRU_MASK: u32 = 0x00000003;

pub const fn CDA2D_RBMXCNFG(n: u32) -> u32 {
    0x14030 + 0x80 * n
}

pub const fn CDA2D_RBMXIR(n: u32) -> u32 {
    0x14014 + 0x80 * n
}

pub const fn CDA2D_RBMXIE(n: u32) -> u32 {
    0x14018 + 0x80 * n
}

pub const fn CDA2D_RBMXID(n: u32) -> u32 {
    0x1401c + 0x80 * n
}

pub const CDA2D_RBMXIX_SPACE: u32 = 0x0008;
pub const CDA2D_RBMXIX_REMAIN: u32 = 0x0010;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
