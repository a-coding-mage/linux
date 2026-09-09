/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of sound/wavefront.h. */

/* NUM_MIDIKEYS and NUM_MIDICHANNELS may be supplied by the surrounding build. */
pub const NUM_MIDIKEYS: usize = 128;
pub const NUM_MIDICHANNELS: usize = 16;

macro_rules! cconst { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: i32 = $v;)* }; }
cconst! {
 WFC_DEBUG_DRIVER=0,WFC_FX_IOCTL=1,WFC_PATCH_STATUS=2,WFC_PROGRAM_STATUS=3,WFC_SAMPLE_STATUS=4,
 WFC_DISABLE_INTERRUPTS=5,WFC_ENABLE_INTERRUPTS=6,WFC_INTERRUPT_STATUS=7,WFC_ROMSAMPLES_RDONLY=8,WFC_IDENTIFY_SLOT_TYPE=9,
 WFC_DOWNLOAD_SAMPLE=0x80,WFC_DOWNLOAD_BLOCK=0x81,WFC_DOWNLOAD_MULTISAMPLE=0x82,WFC_DOWNLOAD_SAMPLE_ALIAS=0x83,
 WFC_DELETE_SAMPLE=0x84,WFC_REPORT_FREE_MEMORY=0x85,WFC_DOWNLOAD_PATCH=0x86,WFC_DOWNLOAD_PROGRAM=0x87,
 WFC_SET_SYNTHVOL=0x89,WFC_SET_NVOICES=0x8b,WFC_DOWNLOAD_DRUM=0x90,WFC_GET_SYNTHVOL=0x92,WFC_GET_NVOICES=0x94,
 WFC_DISABLE_CHANNEL=0x9a,WFC_ENABLE_CHANNEL=0x9b,WFC_MISYNTH_OFF=0x9d,WFC_MISYNTH_ON=0x9e,WFC_FIRMWARE_VERSION=0x9f,
 WFC_GET_NSAMPLES=0xa0,WFC_DISABLE_DRUM_PROGRAM=0xa2,WFC_UPLOAD_PATCH=0xa3,WFC_UPLOAD_PROGRAM=0xa4,WFC_SET_TUNING=0xa6,
 WFC_GET_TUNING=0xa7,WFC_VMIDI_ON=0xa8,WFC_VMIDI_OFF=0xa9,WFC_MIDI_STATUS=0xaa,WFC_GET_CHANNEL_STATUS=0xab,
 WFC_DOWNLOAD_SAMPLE_HEADER=0xac,WFC_UPLOAD_SAMPLE_HEADER=0xad,WFC_UPLOAD_MULTISAMPLE=0xae,WFC_UPLOAD_SAMPLE_ALIAS=0xaf,
 WFC_IDENTIFY_SAMPLE_TYPE=0xb0,WFC_DOWNLOAD_EDRUM_PROGRAM=0xb1,WFC_UPLOAD_EDRUM_PROGRAM=0xb2,WFC_SET_EDRUM_CHANNEL=0xb3,
 WFC_INSTOUT_LEVELS=0xb4,WFC_PEAKOUT_LEVELS=0xb5,WFC_REPORT_CHANNEL_PROGRAMS=0xb6,WFC_HARDWARE_VERSION=0xcf,
 WFC_UPLOAD_SAMPLE_PARAMS=0xd7,WFC_DOWNLOAD_OS=0xf1,WFC_NOOP=0xff,
 WF_MAX_SAMPLE=512,WF_MAX_PATCH=256,WF_MAX_PROGRAM=128,WF_SECTION_MAX=44,WF_PROGRAM_BYTES=32,WF_PATCH_BYTES=132,
 WF_SAMPLE_BYTES=27,WF_SAMPLE_HDR_BYTES=25,WF_ALIAS_BYTES=25,WF_DRUM_BYTES=9,WF_MSAMPLE_BYTES=259,WF_ACK=0x80,WF_DMA_ACK=0x81,
 WF_MIDI_VIRTUAL_ENABLED=1,WF_MIDI_VIRTUAL_IS_EXTERNAL=2,WF_MIDI_IN_TO_SYNTH_DISABLED=4,
 WF_SYNTH_SLOT=0,WF_INTERNAL_MIDI_SLOT=1,WF_EXTERNAL_MIDI_SLOT=2,WF_EXTERNAL_SWITCH=0xfd,WF_INTERNAL_SWITCH=0xf9,
 WF_DEBUG_CMD=1,WF_DEBUG_DATA=2,WF_DEBUG_LOAD_PATCH=4,WF_DEBUG_IO=8,WF_WAVEPATCH_VERSION=120,WF_MAX_COMMENT=64,
 WF_NUM_LAYERS=4,WF_NAME_LENGTH=32,WF_SOURCE_LENGTH=260,
 WF_ST_SAMPLE=0,WF_ST_MULTISAMPLE=1,WF_ST_ALIAS=2,WF_ST_EMPTY=3,WF_ST_DRUM=4,WF_ST_PROGRAM=5,WF_ST_PATCH=6,WF_ST_SAMPLEHDR=7,WF_ST_MASK=0xf,
 WF_SLOT_USED=0x80,WF_SLOT_FILLED=0x40,WF_SLOT_ROM=0x20,WF_SLOT_MASK=0xf0,WF_CH_MONO=0,WF_CH_LEFT=1,WF_CH_RIGHT=2,
 LINEAR_16BIT=0,WHITE_NOISE=1,LINEAR_8BIT=2,MULAW_8BIT=3
}
pub const WF_WAVEPATCH_VERSION_: i32 = 120; // C macro contains a trailing semicolon.
pub const BankFileID: &str = "Bank"; pub const DrumkitFileID: &str = "DrumKit"; pub const ProgramFileID: &str = "Program";

#[repr(C)] pub struct wf_envelope { pub attack_time:u8,pub Unused1:u8,pub decay1_time:u8,pub Unused2:u8,pub decay2_time:u8,pub Unused3:u8,pub sustain_time:u8,pub Unused4:u8,pub release_time:u8,pub Unused5:u8,pub release2_time:u8,pub Unused6:u8,pub attack_level:i8,pub decay1_level:i8,pub decay2_level:i8,pub sustain_level:i8,pub release_level:i8,pub attack_velocity:u8,pub Unused7:u8,pub volume_velocity:u8,pub Unused8:u8,pub keyboard_scaling:u8,pub Unused9:u8 }
pub type wavefront_envelope = wf_envelope;
#[repr(C)] pub struct wf_lfo { pub sample_number:u8,pub frequency:u8,pub Unused1:u8,pub am_src:u8,pub fm_src:u8,pub fm_amount:i8,pub am_amount:i8,pub start_level:i8,pub end_level:i8,pub ramp_delay:u8,pub wave_restart:u8,pub ramp_time:u8,pub Unused2:u8 }
pub type wavefront_lfo = wf_lfo;
#[repr(C)] pub struct wf_patch { pub frequency_bias:i16,pub amplitude_bias:u8,pub Unused1:u8,pub portamento:u8,pub Unused2:u8,pub sample_number:u8,pub pitch_bend:u8,pub sample_msb:u8,pub Unused3:u8,pub mono:u8,pub retrigger:u8,pub nohold:u8,pub restart:u8,pub filterconfig:u8,pub reuse:u8,pub reset_lfo:u8,pub fm_src2:u8,pub fm_src1:u8,pub fm_amount1:i8,pub fm_amount2:i8,pub am_src:u8,pub Unused4:u8,pub am_amount:i8,pub fc1_mode:u8,pub fc2_mode:u8,pub fc1_mod_amount:i8,pub fc1_keyboard_scaling:i8,pub fc1_bias:i8,pub fc2_mod_amount:i8,pub fc2_keyboard_scaling:i8,pub fc2_bias:i8,pub randomizer:u8,pub Unused5:u8,pub envelope1:wf_envelope,pub envelope2:wf_envelope,pub lfo1:wf_lfo,pub lfo2:wf_lfo }
pub type wavefront_patch = wf_patch;
#[repr(C)] pub struct wf_layer { pub patch_number:u8,pub mix_level:u8,pub mute:u8,pub split_point:u8,pub play_below:u8,pub pan_mod_src:u8,pub pan_or_mod:u8,pub pan:u8,pub split_type:u8 }
pub type wavefront_layer = wf_layer;
#[repr(C)] pub struct wf_program { pub layer:[wf_layer;4] }
pub type wavefront_program = wf_program;
#[repr(C)] pub struct wf_sample_offset { pub Fraction:i32,pub Integer:i32,pub Unused:i32 }
pub type wavefront_sample_offset = wf_sample_offset;
#[repr(C)] pub struct wf_sample { pub sampleStartOffset:wf_sample_offset,pub loopStartOffset:wf_sample_offset,pub loopEndOffset:wf_sample_offset,pub sampleEndOffset:wf_sample_offset,pub FrequencyBias:i16,pub SampleResolution:u8,pub Unused1:u8,pub Loop:u8,pub Bidirectional:u8,pub Unused2:u8,pub Reverse:u8,pub Unused3:u8 }
pub type wavefront_sample = wf_sample;
#[repr(C)] pub struct wf_multisample { pub NumberOfSamples:i16,pub SampleNumber:[i16;NUM_MIDIKEYS] }
pub type wavefront_multisample = wf_multisample;
#[repr(C,packed)] pub struct wf_alias { pub OriginalSample:i16,pub sampleStartOffset:wf_sample_offset,pub loopStartOffset:wf_sample_offset,pub sampleEndOffset:wf_sample_offset,pub loopEndOffset:wf_sample_offset,pub FrequencyBias:i16,pub SampleResolution:u8,pub Unused1:u8,pub Loop:u8,pub Bidirectional:u8,pub Unused2:u8,pub Reverse:u8,pub Unused3:u8,pub sixteen_bit_padding:u8 }
pub type wavefront_alias = wf_alias;
#[repr(C)] pub struct wf_drum { pub PatchNumber:u8,pub MixLevel:u8,pub Unmute:u8,pub Group:u8,pub Unused1:u8,pub PanModSource:u8,pub PanModulated:u8,pub PanAmount:u8,pub Unused2:u8 }
pub type wavefront_drum = wf_drum;
#[repr(C)] pub struct wf_drumkit { pub drum:[wf_drum;NUM_MIDIKEYS] }
pub type wavefront_drumkit = wf_drumkit;
#[repr(C)] pub struct wf_channel_programs { pub Program:[u8;NUM_MIDICHANNELS] }
pub type wavefront_channel_programs = wf_channel_programs;
#[repr(C)] pub union wf_any { pub s:wf_sample,pub ms:wf_multisample,pub a:wf_alias,pub pr:wf_program,pub p:wf_patch,pub d:wf_drum }
pub type wavefront_any = wf_any;
#[repr(C)] pub struct wf_patch_info { pub key:i16,pub devno:u16,pub subkey:u8,pub number:u16,pub size:u32,pub hdrptr:*mut wf_any,pub dataptr:*mut u16,pub hdr:wf_any }
pub const WAVEFRONT_FIND_FREE_SAMPLE_SLOT:i32=999;
#[repr(C)] pub struct wavefront_control { pub cmd:i32,pub status:i8,pub rbuf:[u8;std::mem::size_of::<wf_multisample>()],pub wbuf:[u8;std::mem::size_of::<wf_multisample>()] }
pub const WFCTL_WFCMD:i32=1; pub const WFCTL_LOAD_SPP:i32=2;
pub const WF_MAX_READ:usize=std::mem::size_of::<wf_multisample>(); pub const WF_MAX_WRITE:usize=std::mem::size_of::<wf_multisample>();
#[repr(C)] pub struct wf_fx_info { pub request:i32,pub data:[isize;4] } pub type wavefront_fx_info=wf_fx_info;
pub const WF_CHANNEL_STATUS: fn(usize,&[u8])->u8 = |ch,wcp| wcp[ch/7] & (1 << (ch%7));
pub const WF_SAMPLE_IS_8BIT: fn(&wf_sample)->u8 = |smpl| smpl.SampleResolution & 2;

pub const WF_MOD_LFO1:i32=0; pub const WF_MOD_LFO2:i32=1; pub const WF_MOD_ENV1:i32=2; pub const WF_MOD_ENV2:i32=3;
pub const WF_MOD_KEYBOARD:i32=4; pub const WF_MOD_LOGKEY:i32=5; pub const WF_MOD_VELOCITY:i32=6; pub const WF_MOD_LOGVEL:i32=7;
pub const WF_MOD_RANDOM:i32=8; pub const WF_MOD_PRESSURE:i32=9; pub const WF_MOD_MOD_WHEEL:i32=10; pub const WF_MOD_1:i32=10;
pub const WF_MOD_BREATH:i32=11; pub const WF_MOD_2:i32=11; pub const WF_MOD_FOOT:i32=12; pub const WF_MOD_4:i32=12;
pub const WF_MOD_VOLUME:i32=13; pub const WF_MOD_7:i32=13; pub const WF_MOD_PAN:i32=14; pub const WF_MOD_10:i32=14;
pub const WF_MOD_EXPR:i32=15; pub const WF_MOD_11:i32=15;

/* FX request numbers. */
macro_rules! fxconst { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n:i32=$v;)* }; }
fxconst! { WFFX_SETOUTGAIN=0,WFFX_SETSTEREOOUTGAIN=1,WFFX_SETREVERBIN1GAIN=2,WFFX_SETREVERBIN2GAIN=3,WFFX_SETREVERBIN3GAIN=4,
 WFFX_SETCHORUSINPORT=5,WFFX_SETREVERBIN1PORT=6,WFFX_SETREVERBIN2PORT=7,WFFX_SETREVERBIN3PORT=8,WFFX_SETEFFECTPORT=9,
 WFFX_SETAUXPORT=10,WFFX_SETREVERBTYPE=11,WFFX_SETREVERBDELAY=12,WFFX_SETCHORUSLFO=13,WFFX_SETCHORUSPMD=14,WFFX_SETCHORUSAMD=15,
 WFFX_SETEFFECT=16,WFFX_SETBASEALL=17,WFFX_SETREVERBALL=18,WFFX_SETCHORUSALL=20,WFFX_SETREVERBDEF=22,WFFX_SETCHORUSDEF=23,
 WFFX_DELAYSETINGAIN=24,WFFX_DELAYSETFBGAIN=25,WFFX_DELAYSETFBLPF=26,WFFX_DELAYSETGAIN=27,WFFX_DELAYSETTIME=28,WFFX_DELAYSETFBTIME=29,WFFX_DELAYSETALL=30,WFFX_DELAYSETDEF=32,
 WFFX_SDELAYSETINGAIN=33,WFFX_SDELAYSETFBGAIN=34,WFFX_SDELAYSETFBLPF=35,WFFX_SDELAYSETGAIN=36,WFFX_SDELAYSETTIME=37,WFFX_SDELAYSETFBTIME=38,WFFX_SDELAYSETALL=39,WFFX_SDELAYSETDEF=41,
 WFFX_DEQSETINGAIN=42,WFFX_DEQSETFILTER=43,WFFX_DEQSETALL=44,WFFX_DEQSETDEF=46,WFFX_MUTE=47,WFFX_FLANGESETBALANCE=48,WFFX_FLANGESETDELAY=49,WFFX_FLANGESETDWFFX_TH=50,
 WFFX_FLANGESETFBGAIN=51,WFFX_FLANGESETINGAIN=52,WFFX_FLANGESETLFO=53,WFFX_FLANGESETALL=54,WFFX_FLANGESETDEF=56,WFFX_PITCHSETSHIFT=57,WFFX_PITCHSETBALANCE=58,WFFX_PITCHSETALL=59,WFFX_PITCHSETDEF=61,
 WFFX_SRSSETINGAIN=62,WFFX_SRSSETSPACE=63,WFFX_SRSSETCENTER=64,WFFX_SRSSETGAIN=65,WFFX_SRSSETMODE=66,WFFX_SRSSETDEF=68,WFFX_MEMSET=69 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
