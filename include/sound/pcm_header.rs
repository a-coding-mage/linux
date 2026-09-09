/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of sound/pcm.h.  Types supplied by the surrounding kernel
 * translation unit are intentionally referenced but not redefined here. */

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32, pub formats: u64, pub subformats: u32, pub rates: u32,
    pub rate_min: u32, pub rate_max: u32, pub channels_min: u32, pub channels_max: u32,
    pub buffer_bytes_max: usize, pub period_bytes_min: usize, pub period_bytes_max: usize,
    pub periods_min: u32, pub periods_max: u32, pub fifo_size: usize,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub ioctl: Option<unsafe extern "C" fn(*mut snd_pcm_substream, u32, *mut core::ffi::c_void) -> i32>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> i32>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, i32) -> i32>,
    pub sync_stop: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub get_time_info: Option<unsafe extern "C" fn(*mut snd_pcm_substream,*mut timespec64,*mut timespec64,*mut snd_pcm_audio_tstamp_config,*mut snd_pcm_audio_tstamp_report)->i32>,
    pub fill_silence: Option<unsafe extern "C" fn(*mut snd_pcm_substream,i32,usize,usize)->i32>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_pcm_substream,i32,usize,*mut iov_iter,usize)->i32>,
    pub page: Option<unsafe extern "C" fn(*mut snd_pcm_substream,usize)->*mut page>,
    pub mmap: Option<unsafe extern "C" fn(*mut snd_pcm_substream,*mut vm_area_struct)->i32>,
    pub ack: Option<unsafe extern "C" fn(*mut snd_pcm_substream)->i32>,
}

pub const SNDRV_PCM_DEVICES: u32 = 8; // CONFIG_SND_DYNAMIC_MINORS selects SNDRV_OS_MINORS - 2.
pub const SNDRV_PCM_IOCTL1_RESET: u32=0; pub const SNDRV_PCM_IOCTL1_CHANNEL_INFO:u32=2;
pub const SNDRV_PCM_IOCTL1_FIFO_SIZE:u32=4; pub const SNDRV_PCM_IOCTL1_SYNC_ID:u32=5;
pub const SNDRV_PCM_TRIGGER_STOP:u32=0; pub const SNDRV_PCM_TRIGGER_START:u32=1;
pub const SNDRV_PCM_TRIGGER_PAUSE_PUSH:u32=2; pub const SNDRV_PCM_TRIGGER_PAUSE_RELEASE:u32=3;
pub const SNDRV_PCM_TRIGGER_SUSPEND:u32=4; pub const SNDRV_PCM_TRIGGER_RESUME:u32=5;
pub const SNDRV_PCM_TRIGGER_DRAIN:u32=6;

pub const SNDRV_PCM_RATE_5512:u32=1<<0; pub const SNDRV_PCM_RATE_8000:u32=1<<1;
pub const SNDRV_PCM_RATE_11025:u32=1<<2; pub const SNDRV_PCM_RATE_16000:u32=1<<3;
pub const SNDRV_PCM_RATE_22050:u32=1<<4; pub const SNDRV_PCM_RATE_32000:u32=1<<5;
pub const SNDRV_PCM_RATE_44100:u32=1<<6; pub const SNDRV_PCM_RATE_48000:u32=1<<7;
pub const SNDRV_PCM_RATE_64000:u32=1<<8; pub const SNDRV_PCM_RATE_88200:u32=1<<9;
pub const SNDRV_PCM_RATE_96000:u32=1<<10; pub const SNDRV_PCM_RATE_176400:u32=1<<11;
pub const SNDRV_PCM_RATE_192000:u32=1<<12; pub const SNDRV_PCM_RATE_352800:u32=1<<13;
pub const SNDRV_PCM_RATE_384000:u32=1<<14; pub const SNDRV_PCM_RATE_705600:u32=1<<15;
pub const SNDRV_PCM_RATE_768000:u32=1<<16; pub const SNDRV_PCM_RATE_12000:u32=1<<17;
pub const SNDRV_PCM_RATE_24000:u32=1<<18; pub const SNDRV_PCM_RATE_128000:u32=1<<19;
pub const SNDRV_PCM_RATE_CONTINUOUS:u32=1<<30; pub const SNDRV_PCM_RATE_KNOT:u32=1<<31;
pub const SNDRV_PCM_RATE_8000_44100:u32=SNDRV_PCM_RATE_8000|SNDRV_PCM_RATE_11025|SNDRV_PCM_RATE_16000|SNDRV_PCM_RATE_22050|SNDRV_PCM_RATE_32000|SNDRV_PCM_RATE_44100;
pub const SNDRV_PCM_RATE_8000_48000:u32=SNDRV_PCM_RATE_8000_44100|SNDRV_PCM_RATE_48000;
pub const SNDRV_PCM_RATE_8000_96000:u32=SNDRV_PCM_RATE_8000_48000|SNDRV_PCM_RATE_64000|SNDRV_PCM_RATE_88200|SNDRV_PCM_RATE_96000;
pub const SNDRV_PCM_RATE_8000_192000:u32=SNDRV_PCM_RATE_8000_96000|SNDRV_PCM_RATE_176400|SNDRV_PCM_RATE_192000;
pub const SNDRV_PCM_RATE_8000_384000:u32=SNDRV_PCM_RATE_8000_192000|SNDRV_PCM_RATE_352800|SNDRV_PCM_RATE_384000;
pub const SNDRV_PCM_RATE_8000_768000:u32=SNDRV_PCM_RATE_8000_384000|SNDRV_PCM_RATE_705600|SNDRV_PCM_RATE_768000;

#[repr(C)] pub struct snd_pcm_audio_tstamp_config { pub type_requested:u32, pub report_delay:u32 }
#[repr(C)] pub struct snd_pcm_audio_tstamp_report { pub valid:u32, pub actual_type:u32, pub accuracy_report:u32, pub accuracy:u32 }
#[inline] pub unsafe fn snd_pcm_unpack_audio_tstamp_config(data:u32,c:*mut snd_pcm_audio_tstamp_config){(*c).type_requested=data&0xf;(*c).report_delay=(data>>4)&1;}
#[inline] pub unsafe fn snd_pcm_pack_audio_tstamp_report(data:*mut u32,accuracy:*mut u32,r:*const snd_pcm_audio_tstamp_report){let mut t=(*r).accuracy_report;t=(t<<4)|(*r).actual_type;t=(t<<1)|(*r).valid;*data=(*data&0xffff)|(t<<16);*accuracy=(*r).accuracy;}

#[repr(C)] pub struct snd_pcm_file { pub substream:*mut snd_pcm_substream, pub no_compat_mmap:i32, pub user_pversion:u32 }
#[repr(C)] pub struct snd_ratnum {pub num:u32,pub den_min:u32,pub den_max:u32,pub den_step:u32}
#[repr(C)] pub struct snd_ratden {pub num_min:u32,pub num_max:u32,pub num_step:u32,pub den:u32}
#[repr(C)] pub struct snd_pcm_hw_constraint_ratnums {pub nrats:i32,pub rats:*const snd_ratnum}
#[repr(C)] pub struct snd_pcm_hw_constraint_ratdens {pub nrats:i32,pub rats:*const snd_ratden}
#[repr(C)] pub struct snd_pcm_hw_constraint_list {pub list:*const u32,pub count:u32,pub mask:u32}
#[repr(C)] pub struct snd_pcm_hw_constraint_ranges {pub count:u32,pub ranges:*const snd_interval,pub mask:u32}

/* The remainder consists of C ABI declarations and inline helpers whose
 * dependent kernel layouts are supplied by neighboring translated headers. */
extern "C" {
    pub fn snd_pcm_new(card:*mut snd_card,id:*const i8,device:i32,playback_count:i32,capture_count:i32,rpcm:*mut *mut snd_pcm)->i32;
    pub fn snd_pcm_new_internal(card:*mut snd_card,id:*const i8,device:i32,playback_count:i32,capture_count:i32,rpcm:*mut *mut snd_pcm)->i32;
    pub fn snd_pcm_new_stream(pcm:*mut snd_pcm,stream:i32,substream_count:i32)->i32;
    pub fn snd_pcm_start(s:*mut snd_pcm_substream)->i32;
    pub fn snd_pcm_stop(s:*mut snd_pcm_substream,status:snd_pcm_state_t)->i32;
    pub fn snd_pcm_set_state(s:*mut snd_pcm_substream,state:snd_pcm_state_t);
    pub fn snd_pcm_get_state(s:*mut snd_pcm_substream)->snd_pcm_state_t;
    pub fn snd_pcm_hw_refine(s:*mut snd_pcm_substream,p:*mut snd_pcm_hw_params)->i32;
    pub fn snd_pcm_format_signed(f:snd_pcm_format_t)->i32;
    pub fn snd_pcm_format_unsigned(f:snd_pcm_format_t)->i32;
    pub fn snd_pcm_format_width(f:snd_pcm_format_t)->i32;
    pub fn snd_pcm_format_physical_width(f:snd_pcm_format_t)->i32;
    pub fn snd_pcm_format_size(f:snd_pcm_format_t,samples:usize)->isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
