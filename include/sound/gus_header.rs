/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of sound/gus.h. External kernel/ALSA types and functions
 * are intentionally left as dependencies of the surrounding crate. */

#[macro_export]
macro_rules! GUSP { ($gus:expr, $x:ident) => { unsafe { (*$gus).gf1.port + SNDRV_g_u_s_$x } }; }

pub const SNDRV_g_u_s_MIDICTRL: usize = 0x320 - 0x220;
pub const SNDRV_g_u_s_MIDISTAT: usize = 0x320 - 0x220;
pub const SNDRV_g_u_s_MIDIDATA: usize = 0x321 - 0x220;
pub const SNDRV_g_u_s_GF1PAGE: usize = 0x322 - 0x220;
pub const SNDRV_g_u_s_GF1REGSEL: usize = 0x323 - 0x220;
pub const SNDRV_g_u_s_GF1DATALOW: usize = 0x324 - 0x220;
pub const SNDRV_g_u_s_GF1DATAHIGH: usize = 0x325 - 0x220;
pub const SNDRV_g_u_s_IRQSTAT: usize = 0x226 - 0x220;
pub const SNDRV_g_u_s_TIMERCNTRL: usize = 0x228 - 0x220;
pub const SNDRV_g_u_s_TIMERDATA: usize = 0x229 - 0x220;
pub const SNDRV_g_u_s_DRAM: usize = 0x327 - 0x220;
pub const SNDRV_g_u_s_MIXCNTRLREG: usize = 0;
pub const SNDRV_g_u_s_IRQDMACNTRLREG: usize = 0x22b - 0x220;
pub const SNDRV_g_u_s_REGCNTRLS: usize = 0x22f - 0x220;
pub const SNDRV_g_u_s_BOARDVERSION: usize = 0x726 - 0x220;
pub const SNDRV_g_u_s_MIXCNTRLPORT: usize = 0x726 - 0x220;
pub const SNDRV_g_u_s_IVER: usize = 0x325 - 0x220;
pub const SNDRV_g_u_s_MIXDATAPORT: usize = 0x326 - 0x220;
pub const SNDRV_g_u_s_MAXCNTRLPORT: usize = 0x326 - 0x220;

pub const SNDRV_GF1_GB_ACTIVE_VOICES: u8 = 0x0e;
pub const SNDRV_GF1_GB_VOICES_IRQ: u8 = 0x0f;
pub const SNDRV_GF1_GB_GLOBAL_MODE: u8 = 0x19;
pub const SNDRV_GF1_GW_LFO_BASE: u8 = 0x1a;
pub const SNDRV_GF1_GB_VOICES_IRQ_READ: u8 = 0x1f;
pub const SNDRV_GF1_GB_DRAM_DMA_CONTROL: u8 = 0x41;
pub const SNDRV_GF1_GW_DRAM_DMA_LOW: u8 = 0x42;
pub const SNDRV_GF1_GW_DRAM_IO_LOW: u8 = 0x43;
pub const SNDRV_GF1_GB_DRAM_IO_HIGH: u8 = 0x44;
pub const SNDRV_GF1_GB_SOUND_BLASTER_CONTROL: u8 = 0x45;
pub const SNDRV_GF1_GB_ADLIB_TIMER_1: u8 = 0x46;
pub const SNDRV_GF1_GB_ADLIB_TIMER_2: u8 = 0x47;
pub const SNDRV_GF1_GB_RECORD_RATE: u8 = 0x48;
pub const SNDRV_GF1_GB_REC_DMA_CONTROL: u8 = 0x49;
pub const SNDRV_GF1_GB_JOYSTICK_DAC_LEVEL: u8 = 0x4b;
pub const SNDRV_GF1_GB_RESET: u8 = 0x4c;
pub const SNDRV_GF1_GB_DRAM_DMA_HIGH: u8 = 0x50;
pub const SNDRV_GF1_GW_DRAM_IO16: u8 = 0x51;
pub const SNDRV_GF1_GW_MEMORY_CONFIG: u8 = 0x52;
pub const SNDRV_GF1_GB_MEMORY_CONTROL: u8 = 0x53;
pub const SNDRV_GF1_GW_FIFO_RECORD_BASE_ADDR: u8 = 0x54;
pub const SNDRV_GF1_GW_FIFO_PLAY_BASE_ADDR: u8 = 0x55;
pub const SNDRV_GF1_GW_FIFO_SIZE: u8 = 0x56;
pub const SNDRV_GF1_GW_INTERLEAVE: u8 = 0x57;
pub const SNDRV_GF1_GB_COMPATIBILITY: u8 = 0x59;
pub const SNDRV_GF1_GB_DECODE_CONTROL: u8 = 0x5a;
pub const SNDRV_GF1_GB_VERSION_NUMBER: u8 = 0x5b;
pub const SNDRV_GF1_GB_MPU401_CONTROL_A: u8 = 0x5c;
pub const SNDRV_GF1_GB_MPU401_CONTROL_B: u8 = 0x5d;
pub const SNDRV_GF1_GB_EMULATION_IRQ: u8 = 0x60;

pub const SNDRV_GF1_VB_ADDRESS_CONTROL: u8 = 0x00;
pub const SNDRV_GF1_VW_FREQUENCY: u8 = 0x01;
pub const SNDRV_GF1_VW_START_HIGH: u8 = 0x02;
pub const SNDRV_GF1_VW_START_LOW: u8 = 0x03;
pub const SNDRV_GF1_VA_START: u8 = SNDRV_GF1_VW_START_HIGH;
pub const SNDRV_GF1_VW_END_HIGH: u8 = 0x04;
pub const SNDRV_GF1_VW_END_LOW: u8 = 0x05;
pub const SNDRV_GF1_VA_END: u8 = SNDRV_GF1_VW_END_HIGH;
pub const SNDRV_GF1_VB_VOLUME_RATE: u8 = 0x06;
pub const SNDRV_GF1_VB_VOLUME_START: u8 = 0x07;
pub const SNDRV_GF1_VB_VOLUME_END: u8 = 0x08;
pub const SNDRV_GF1_VW_VOLUME: u8 = 0x09;
pub const SNDRV_GF1_VW_CURRENT_HIGH: u8 = 0x0a;
pub const SNDRV_GF1_VW_CURRENT_LOW: u8 = 0x0b;
pub const SNDRV_GF1_VA_CURRENT: u8 = SNDRV_GF1_VW_CURRENT_HIGH;
pub const SNDRV_GF1_VB_PAN: u8 = 0x0c;
pub const SNDRV_GF1_VW_OFFSET_RIGHT: u8 = 0x0c;
pub const SNDRV_GF1_VB_VOLUME_CONTROL: u8 = 0x0d;
pub const SNDRV_GF1_VB_UPPER_ADDRESS: u8 = 0x10;
pub const SNDRV_GF1_VW_EFFECT_HIGH: u8 = 0x11;
pub const SNDRV_GF1_VW_EFFECT_LOW: u8 = 0x12;
pub const SNDRV_GF1_VA_EFFECT: u8 = SNDRV_GF1_VW_EFFECT_HIGH;
pub const SNDRV_GF1_VW_OFFSET_LEFT: u8 = 0x13;
pub const SNDRV_GF1_VB_ACCUMULATOR: u8 = 0x14;
pub const SNDRV_GF1_VB_MODE: u8 = 0x15;
pub const SNDRV_GF1_VW_EFFECT_VOLUME: u8 = 0x16;
pub const SNDRV_GF1_VB_FREQUENCY_LFO: u8 = 0x17;
pub const SNDRV_GF1_VB_VOLUME_LFO: u8 = 0x18;
pub const SNDRV_GF1_VW_OFFSET_RIGHT_FINAL: u8 = 0x1b;
pub const SNDRV_GF1_VW_OFFSET_LEFT_FINAL: u8 = 0x1c;
pub const SNDRV_GF1_VW_EFFECT_VOLUME_FINAL: u8 = 0x1d;

pub const SNDRV_ICS_MIC_DEV: u32 = 0; pub const SNDRV_ICS_LINE_DEV: u32 = 1;
pub const SNDRV_ICS_CD_DEV: u32 = 2; pub const SNDRV_ICS_GF1_DEV: u32 = 3;
pub const SNDRV_ICS_NONE_DEV: u32 = 4; pub const SNDRV_ICS_MASTER_DEV: u32 = 5;
pub const SNDRV_LFO_TREMOLO: u32 = 0; pub const SNDRV_LFO_VIBRATO: u32 = 1;
pub const SNDRV_GF1_DMA_UNSIGNED: u32 = 0x80; pub const SNDRV_GF1_DMA_16BIT: u32 = 0x40;
pub const SNDRV_GF1_DMA_IRQ: u32 = 0x20; pub const SNDRV_GF1_DMA_WIDTH16: u32 = 0x04;
pub const SNDRV_GF1_DMA_READ: u32 = 0x02; pub const SNDRV_GF1_DMA_ENABLE: u32 = 0x01;
pub const SNDRV_GF1_MIN_VOLUME: u32 = 1800; pub const SNDRV_GF1_MAX_VOLUME: u32 = 4095;
pub const SNDRV_GF1_MIN_OFFSET: u32 = SNDRV_GF1_MIN_VOLUME >> 4; pub const SNDRV_GF1_MAX_OFFSET: u32 = 255;
pub const SNDRV_GF1_MAX_TDEPTH: u32 = 90; pub const SNDRV_GF1_MEM_BLOCK_16BIT: u16 = 1;
pub const SNDRV_GF1_MEM_OWNER_DRIVER: u16 = 1; pub const SNDRV_GF1_MEM_OWNER_WAVE_SIMPLE: u16 = 2;
pub const SNDRV_GF1_MEM_OWNER_WAVE_GF1: u16 = 3; pub const SNDRV_GF1_MEM_OWNER_WAVE_IWFFFF: u16 = 4;
pub const SNDRV_GF1_HANDLER_MIDI_OUT: u32 = 0x00010000; pub const SNDRV_GF1_HANDLER_MIDI_IN: u32 = 0x00020000;
pub const SNDRV_GF1_HANDLER_TIMER1: u32 = 0x00040000; pub const SNDRV_GF1_HANDLER_TIMER2: u32 = 0x00080000;
pub const SNDRV_GF1_HANDLER_VOICE: u32 = 0x00100000; pub const SNDRV_GF1_HANDLER_DMA_WRITE: u32 = 0x00200000;
pub const SNDRV_GF1_HANDLER_DMA_READ: u32 = 0x00400000;
pub const SNDRV_GF1_HANDLER_ALL: u32 = 0xffff0000 & !SNDRV_GF1_HANDLER_VOICE;
pub const SNDRV_GF1_DMA_TRIGGER: u32 = 1;

#[repr(C)] pub struct snd_gus_card;
#[repr(C)] pub struct snd_gf1_bank_info { pub address: u32, pub size: u32 }
#[repr(C)] pub struct snd_gf1_mem_block { pub flags:u16, pub owner:u16, pub share:u32, pub share_id:[u32;4], pub ptr:u32, pub size:u32, pub name:*mut i8, pub next:*mut snd_gf1_mem_block, pub prev:*mut snd_gf1_mem_block }
#[repr(C)] pub struct snd_gf1_mem { pub banks_8:[snd_gf1_bank_info;4], pub banks_16:[snd_gf1_bank_info;4], pub first:*mut snd_gf1_mem_block, pub last:*mut snd_gf1_mem_block, pub memory_mutex: mutex }
#[repr(C)] pub struct snd_gf1_dma_block { pub buffer:*mut core::ffi::c_void, pub buf_addr:usize, pub addr:u32, pub count:u32, pub cmd:u32, pub ack:Option<unsafe extern "C" fn(*mut snd_gus_card,*mut core::ffi::c_void)>, pub private_data:*mut core::ffi::c_void, pub next:*mut snd_gf1_dma_block }
#[repr(C)] pub struct snd_gus_port { pub chset:*mut snd_midi_channel_set, pub gus:*mut snd_gus_card, pub mode:i32, pub client:i32, pub port:i32, pub midi_has_voices:u32 }
#[repr(C)] pub struct snd_gus_voice;
pub const SNDRV_GF1_VOICE_TYPE_PCM:i32=0; pub const SNDRV_GF1_VOICE_TYPE_SYNTH:i32=1; pub const SNDRV_GF1_VOICE_TYPE_MIDI:i32=2;
pub const SNDRV_GF1_VFLG_RUNNING:u32=1; pub const SNDRV_GF1_VFLG_EFFECT_TIMER1:u32=2; pub const SNDRV_GF1_VFLG_PAN:u32=4;
#[repr(C)] pub enum snd_gus_volume_state { VENV_BEFORE, VENV_ATTACK, VENV_SUSTAIN, VENV_RELEASE, VENV_DONE, VENV_VOLUME }

#[repr(C)] pub struct snd_gus_voice {
 pub number:i32, pub use_:u32, pub pcm:u32, pub synth:u32, pub midi:u32, pub flags:u32,
 pub client:u8, pub port:u8, pub index:u8, pub pad:u8,
 pub handler_wave:Option<unsafe extern "C" fn(*mut snd_gus_card,*mut snd_gus_voice)>, pub handler_volume:Option<unsafe extern "C" fn(*mut snd_gus_card,*mut snd_gus_voice)>, pub handler_effect:Option<unsafe extern "C" fn(*mut snd_gus_card,*mut snd_gus_voice)>, pub volume_change:Option<unsafe extern "C" fn(*mut snd_gus_card)>, pub sample_ops:*mut snd_gus_sample_ops,
 pub fc_register:u16, pub fc_lfo:u16, pub gf1_volume:u16, pub control:u8, pub mode:u8, pub gf1_pan:u8, pub effect_accumulator:u8, pub volume_control:u8, pub venv_value_next:u8, pub venv_state:snd_gus_volume_state, pub venv_state_prev:snd_gus_volume_state, pub vlo:u16, pub vro:u16, pub gf1_effect_volume:u16, pub private_data:*mut core::ffi::c_void, pub private_free:Option<unsafe extern "C" fn(*mut snd_gus_voice)>
}
#[repr(C)] pub struct snd_gf1 { pub enh_mode:u32, pub hw_lfo:u32, pub sw_lfo:u32, pub effect:u32, pub port:usize, pub res_port1:*mut resource, pub res_port2:*mut resource, pub irq:i32, pub dma1:i32, pub dma2:i32, pub memory:u32, pub rom_memory:u32, pub rom_present:u32, pub rom_banks:u32, pub mem_alloc:snd_gf1_mem, pub reg_page:u16, pub reg_regsel:u16, pub reg_data8:u16, pub reg_data16:u16, pub reg_irqstat:u16, pub reg_dram:u16, pub reg_timerctrl:u16, pub reg_timerdata:u16, pub ics_regs:[[u8;2];6], pub active_voices:u8, pub active_voice:u8, pub voices:[snd_gus_voice;32], pub default_voice_address:u32, pub playback_freq:u16, pub mode:u16, pub volume_ramp:u8, pub smooth_pan:u8, pub full_range_pan:u8, pub pad0:u8, pub lfos:*mut u8, pub seq_client:i32, pub seq_ports:[snd_gus_port;4], pub timer_enabled:u16, pub timer1:*mut snd_timer, pub timer2:*mut snd_timer, pub uart_cmd:u16, pub uart_framing:u32, pub uart_overrun:u32, pub dma_flags:u32, pub dma_shared:u32, pub dma_data_pcm:*mut snd_gf1_dma_block, pub dma_data_pcm_last:*mut snd_gf1_dma_block, pub dma_data_synth:*mut snd_gf1_dma_block, pub dma_data_synth_last:*mut snd_gf1_dma_block, pub dma_ack:Option<unsafe extern "C" fn(*mut snd_gus_card,*mut core::ffi::c_void)>, pub dma_private_data:*mut core::ffi::c_void, pub pcm_channels:i32, pub pcm_alloc_voices:i32, pub pcm_volume_level_left:u16, pub pcm_volume_level_right:u16, pub pcm_volume_level_left1:u16, pub pcm_volume_level_right1:u16, pub pcm_rcntrl_reg:u8, pub pad_end:u8 }
#[repr(C)] pub struct snd_gus_card { pub card:*mut snd_card, pub initialized:u32, pub equal_irq:u32, pub equal_dma:u32, pub ics_flag:u32, pub ics_flipped:u32, pub codec_flag:u32, pub max_flag:u32, pub max_ctrl_flag:u32, pub daughter_flag:u32, pub interwave:u32, pub ess_flag:u32, pub ace_flag:u32, pub uart_enable:u32, pub revision:u16, pub max_cntrl_val:u16, pub mix_cntrl_reg:u16, pub joystick_dac:u16, pub timer_dev:i32, pub gf1:snd_gf1, pub pcm:*mut snd_pcm, pub pcm_cap_substream:*mut snd_pcm_substream, pub c_dma_size:u32, pub c_period_size:u32, pub c_pos:u32, pub midi_uart:*mut snd_rawmidi, pub midi_substream_output:*mut snd_rawmidi_substream, pub midi_substream_input:*mut snd_rawmidi_substream, pub reg_lock:spinlock_t, pub voice_alloc:spinlock_t, pub active_voice_lock:spinlock_t, pub event_lock:spinlock_t, pub dma_lock:spinlock_t, pub pcm_volume_level_lock:spinlock_t, pub uart_cmd_lock:spinlock_t, pub dma_mutex:mutex, pub register_mutex:mutex }

#[repr(C)] pub struct _SND_IW_LFO_PROGRAM { pub freq_and_control:u16, pub depth_final:u8, pub depth_inc:u8, pub twave:u16, pub depth:u16 }

/* External declarations from the ALSA/kernel headers and implementation files. */
extern "C" { pub fn snd_gf1_delay(gus:*mut snd_gus_card); pub fn snd_gf1_ctrl_stop(gus:*mut snd_gus_card,reg:u8); pub fn snd_gf1_write8(gus:*mut snd_gus_card,reg:u8,data:u8); pub fn snd_gf1_look8(gus:*mut snd_gus_card,reg:u8)->u8; pub fn snd_gf1_write16(gus:*mut snd_gus_card,reg:u8,data:u32); pub fn snd_gf1_look16(gus:*mut snd_gus_card,reg:u8)->u16; pub fn snd_gf1_select_active_voices(gus:*mut snd_gus_card); pub fn snd_gf1_i_look8(gus:*mut snd_gus_card,reg:u8)->u8; pub fn snd_gf1_i_look16(gus:*mut snd_gus_card,reg:u8)->u16; }
#[inline] pub unsafe fn snd_gf1_read8(gus:*mut snd_gus_card,reg:u8)->u8 { snd_gf1_look8(gus,reg|0x80) }
#[inline] pub unsafe fn snd_gf1_read16(gus:*mut snd_gus_card,reg:u8)->u16 { snd_gf1_look16(gus,reg|0x80) }
#[inline] pub unsafe fn snd_gf1_i_read8(gus:*mut snd_gus_card,reg:u8)->u8 { snd_gf1_i_look8(gus,reg|0x80) }
#[inline] pub unsafe fn snd_gf1_i_read16(gus:*mut snd_gus_card,reg:u8)->u16 { snd_gf1_i_look16(gus,reg|0x80) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
