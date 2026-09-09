/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of sound/sb.h. C includes and configuration conditions are
 * represented by external types/declarations and comments where applicable. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum sb_hw_type {
    SB_HW_AUTO,
    SB_HW_10,
    SB_HW_20,
    SB_HW_201,
    SB_HW_PRO,
    SB_HW_JAZZ16,
    SB_HW_16,
    SB_HW_16CSP,
    SB_HW_ALS100,
    SB_HW_ALS4000,
    SB_HW_DT019X,
    SB_HW_CS5530,
}

pub const SB_OPEN_PCM: u32 = 0x01;
pub const SB_OPEN_MIDI_INPUT: u32 = 0x02;
pub const SB_OPEN_MIDI_OUTPUT: u32 = 0x04;
pub const SB_OPEN_MIDI_INPUT_TRIGGER: u32 = 0x08;
pub const SB_OPEN_MIDI_OUTPUT_TRIGGER: u32 = 0x10;
pub const SB_MODE_HALT: u32 = 0x00;
pub const SB_MODE_PLAYBACK_8: u32 = 0x01;
pub const SB_MODE_PLAYBACK_16: u32 = 0x02;
pub const SB_MODE_PLAYBACK: u32 = SB_MODE_PLAYBACK_8 | SB_MODE_PLAYBACK_16;
pub const SB_MODE_CAPTURE_8: u32 = 0x04;
pub const SB_MODE_CAPTURE_16: u32 = 0x08;
pub const SB_MODE_CAPTURE: u32 = SB_MODE_CAPTURE_8 | SB_MODE_CAPTURE_16;
pub const SB_RATE_LOCK_PLAYBACK: u32 = 0x10;
pub const SB_RATE_LOCK_CAPTURE: u32 = 0x20;
pub const SB_RATE_LOCK: u32 = SB_RATE_LOCK_PLAYBACK | SB_RATE_LOCK_CAPTURE;
pub const SB_MPU_INPUT: u32 = 1;

#[repr(C)]
pub struct snd_sb {
    pub port: c_ulong,
    pub res_port: *mut resource,
    pub mpu_port: c_ulong,
    pub irq: c_int,
    pub dma8: c_int,
    pub dma16: c_int,
    pub version: u16,
    pub hardware: sb_hw_type,
    pub alt_port: c_ulong,
    pub pci: *mut pci_dev,
    pub open: u32,
    pub mode: u32,
    pub force_mode16: u32,
    pub locked_rate: u32,
    pub playback_format: u32,
    pub capture_format: u32,
    pub midi_timer: timer_list,
    pub p_dma_size: u32,
    pub p_period_size: u32,
    pub c_dma_size: u32,
    pub c_period_size: u32,
    pub mixer_lock: spinlock_t,
    pub name: [c_char; 32],
    pub csp: *mut c_void,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub rmidi: *mut snd_rawmidi,
    pub midi_substream_input: *mut snd_rawmidi_substream,
    pub midi_substream_output: *mut snd_rawmidi_substream,
    pub rmidi_callback: irq_handler_t,
    pub reg_lock: spinlock_t,
    pub open_lock: spinlock_t,
    pub midi_input_lock: spinlock_t,
    pub proc_entry: *mut snd_info_entry,
    #[cfg(CONFIG_PM)]
    pub saved_regs: [u8; 0x20],
}

pub type c_ulong = usize;
pub enum resource {}
pub enum pci_dev {}
pub enum timer_list {}
pub enum spinlock_t {}
pub enum snd_card {}
pub enum snd_pcm {}
pub enum snd_pcm_substream {}
pub enum snd_rawmidi {}
pub enum snd_rawmidi_substream {}
pub enum snd_info_entry {}
pub enum snd_pcm_ops {}
pub type irq_handler_t = Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>;
pub type irqreturn_t = c_int;

extern "C" {
    pub fn inb(port: c_ulong) -> u8;
}

pub const s_b_SB_RESET: c_ulong = 0x6;
pub const s_b_SB_READ: c_ulong = 0xa;
pub const s_b_SB_WRITE: c_ulong = 0xc;
pub const s_b_SB_COMMAND: c_ulong = 0xc;
pub const s_b_SB_STATUS: c_ulong = 0xc;
pub const s_b_SB_DATA_AVAIL: c_ulong = 0xe;
pub const s_b_SB_DATA_AVAIL_16: c_ulong = 0xf;
pub const s_b_SB_MIXER_ADDR: c_ulong = 0x4;
pub const s_b_SB_MIXER_DATA: c_ulong = 0x5;
pub const s_b_SB_OPL3_LEFT: c_ulong = 0x0;
pub const s_b_SB_OPL3_RIGHT: c_ulong = 0x2;
pub const s_b_SB_OPL3_BOTH: c_ulong = 0x8;

macro_rules! SBP { ($chip:expr, RESET) => { $chip.port + s_b_SB_RESET }; ($chip:expr, READ) => { $chip.port + s_b_SB_READ }; ($chip:expr, WRITE) => { $chip.port + s_b_SB_WRITE }; ($chip:expr, COMMAND) => { $chip.port + s_b_SB_COMMAND }; ($chip:expr, STATUS) => { $chip.port + s_b_SB_STATUS }; ($chip:expr, DATA_AVAIL) => { $chip.port + s_b_SB_DATA_AVAIL }; ($chip:expr, DATA_AVAIL_16) => { $chip.port + s_b_SB_DATA_AVAIL_16 }; }
macro_rules! SBP1 { ($port:expr, RESET) => { $port + s_b_SB_RESET }; ($port:expr, READ) => { $port + s_b_SB_READ }; ($port:expr, WRITE) => { $port + s_b_SB_WRITE }; ($port:expr, COMMAND) => { $port + s_b_SB_COMMAND }; ($port:expr, STATUS) => { $port + s_b_SB_STATUS }; ($port:expr, DATA_AVAIL) => { $port + s_b_SB_DATA_AVAIL }; ($port:expr, DATA_AVAIL_16) => { $port + s_b_SB_DATA_AVAIL_16 }; }

pub const SB_DSP_OUTPUT: u32 = 0x14;
pub const SB_DSP_INPUT: u32 = 0x24;
pub const SB_DSP_BLOCK_SIZE: u32 = 0x48;
pub const SB_DSP_HI_OUTPUT: u32 = 0x91;
pub const SB_DSP_HI_INPUT: u32 = 0x99;
pub const SB_DSP_LO_OUTPUT_AUTO: u32 = 0x1c;
pub const SB_DSP_LO_INPUT_AUTO: u32 = 0x2c;
pub const SB_DSP_HI_OUTPUT_AUTO: u32 = 0x90;
pub const SB_DSP_HI_INPUT_AUTO: u32 = 0x98;
pub const SB_DSP_IMMED_INT: u32 = 0xf2;
pub const SB_DSP_GET_VERSION: u32 = 0xe1;
pub const SB_DSP_SPEAKER_ON: u32 = 0xd1;
pub const SB_DSP_SPEAKER_OFF: u32 = 0xd3;
pub const SB_DSP_DMA8_OFF: u32 = 0xd0;
pub const SB_DSP_DMA8_ON: u32 = 0xd4;
pub const SB_DSP_DMA8_EXIT: u32 = 0xda;
pub const SB_DSP_DMA16_OFF: u32 = 0xd5;
pub const SB_DSP_DMA16_ON: u32 = 0xd6;
pub const SB_DSP_DMA16_EXIT: u32 = 0xd9;
pub const SB_DSP_SAMPLE_RATE: u32 = 0x40;
pub const SB_DSP_SAMPLE_RATE_OUT: u32 = 0x41;
pub const SB_DSP_SAMPLE_RATE_IN: u32 = 0x42;
pub const SB_DSP_MONO_8BIT: u32 = 0xa0;
pub const SB_DSP_MONO_16BIT: u32 = 0xa4;
pub const SB_DSP_STEREO_8BIT: u32 = 0xa8;
pub const SB_DSP_STEREO_16BIT: u32 = 0xac;
pub const SB_DSP_MIDI_INPUT_IRQ: u32 = 0x31;
pub const SB_DSP_MIDI_UART_IRQ: u32 = 0x35;
pub const SB_DSP_MIDI_OUTPUT: u32 = 0x38;
pub const SB_DSP4_OUT8_AI: u32 = 0xc6;
pub const SB_DSP4_IN8_AI: u32 = 0xce;
pub const SB_DSP4_OUT16_AI: u32 = 0xb6;
pub const SB_DSP4_IN16_AI: u32 = 0xbe;
pub const SB_DSP4_MODE_UNS_MONO: u32 = 0x00;
pub const SB_DSP4_MODE_SIGN_MONO: u32 = 0x10;
pub const SB_DSP4_MODE_UNS_STEREO: u32 = 0x20;
pub const SB_DSP4_MODE_SIGN_STEREO: u32 = 0x30;
pub const SB_DSP4_OUTPUT: u32 = 0x3c;
pub const SB_DSP4_INPUT_LEFT: u32 = 0x3d;
pub const SB_DSP4_INPUT_RIGHT: u32 = 0x3e;

pub const SB_DSP20_MASTER_DEV: u32 = 0x02; pub const SB_DSP20_PCM_DEV: u32 = 0x0a; pub const SB_DSP20_CD_DEV: u32 = 0x08; pub const SB_DSP20_FM_DEV: u32 = 0x06;
pub const SB_DSP_MASTER_DEV: u32 = 0x22; pub const SB_DSP_PCM_DEV: u32 = 0x04; pub const SB_DSP_LINE_DEV: u32 = 0x2e; pub const SB_DSP_CD_DEV: u32 = 0x28; pub const SB_DSP_FM_DEV: u32 = 0x26; pub const SB_DSP_MIC_DEV: u32 = 0x0a; pub const SB_DSP_CAPTURE_SOURCE: u32 = 0x0c; pub const SB_DSP_CAPTURE_FILT: u32 = 0x0c; pub const SB_DSP_PLAYBACK_FILT: u32 = 0x0e; pub const SB_DSP_STEREO_SW: u32 = 0x0e;
pub const SB_DSP_MIXS_MIC0: u32 = 0; pub const SB_DSP_MIXS_CD: u32 = 1; pub const SB_DSP_MIXS_MIC: u32 = 2; pub const SB_DSP_MIXS_LINE: u32 = 3;
pub const SB_DSP4_MASTER_DEV: u32 = 0x30; pub const SB_DSP4_BASS_DEV: u32 = 0x46; pub const SB_DSP4_TREBLE_DEV: u32 = 0x44; pub const SB_DSP4_SYNTH_DEV: u32 = 0x34; pub const SB_DSP4_PCM_DEV: u32 = 0x32; pub const SB_DSP4_SPEAKER_DEV: u32 = 0x3b; pub const SB_DSP4_LINE_DEV: u32 = 0x38; pub const SB_DSP4_MIC_DEV: u32 = 0x3a; pub const SB_DSP4_OUTPUT_SW: u32 = 0x3c; pub const SB_DSP4_CD_DEV: u32 = 0x36; pub const SB_DSP4_IGAIN_DEV: u32 = 0x3f; pub const SB_DSP4_OGAIN_DEV: u32 = 0x41; pub const SB_DSP4_MIC_AGC: u32 = 0x43;
pub const SB_DSP4_IRQSETUP: u32 = 0x80; pub const SB_DSP4_DMASETUP: u32 = 0x81; pub const SB_DSP4_IRQSTATUS: u32 = 0x82; pub const SB_DSP4_MPUSETUP: u32 = 0x84; pub const SB_DSP4_3DSE: u32 = 0x90;
pub const SB_DT019X_MASTER_DEV: u32 = 0x62; pub const SB_DT019X_PCM_DEV: u32 = 0x64; pub const SB_DT019X_SYNTH_DEV: u32 = 0x66; pub const SB_DT019X_CD_DEV: u32 = 0x68; pub const SB_DT019X_MIC_DEV: u32 = 0x6a; pub const SB_DT019X_SPKR_DEV: u32 = 0x6a; pub const SB_DT019X_LINE_DEV: u32 = 0x6e; pub const SB_DT019X_OUTPUT_SW2: u32 = 0x4c; pub const SB_DT019X_CAPTURE_SW: u32 = 0x6c;
pub const SB_DT019X_CAP_CD: u32 = 2; pub const SB_DT019X_CAP_MIC: u32 = 4; pub const SB_DT019X_CAP_LINE: u32 = 6; pub const SB_DT019X_CAP_SYNTH: u32 = 7; pub const SB_DT019X_CAP_MAIN: u32 = 7;
pub const SB_ALS4000_MONO_IO_CTRL: u32 = 0x4b; pub const SB_ALS4000_OUT_MIXER_CTRL_2: u32 = 0x4c; pub const SB_ALS4000_MIC_IN_GAIN: u32 = 0x4d; pub const SB_ALS4000_ANALOG_REFRNC_VOLT_CTRL: u32 = 0x4e; pub const SB_ALS4000_FMDAC: u32 = 0x4f; pub const SB_ALS4000_3D_SND_FX: u32 = 0x50; pub const SB_ALS4000_3D_TIME_DELAY: u32 = 0x51; pub const SB_ALS4000_3D_AUTO_MUTE: u32 = 0x52; pub const SB_ALS4000_ANALOG_BLOCK_CTRL: u32 = 0x53; pub const SB_ALS4000_3D_DELAYLINE_PATTERN: u32 = 0x54; pub const SB_ALS4000_CR3_CONFIGURATION: u32 = 0xc3; pub const SB_ALS4000_QSOUND: u32 = 0xdb;
pub const SB_IRQSETUP_IRQ9: u32 = 1; pub const SB_IRQSETUP_IRQ5: u32 = 2; pub const SB_IRQSETUP_IRQ7: u32 = 4; pub const SB_IRQSETUP_IRQ10: u32 = 8;
pub const SB_IRQTYPE_8BIT: u32 = 1; pub const SB_IRQTYPE_16BIT: u32 = 2; pub const SB_IRQTYPE_MPUIN: u32 = 4; pub const ALS4K_IRQTYPE_CR1E_DMA: u32 = 0x20;
pub const SB_DMASETUP_DMA0: u32 = 1; pub const SB_DMASETUP_DMA1: u32 = 2; pub const SB_DMASETUP_DMA3: u32 = 8; pub const SB_DMASETUP_DMA5: u32 = 0x20; pub const SB_DMASETUP_DMA6: u32 = 0x40; pub const SB_DMASETUP_DMA7: u32 = 0x80;

pub unsafe fn snd_sb_ack_8bit(chip: *mut snd_sb) { inb(SBP!((*chip), DATA_AVAIL)); }
pub unsafe fn snd_sb_ack_16bit(chip: *mut snd_sb) { inb(SBP!((*chip), DATA_AVAIL_16)); }

extern "C" {
    pub fn snd_sbdsp_command(chip: *mut snd_sb, val: u8) -> c_int;
    pub fn snd_sbdsp_get_byte(chip: *mut snd_sb) -> c_int;
    pub fn snd_sbdsp_reset(chip: *mut snd_sb) -> c_int;
    pub fn snd_sbdsp_create(card: *mut snd_card, port: c_ulong, irq: c_int, irq_handler: irq_handler_t, dma8: c_int, dma16: c_int, hardware: u16, r_chip: *mut *mut snd_sb) -> c_int;
    pub fn snd_sbmixer_write(chip: *mut snd_sb, reg: u8, data: u8);
    pub fn snd_sbmixer_read(chip: *mut snd_sb, reg: u8) -> u8;
    pub fn snd_sbmixer_new(chip: *mut snd_sb) -> c_int;
    pub fn snd_sb8dsp_pcm(chip: *mut snd_sb, device: c_int) -> c_int;
    pub fn snd_sb8dsp_interrupt(chip: *mut snd_sb) -> irqreturn_t;
    pub fn snd_sb8_playback_open(substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_sb8_capture_open(substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_sb8_playback_close(substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_sb8_capture_close(substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_sb8dsp_midi_interrupt(chip: *mut snd_sb) -> irqreturn_t;
    pub fn snd_sb8dsp_midi(chip: *mut snd_sb, device: c_int) -> c_int;
    pub fn snd_sb16dsp_pcm(chip: *mut snd_sb, device: c_int) -> c_int;
    pub fn snd_sb16dsp_get_pcm_ops(direction: c_int) -> *const snd_pcm_ops;
    pub fn snd_sb16dsp_configure(chip: *mut snd_sb) -> c_int;
    pub fn snd_sb16dsp_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}

#[repr(C)] pub struct sbmix_elem { pub name: *const c_char, pub type_: c_int, pub private_value: c_ulong }
pub const SB_MIX_SINGLE: c_int = 0; pub const SB_MIX_DOUBLE: c_int = 1; pub const SB_MIX_INPUT_SW: c_int = 2; pub const SB_MIX_CAPTURE_PRO: c_int = 3; pub const SB_MIX_CAPTURE_DT019X: c_int = 4; pub const SB_MIX_MONO_CAPTURE_ALS4K: c_int = 5;
pub const fn SB_MIXVAL_DOUBLE(l:u64,r:u64,ls:u64,rs:u64,m:u64)->u64 { l | (r<<8) | (ls<<16) | (rs<<19) | (m<<24) }
pub const fn SB_MIXVAL_SINGLE(r:u64,s:u64,m:u64)->u64 { r | (s<<16) | (m<<24) }
pub const fn SB_MIXVAL_INPUT_SW(r1:u64,r2:u64,ls:u64,rs:u64)->u64 { r1 | (r2<<8) | (ls<<16) | (rs<<24) }
extern "C" { pub fn snd_sbmixer_add_ctl(chip:*mut snd_sb,name:*const c_char,index:c_int,type_:c_int,value:c_ulong)->c_int; }
pub unsafe fn snd_sbmixer_add_ctl_elem(chip:*mut snd_sb,c:*const sbmix_elem)->c_int { snd_sbmixer_add_ctl(chip,(*c).name,0,(*c).type_,(*c).private_value) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
