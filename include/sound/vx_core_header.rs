/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of vx_core.h. External kernel/ALSA types are declarations. */

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_void};

#[repr(C)] pub struct firmware { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct snd_card { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_hwdep { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
pub type irqreturn_t = c_int;
pub type pcx_time_t = u64;

pub const VX_DRIVER_VERSION: u32 = 0x010000;
pub const SIZE_MAX_CMD: usize = 0x10;
pub const SIZE_MAX_STATUS: usize = 0x10;
pub const VX_MAX_PIPES: usize = 16;
pub const VX_MAX_PERIODS: usize = 32;
pub const VX_MAX_CODECS: usize = 2;

#[repr(C)] pub struct vx_rmh { pub LgCmd: u16, pub LgStat: u16, pub Cmd: [u32; SIZE_MAX_CMD], pub Stat: [u32; SIZE_MAX_STATUS], pub DspStat: u16 }
#[repr(C)] pub struct vx_ibl_info { pub size: c_int, pub max_size: c_int, pub min_size: c_int, pub granularity: c_int }
#[repr(C)] pub struct vx_pipe {
    pub number: c_int, pub is_capture: c_uint, pub data_mode: c_uint, pub running: c_uint, pub prepared: c_uint,
    pub channels: c_int, pub differed_type: c_uint, pub pcx_time: pcx_time_t, pub substream: *mut snd_pcm_substream,
    pub hbuf_size: c_int, pub buffer_bytes: c_int, pub period_bytes: c_int, pub hw_ptr: c_int, pub position: c_int,
    pub transferred: c_int, pub align: c_int, pub cur_count: u64, pub references: c_uint,
    pub monitoring_pipe: *mut vx_pipe,
}
#[repr(C)] pub struct vx_core { _private: [u8; 0] }

pub type In8 = unsafe extern "C" fn(*mut vx_core, c_int) -> c_uchar;
pub type In32 = unsafe extern "C" fn(*mut vx_core, c_int) -> c_uint;
pub type Out8 = unsafe extern "C" fn(*mut vx_core, c_int, c_uchar);
pub type Out32 = unsafe extern "C" fn(*mut vx_core, c_int, c_uint);
#[repr(C)] pub struct snd_vx_ops {
    pub in8: Option<In8>, pub in32: Option<In32>, pub out8: Option<Out8>, pub out32: Option<Out32>,
    pub test_and_ack: Option<unsafe extern "C" fn(*mut vx_core) -> c_int>,
    pub validate_irq: Option<unsafe extern "C" fn(*mut vx_core, c_int)>,
    pub write_codec: Option<unsafe extern "C" fn(*mut vx_core, c_int, c_uint)>,
    pub akm_write: Option<unsafe extern "C" fn(*mut vx_core, c_int, c_uint)>,
    pub reset_codec: Option<unsafe extern "C" fn(*mut vx_core)>,
    pub change_audio_source: Option<unsafe extern "C" fn(*mut vx_core, c_int)>,
    pub set_clock_source: Option<unsafe extern "C" fn(*mut vx_core, c_int)>,
    pub load_dsp: Option<unsafe extern "C" fn(*mut vx_core, c_int, *const firmware) -> c_int>,
    pub reset_dsp: Option<unsafe extern "C" fn(*mut vx_core)>,
    pub reset_board: Option<unsafe extern "C" fn(*mut vx_core, c_int)>,
    pub add_controls: Option<unsafe extern "C" fn(*mut vx_core) -> c_int>,
    pub dma_write: Option<unsafe extern "C" fn(*mut vx_core, *mut snd_pcm_runtime, *mut vx_pipe, c_int)>,
    pub dma_read: Option<unsafe extern "C" fn(*mut vx_core, *mut snd_pcm_runtime, *mut vx_pipe, c_int)>,
}
#[repr(C)] pub struct snd_vx_hardware { pub name: *const c_char, pub type_: c_int, pub num_codecs: c_uint, pub num_ins: c_uint, pub num_outs: c_uint, pub output_level_max: c_uint, pub output_level_db_scale: *const c_uint }

pub const SND_VX_HWDEP_ID: &str = "VX Loader";
pub const VX_TYPE_BOARD: c_int = 0; pub const VX_TYPE_V2: c_int = 1; pub const VX_TYPE_MIC: c_int = 2; pub const VX_TYPE_VXPOCKET: c_int = 3; pub const VX_TYPE_VXP440: c_int = 4; pub const VX_TYPE_NUMS: c_int = 5;
pub const VX_STAT_XILINX_LOADED: c_uint = 1 << 0; pub const VX_STAT_DEVICE_INIT: c_uint = 1 << 1; pub const VX_STAT_CHIP_INIT: c_uint = 1 << 2; pub const VX_STAT_IN_SUSPEND: c_uint = 1 << 10; pub const VX_STAT_IS_STALE: c_uint = 1 << 15;
pub const VX_ANALOG_OUT_LEVEL_MAX: c_uint = 0xe3;

extern "C" {
    pub fn snd_vx_create(card: *mut snd_card, hw: *const snd_vx_hardware, ops: *const snd_vx_ops, extra_size: c_int) -> *mut vx_core;
    pub fn snd_vx_setup_firmware(chip: *mut vx_core) -> c_int; pub fn snd_vx_load_boot_image(chip: *mut vx_core, dsp: *const firmware) -> c_int; pub fn snd_vx_dsp_boot(chip: *mut vx_core, dsp: *const firmware) -> c_int; pub fn snd_vx_dsp_load(chip: *mut vx_core, dsp: *const firmware) -> c_int; pub fn snd_vx_free_firmware(chip: *mut vx_core);
    pub fn snd_vx_irq_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t; pub fn snd_vx_threaded_irq_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t;
    pub fn vx_send_msg(chip: *mut vx_core, rmh: *mut vx_rmh) -> c_int; pub fn vx_send_msg_nolock(chip: *mut vx_core, rmh: *mut vx_rmh) -> c_int; pub fn vx_send_rih(chip: *mut vx_core, cmd: c_int) -> c_int; pub fn vx_send_rih_nolock(chip: *mut vx_core, cmd: c_int) -> c_int; pub fn vx_reset_codec(chip: *mut vx_core, cold_reset: c_int);
    pub fn snd_vx_check_reg_bit(chip: *mut vx_core, reg: c_int, mask: c_int, bit: c_int, time: c_int) -> c_int;
    pub fn snd_vx_pcm_new(chip: *mut vx_core) -> c_int; pub fn vx_pcm_update_intr(chip: *mut vx_core, events: c_uint); pub fn snd_vx_mixer_new(chip: *mut vx_core) -> c_int; pub fn vx_toggle_dac_mute(chip: *mut vx_core, mute: c_int); pub fn vx_sync_audio_source(chip: *mut vx_core) -> c_int; pub fn vx_set_monitor_level(chip: *mut vx_core, audio: c_int, level: c_int, active: c_int) -> c_int;
    pub fn vx_set_iec958_status(chip: *mut vx_core, bits: c_uint); pub fn vx_set_clock(chip: *mut vx_core, freq: c_uint) -> c_int; pub fn vx_set_internal_clock(chip: *mut vx_core, freq: c_uint); pub fn vx_change_frequency(chip: *mut vx_core) -> c_int; pub fn snd_vx_suspend(card: *mut vx_core) -> c_int; pub fn snd_vx_resume(card: *mut vx_core) -> c_int;
}

pub const VX_ERR_MASK: c_int = 0x1000000;
#[inline] pub const fn vx_get_error(err: c_int) -> c_int { (-err) & !VX_ERR_MASK }
#[inline] pub unsafe fn vx_has_new_dsp(chip_type: c_int) -> bool { chip_type != VX_TYPE_BOARD }
#[inline] pub unsafe fn vx_is_pcmcia(chip_type: c_int) -> bool { chip_type >= VX_TYPE_VXPOCKET }
pub const VX_DATA_CODEC_MASK:u32=0x80; pub const VX_DATA_XICOR_MASK:u32=0x80;
pub const VX_SUER_FREQ_MASK:u32=0x0c; pub const VX_SUER_FREQ_32KHZ_MASK:u32=0x0c; pub const VX_SUER_FREQ_44KHZ_MASK:u32=0; pub const VX_SUER_FREQ_48KHZ_MASK:u32=0x04; pub const VX_SUER_DATA_PRESENT_MASK:u32=0x02; pub const VX_SUER_CLOCK_PRESENT_MASK:u32=0x01;
pub const VX_CUER_HH_BITC_SEL_MASK:u32=0x08; pub const VX_CUER_MH_BITC_SEL_MASK:u32=0x04; pub const VX_CUER_ML_BITC_SEL_MASK:u32=0x02; pub const VX_CUER_LL_BITC_SEL_MASK:u32=0x01; pub const XX_UER_CBITS_OFFSET_MASK:u32=0x1f;
pub const VX_AUDIO_INFO_REAL_TIME:u32=1<<0; pub const VX_AUDIO_INFO_OFFLINE:u32=1<<1; pub const VX_AUDIO_INFO_MPEG1:u32=1<<5; pub const VX_AUDIO_INFO_MPEG2:u32=1<<6; pub const VX_AUDIO_INFO_LINEAR_8:u32=1<<7; pub const VX_AUDIO_INFO_LINEAR_16:u32=1<<8; pub const VX_AUDIO_INFO_LINEAR_24:u32=1<<9;
#[inline] pub unsafe fn vx_test_and_ack(chip: *mut vx_core, ops: *const snd_vx_ops) -> c_int { ((*ops).test_and_ack.unwrap())(chip) }
#[inline] pub unsafe fn vx_validate_irq(chip: *mut vx_core, ops: *const snd_vx_ops, enable: c_int) { ((*ops).validate_irq.unwrap())(chip, enable) }
#[inline] pub unsafe fn snd_vx_inb(chip: *mut vx_core, ops: *const snd_vx_ops, reg: c_int) -> c_uchar { ((*ops).in8.unwrap())(chip, reg) }
#[inline] pub unsafe fn snd_vx_inl(chip: *mut vx_core, ops: *const snd_vx_ops, reg: c_int) -> c_uint { ((*ops).in32.unwrap())(chip, reg) }
#[inline] pub unsafe fn snd_vx_outb(chip: *mut vx_core, ops: *const snd_vx_ops, reg: c_int, val: c_uchar) { ((*ops).out8.unwrap())(chip, reg, val) }
#[inline] pub unsafe fn snd_vx_outl(chip: *mut vx_core, ops: *const snd_vx_ops, reg: c_int, val: c_uint) { ((*ops).out32.unwrap())(chip, reg, val) }
#[inline] pub unsafe fn vx_reset_dsp(chip: *mut vx_core, ops: *const snd_vx_ops) { ((*ops).reset_dsp.unwrap())(chip) }
#[inline] pub unsafe fn vx_pseudo_dma_write(chip: *mut vx_core, ops: *const snd_vx_ops, runtime: *mut snd_pcm_runtime, pipe: *mut vx_pipe, count: c_int) { ((*ops).dma_write.unwrap())(chip, runtime, pipe, count) }
#[inline] pub unsafe fn vx_pseudo_dma_read(chip: *mut vx_core, ops: *const snd_vx_ops, runtime: *mut snd_pcm_runtime, pipe: *mut vx_pipe, count: c_int) { ((*ops).dma_read.unwrap())(chip, runtime, pipe, count) }

pub const VX_AUDIO_SRC_DIGITAL: c_int = 0; pub const VX_AUDIO_SRC_LINE: c_int = 1; pub const VX_AUDIO_SRC_MIC: c_int = 2; pub const INTERNAL_QUARTZ: c_int = 0; pub const UER_SYNC: c_int = 1;
pub const VX_CLOCK_MODE_AUTO: c_int = 0; pub const VX_CLOCK_MODE_INTERNAL: c_int = 1; pub const VX_CLOCK_MODE_EXTERNAL: c_int = 2; pub const VX_UER_MODE_CONSUMER: c_int = 0; pub const VX_UER_MODE_PROFESSIONAL: c_int = 1; pub const VX_UER_MODE_NOT_PRESENT: c_int = 2;
pub const VX_ICR: c_int=0; pub const VX_CVR: c_int=1; pub const VX_ISR: c_int=2; pub const VX_IVR: c_int=3; pub const VX_RXH: c_int=4; pub const VX_TXH: c_int=4; pub const VX_RXM: c_int=5; pub const VX_TXM: c_int=5; pub const VX_RXL: c_int=6; pub const VX_TXL: c_int=6; pub const VX_DMA: c_int=7; pub const VX_CDSP: c_int=8; pub const VX_RFREQ: c_int=9; pub const VX_RUER_V2: c_int=10; pub const VX_GAIN: c_int=11; pub const VX_DATA: c_int=11; pub const VX_MEMIRQ: c_int=12; pub const VX_ACQ: c_int=13; pub const VX_BIT0: c_int=14; pub const VX_BIT1: c_int=15; pub const VX_MIC0: c_int=16; pub const VX_MIC1: c_int=17; pub const VX_MIC2: c_int=18; pub const VX_MIC3: c_int=19; pub const VX_PLX0: c_int=20; pub const VX_PLX1: c_int=21; pub const VX_PLX2: c_int=22; pub const VX_LOFREQ: c_int=23; pub const VX_HIFREQ: c_int=24; pub const VX_CSUER: c_int=25; pub const VX_RUER: c_int=26; pub const VX_REG_MAX: c_int=27;
pub const VX_RESET_DMA: c_int=2; pub const VX_CFG: c_int=9; pub const VX_STATUS: c_int=12; pub const VX_SELMIC: c_int=16; pub const VX_COMPOT: c_int=17; pub const VX_SCOMPR: c_int=18; pub const VX_GLIMIT: c_int=19; pub const VX_INTCSR: c_int=20; pub const VX_CNTRL: c_int=21; pub const VX_GPIOC: c_int=22; pub const VX_MICRO: c_int=12; pub const VX_CODEC2: c_int=12; pub const VX_DIALOG: c_int=13;
pub const RMH_SSIZE_FIXED: c_int=0; pub const RMH_SSIZE_ARG: c_int=1; pub const RMH_SSIZE_MASK: c_int=2;
pub const ICR_HF1:u32=0x10; pub const ICR_HF0:u32=0x08; pub const ICR_TREQ:u32=0x02; pub const ICR_RREQ:u32=0x01; pub const CVR_HC:u32=0x80; pub const ISR_HF3:u32=0x10; pub const ISR_HF2:u32=0x08; pub const ISR_CHK:u32=0x10; pub const ISR_ERR:u32=0x08; pub const ISR_TX_READY:u32=0x04; pub const ISR_TX_EMPTY:u32=0x02; pub const ISR_RX_FULL:u32=0x01;
pub const VXP_IRQ_OFFSET:u32=0x40; pub const IRQ_MESS_WRITE_END:u32=0x30; pub const IRQ_MESS_WRITE_NEXT:u32=0x32; pub const IRQ_MESS_READ_NEXT:u32=0x34; pub const IRQ_MESS_READ_END:u32=0x36; pub const IRQ_MESSAGE:u32=0x38; pub const IRQ_RESET_CHK:u32=0x3a; pub const IRQ_CONNECT_STREAM_NEXT:u32=0x26; pub const IRQ_CONNECT_STREAM_END:u32=0x28;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
