/* Translated from soundcard.h. External ioctl/patch-key definitions are supplied by dependent headers. */
#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::mem::size_of;

pub const SOUND_VERSION: u32 = 0x030802;
pub const OPEN_SOUND_SYSTEM: bool = true;

pub const SNDCARD_ADLIB:i32=1; pub const SNDCARD_SB:i32=2; pub const SNDCARD_PAS:i32=3; pub const SNDCARD_GUS:i32=4;
pub const SNDCARD_MPU401:i32=5; pub const SNDCARD_SB16:i32=6; pub const SNDCARD_SB16MIDI:i32=7; pub const SNDCARD_UART6850:i32=8;
pub const SNDCARD_GUS16:i32=9; pub const SNDCARD_MSS:i32=10; pub const SNDCARD_PSS:i32=11; pub const SNDCARD_SSCAPE:i32=12;
pub const SNDCARD_PSS_MPU:i32=13; pub const SNDCARD_PSS_MSS:i32=14; pub const SNDCARD_SSCAPE_MSS:i32=15; pub const SNDCARD_TRXPRO:i32=16;
pub const SNDCARD_TRXPRO_SB:i32=17; pub const SNDCARD_TRXPRO_MPU:i32=18; pub const SNDCARD_MAD16:i32=19; pub const SNDCARD_MAD16_MPU:i32=20;
pub const SNDCARD_CS4232:i32=21; pub const SNDCARD_CS4232_MPU:i32=22; pub const SNDCARD_MAUI:i32=23; pub const SNDCARD_PSEUDO_MSS:i32=24;
pub const SNDCARD_GUSPNP:i32=25; pub const SNDCARD_UART401:i32=26;

/* ioctl encoding used by this header; platform ioctl constants may override these. */
pub const SIOCPARM_MASK:u32=0x1fff; pub const SIOC_VOID:u32=0; pub const SIOC_OUT:u32=0x20000000; pub const SIOC_IN:u32=0x40000000; pub const SIOC_INOUT:u32=SIOC_IN|SIOC_OUT;
pub const fn _SIO(x:u32,y:u32)->u32 { SIOC_VOID|(x<<8)|y }
pub const fn _SIOR<T>(x:u32,y:u32)->u32 { SIOC_OUT|(((size_of::<T>() as u32)&SIOCPARM_MASK)<<16)|(x<<8)|y }
pub const fn _SIOW<T>(x:u32,y:u32)->u32 { SIOC_IN|(((size_of::<T>() as u32)&SIOCPARM_MASK)<<16)|(x<<8)|y }
pub const fn _SIOWR<T>(x:u32,y:u32)->u32 { SIOC_INOUT|(((size_of::<T>() as u32)&SIOCPARM_MASK)<<16)|(x<<8)|y }

#[repr(C)] pub struct synth_control { pub devno:i32, pub data:[i8;4000] }
#[repr(C)] pub struct remove_sample { pub devno:i32,pub bankno:i32,pub instrno:i32 }
#[repr(C)] pub struct seq_event_rec { pub arr:[u8;8] }
#[repr(C)] pub struct patch_info { pub key:u16,pub device_no:i16,pub instr_no:i16,pub mode:u32,pub len:i32,pub loop_start:i32,pub loop_end:i32,pub base_freq:u32,pub base_note:u32,pub high_note:u32,pub low_note:u32,pub panning:i32,pub detuning:i32,pub env_rate:[u8;6],pub env_offset:[u8;6],pub tremolo_sweep:u8,pub tremolo_rate:u8,pub tremolo_depth:u8,pub vibrato_sweep:u8,pub vibrato_rate:u8,pub vibrato_depth:u8,pub scale_frequency:i32,pub scale_factor:u32,pub volume:i32,pub fractions:i32,pub reserved1:i32,pub spare:[i32;2],pub data:[i8;1] }
#[repr(C)] pub struct sysex_info { pub key:i16,pub device_no:i16,pub len:i32,pub data:[u8;1] }
#[repr(C)] pub struct sbi_instrument { pub key:u16,pub device:i16,pub channel:i32,pub operators:[u8;32] }
pub type sbi_instr_data=[u8;32];
#[repr(C)] pub struct synth_info { pub name:[i8;30],pub device:i32,pub synth_type:i32,pub synth_subtype:i32,pub perc_mode:i32,pub nr_voices:i32,pub nr_drums:i32,pub instr_bank_size:i32,pub capabilities:u32,pub dummies:[i32;19] }
#[repr(C)] pub struct sound_timer_info { pub name:[i8;32],pub caps:i32 }
#[repr(C)] pub struct midi_info { pub name:[i8;30],pub device:i32,pub capabilities:u32,pub dev_type:i32,pub dummies:[i32;18] }
#[repr(C)] pub struct mpu_command_rec { pub cmd:u8,pub nr_args:i8,pub nr_returns:i8,pub data:[u8;30] }
#[repr(C)] pub struct audio_buf_info { pub fragments:i32,pub fragstotal:i32,pub fragsize:i32,pub bytes:i32 }
#[repr(C)] pub struct count_info { pub bytes:i32,pub blocks:i32,pub ptr:i32 }
#[repr(C)] pub struct buffmem_desc { pub buffer:*mut u32,pub size:i32 }
#[repr(C)] pub struct copr_buffer { pub command:i32,pub flags:i32,pub len:i32,pub offs:i32,pub data:[u8;4000] }
#[repr(C)] pub struct copr_debug_buf { pub command:i32,pub parm1:i32,pub parm2:i32,pub flags:i32,pub len:i32 }
#[repr(C)] pub struct copr_msg { pub len:i32,pub data:[u8;4000] }
#[repr(C)] pub struct mixer_info { pub id:[i8;16],pub name:[i8;32],pub modify_counter:i32,pub fillers:[i32;10] }
#[repr(C)] pub struct _old_mixer_info { pub id:[i8;16],pub name:[i8;32] }
pub type mixer_record=[u8;128];
#[repr(C)] pub struct mixer_vol_table { pub num:i32,pub name:[i8;32],pub levels:[i32;32] }

/* Constants, aliases, and ioctl values below retain the original numeric interface. */
pub const TMR_WAIT_REL:i32=1; pub const TMR_WAIT_ABS:i32=2; pub const TMR_STOP:i32=3; pub const TMR_START:i32=4; pub const TMR_CONTINUE:i32=5; pub const TMR_TEMPO:i32=6; pub const TMR_ECHO:i32=8; pub const TMR_CLOCK:i32=9; pub const TMR_SPP:i32=10; pub const TMR_TIMESIG:i32=11;
pub const SEQ_NOTEOFF:i32=0; pub const SEQ_NOTEON:i32=1; pub const SEQ_WAIT:i32=TMR_WAIT_ABS; pub const SEQ_PGMCHANGE:i32=3; pub const SEQ_SYNCTIMER:i32=TMR_START; pub const SEQ_MIDIPUTC:i32=5; pub const SEQ_DRUMON:i32=6; pub const SEQ_DRUMOFF:i32=7; pub const SEQ_ECHO:i32=TMR_ECHO; pub const SEQ_AFTERTOUCH:i32=9; pub const SEQ_CONTROLLER:i32=10; pub const SEQ_BALANCE:i32=11; pub const SEQ_VOLMODE:i32=12; pub const SEQ_FULLSIZE:i32=0xfd; pub const SEQ_PRIVATE:i32=0xfe; pub const SEQ_EXTENDED:i32=0xff;
pub const EV_SEQ_LOCAL:u8=0x80; pub const EV_TIMING:u8=0x81; pub const EV_CHN_COMMON:u8=0x92; pub const EV_CHN_VOICE:u8=0x93; pub const EV_SYSEX:u8=0x94;
pub const MIDI_NOTEOFF:u8=0x80; pub const MIDI_NOTEON:u8=0x90; pub const MIDI_KEY_PRESSURE:u8=0xa0; pub const MIDI_CTL_CHANGE:u8=0xb0; pub const MIDI_PGM_CHANGE:u8=0xc0; pub const MIDI_CHN_PRESSURE:u8=0xd0; pub const MIDI_PITCH_BEND:u8=0xe0; pub const MIDI_SYSTEM_PREFIX:u8=0xf0;
pub const LOCL_STARTAUDIO:i32=1; pub const VOL_METHOD_ADAGIO:i32=1; pub const VOL_METHOD_LINEAR:i32=2;

pub const AFMT_QUERY:u32=0; pub const AFMT_MU_LAW:u32=1; pub const AFMT_A_LAW:u32=2; pub const AFMT_IMA_ADPCM:u32=4; pub const AFMT_U8:u32=8; pub const AFMT_S16_LE:u32=0x10; pub const AFMT_S16_BE:u32=0x20; pub const AFMT_S8:u32=0x40; pub const AFMT_U16_LE:u32=0x80; pub const AFMT_U16_BE:u32=0x100; pub const AFMT_MPEG:u32=0x200; pub const AFMT_AC3:u32=0x400;
pub const DSP_CAP_REVISION:u32=0xff; pub const DSP_CAP_DUPLEX:u32=0x100; pub const DSP_CAP_REALTIME:u32=0x200; pub const DSP_CAP_BATCH:u32=0x400; pub const DSP_CAP_COPROC:u32=0x800; pub const DSP_CAP_TRIGGER:u32=0x1000; pub const DSP_CAP_MMAP:u32=0x2000; pub const DSP_CAP_MULTI:u32=0x4000; pub const DSP_CAP_BIND:u32=0x8000; pub const PCM_ENABLE_INPUT:u32=1; pub const PCM_ENABLE_OUTPUT:u32=2;
pub const SOUND_MIXER_NRDEVICES:i32=25; pub const SOUND_ONOFF_MIN:i32=28; pub const SOUND_ONOFF_MAX:i32=30; pub const SOUND_MIXER_NONE:i32=31;
pub const SOUND_MIXER_VOLUME:i32=0; pub const SOUND_MIXER_BASS:i32=1; pub const SOUND_MIXER_TREBLE:i32=2; pub const SOUND_MIXER_SYNTH:i32=3; pub const SOUND_MIXER_PCM:i32=4; pub const SOUND_MIXER_SPEAKER:i32=5; pub const SOUND_MIXER_LINE:i32=6; pub const SOUND_MIXER_MIC:i32=7; pub const SOUND_MIXER_CD:i32=8; pub const SOUND_MIXER_IMIX:i32=9; pub const SOUND_MIXER_ALTPCM:i32=10; pub const SOUND_MIXER_RECLEV:i32=11; pub const SOUND_MIXER_IGAIN:i32=12; pub const SOUND_MIXER_OGAIN:i32=13; pub const SOUND_MIXER_LINE1:i32=14; pub const SOUND_MIXER_LINE2:i32=15; pub const SOUND_MIXER_LINE3:i32=16; pub const SOUND_MIXER_DIGITAL1:i32=17; pub const SOUND_MIXER_DIGITAL2:i32=18; pub const SOUND_MIXER_DIGITAL3:i32=19; pub const SOUND_MIXER_PHONEIN:i32=20; pub const SOUND_MIXER_PHONEOUT:i32=21; pub const SOUND_MIXER_VIDEO:i32=22; pub const SOUND_MIXER_RADIO:i32=23; pub const SOUND_MIXER_MONITOR:i32=24;
pub const SOUND_MIXER_RECSRC:i32=0xff; pub const SOUND_MIXER_DEVMASK:i32=0xfe; pub const SOUND_MIXER_RECMASK:i32=0xfd; pub const SOUND_MIXER_CAPS:i32=0xfc; pub const SOUND_MIXER_STEREODEVS:i32=0xfb; pub const SOUND_MIXER_OUTSRC:i32=0xfa; pub const SOUND_MIXER_OUTMASK:i32=0xf9; pub const SOUND_CAP_EXCL_INPUT:u32=1;
pub const APF_NORMAL:i32=0; pub const APF_NETWORK:i32=1; pub const APF_CPUINTENS:i32=2;
pub const CPF_NONE:i32=0; pub const CPF_FIRST:i32=1; pub const CPF_LAST:i32=2;
pub const CTRL_PITCH_BENDER:i32=255; pub const CTRL_PITCH_BENDER_RANGE:i32=254; pub const CTRL_EXPRESSION:i32=253; pub const CTRL_MAIN_VOLUME:i32=252;
pub const CTL_BANK_SELECT:i32=0; pub const CTL_MODWHEEL:i32=1; pub const CTL_BREATH:i32=2; pub const CTL_FOOT:i32=4; pub const CTL_PORTAMENTO_TIME:i32=5; pub const CTL_DATA_ENTRY:i32=6; pub const CTL_MAIN_VOLUME:i32=7; pub const CTL_BALANCE:i32=8; pub const CTL_PAN:i32=10; pub const CTL_EXPRESSION:i32=11; pub const CTL_DAMPER_PEDAL:i32=0x40; pub const CTL_SUSTAIN:i32=0x40; pub const CTL_HOLD:i32=0x40; pub const CTL_PORTAMENTO:i32=0x41; pub const CTL_SOSTENUTO:i32=0x42; pub const CTL_SOFT_PEDAL:i32=0x43; pub const CTL_HOLD2:i32=0x45; pub const CTL_GENERAL_PURPOSE1:i32=0x10; pub const CTL_GENERAL_PURPOSE2:i32=0x11; pub const CTL_GENERAL_PURPOSE3:i32=0x12; pub const CTL_GENERAL_PURPOSE4:i32=0x13; pub const CTL_GENERAL_PURPOSE5:i32=0x50; pub const CTL_GENERAL_PURPOSE6:i32=0x51; pub const CTL_GENERAL_PURPOSE7:i32=0x52; pub const CTL_GENERAL_PURPOSE8:i32=0x53; pub const CTL_EXT_EFF_DEPTH:i32=0x5b; pub const CTL_TREMOLO_DEPTH:i32=0x5c; pub const CTL_CHORUS_DEPTH:i32=0x5d; pub const CTL_DETUNE_DEPTH:i32=0x5e; pub const CTL_CELESTE_DEPTH:i32=0x5e; pub const CTL_PHASER_DEPTH:i32=0x5f; pub const CTL_DATA_INCREMENT:i32=0x60; pub const CTL_DATA_DECREMENT:i32=0x61; pub const CTL_NONREG_PARM_NUM_LSB:i32=0x62; pub const CTL_NONREG_PARM_NUM_MSB:i32=0x63; pub const CTL_REGIST_PARM_NUM_LSB:i32=0x64; pub const CTL_REGIST_PARM_NUM_MSB:i32=0x65;

extern "C" { pub fn seqbuf_dump(); }

pub const WAVE_16_BITS:u32=1; pub const WAVE_UNSIGNED:u32=2; pub const WAVE_LOOPING:u32=4; pub const WAVE_BIDIR_LOOP:u32=8; pub const WAVE_LOOP_BACK:u32=0x10; pub const WAVE_SUSTAIN_ON:u32=0x20; pub const WAVE_ENVELOPES:u32=0x40; pub const WAVE_FAST_RELEASE:u32=0x80; pub const WAVE_VIBRATO:u32=0x10000; pub const WAVE_TREMOLO:u32=0x20000; pub const WAVE_SCALE:u32=0x40000; pub const WAVE_FRACTIONS:u32=0x80000; pub const WAVE_ROM:u32=0x40000000; pub const WAVE_MULAW:u32=0x20000000;
pub const SYNTH_TYPE_FM:i32=0; pub const SYNTH_TYPE_SAMPLE:i32=1; pub const SYNTH_TYPE_MIDI:i32=2; pub const FM_TYPE_ADLIB:i32=0; pub const FM_TYPE_OPL3:i32=1; pub const MIDI_TYPE_MPU401:i32=0x401; pub const SAMPLE_TYPE_BASIC:i32=0x10; pub const SAMPLE_TYPE_GUS:i32=0x10; pub const SAMPLE_TYPE_WAVEFRONT:i32=0x11; pub const SYNTH_CAP_PERCMODE:u32=1; pub const SYNTH_CAP_OPL3:u32=2; pub const SYNTH_CAP_INPUT:u32=4; pub const MIDI_CAP_MPU401:i32=1;
pub const DSP_BIND_QUERY:i32=0; pub const DSP_BIND_FRONT:i32=1; pub const DSP_BIND_SURR:i32=2; pub const DSP_BIND_CENTER_LFE:i32=4; pub const DSP_BIND_HANDSET:i32=8; pub const DSP_BIND_MIC:i32=0x10; pub const DSP_BIND_MODEM1:i32=0x20; pub const DSP_BIND_MODEM2:i32=0x40; pub const DSP_BIND_I2S:i32=0x80; pub const DSP_BIND_SPDIF:i32=0x100;
pub const SPDIF_PRO:i32=1; pub const SPDIF_N_AUD:i32=2; pub const SPDIF_COPY:i32=4; pub const SPDIF_PRE:i32=8; pub const SPDIF_CC:i32=0x7f0; pub const SPDIF_L:i32=0x800; pub const SPDIF_DRS:i32=0x4000; pub const SPDIF_V:i32=0x8000;
pub const FM_PATCH:u16=1; pub const OPL3_PATCH:u16=3; pub const WAVE_PATCH:u16=4; pub const GUS_PATCH:u16=4; pub const WAVEFRONT_PATCH:u16=6; pub const SYSEX_PATCH:i16=5; pub const MAUI_PATCH:i16=6;

macro_rules! ioctl0 { ($n:ident,$c:expr,$v:expr) => { pub const $n:u32=_SIO($c,$v); }; }
macro_rules! ioctlr { ($n:ident,$c:expr,$v:expr,$t:ty) => { pub const $n:u32=_SIOR::<$t>($c,$v); }; }
macro_rules! ioctlw { ($n:ident,$c:expr,$v:expr,$t:ty) => { pub const $n:u32=_SIOW::<$t>($c,$v); }; }
macro_rules! ioctlrw { ($n:ident,$c:expr,$v:expr,$t:ty) => { pub const $n:u32=_SIOWR::<$t>($c,$v); }; }
ioctl0!(SNDCTL_SEQ_RESET,b'Q' as u32,0); ioctl0!(SNDCTL_SEQ_SYNC,b'Q' as u32,1); ioctlrw!(SNDCTL_SEQ_CTRLRATE,b'Q' as u32,3,i32); ioctlr!(SNDCTL_SEQ_GETOUTCOUNT,b'Q' as u32,4,i32); ioctlr!(SNDCTL_SEQ_GETINCOUNT,b'Q' as u32,5,i32); ioctlw!(SNDCTL_SEQ_PERCMODE,b'Q' as u32,6,i32); ioctlw!(SNDCTL_SEQ_TESTMIDI,b'Q' as u32,8,i32); ioctlw!(SNDCTL_SEQ_RESETSAMPLES,b'Q' as u32,9,i32); ioctlr!(SNDCTL_SEQ_NRSYNTHS,b'Q' as u32,10,i32); ioctlr!(SNDCTL_SEQ_NRMIDIS,b'Q' as u32,11,i32);
ioctl0!(SNDCTL_TMR_START,b'T' as u32,2); ioctl0!(SNDCTL_TMR_STOP,b'T' as u32,3); ioctl0!(SNDCTL_TMR_CONTINUE,b'T' as u32,4); ioctlrw!(SNDCTL_TMR_TIMEBASE,b'T' as u32,1,i32); ioctlrw!(SNDCTL_TMR_TEMPO,b'T' as u32,5,i32); ioctlrw!(SNDCTL_TMR_SOURCE,b'T' as u32,6,i32); ioctlw!(SNDCTL_TMR_METRONOME,b'T' as u32,7,i32); ioctlw!(SNDCTL_TMR_SELECT,b'T' as u32,8,i32);
ioctl0!(SNDCTL_DSP_RESET,b'P' as u32,0); ioctl0!(SNDCTL_DSP_SYNC,b'P' as u32,1); ioctlrw!(SNDCTL_DSP_SPEED,b'P' as u32,2,i32); ioctlrw!(SNDCTL_DSP_STEREO,b'P' as u32,3,i32); ioctlrw!(SNDCTL_DSP_GETBLKSIZE,b'P' as u32,4,i32); ioctlrw!(SNDCTL_DSP_SETFMT,b'P' as u32,5,i32); pub const SNDCTL_DSP_SAMPLESIZE:u32=SNDCTL_DSP_SETFMT; ioctlrw!(SNDCTL_DSP_CHANNELS,b'P' as u32,6,i32); pub const SOUND_PCM_WRITE_CHANNELS:u32=SNDCTL_DSP_CHANNELS; ioctlw!(SOUND_PCM_WRITE_FILTER,b'P' as u32,7,i32); ioctl0!(SNDCTL_DSP_POST,b'P' as u32,8); ioctlrw!(SNDCTL_DSP_SUBDIVIDE,b'P' as u32,9,i32); ioctlrw!(SNDCTL_DSP_SETFRAGMENT,b'P' as u32,10,i32); ioctlr!(SNDCTL_DSP_GETFMTS,b'P' as u32,11,i32); ioctlr!(SNDCTL_DSP_GETOSPACE,b'P' as u32,12,audio_buf_info); ioctlr!(SNDCTL_DSP_GETISPACE,b'P' as u32,13,audio_buf_info); ioctl0!(SNDCTL_DSP_NONBLOCK,b'P' as u32,14); ioctlr!(SNDCTL_DSP_GETCAPS,b'P' as u32,15,i32); ioctlr!(SNDCTL_DSP_GETTRIGGER,b'P' as u32,16,i32); ioctlw!(SNDCTL_DSP_SETTRIGGER,b'P' as u32,16,i32); ioctlr!(SNDCTL_DSP_GETIPTR,b'P' as u32,17,count_info); ioctlr!(SNDCTL_DSP_GETOPTR,b'P' as u32,18,count_info); ioctlr!(SNDCTL_DSP_MAPINBUF,b'P' as u32,19,buffmem_desc); ioctlr!(SNDCTL_DSP_MAPOUTBUF,b'P' as u32,20,buffmem_desc); ioctl0!(SNDCTL_DSP_SETSYNCRO,b'P' as u32,21); ioctl0!(SNDCTL_DSP_SETDUPLEX,b'P' as u32,22); ioctlr!(SNDCTL_DSP_GETODELAY,b'P' as u32,23,i32);

macro_rules! seqbuf_event { ($name:ident,$body:block) => { #[macro_export] macro_rules! $name { ($($arg:tt)*) => { $body }; } }; }
/* The original convenience macros operate on caller-provided _seqbuf/_seqbufptr symbols. */
#[macro_export] macro_rules! SEQ_MIDIOUT { ($device:expr,$byte:expr) => {{ _seqbuf[_seqbufptr]=SEQ_MIDIPUTC as u8; _seqbuf[_seqbufptr+1]=$byte; _seqbuf[_seqbufptr+2]=$device; _seqbuf[_seqbufptr+3]=0; _seqbufptr+=4; }}; }
#[macro_export] macro_rules! SEQ_START_NOTE { ($dev:expr,$chn:expr,$note:expr,$vol:expr) => {{ _seqbuf[_seqbufptr]=EV_CHN_VOICE; _seqbuf[_seqbufptr+1]=$dev; _seqbuf[_seqbufptr+2]=MIDI_NOTEON; _seqbuf[_seqbufptr+3]=$chn; _seqbuf[_seqbufptr+4]=$note; _seqbuf[_seqbufptr+5]=$vol; _seqbuf[_seqbufptr+6]=0; _seqbuf[_seqbufptr+7]=0; _seqbufptr+=8; }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
