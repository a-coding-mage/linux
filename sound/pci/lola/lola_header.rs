/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Support for Digigram Lola PCI-e boards
 *
 *  Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
 */

pub const DRVNAME: &[u8] = b"snd-lola\0";
pub const SFX: &[u8] = b"snd-lola: \0";

/*
 * Lola HD Audio Registers BAR0
 */
pub const LOLA_BAR0_GCAP: u32 = 0x00;
pub const LOLA_BAR0_VMIN: u32 = 0x02;
pub const LOLA_BAR0_VMAJ: u32 = 0x03;
pub const LOLA_BAR0_OUTPAY: u32 = 0x04;
pub const LOLA_BAR0_INPAY: u32 = 0x06;
pub const LOLA_BAR0_GCTL: u32 = 0x08;
pub const LOLA_BAR0_WAKEEN: u32 = 0x0c;
pub const LOLA_BAR0_STATESTS: u32 = 0x0e;
pub const LOLA_BAR0_GSTS: u32 = 0x10;
pub const LOLA_BAR0_OUTSTRMPAY: u32 = 0x18;
pub const LOLA_BAR0_INSTRMPAY: u32 = 0x1a;
pub const LOLA_BAR0_INTCTL: u32 = 0x20;
pub const LOLA_BAR0_INTSTS: u32 = 0x24;
pub const LOLA_BAR0_WALCLK: u32 = 0x30;
pub const LOLA_BAR0_SSYNC: u32 = 0x38;

pub const LOLA_BAR0_CORBLBASE: u32 = 0x40;
pub const LOLA_BAR0_CORBUBASE: u32 = 0x44;
pub const LOLA_BAR0_CORBWP: u32 = 0x48; /* no ULONG access */
pub const LOLA_BAR0_CORBRP: u32 = 0x4a; /* no ULONG access */
pub const LOLA_BAR0_CORBCTL: u32 = 0x4c; /* no ULONG access */
pub const LOLA_BAR0_CORBSTS: u32 = 0x4d; /* UCHAR access only */
pub const LOLA_BAR0_CORBSIZE: u32 = 0x4e; /* no ULONG access */

pub const LOLA_BAR0_RIRBLBASE: u32 = 0x50;
pub const LOLA_BAR0_RIRBUBASE: u32 = 0x54;
pub const LOLA_BAR0_RIRBWP: u32 = 0x58;
pub const LOLA_BAR0_RINTCNT: u32 = 0x5a; /* no ULONG access */
pub const LOLA_BAR0_RIRBCTL: u32 = 0x5c;
pub const LOLA_BAR0_RIRBSTS: u32 = 0x5d; /* UCHAR access only */
pub const LOLA_BAR0_RIRBSIZE: u32 = 0x5e; /* no ULONG access */

pub const LOLA_BAR0_ICW: u32 = 0x60;
pub const LOLA_BAR0_IRR: u32 = 0x64;
pub const LOLA_BAR0_ICS: u32 = 0x68;
pub const LOLA_BAR0_DPLBASE: u32 = 0x70;
pub const LOLA_BAR0_DPUBASE: u32 = 0x74;

/* stream register offsets from stream base 0x80 */
pub const LOLA_BAR0_SD0_OFFSET: u32 = 0x80;
pub const LOLA_REG0_SD_CTL: u32 = 0x00;
pub const LOLA_REG0_SD_STS: u32 = 0x03;
pub const LOLA_REG0_SD_LPIB: u32 = 0x04;
pub const LOLA_REG0_SD_CBL: u32 = 0x08;
pub const LOLA_REG0_SD_LVI: u32 = 0x0c;
pub const LOLA_REG0_SD_FIFOW: u32 = 0x0e;
pub const LOLA_REG0_SD_FIFOSIZE: u32 = 0x10;
pub const LOLA_REG0_SD_FORMAT: u32 = 0x12;
pub const LOLA_REG0_SD_BDLPL: u32 = 0x18;
pub const LOLA_REG0_SD_BDLPU: u32 = 0x1c;

/*
 * Lola Digigram Registers BAR1
 */
pub const LOLA_BAR1_FPGAVER: u32 = 0x00;
pub const LOLA_BAR1_DEVER: u32 = 0x04;
pub const LOLA_BAR1_UCBMV: u32 = 0x08;
pub const LOLA_BAR1_JTAG: u32 = 0x0c;
pub const LOLA_BAR1_UARTRX: u32 = 0x10;
pub const LOLA_BAR1_UARTTX: u32 = 0x14;
pub const LOLA_BAR1_UARTCR: u32 = 0x18;
pub const LOLA_BAR1_NVRAMVER: u32 = 0x1c;
pub const LOLA_BAR1_CTRLSPI: u32 = 0x20;
pub const LOLA_BAR1_DSPI: u32 = 0x24;
pub const LOLA_BAR1_AISPI: u32 = 0x28;
pub const LOLA_BAR1_GRAN: u32 = 0x2c;

pub const LOLA_BAR1_DINTCTL: u32 = 0x80;
pub const LOLA_BAR1_DIINTCTL: u32 = 0x84;
pub const LOLA_BAR1_DOINTCTL: u32 = 0x88;
pub const LOLA_BAR1_LRC: u32 = 0x90;
pub const LOLA_BAR1_DINTSTS: u32 = 0x94;
pub const LOLA_BAR1_DIINTSTS: u32 = 0x98;
pub const LOLA_BAR1_DOINTSTS: u32 = 0x9c;

pub const LOLA_BAR1_DSD0_OFFSET: u32 = 0xa0;
pub const LOLA_BAR1_DSD_SIZE: u32 = 0x18;

pub const LOLA_BAR1_DSDN_STS: u32 = 0x00;
pub const LOLA_BAR1_DSDN_LPIB: u32 = 0x04;
pub const LOLA_BAR1_DSDN_CTL: u32 = 0x08;
pub const LOLA_BAR1_DSDN_LVI: u32 = 0x0c;
pub const LOLA_BAR1_DSDN_BDPL: u32 = 0x10;
pub const LOLA_BAR1_DSDN_BDPU: u32 = 0x14;

pub const LOLA_BAR1_SSYNC: u32 = 0x03e8;

pub const LOLA_BAR1_BOARD_CTRL: u32 = 0x0f00;
pub const LOLA_BAR1_BOARD_MODE: u32 = 0x0f02;

pub const LOLA_BAR1_SOURCE_GAIN_ENABLE: u32 = 0x1000;
pub const LOLA_BAR1_DEST00_MIX_GAIN_ENABLE: u32 = 0x1004;
pub const LOLA_BAR1_DEST31_MIX_GAIN_ENABLE: u32 = 0x1080;
pub const LOLA_BAR1_SOURCE00_01_GAIN: u32 = 0x1084;
pub const LOLA_BAR1_SOURCE30_31_GAIN: u32 = 0x10c0;
pub const fn LOLA_BAR1_SOURCE_GAIN(src: u32) -> u32 {
    LOLA_BAR1_SOURCE00_01_GAIN + src * 2
}
pub const LOLA_BAR1_DEST00_MIX00_01_GAIN: u32 = 0x10c4;
pub const LOLA_BAR1_DEST00_MIX30_31_GAIN: u32 = 0x1100;
pub const LOLA_BAR1_DEST01_MIX00_01_GAIN: u32 = 0x1104;
pub const LOLA_BAR1_DEST01_MIX30_31_GAIN: u32 = 0x1140;
pub const LOLA_BAR1_DEST31_MIX00_01_GAIN: u32 = 0x1884;
pub const LOLA_BAR1_DEST31_MIX30_31_GAIN: u32 = 0x18c0;
pub const fn LOLA_BAR1_MIX_GAIN(dest: u32, mix: u32) -> u32 {
    LOLA_BAR1_DEST00_MIX00_01_GAIN + dest * 0x40 + mix * 2
}
pub const LOLA_BAR1_ANALOG_CLIP_IN: u32 = 0x18c4;
pub const LOLA_BAR1_PEAKMETERS_SOURCE00_01: u32 = 0x18c8;
pub const LOLA_BAR1_PEAKMETERS_SOURCE30_31: u32 = 0x1904;
pub const fn LOLA_BAR1_PEAKMETERS_SOURCE(src: u32) -> u32 {
    LOLA_BAR1_PEAKMETERS_SOURCE00_01 + src * 2
}
pub const LOLA_BAR1_PEAKMETERS_DEST00_01: u32 = 0x1908;
pub const LOLA_BAR1_PEAKMETERS_DEST30_31: u32 = 0x1944;
pub const fn LOLA_BAR1_PEAKMETERS_DEST(dest: u32) -> u32 {
    LOLA_BAR1_PEAKMETERS_DEST00_01 + dest * 2
}
pub const LOLA_BAR1_PEAKMETERS_AGC00_01: u32 = 0x1948;
pub const LOLA_BAR1_PEAKMETERS_AGC14_15: u32 = 0x1964;
pub const fn LOLA_BAR1_PEAKMETERS_AGC(x: u32) -> u32 {
    LOLA_BAR1_PEAKMETERS_AGC00_01 + x * 2
}

/* GCTL reset bit */
pub const LOLA_GCTL_RESET: u32 = 1 << 0;
/* GCTL unsolicited response enable bit */
pub const LOLA_GCTL_UREN: u32 = 1 << 8;

/* CORB/RIRB control, read/write pointer */
pub const LOLA_RBCTL_DMA_EN: u32 = 0x02; /* enable DMA */
pub const LOLA_RBCTL_IRQ_EN: u32 = 0x01; /* enable IRQ */
pub const LOLA_RBRWP_CLR: u32 = 0x8000; /* read/write pointer clear */

pub const LOLA_RIRB_EX_UNSOL_EV: u32 = 0x40000000;
pub const LOLA_RIRB_EX_ERROR: u32 = 0x80000000;

/* CORB int mask: CMEI[0] */
pub const LOLA_CORB_INT_CMEI: u32 = 0x01;
pub const LOLA_CORB_INT_MASK: u32 = LOLA_CORB_INT_CMEI;

/* RIRB int mask: overrun[2], response[0] */
pub const LOLA_RIRB_INT_RESPONSE: u32 = 0x01;
pub const LOLA_RIRB_INT_OVERRUN: u32 = 0x04;
pub const LOLA_RIRB_INT_MASK: u32 = LOLA_RIRB_INT_RESPONSE | LOLA_RIRB_INT_OVERRUN;

/* DINTCTL and DINTSTS */
pub const LOLA_DINT_GLOBAL: u32 = 0x80000000; /* global interrupt enable bit */
pub const LOLA_DINT_CTRL: u32 = 0x40000000; /* controller interrupt enable bit */
pub const LOLA_DINT_FIFOERR: u32 = 0x20000000; /* global fifo error enable bit */
pub const LOLA_DINT_MUERR: u32 = 0x10000000; /* global microcontroller underrun error */

/* DSDnCTL bits */
pub const LOLA_DSD_CTL_SRST: u32 = 0x01; /* stream reset bit */
pub const LOLA_DSD_CTL_SRUN: u32 = 0x02; /* stream DMA start bit */
pub const LOLA_DSD_CTL_IOCE: u32 = 0x04; /* interrupt on completion enable */
pub const LOLA_DSD_CTL_DEIE: u32 = 0x10; /* descriptor error interrupt enable */
pub const LOLA_DSD_CTL_VLRCV: u32 = 0x20; /* valid LRCountValue information in bits 8..31 */
pub const LOLA_LRC_MASK: u32 = 0xffffff00;

/* DSDnSTS */
pub const LOLA_DSD_STS_BCIS: u32 = 0x04; /* buffer completion interrupt status */
pub const LOLA_DSD_STS_DESE: u32 = 0x10; /* descriptor error interrupt */
pub const LOLA_DSD_STS_FIFORDY: u32 = 0x20; /* fifo ready */

pub const LOLA_CORB_ENTRIES: usize = 256;

pub const MAX_STREAM_IN_COUNT: usize = 16;
pub const MAX_STREAM_OUT_COUNT: usize = 16;
pub const MAX_STREAM_COUNT: usize = 16;
pub const MAX_PINS: usize = MAX_STREAM_COUNT;
pub const MAX_STREAM_BUFFER_COUNT: usize = 16;
pub const MAX_AUDIO_INOUT_COUNT: usize = 16;

pub const LOLA_CLOCK_TYPE_INTERNAL: u32 = 0;
pub const LOLA_CLOCK_TYPE_AES: u32 = 1;
pub const LOLA_CLOCK_TYPE_AES_SYNC: u32 = 2;
pub const LOLA_CLOCK_TYPE_WORDCLOCK: u32 = 3;
pub const LOLA_CLOCK_TYPE_ETHERSOUND: u32 = 4;
pub const LOLA_CLOCK_TYPE_VIDEO: u32 = 5;

pub const LOLA_CLOCK_FORMAT_NONE: u32 = 0;
pub const LOLA_CLOCK_FORMAT_NTSC: u32 = 1;
pub const LOLA_CLOCK_FORMAT_PAL: u32 = 2;

pub const MAX_SAMPLE_CLOCK_COUNT: usize = 48;

/* parameters used with mixer widget's mixer capabilities */
pub const LOLA_PEAK_METER_CAN_AGC_MASK: u32 = 1;
pub const LOLA_PEAK_METER_CAN_ANALOG_CLIP_MASK: u32 = 2;

pub enum SndCard {}
pub enum PciDev {}
pub enum SndPcmSubstream {}
pub enum SndDmaBuffer {}
pub enum SpinlockT {}
pub enum Mutex {}

pub type DmaAddrT = usize;

unsafe extern "C" {
    pub fn readl(addr: *const core::ffi::c_void) -> u32;
    pub fn readw(addr: *const core::ffi::c_void) -> u16;
    pub fn readb(addr: *const core::ffi::c_void) -> u8;
    pub fn writel(val: u32, addr: *mut core::ffi::c_void);
    pub fn writew(val: u16, addr: *mut core::ffi::c_void);
    pub fn writeb(val: u8, addr: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct lola_bar {
    pub addr: core::ffi::c_ulong,
    pub remap_addr: *mut core::ffi::c_void,
}

/* CORB/RIRB */
#[repr(C)]
pub struct lola_rb {
    pub buf: *mut u32,      /* CORB/RIRB buffer, 8 byte per each entry */
    pub addr: DmaAddrT,     /* physical address of CORB/RIRB buffer */
    pub rp: core::ffi::c_ushort,
    pub wp: core::ffi::c_ushort, /* read/write pointers */
    pub cmds: core::ffi::c_int, /* number of pending requests */
}

/* Pin widget setup */
#[repr(C)]
pub struct lola_pin {
    pub nid: core::ffi::c_uint,
    pub is_analog: bool,
    pub amp_mute: core::ffi::c_uint,
    pub amp_step_size: core::ffi::c_uint,
    pub amp_num_steps: core::ffi::c_uint,
    pub amp_offset: core::ffi::c_uint,
    pub max_level: core::ffi::c_uint,
    pub config_default_reg: core::ffi::c_uint,
    pub fixed_gain_list_len: core::ffi::c_uint,
    pub cur_gain_step: core::ffi::c_uint,
}

#[repr(C)]
pub struct lola_pin_array {
    pub num_pins: core::ffi::c_uint,
    pub num_analog_pins: core::ffi::c_uint,
    pub pins: [lola_pin; MAX_PINS],
}

/* Clock widget setup */
#[repr(C)]
pub struct lola_sample_clock {
    pub type_: core::ffi::c_uint,
    pub format: core::ffi::c_uint,
    pub freq: core::ffi::c_uint,
}

#[repr(C)]
pub struct lola_clock_widget {
    pub nid: core::ffi::c_uint,
    pub items: core::ffi::c_uint,
    pub cur_index: core::ffi::c_uint,
    pub cur_freq: core::ffi::c_uint,
    pub cur_valid: bool,
    pub sample_clock: [lola_sample_clock; MAX_SAMPLE_CLOCK_COUNT],
    pub idx_lookup: [core::ffi::c_uint; MAX_SAMPLE_CLOCK_COUNT],
}

pub const LOLA_MIXER_DIM: usize = 32;

#[repr(C)]
pub struct lola_mixer_array {
    pub src_gain_enable: u32,
    pub dest_mix_gain_enable: [u32; LOLA_MIXER_DIM],
    pub src_gain: [u16; LOLA_MIXER_DIM],
    pub dest_mix_gain: [[u16; LOLA_MIXER_DIM]; LOLA_MIXER_DIM],
}

/* Mixer widget setup */
#[repr(C)]
pub struct lola_mixer_widget {
    pub nid: core::ffi::c_uint,
    pub caps: core::ffi::c_uint,
    pub array: *mut lola_mixer_array,
    pub array_saved: *mut lola_mixer_array,
    pub src_stream_outs: core::ffi::c_uint,
    pub src_phys_ins: core::ffi::c_uint,
    pub dest_stream_ins: core::ffi::c_uint,
    pub dest_phys_outs: core::ffi::c_uint,
    pub src_stream_out_ofs: core::ffi::c_uint,
    pub dest_phys_out_ofs: core::ffi::c_uint,
    pub src_mask: core::ffi::c_uint,
    pub dest_mask: core::ffi::c_uint,
}

/* Audio stream */
#[repr(C)]
pub struct lola_stream {
    pub nid: core::ffi::c_uint,   /* audio widget NID */
    pub index: core::ffi::c_uint, /* array index */
    pub dsd: core::ffi::c_uint,   /* DSD index */
    pub can_float: bool,
    pub substream: *mut SndPcmSubstream, /* assigned PCM substream */
    pub master: *mut lola_stream,        /* master stream (for multi-channel) */

    /* buffer setup */
    pub bufsize: core::ffi::c_uint,
    pub period_bytes: core::ffi::c_uint,
    pub frags: core::ffi::c_uint,

    /* format + channel setup */
    pub format_verb: core::ffi::c_uint,

    /* flags: C source stores these as unsigned int bitfields */
    pub opened: core::ffi::c_uint,
    pub prepared: core::ffi::c_uint,
    pub paused: core::ffi::c_uint,
    pub running: core::ffi::c_uint,
}

pub const PLAY: u32 = SNDRV_PCM_STREAM_PLAYBACK;
pub const CAPT: u32 = SNDRV_PCM_STREAM_CAPTURE;

unsafe extern "C" {
    pub static SNDRV_PCM_STREAM_PLAYBACK: u32;
    pub static SNDRV_PCM_STREAM_CAPTURE: u32;
}

#[repr(C)]
pub struct lola_pcm {
    pub num_streams: core::ffi::c_uint,
    pub bdl: *mut SndDmaBuffer, /* BDL buffer */
    pub streams: [lola_stream; MAX_STREAM_COUNT],
}

/* card instance */
#[repr(C)]
pub struct lola {
    pub card: *mut SndCard,
    pub pci: *mut PciDev,

    /* pci resources */
    pub bar: [lola_bar; 2],
    pub irq: core::ffi::c_int,

    /* locks */
    pub reg_lock: SpinlockT,
    pub open_mutex: Mutex,

    /* CORB/RIRB */
    pub corb: lola_rb,
    pub rirb: lola_rb,
    pub res: core::ffi::c_uint,
    pub res_ex: core::ffi::c_uint, /* last read values */
    /* last command (for debugging) */
    pub last_cmd_nid: core::ffi::c_uint,
    pub last_verb: core::ffi::c_uint,
    pub last_data: core::ffi::c_uint,
    pub last_extdata: core::ffi::c_uint,

    /* CORB/RIRB buffers */
    pub rb: *mut SndDmaBuffer,

    /* unsolicited events */
    pub last_unsol_res: core::ffi::c_uint,

    /* streams */
    pub pcm: [lola_pcm; 2],

    /* input src */
    pub input_src_caps_mask: core::ffi::c_uint,
    pub input_src_mask: core::ffi::c_uint,

    /* pins */
    pub pin: [lola_pin_array; 2],

    /* clock */
    pub clock: lola_clock_widget,
    pub ref_count_rate: core::ffi::c_int,
    pub sample_rate: core::ffi::c_uint,

    /* mixer */
    pub mixer: lola_mixer_widget,

    /* hw info */
    pub version: core::ffi::c_uint,
    pub lola_caps: core::ffi::c_uint,

    /* parameters */
    pub granularity: core::ffi::c_uint,
    pub sample_rate_min: core::ffi::c_uint,
    pub sample_rate_max: core::ffi::c_uint,

    /* flags: C source stores these as unsigned int bitfields */
    pub initialized: core::ffi::c_uint,
    pub cold_reset: core::ffi::c_uint,
    pub polling_mode: core::ffi::c_uint,

    /* for debugging */
    pub debug_res: core::ffi::c_uint,
    pub debug_res_ex: core::ffi::c_uint,
}

pub const BAR0: usize = 0;
pub const BAR1: usize = 1;

/* Helper macros */
pub unsafe fn lola_readl(chip: *mut lola, idx: usize, offset: u32) -> u32 {
    unsafe { readl((*chip).bar[idx].remap_addr.add(offset as usize) as *const core::ffi::c_void) }
}

pub unsafe fn lola_readw(chip: *mut lola, idx: usize, offset: u32) -> u16 {
    unsafe { readw((*chip).bar[idx].remap_addr.add(offset as usize) as *const core::ffi::c_void) }
}

pub unsafe fn lola_readb(chip: *mut lola, idx: usize, offset: u32) -> u8 {
    unsafe { readb((*chip).bar[idx].remap_addr.add(offset as usize) as *const core::ffi::c_void) }
}

pub unsafe fn lola_writel(chip: *mut lola, idx: usize, offset: u32, val: u32) {
    unsafe { writel(val, (*chip).bar[idx].remap_addr.add(offset as usize)) }
}

pub unsafe fn lola_writew(chip: *mut lola, idx: usize, offset: u32, val: u16) {
    unsafe { writew(val, (*chip).bar[idx].remap_addr.add(offset as usize)) }
}

pub unsafe fn lola_writeb(chip: *mut lola, idx: usize, offset: u32, val: u8) {
    unsafe { writeb(val, (*chip).bar[idx].remap_addr.add(offset as usize)) }
}

pub unsafe fn lola_dsd_read(chip: *mut lola, dsd: u32, offset: u32) -> u32 {
    unsafe {
        readl(
            (*chip).bar[BAR1]
                .remap_addr
                .add((LOLA_BAR1_DSD0_OFFSET + LOLA_BAR1_DSD_SIZE * dsd + offset) as usize)
                as *const core::ffi::c_void,
        )
    }
}

pub unsafe fn lola_dsd_write(chip: *mut lola, dsd: u32, offset: u32, val: u32) {
    unsafe {
        writel(
            val,
            (*chip).bar[BAR1]
                .remap_addr
                .add((LOLA_BAR1_DSD0_OFFSET + LOLA_BAR1_DSD_SIZE * dsd + offset) as usize),
        )
    }
}

/* GET verbs HDAudio */
pub const LOLA_VERB_GET_STREAM_FORMAT: u32 = 0xa00;
pub const LOLA_VERB_GET_AMP_GAIN_MUTE: u32 = 0xb00;
pub const LOLA_VERB_PARAMETERS: u32 = 0xf00;
pub const LOLA_VERB_GET_POWER_STATE: u32 = 0xf05;
pub const LOLA_VERB_GET_CONV: u32 = 0xf06;
pub const LOLA_VERB_GET_UNSOLICITED_RESPONSE: u32 = 0xf08;
pub const LOLA_VERB_GET_DIGI_CONVERT_1: u32 = 0xf0d;
pub const LOLA_VERB_GET_CONFIG_DEFAULT: u32 = 0xf1c;
pub const LOLA_VERB_GET_SUBSYSTEM_ID: u32 = 0xf20;
/* GET verbs Digigram */
pub const LOLA_VERB_GET_FIXED_GAIN: u32 = 0xfc0;
pub const LOLA_VERB_GET_GAIN_SELECT: u32 = 0xfc1;
pub const LOLA_VERB_GET_MAX_LEVEL: u32 = 0xfc2;
pub const LOLA_VERB_GET_CLOCK_LIST: u32 = 0xfc3;
pub const LOLA_VERB_GET_CLOCK_SELECT: u32 = 0xfc4;
pub const LOLA_VERB_GET_CLOCK_STATUS: u32 = 0xfc5;

/* SET verbs HDAudio */
pub const LOLA_VERB_SET_STREAM_FORMAT: u32 = 0x200;
pub const LOLA_VERB_SET_AMP_GAIN_MUTE: u32 = 0x300;
pub const LOLA_VERB_SET_POWER_STATE: u32 = 0x705;
pub const LOLA_VERB_SET_CHANNEL_STREAMID: u32 = 0x706;
pub const LOLA_VERB_SET_UNSOLICITED_ENABLE: u32 = 0x708;
pub const LOLA_VERB_SET_DIGI_CONVERT_1: u32 = 0x70d;
/* SET verbs Digigram */
pub const LOLA_VERB_SET_GAIN_SELECT: u32 = 0xf81;
pub const LOLA_VERB_SET_CLOCK_SELECT: u32 = 0xf84;
pub const LOLA_VERB_SET_GRANULARITY_STEPS: u32 = 0xf86;
pub const LOLA_VERB_SET_SOURCE_GAIN: u32 = 0xf87;
pub const LOLA_VERB_SET_MIX_GAIN: u32 = 0xf88;
pub const LOLA_VERB_SET_DESTINATION_GAIN: u32 = 0xf89;
pub const LOLA_VERB_SET_SRC: u32 = 0xf8a;

/* Parameter IDs used with LOLA_VERB_PARAMETERS */
pub const LOLA_PAR_VENDOR_ID: u32 = 0x00;
pub const LOLA_PAR_FUNCTION_TYPE: u32 = 0x05;
pub const LOLA_PAR_AUDIO_WIDGET_CAP: u32 = 0x09;
pub const LOLA_PAR_PCM: u32 = 0x0a;
pub const LOLA_PAR_STREAM_FORMATS: u32 = 0x0b;
pub const LOLA_PAR_PIN_CAP: u32 = 0x0c;
pub const LOLA_PAR_AMP_IN_CAP: u32 = 0x0d;
pub const LOLA_PAR_CONNLIST_LEN: u32 = 0x0e;
pub const LOLA_PAR_POWER_STATE: u32 = 0x0f;
pub const LOLA_PAR_GPIO_CAP: u32 = 0x11;
pub const LOLA_PAR_AMP_OUT_CAP: u32 = 0x12;
pub const LOLA_PAR_SPECIFIC_CAPS: u32 = 0x80;
pub const LOLA_PAR_FIXED_GAIN_LIST: u32 = 0x81;

/* extract results of LOLA_PAR_SPECIFIC_CAPS */
pub const fn LOLA_AFG_MIXER_WIDGET_PRESENT(res: u32) -> bool {
    (res & (1 << 21)) != 0
}
pub const fn LOLA_AFG_CLOCK_WIDGET_PRESENT(res: u32) -> bool {
    (res & (1 << 20)) != 0
}
pub const fn LOLA_AFG_INPUT_PIN_COUNT(res: u32) -> u32 {
    (res >> 10) & 0x2ff
}
pub const fn LOLA_AFG_OUTPUT_PIN_COUNT(res: u32) -> u32 {
    res & 0x2ff
}

/* extract results of LOLA_PAR_AMP_IN_CAP / LOLA_PAR_AMP_OUT_CAP */
pub const fn LOLA_AMP_MUTE_CAPABLE(res: u32) -> bool {
    (res & (1 << 31)) != 0
}
pub const fn LOLA_AMP_STEP_SIZE(res: u32) -> u32 {
    (res >> 24) & 0x7f
}
pub const fn LOLA_AMP_NUM_STEPS(res: u32) -> u32 {
    (res >> 12) & 0x3ff
}
pub const fn LOLA_AMP_OFFSET(res: u32) -> u32 {
    res & 0x3ff
}

pub const LOLA_GRANULARITY_MIN: u32 = 8;
pub const LOLA_GRANULARITY_MAX: u32 = 32;
pub const LOLA_GRANULARITY_STEP: u32 = 8;

/* parameters used with unsolicited command/response */
pub const LOLA_UNSOLICITED_TAG_MASK: u32 = 0x3f;
pub const LOLA_UNSOLICITED_TAG: u32 = 0x1a;
pub const LOLA_UNSOLICITED_ENABLE: u32 = 0x80;
pub const LOLA_UNSOL_RESP_TAG_OFFSET: u32 = 26;

/* count values in the Vendor Specific Mixer Widget's Audio Widget Capabilities */
pub const fn LOLA_MIXER_SRC_INPUT_PLAY_SEPARATION(res: u32) -> u32 {
    (res >> 2) & 0x1f
}
pub const fn LOLA_MIXER_DEST_REC_OUTPUT_SEPARATION(res: u32) -> u32 {
    (res >> 7) & 0x1f
}

unsafe extern "C" {
    pub fn lola_codec_write(
        chip: *mut lola,
        nid: core::ffi::c_uint,
        verb: core::ffi::c_uint,
        data: core::ffi::c_uint,
        extdata: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn lola_codec_read(
        chip: *mut lola,
        nid: core::ffi::c_uint,
        verb: core::ffi::c_uint,
        data: core::ffi::c_uint,
        extdata: core::ffi::c_uint,
        val: *mut core::ffi::c_uint,
        extval: *mut core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn lola_codec_flush(chip: *mut lola) -> core::ffi::c_int;
}

pub unsafe fn lola_read_param(
    chip: *mut lola,
    nid: core::ffi::c_uint,
    param: core::ffi::c_uint,
    val: *mut core::ffi::c_uint,
) -> core::ffi::c_int {
    unsafe {
        lola_codec_read(
            chip,
            nid,
            LOLA_VERB_PARAMETERS,
            param,
            0,
            val,
            core::ptr::null_mut(),
        )
    }
}

unsafe extern "C" {
    /* PCM */
    pub fn lola_create_pcm(chip: *mut lola) -> core::ffi::c_int;
    pub fn lola_init_pcm(
        chip: *mut lola,
        dir: core::ffi::c_int,
        nidp: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn lola_pcm_update(chip: *mut lola, pcm: *mut lola_pcm, bits: core::ffi::c_uint);

    /* clock */
    pub fn lola_init_clock_widget(chip: *mut lola, nid: core::ffi::c_int) -> core::ffi::c_int;
    pub fn lola_set_granularity(
        chip: *mut lola,
        val: core::ffi::c_uint,
        force: bool,
    ) -> core::ffi::c_int;
    pub fn lola_enable_clock_events(chip: *mut lola) -> core::ffi::c_int;
    pub fn lola_set_clock_index(chip: *mut lola, idx: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn lola_set_clock(chip: *mut lola, idx: core::ffi::c_int) -> core::ffi::c_int;
    pub fn lola_set_sample_rate(chip: *mut lola, rate: core::ffi::c_int) -> core::ffi::c_int;
    pub fn lola_update_ext_clock_freq(chip: *mut lola, val: core::ffi::c_uint) -> bool;
    pub fn lola_sample_rate_convert(coded: core::ffi::c_uint) -> core::ffi::c_uint;

    /* mixer */
    pub fn lola_init_pins(
        chip: *mut lola,
        dir: core::ffi::c_int,
        nidp: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn lola_init_mixer_widget(chip: *mut lola, nid: core::ffi::c_int) -> core::ffi::c_int;
    pub fn lola_free_mixer(chip: *mut lola);
    pub fn lola_create_mixer(chip: *mut lola) -> core::ffi::c_int;
    pub fn lola_setup_all_analog_gains(
        chip: *mut lola,
        dir: core::ffi::c_int,
        mute: bool,
    ) -> core::ffi::c_int;
    pub fn lola_set_src_config(
        chip: *mut lola,
        src_mask: core::ffi::c_uint,
        update: bool,
    ) -> core::ffi::c_int;
}

/* proc */
/* CONFIG_SND_DEBUG condition preserved from C header. */
#[cfg(CONFIG_SND_DEBUG)]
unsafe extern "C" {
    pub fn lola_proc_debug_new(chip: *mut lola);
}

#[cfg(not(CONFIG_SND_DEBUG))]
pub unsafe fn lola_proc_debug_new(_chip: *mut lola) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
