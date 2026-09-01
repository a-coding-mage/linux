// SPDX-License-Identifier: GPL-2.0
/*
    Aureal Vortex Soundcard driver.

    IO addr collected from asp4core.vxd:
    function    address
    0005D5A0    13004
    00080674    14004
    00080AFF    12818

 */

pub const CHIP_AU8820: bool = true;

pub const CARD_NAME: &str = "Aureal Vortex";
pub const CARD_NAME_SHORT: &str = "au8820";

/* Number of ADB and WT channels */
pub const NR_ADB: u32 = 0x10;
pub const NR_WT: u32 = 0x20;
pub const NR_SRC: u32 = 0x10;
pub const NR_A3D: u32 = 0x00;
pub const NR_MIXIN: u32 = 0x10;
pub const NR_MIXOUT: u32 = 0x10;

/* ADBDMA */
pub const VORTEX_ADBDMA_STAT: u32 = 0x105c0; /* read only, subbuffer, DMA pos */
pub const POS_MASK: u32 = 0x00000fff;
pub const POS_SHIFT: u32 = 0x0;
pub const ADB_SUBBUF_MASK: u32 = 0x00003000; /* ADB only. */
pub const ADB_SUBBUF_SHIFT: u32 = 0xc; /* ADB only. */
pub const VORTEX_ADBDMA_CTRL: u32 = 0x10580; /* write only, format, flags, DMA pos */
pub const OFFSET_MASK: u32 = 0x00000fff;
pub const OFFSET_SHIFT: u32 = 0x0;
pub const IE_MASK: u32 = 0x00001000; /* interrupt enable. */
pub const IE_SHIFT: u32 = 0xc;
pub const DIR_MASK: u32 = 0x00002000; /* Direction. */
pub const DIR_SHIFT: u32 = 0xd;
pub const FMT_MASK: u32 = 0x0003c000;
pub const FMT_SHIFT: u32 = 0xe;
// The masks and shift also work for the wtdma, if not specified otherwise.
pub const VORTEX_ADBDMA_BUFCFG0: u32 = 0x10400;
pub const VORTEX_ADBDMA_BUFCFG1: u32 = 0x10404;
pub const VORTEX_ADBDMA_BUFBASE: u32 = 0x10200;
pub const VORTEX_ADBDMA_START: u32 = 0x106c0; /* Which subbuffer starts */
pub const VORTEX_ADBDMA_STATUS: u32 = 0x10600; /* stored at AdbDma->this_10 / 2 DWORD in size. */

/* ADB */
pub const VORTEX_ADB_SR: u32 = 0x10a00; /* Samplerates enable/disable */
pub const VORTEX_ADB_RTBASE: u32 = 0x10800;
pub const VORTEX_ADB_RTBASE_COUNT: u32 = 103;
pub const VORTEX_ADB_CHNBASE: u32 = 0x1099c;
pub const VORTEX_ADB_CHNBASE_COUNT: u32 = 22;
pub const ROUTE_MASK: u32 = 0x3fff;
pub const ADB_MASK: u32 = 0x7f;
pub const ADB_SHIFT: u32 = 0x7;
// #define     ADB_MIX_MASK 0xf
/* ADB address */
pub const OFFSET_ADBDMA: u32 = 0x00;
pub const OFFSET_SRCOUT: u32 = 0x10; /* on channel 0x11 */
pub const OFFSET_SRCIN: u32 = 0x10; /* on channel < 0x11 */
pub const OFFSET_MIXOUT: u32 = 0x20; /* source */
pub const OFFSET_MIXIN: u32 = 0x30; /* sink */
pub const OFFSET_CODECIN: u32 = 0x48; /* ADB source */
pub const OFFSET_CODECOUT: u32 = 0x58; /* ADB sink/target */
pub const OFFSET_SPORTOUT: u32 = 0x60; /* sink */
pub const OFFSET_SPORTIN: u32 = 0x50; /* source */
pub const OFFSET_EFXOUT: u32 = 0x50; /* sink */
pub const OFFSET_EFXIN: u32 = 0x40; /* source */
pub const OFFSET_A3DOUT: u32 = 0x00; /* This card has no HRTF :( */
pub const OFFSET_A3DIN: u32 = 0x00;
pub const OFFSET_WTOUT: u32 = 0x58; /*  */

/* ADB route translate helper */
pub const fn ADB_DMA(x: u32) -> u32 {
    x.wrapping_add(OFFSET_ADBDMA)
}

pub const fn ADB_SRCOUT(x: u32) -> u32 {
    x.wrapping_add(OFFSET_SRCOUT)
}

pub const fn ADB_SRCIN(x: u32) -> u32 {
    x.wrapping_add(OFFSET_SRCIN)
}

pub const fn ADB_MIXOUT(x: u32) -> u32 {
    x.wrapping_add(OFFSET_MIXOUT)
}

pub const fn ADB_MIXIN(x: u32) -> u32 {
    x.wrapping_add(OFFSET_MIXIN)
}

pub const fn ADB_CODECIN(x: u32) -> u32 {
    x.wrapping_add(OFFSET_CODECIN)
}

pub const fn ADB_CODECOUT(x: u32) -> u32 {
    x.wrapping_add(OFFSET_CODECOUT)
}

pub const fn ADB_SPORTOUT(x: u32) -> u32 {
    x.wrapping_add(OFFSET_SPORTOUT)
}

pub const fn ADB_SPORTIN(x: u32) -> u32 {
    x.wrapping_add(OFFSET_SPORTIN)
} /*  */

pub const fn ADB_A3DOUT(x: u32) -> u32 {
    x.wrapping_add(OFFSET_A3DOUT)
} /* 8 A3D blocks */

pub const fn ADB_A3DIN(x: u32) -> u32 {
    x.wrapping_add(OFFSET_A3DIN)
}

pub const fn ADB_WTOUT(_x: u32, y: u32) -> u32 {
    y.wrapping_add(OFFSET_WTOUT)
}

/* WTDMA */
pub const VORTEX_WTDMA_CTRL: u32 = 0x10500; /* format, DMA pos */
pub const VORTEX_WTDMA_STAT: u32 = 0x10500; /* DMA subbuf, DMA pos */
pub const WT_SUBBUF_SHIFT: u32 = 0x15;
pub const WT_SUBBUF_MASK: u32 = 0x3 << WT_SUBBUF_SHIFT;
pub const VORTEX_WTDMA_BUFBASE: u32 = 0x10000;
pub const VORTEX_WTDMA_BUFCFG0: u32 = 0x10300;
pub const VORTEX_WTDMA_BUFCFG1: u32 = 0x10304;
pub const VORTEX_WTDMA_START: u32 = 0x10640; /* which subbuffer is first */

pub const VORTEX_WT_BASE: u32 = 0x9000;

/* MIXER */
pub const VORTEX_MIXER_SR: u32 = 0x9f00;
pub const VORTEX_MIXER_CLIP: u32 = 0x9f80;
pub const VORTEX_MIXER_CHNBASE: u32 = 0x9e40;
pub const VORTEX_MIXER_RTBASE: u32 = 0x9e00;
pub const MIXER_RTBASE_SIZE: u32 = 0x26;
pub const VORTEX_MIX_ENIN: u32 = 0x9a00; /* Input enable bits. 4 bits wide. */
pub const VORTEX_MIX_SMP: u32 = 0x9c00;

/* MIX */
pub const VORTEX_MIX_INVOL_A: u32 = 0x9000; /* in? */
pub const VORTEX_MIX_INVOL_B: u32 = 0x8000; /* out? */
pub const VORTEX_MIX_VOL_A: u32 = 0x9800;
pub const VORTEX_MIX_VOL_B: u32 = 0x8800;

pub const VOL_MIN: u32 = 0x80; /* Input volume when muted. */
pub const VOL_MAX: u32 = 0x7f; /* FIXME: Not confirmed! Just guessed. */

// #define MIX_OUTL    0xe
// #define MIX_OUTR    0xf
// #define MIX_INL     0xe
// #define MIX_INR     0xf
pub const MIX_DEFIGAIN: u32 = 0x08; /* 0x8 => 6dB */
pub const MIX_DEFOGAIN: u32 = 0x08;

/* SRC */
pub const VORTEX_SRCBLOCK_SR: u32 = 0xccc0;
pub const VORTEX_SRC_CHNBASE: u32 = 0xcc40;
pub const VORTEX_SRC_RTBASE: u32 = 0xcc00;
pub const VORTEX_SRC_SOURCE: u32 = 0xccc4;
pub const VORTEX_SRC_SOURCESIZE: u32 = 0xccc8;
pub const VORTEX_SRC_U0: u32 = 0xce00;
pub const VORTEX_SRC_DRIFT0: u32 = 0xce80;
pub const VORTEX_SRC_DRIFT1: u32 = 0xcec0;
pub const VORTEX_SRC_U1: u32 = 0xcf00;
pub const VORTEX_SRC_DRIFT2: u32 = 0xcf40;
pub const VORTEX_SRC_U2: u32 = 0xcf80;
pub const VORTEX_SRC_DATA: u32 = 0xc800;
pub const VORTEX_SRC_DATA0: u32 = 0xc000;
pub const VORTEX_SRC_CONVRATIO: u32 = 0xce40;
// #define     SRC_RATIO(x) ((((x<<15)/48000) + 1)/2) /* Playback */
// #define     SRC_RATIO2(x) ((((48000<<15)/x) + 1)/2) /* Recording */

/* FIFO */
pub const VORTEX_FIFO_ADBCTRL: u32 = 0xf800; /* Control bits. */
pub const VORTEX_FIFO_WTCTRL: u32 = 0xf840;
pub const FIFO_RDONLY: u32 = 0x00000001;
pub const FIFO_CTRL: u32 = 0x00000002; /* Allow ctrl. ? */
pub const FIFO_VALID: u32 = 0x00000010;
pub const FIFO_EMPTY: u32 = 0x00000020;
pub const FIFO_U0: u32 = 0x00001000; /* Unknown. */
pub const FIFO_U1: u32 = 0x00010000;
pub const FIFO_SIZE_BITS: u32 = 5;
pub const FIFO_SIZE: u32 = 1 << FIFO_SIZE_BITS; // 0x20
pub const FIFO_MASK: u32 = FIFO_SIZE - 1; // 0x1f    /* at shift left 0xc */
pub const VORTEX_FIFO_ADBDATA: u32 = 0xe000;
pub const VORTEX_FIFO_WTDATA: u32 = 0xe800;

/* CODEC */
pub const VORTEX_CODEC_CTRL: u32 = 0x11984;
pub const VORTEX_CODEC_EN: u32 = 0x11990;
pub const EN_CODEC: u32 = 0x00000300;
pub const EN_SPORT: u32 = 0x00030000;
pub const EN_SPDIF: u32 = 0x000c0000;
pub const VORTEX_CODEC_CHN: u32 = 0x11880;
pub const VORTEX_CODEC_IO: u32 = 0x11988;

pub const VORTEX_SPDIF_FLAGS: u32 = 0x1005c; /* FIXME */
pub const VORTEX_SPDIF_CFG0: u32 = 0x119D0;
pub const VORTEX_SPDIF_CFG1: u32 = 0x119D4;
pub const VORTEX_SPDIF_SMPRATE: u32 = 0x11994;

/* Sample timer */
pub const VORTEX_SMP_TIME: u32 = 0x11998;

/* IRQ */
pub const VORTEX_IRQ_SOURCE: u32 = 0x12800; /* Interrupt source flags. */
pub const VORTEX_IRQ_CTRL: u32 = 0x12804; /* Interrupt source mask. */

pub const VORTEX_STAT: u32 = 0x12808; /* ?? */

pub const VORTEX_CTRL: u32 = 0x1280c;
pub const CTRL_MIDI_EN: u32 = 0x00000001;
pub const CTRL_MIDI_PORT: u32 = 0x00000060;
pub const CTRL_GAME_EN: u32 = 0x00000008;
pub const CTRL_GAME_PORT: u32 = 0x00000e00;
pub const CTRL_IRQ_ENABLE: u32 = 0x4000;

/* write: Timer period config / read: TIMER IRQ ack. */
pub const VORTEX_IRQ_STAT: u32 = 0x1199c;

/* DMA */
pub const VORTEX_DMA_BUFFER: u32 = 0x10200;
pub const VORTEX_ENGINE_CTRL: u32 = 0x1060c;
pub const ENGINE_INIT: core::ffi::c_long = 0x0;

/* MIDI *//* GAME. */
pub const VORTEX_MIDI_DATA: u32 = 0x11000;
pub const VORTEX_MIDI_CMD: u32 = 0x11004; /* Write command / Read status */
pub const VORTEX_GAME_LEGACY: u32 = 0x11008;
pub const VORTEX_CTRL2: u32 = 0x1100c;
pub const CTRL2_GAME_ADCMODE: u32 = 0x40;
pub const VORTEX_GAME_AXIS: u32 = 0x11010;
pub const AXIS_SIZE: u32 = 4;
pub const AXIS_RANGE: u32 = 0x1fff;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
