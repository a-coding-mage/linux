/* SPDX-License-Identifier: GPL-2.0 */
/*
    Aureal Vortex Soundcard driver.

    IO addr collected from asp4core.vxd:
    function    address
    0005D5A0    13004
    00080674    14004
    00080AFF    12818

 */

pub const CHIP_AU8830: bool = true;

pub const CARD_NAME: &str = "Aureal Vortex 2";
pub const CARD_NAME_SHORT: &str = "au8830";

pub const NR_ADB: u32 = 0x20;
pub const NR_SRC: u32 = 0x10;
pub const NR_A3D: u32 = 0x10;
pub const NR_MIXIN: u32 = 0x20;
pub const NR_MIXOUT: u32 = 0x10;
pub const NR_WT: u32 = 0x40;

/* ADBDMA */
pub const VORTEX_ADBDMA_STAT: u32 = 0x27e00; /* read only, subbuffer, DMA pos */
pub const POS_MASK: u32 = 0x00000fff;
pub const POS_SHIFT: u32 = 0x0;
pub const ADB_SUBBUF_MASK: u32 = 0x00003000; /* ADB only. */
pub const ADB_SUBBUF_SHIFT: u32 = 0xc; /* ADB only. */
pub const VORTEX_ADBDMA_CTRL: u32 = 0x27a00; /* write only; format, flags, DMA pos */
pub const OFFSET_MASK: u32 = 0x00000fff;
pub const OFFSET_SHIFT: u32 = 0x0;
pub const IE_MASK: u32 = 0x00001000; /* interrupt enable. */
pub const IE_SHIFT: u32 = 0xc;
pub const DIR_MASK: u32 = 0x00002000; /* Direction. */
pub const DIR_SHIFT: u32 = 0xd;
pub const FMT_MASK: u32 = 0x0003c000;
pub const FMT_SHIFT: u32 = 0xe;
pub const ADB_FIFO_EN_SHIFT: u32 = 0x15;
pub const ADB_FIFO_EN: u32 = 1 << 0x15;
// The ADB masks and shift also are valid for the wtdma, except if specified otherwise.
pub const VORTEX_ADBDMA_BUFCFG0: u32 = 0x27800;
pub const VORTEX_ADBDMA_BUFCFG1: u32 = 0x27804;
pub const VORTEX_ADBDMA_BUFBASE: u32 = 0x27400;
pub const VORTEX_ADBDMA_START: u32 = 0x27c00; /* Which subbuffer starts */

pub const VORTEX_ADBDMA_STATUS: u32 = 0x27A90; /* stored at AdbDma->this_10 / 2 DWORD in size. */
/* Starting at the MSB, each pair of bits seem to be the current DMA page. */
/* This current page bits are consistent (same value) with VORTEX_ADBDMA_STAT) */

/* DMA */
pub const VORTEX_ENGINE_CTRL: u32 = 0x27ae8;
pub const ENGINE_INIT: u32 = 0x1380000;

/* WTDMA */
pub const VORTEX_WTDMA_CTRL: u32 = 0x27900; /* format, DMA pos */
pub const VORTEX_WTDMA_STAT: u32 = 0x27d00; /* DMA subbuf, DMA pos */
pub const WT_SUBBUF_MASK: u32 = 0x3;
pub const WT_SUBBUF_SHIFT: u32 = 0xc;
pub const VORTEX_WTDMA_BUFBASE: u32 = 0x27000;
pub const VORTEX_WTDMA_BUFCFG0: u32 = 0x27600;
pub const VORTEX_WTDMA_BUFCFG1: u32 = 0x27604;
pub const VORTEX_WTDMA_START: u32 = 0x27b00; /* which subbuffer is first */

/* ADB */
pub const VORTEX_ADB_SR: u32 = 0x28400; /* Samplerates enable/disable */
pub const VORTEX_ADB_RTBASE: u32 = 0x28000;
pub const VORTEX_ADB_RTBASE_COUNT: u32 = 173;
pub const VORTEX_ADB_CHNBASE: u32 = 0x282b4;
pub const VORTEX_ADB_CHNBASE_COUNT: u32 = 24;
pub const ROUTE_MASK: u32 = 0xffff;
pub const SOURCE_MASK: u32 = 0xff00;
pub const ADB_MASK: u32 = 0xff;
pub const ADB_SHIFT: u32 = 0x8;
/* ADB address */
pub const OFFSET_ADBDMA: u32 = 0x00;
pub const OFFSET_ADBDMAB: u32 = 0x20;
pub const OFFSET_SRCIN: u32 = 0x40;
pub const OFFSET_SRCOUT: u32 = 0x20; /* ch 0x11 */
pub const OFFSET_MIXIN: u32 = 0x50; /* ch 0x11 */
pub const OFFSET_MIXOUT: u32 = 0x30; /* ch 0x11 */
pub const OFFSET_CODECIN: u32 = 0x70; /* ch 0x11 */ /* adb source */
pub const OFFSET_CODECOUT: u32 = 0x88; /* ch 0x11 */ /* adb target */
pub const OFFSET_SPORTIN: u32 = 0x78; /* ch 0x13 ADB source. 2 routes. */
pub const OFFSET_SPORTOUT: u32 = 0x90; /* ch 0x13 ADB sink. 2 routes. */
pub const OFFSET_SPDIFIN: u32 = 0x7A; /* ch 0x14 ADB source. */
pub const OFFSET_SPDIFOUT: u32 = 0x92; /* ch 0x14 ADB sink. */
pub const OFFSET_AC98IN: u32 = 0x7c; /* ch 0x14 ADB source. */
pub const OFFSET_AC98OUT: u32 = 0x94; /* ch 0x14 ADB sink. */
pub const OFFSET_EQIN: u32 = 0xa0; /* ch 0x11 */
pub const OFFSET_EQOUT: u32 = 0x7e; /* ch 0x11 */ /* 2 routes on ch 0x11 */
pub const OFFSET_A3DIN: u32 = 0x70; /* ADB sink. */
pub const OFFSET_A3DOUT: u32 = 0xA6; /* ADB source. 2 routes per slice = 8 */
pub const OFFSET_WT0: u32 = 0x40; /* WT bank 0 output. 0x40 - 0x65 */
pub const OFFSET_WT1: u32 = 0x80; /* WT bank 1 output. 0x80 - 0xA5 */
/* WT sources offset : 0x00-0x1f Direct stream. */
/* WT sources offset : 0x20-0x25 Mixed Output. */
pub const OFFSET_XTALKOUT: u32 = 0x66; /* crosstalk canceller (source) 2 routes */
pub const OFFSET_XTALKIN: u32 = 0x96; /* crosstalk canceller (sink). 10 routes */
pub const OFFSET_EFXOUT: u32 = 0x68; /* ADB source. 8 routes. */
pub const OFFSET_EFXIN: u32 = 0x80; /* ADB sink. 8 routes. */

/* ADB route translate helper */
pub const fn ADB_DMA(x: u32) -> u32 {
    x
}
pub const fn ADB_SRCOUT(x: u32) -> u32 {
    x + OFFSET_SRCOUT
}
pub const fn ADB_SRCIN(x: u32) -> u32 {
    x + OFFSET_SRCIN
}
pub const fn ADB_MIXOUT(x: u32) -> u32 {
    x + OFFSET_MIXOUT
}
pub const fn ADB_MIXIN(x: u32) -> u32 {
    x + OFFSET_MIXIN
}
pub const fn ADB_CODECIN(x: u32) -> u32 {
    x + OFFSET_CODECIN
}
pub const fn ADB_CODECOUT(x: u32) -> u32 {
    x + OFFSET_CODECOUT
}
pub const fn ADB_SPORTIN(x: u32) -> u32 {
    x + OFFSET_SPORTIN
}
pub const fn ADB_SPORTOUT(x: u32) -> u32 {
    x + OFFSET_SPORTOUT
}
pub const fn ADB_SPDIFIN(x: u32) -> u32 {
    x + OFFSET_SPDIFIN
}
pub const fn ADB_SPDIFOUT(x: u32) -> u32 {
    x + OFFSET_SPDIFOUT
}
pub const fn ADB_EQIN(x: u32) -> u32 {
    x + OFFSET_EQIN
}
pub const fn ADB_EQOUT(x: u32) -> u32 {
    x + OFFSET_EQOUT
}
pub const fn ADB_A3DOUT(x: u32) -> u32 {
    x + OFFSET_A3DOUT
} /* 0x10 A3D blocks */
pub const fn ADB_A3DIN(x: u32) -> u32 {
    x + OFFSET_A3DIN
}
// Original disabled C macro:
// #define ADB_WTOUT(x) ((x<x20)?(x + OFFSET_WT0):(x + OFFSET_WT1))
pub const fn ADB_WTOUT(x: u32, y: u32) -> u32 {
    if x == 0 {
        y + OFFSET_WT0
    } else {
        y + OFFSET_WT1
    }
}
pub const fn ADB_XTALKIN(x: u32) -> u32 {
    x + OFFSET_XTALKIN
}
pub const fn ADB_XTALKOUT(x: u32) -> u32 {
    x + OFFSET_XTALKOUT
}

pub const MIX_DEFIGAIN: u32 = 0x08;
pub const MIX_DEFOGAIN: u32 = 0x08; /* 0x8->6dB  (6dB = x4) 16 to 18 bit conversion? */

/* MIXER */
pub const VORTEX_MIXER_SR: u32 = 0x21f00;
pub const VORTEX_MIXER_CLIP: u32 = 0x21f80;
pub const VORTEX_MIXER_CHNBASE: u32 = 0x21e40;
pub const VORTEX_MIXER_RTBASE: u32 = 0x21e00;
pub const MIXER_RTBASE_SIZE: u32 = 0x38;
pub const VORTEX_MIX_ENIN: u32 = 0x21a00; /* Input enable bits. 4 bits wide. */
pub const VORTEX_MIX_SMP: u32 = 0x21c00; /* wave data buffers. AU8820: 0x9c00 */

/* MIX */
pub const VORTEX_MIX_INVOL_B: u32 = 0x20000; /* Input volume current */
pub const VORTEX_MIX_VOL_B: u32 = 0x20800; /* Output Volume current */
pub const VORTEX_MIX_INVOL_A: u32 = 0x21000; /* Input Volume target */
pub const VORTEX_MIX_VOL_A: u32 = 0x21800; /* Output Volume target */

pub const VOL_MIN: u32 = 0x80; /* Input volume when muted. */
pub const VOL_MAX: u32 = 0x7f; /* FIXME: Not confirmed! Just guessed. */

/* SRC */
pub const VORTEX_SRC_CHNBASE: u32 = 0x26c40;
pub const VORTEX_SRC_RTBASE: u32 = 0x26c00;
pub const VORTEX_SRCBLOCK_SR: u32 = 0x26cc0;
pub const VORTEX_SRC_SOURCE: u32 = 0x26cc4;
pub const VORTEX_SRC_SOURCESIZE: u32 = 0x26cc8;
/* Params
    0x26e00 : 1 U0
    0x26e40 : 2 CR
    0x26e80 : 3 U3
    0x26ec0 : 4 DRIFT1
    0x26f00 : 5 U1
    0x26f40 : 6 DRIFT2
    0x26f80 : 7 U2 : Target rate, direction
*/

pub const VORTEX_SRC_CONVRATIO: u32 = 0x26e40;
pub const VORTEX_SRC_DRIFT0: u32 = 0x26e80;
pub const VORTEX_SRC_DRIFT1: u32 = 0x26ec0;
pub const VORTEX_SRC_DRIFT2: u32 = 0x26f40;
pub const VORTEX_SRC_U0: u32 = 0x26e00;
pub const U0_SLOWLOCK: u32 = 0x200;
pub const VORTEX_SRC_U1: u32 = 0x26f00;
pub const VORTEX_SRC_U2: u32 = 0x26f80;
pub const VORTEX_SRC_DATA: u32 = 0x26800; /* 0xc800 */
pub const VORTEX_SRC_DATA0: u32 = 0x26000;

/* FIFO */
pub const VORTEX_FIFO_ADBCTRL: u32 = 0x16100; /* Control bits. */
pub const VORTEX_FIFO_WTCTRL: u32 = 0x16000;
pub const FIFO_RDONLY: u32 = 0x00000001;
pub const FIFO_CTRL: u32 = 0x00000002; /* Allow ctrl. ? */
pub const FIFO_VALID: u32 = 0x00000010;
pub const FIFO_EMPTY: u32 = 0x00000020;
pub const FIFO_U0: u32 = 0x00002000; /* Unknown. */
pub const FIFO_U1: u32 = 0x00040000;
pub const FIFO_SIZE_BITS: u32 = 6;
pub const FIFO_SIZE: u32 = 1 << FIFO_SIZE_BITS; // 0x40
pub const FIFO_MASK: u32 = FIFO_SIZE - 1; //0x3f    /* at shift left 0xc */
pub const FIFO_BITS: u32 = 0x1c400000;
pub const VORTEX_FIFO_ADBDATA: u32 = 0x14000;
pub const VORTEX_FIFO_WTDATA: u32 = 0x10000;

pub const VORTEX_FIFO_GIRT: u32 = 0x17000; /* wt0, wt1, adb */
pub const GIRT_COUNT: u32 = 3;

/* CODEC */

pub const VORTEX_CODEC_CHN: u32 = 0x29080; /* The name "CHN" is wrong. */

pub const VORTEX_CODEC_CTRL: u32 = 0x29184;
pub const VORTEX_CODEC_IO: u32 = 0x29188;

pub const VORTEX_CODEC_SPORTCTRL: u32 = 0x2918c;

pub const VORTEX_CODEC_EN: u32 = 0x29190;
pub const EN_AUDIO0: u32 = 0x00000300;
pub const EN_MODEM: u32 = 0x00000c00;
pub const EN_AUDIO1: u32 = 0x00003000;
pub const EN_SPORT: u32 = 0x00030000;
pub const EN_SPDIF: u32 = 0x000c0000;
pub const EN_CODEC: u32 = EN_AUDIO1 | EN_AUDIO0;

pub const VORTEX_SPDIF_SMPRATE: u32 = 0x29194;

pub const VORTEX_SPDIF_FLAGS: u32 = 0x2205c;
pub const VORTEX_SPDIF_CFG0: u32 = 0x291D0; /* status data */
pub const VORTEX_SPDIF_CFG1: u32 = 0x291D4;

pub const VORTEX_SMP_TIME: u32 = 0x29198; /* Sample counter/timer */
pub const VORTEX_SMP_TIMER: u32 = 0x2919c;
pub const VORTEX_CODEC2_CTRL: u32 = 0x291a0;

pub const VORTEX_MODEM_CTRL: u32 = 0x291ac;

/* IRQ */
pub const VORTEX_IRQ_SOURCE: u32 = 0x2a000; /* Interrupt source flags. */
pub const VORTEX_IRQ_CTRL: u32 = 0x2a004; /* Interrupt source mask. */

// Original disabled C macro:
// #define VORTEX_IRQ_U0 0x2a008 /* ?? */
pub const VORTEX_STAT: u32 = 0x2a008; /* Some sort of status */
pub const STAT_IRQ: u32 = 0x00000001; /* This bitis set if the IRQ is valid. */

pub const VORTEX_CTRL: u32 = 0x2a00c;
pub const CTRL_MIDI_EN: u32 = 0x00000001;
pub const CTRL_MIDI_PORT: u32 = 0x00000060;
pub const CTRL_GAME_EN: u32 = 0x00000008;
pub const CTRL_GAME_PORT: u32 = 0x00000e00;
pub const CTRL_IRQ_ENABLE: u32 = 0x00004000;
pub const CTRL_SPDIF: u32 = 0x00000000; /* unknown. Please find this value */
pub const CTRL_SPORT: u32 = 0x00200000;
pub const CTRL_RST: u32 = 0x00800000;
pub const CTRL_UNKNOWN: u32 = 0x01000000;

/* write: Timer period config / read: TIMER IRQ ack. */
pub const VORTEX_IRQ_STAT: u32 = 0x2919c;

/* MIDI */ /* GAME. */
pub const VORTEX_MIDI_DATA: u32 = 0x28800;
pub const VORTEX_MIDI_CMD: u32 = 0x28804; /* Write command / Read status */

pub const VORTEX_GAME_LEGACY: u32 = 0x28808;
pub const VORTEX_CTRL2: u32 = 0x2880c;
pub const CTRL2_GAME_ADCMODE: u32 = 0x40;
pub const VORTEX_GAME_AXIS: u32 = 0x28810; /* Axis base register. 4 axis's */
pub const AXIS_SIZE: u32 = 4;
pub const AXIS_RANGE: u32 = 0x1fff;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
