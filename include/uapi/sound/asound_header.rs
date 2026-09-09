/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Rust translation of the UAPI ALSA sound header. */

pub const fn sndrv_protocol_version(major: u32, minor: u32, subminor: u32) -> u32 { (major << 16) | (minor << 8) | subminor }
pub const fn sndrv_protocol_major(version: u32) -> u32 { (version >> 16) & 0xffff }
pub const fn sndrv_protocol_minor(version: u32) -> u32 { (version >> 8) & 0xff }
pub const fn sndrv_protocol_micro(version: u32) -> u32 { version & 0xff }
pub const AES_IEC958_STATUS_SIZE: usize = 24;

#[repr(C)] pub struct snd_aes_iec958 { pub status: [u8;24], pub subcode: [u8;147], pub pad: u8, pub dig_subframe: [u8;4] }
#[repr(C)] pub struct snd_cea_861_aud_if { pub db1_ct_cc:u8, pub db2_sf_ss:u8, pub db3:u8, pub db4_ca:u8, pub db5_dminh_lsv:u8 }

pub const SNDRV_HWDEP_VERSION:u32=sndrv_protocol_version(1,0,1);
pub const SNDRV_HWDEP_IFACE_OPL2:i32=0; pub const SNDRV_HWDEP_IFACE_OPL3:i32=1; pub const SNDRV_HWDEP_IFACE_OPL4:i32=2;
pub const SNDRV_HWDEP_IFACE_SB16CSP:i32=3; pub const SNDRV_HWDEP_IFACE_EMU10K1:i32=4; pub const SNDRV_HWDEP_IFACE_YSS225:i32=5;
pub const SNDRV_HWDEP_IFACE_ICS2115:i32=6; pub const SNDRV_HWDEP_IFACE_SSCAPE:i32=7; pub const SNDRV_HWDEP_IFACE_VX:i32=8;
pub const SNDRV_HWDEP_IFACE_MIXART:i32=9; pub const SNDRV_HWDEP_IFACE_USX2Y:i32=10; pub const SNDRV_HWDEP_IFACE_EMUX_WAVETABLE:i32=11;
pub const SNDRV_HWDEP_IFACE_BLUETOOTH:i32=12; pub const SNDRV_HWDEP_IFACE_USX2Y_PCM:i32=13; pub const SNDRV_HWDEP_IFACE_PCXHR:i32=14;
pub const SNDRV_HWDEP_IFACE_SB_RC:i32=15; pub const SNDRV_HWDEP_IFACE_HDA:i32=16; pub const SNDRV_HWDEP_IFACE_USB_STREAM:i32=17;
pub const SNDRV_HWDEP_IFACE_FW_DICE:i32=18; pub const SNDRV_HWDEP_IFACE_FW_FIREWORKS:i32=19; pub const SNDRV_HWDEP_IFACE_FW_BEBOB:i32=20;
pub const SNDRV_HWDEP_IFACE_FW_OXFW:i32=21; pub const SNDRV_HWDEP_IFACE_FW_DIGI00X:i32=22; pub const SNDRV_HWDEP_IFACE_FW_TASCAM:i32=23;
pub const SNDRV_HWDEP_IFACE_LINE6:i32=24; pub const SNDRV_HWDEP_IFACE_FW_MOTU:i32=25; pub const SNDRV_HWDEP_IFACE_FW_FIREFACE:i32=26;
pub const SNDRV_HWDEP_IFACE_LAST:i32=26;
#[repr(C)] pub struct snd_hwdep_info { pub device:u32,pub card:i32,pub id:[u8;64],pub name:[u8;80],pub iface:i32,pub reserved:[u8;64] }
#[repr(C)] pub struct snd_hwdep_dsp_status { pub version:u32,pub id:[u8;32],pub num_dsps:u32,pub dsp_loaded:u32,pub chip_ready:u32,pub reserved:[u8;16] }
#[repr(C)] pub struct snd_hwdep_dsp_image { pub index:u32,pub name:[u8;64],pub image:*mut u8,pub length:usize,pub driver_data:usize }

pub const SNDRV_PCM_VERSION:u32=sndrv_protocol_version(2,0,18);
pub type snd_pcm_uframes_t = usize; pub type snd_pcm_sframes_t = isize;
pub const SNDRV_PCM_STREAM_PLAYBACK:i32=0; pub const SNDRV_PCM_STREAM_CAPTURE:i32=1; pub const SNDRV_PCM_STREAM_LAST:i32=1;
pub type snd_pcm_access_t=i32; pub const SNDRV_PCM_ACCESS_MMAP_INTERLEAVED:i32=0; pub const SNDRV_PCM_ACCESS_MMAP_NONINTERLEAVED:i32=1; pub const SNDRV_PCM_ACCESS_MMAP_COMPLEX:i32=2; pub const SNDRV_PCM_ACCESS_RW_INTERLEAVED:i32=3; pub const SNDRV_PCM_ACCESS_RW_NONINTERLEAVED:i32=4; pub const SNDRV_PCM_ACCESS_LAST:i32=4;
pub type snd_pcm_format_t=i32;
pub const SNDRV_PCM_FORMAT_S8:i32=0; pub const SNDRV_PCM_FORMAT_U8:i32=1; pub const SNDRV_PCM_FORMAT_S16_LE:i32=2; pub const SNDRV_PCM_FORMAT_S16_BE:i32=3; pub const SNDRV_PCM_FORMAT_U16_LE:i32=4; pub const SNDRV_PCM_FORMAT_U16_BE:i32=5; pub const SNDRV_PCM_FORMAT_S24_LE:i32=6; pub const SNDRV_PCM_FORMAT_S24_BE:i32=7; pub const SNDRV_PCM_FORMAT_U24_LE:i32=8; pub const SNDRV_PCM_FORMAT_U24_BE:i32=9; pub const SNDRV_PCM_FORMAT_S32_LE:i32=10; pub const SNDRV_PCM_FORMAT_S32_BE:i32=11; pub const SNDRV_PCM_FORMAT_U32_LE:i32=12; pub const SNDRV_PCM_FORMAT_U32_BE:i32=13; pub const SNDRV_PCM_FORMAT_FLOAT_LE:i32=14; pub const SNDRV_PCM_FORMAT_FLOAT_BE:i32=15; pub const SNDRV_PCM_FORMAT_FLOAT64_LE:i32=16; pub const SNDRV_PCM_FORMAT_FLOAT64_BE:i32=17; pub const SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE:i32=18; pub const SNDRV_PCM_FORMAT_IEC958_SUBFRAME_BE:i32=19; pub const SNDRV_PCM_FORMAT_MU_LAW:i32=20; pub const SNDRV_PCM_FORMAT_A_LAW:i32=21; pub const SNDRV_PCM_FORMAT_IMA_ADPCM:i32=22; pub const SNDRV_PCM_FORMAT_MPEG:i32=23; pub const SNDRV_PCM_FORMAT_GSM:i32=24; pub const SNDRV_PCM_FORMAT_S20_LE:i32=25; pub const SNDRV_PCM_FORMAT_S20_BE:i32=26; pub const SNDRV_PCM_FORMAT_U20_LE:i32=27; pub const SNDRV_PCM_FORMAT_U20_BE:i32=28; pub const SNDRV_PCM_FORMAT_SPECIAL:i32=31; pub const SNDRV_PCM_FORMAT_S24_3LE:i32=32; pub const SNDRV_PCM_FORMAT_S24_3BE:i32=33; pub const SNDRV_PCM_FORMAT_U24_3LE:i32=34; pub const SNDRV_PCM_FORMAT_U24_3BE:i32=35; pub const SNDRV_PCM_FORMAT_S20_3LE:i32=36; pub const SNDRV_PCM_FORMAT_S20_3BE:i32=37; pub const SNDRV_PCM_FORMAT_U20_3LE:i32=38; pub const SNDRV_PCM_FORMAT_U20_3BE:i32=39; pub const SNDRV_PCM_FORMAT_S18_3LE:i32=40; pub const SNDRV_PCM_FORMAT_S18_3BE:i32=41; pub const SNDRV_PCM_FORMAT_U18_3LE:i32=42; pub const SNDRV_PCM_FORMAT_U18_3BE:i32=43; pub const SNDRV_PCM_FORMAT_G723_24:i32=44; pub const SNDRV_PCM_FORMAT_G723_24_1B:i32=45; pub const SNDRV_PCM_FORMAT_G723_40:i32=46; pub const SNDRV_PCM_FORMAT_G723_40_1B:i32=47; pub const SNDRV_PCM_FORMAT_DSD_U8:i32=48; pub const SNDRV_PCM_FORMAT_DSD_U16_LE:i32=49; pub const SNDRV_PCM_FORMAT_DSD_U32_LE:i32=50; pub const SNDRV_PCM_FORMAT_DSD_U16_BE:i32=51; pub const SNDRV_PCM_FORMAT_DSD_U32_BE:i32=52; pub const SNDRV_PCM_FORMAT_FIRST:i32=0; pub const SNDRV_PCM_FORMAT_LAST:i32=52;

pub type __u8=u8; pub type __u16=u16; pub type __u32=u32; pub type __u64=u64; pub type __s32=i32; pub type __s64=i64;
#[repr(C)] pub struct snd_pcm_info { pub device:u32,pub subdevice:u32,pub stream:i32,pub card:i32,pub id:[u8;64],pub name:[u8;80],pub subname:[u8;32],pub dev_class:i32,pub dev_subclass:i32,pub subdevices_count:u32,pub subdevices_avail:u32,pub pad1:[u8;16],pub reserved:[u8;64] }
#[repr(C)] pub struct snd_interval { pub min:u32,pub max:u32,pub openmin:u32,pub openmax:u32,pub integer:u32,pub empty:u32 }
#[repr(C)] pub struct snd_mask { pub bits:[u32;8] }
#[repr(C)] pub struct snd_pcm_hw_params { pub flags:u32,pub masks:[snd_mask;3],pub mres:[snd_mask;5],pub intervals:[snd_interval;12],pub ires:[snd_interval;9],pub rmask:u32,pub cmask:u32,pub info:u32,pub msbits:u32,pub rate_num:u32,pub rate_den:u32,pub fifo_size:snd_pcm_uframes_t,pub sync:[u8;16],pub reserved:[u8;48] }
#[repr(C)] pub struct snd_pcm_sw_params { pub tstamp_mode:i32,pub period_step:u32,pub sleep_min:u32,pub avail_min:snd_pcm_uframes_t,pub xfer_align:snd_pcm_uframes_t,pub start_threshold:snd_pcm_uframes_t,pub stop_threshold:snd_pcm_uframes_t,pub silence_threshold:snd_pcm_uframes_t,pub silence_size:snd_pcm_uframes_t,pub boundary:snd_pcm_uframes_t,pub proto:u32,pub tstamp_type:u32,pub reserved:[u8;56] }
#[repr(C)] pub struct snd_pcm_channel_info { pub channel:u32,pub offset:i64,pub first:u32,pub step:u32 }
#[repr(C)] pub struct snd_xferi { pub result:snd_pcm_sframes_t,pub buf:*mut core::ffi::c_void,pub frames:snd_pcm_uframes_t }
#[repr(C)] pub struct snd_xfern { pub result:snd_pcm_sframes_t,pub bufs:*mut *mut core::ffi::c_void,pub frames:snd_pcm_uframes_t }

pub const SNDRV_RAWMIDI_VERSION:u32=sndrv_protocol_version(2,0,5); pub const SNDRV_RAWMIDI_STREAM_OUTPUT:i32=0; pub const SNDRV_RAWMIDI_STREAM_INPUT:i32=1; pub const SNDRV_RAWMIDI_STREAM_LAST:i32=1;
#[repr(C)] pub struct snd_rawmidi_info { pub device:u32,pub subdevice:u32,pub stream:i32,pub card:i32,pub flags:u32,pub id:[u8;64],pub name:[u8;80],pub subname:[u8;32],pub subdevices_count:u32,pub subdevices_avail:u32,pub tied_device:i32,pub reserved:[u8;60] }
#[repr(C)] pub struct snd_rawmidi_params { pub stream:i32,pub buffer_size:usize,pub avail_min:usize,pub no_active_sensing:u32,pub mode:u32,pub reserved:[u8;12] }
#[repr(C)] pub struct snd_timer_id { pub dev_class:i32,pub dev_sclass:i32,pub card:i32,pub device:i32,pub subdevice:i32 }
#[repr(C)] pub struct snd_timer_select { pub id:snd_timer_id,pub reserved:[u8;32] }
#[repr(C)] pub struct snd_timer_read { pub resolution:u32,pub ticks:u32 }
pub const SNDRV_TIMER_VERSION:u32=sndrv_protocol_version(2,0,8);

pub const SNDRV_CTL_VERSION:u32=sndrv_protocol_version(2,0,10);
#[repr(C)] pub struct snd_ctl_card_info { pub card:i32,pub pad:i32,pub id:[u8;16],pub driver:[u8;16],pub name:[u8;32],pub longname:[u8;80],pub reserved_:[u8;16],pub mixername:[u8;80],pub components:[u8;128] }
#[repr(C)] pub struct snd_ctl_card_bytes { pub type_:u32,pub data_allocated:u32,pub data_len:u32,pub reserved:u32,pub data:u64 }
pub type snd_ctl_elem_type_t=i32; pub const SNDRV_CTL_ELEM_TYPE_NONE:i32=0; pub const SNDRV_CTL_ELEM_TYPE_BOOLEAN:i32=1; pub const SNDRV_CTL_ELEM_TYPE_INTEGER:i32=2; pub const SNDRV_CTL_ELEM_TYPE_ENUMERATED:i32=3; pub const SNDRV_CTL_ELEM_TYPE_BYTES:i32=4; pub const SNDRV_CTL_ELEM_TYPE_IEC958:i32=5; pub const SNDRV_CTL_ELEM_TYPE_INTEGER64:i32=6; pub const SNDRV_CTL_ELEM_TYPE_LAST:i32=6;
pub type snd_ctl_elem_iface_t=i32; pub const SNDRV_CTL_ELEM_IFACE_CARD:i32=0; pub const SNDRV_CTL_ELEM_IFACE_HWDEP:i32=1; pub const SNDRV_CTL_ELEM_IFACE_MIXER:i32=2; pub const SNDRV_CTL_ELEM_IFACE_PCM:i32=3; pub const SNDRV_CTL_ELEM_IFACE_RAWMIDI:i32=4; pub const SNDRV_CTL_ELEM_IFACE_TIMER:i32=5; pub const SNDRV_CTL_ELEM_IFACE_SEQUENCER:i32=6; pub const SNDRV_CTL_ELEM_IFACE_LAST:i32=6;
#[repr(C)] pub struct snd_ctl_elem_id { pub numid:u32,pub iface:snd_ctl_elem_iface_t,pub device:u32,pub subdevice:u32,pub name:[u8;44],pub index:u32 }
#[repr(C)] pub struct snd_ctl_elem_list { pub offset:u32,pub space:u32,pub used:u32,pub count:u32,pub pids:*mut snd_ctl_elem_id,pub reserved:[u8;50] }
#[repr(C)] pub struct snd_ctl_elem_info { pub id:snd_ctl_elem_id,pub type_:snd_ctl_elem_type_t,pub access:u32,pub count:u32,pub owner:i32,pub value:[u8;128],pub reserved:[u8;64] }
#[repr(C)] pub struct snd_ctl_elem_value { pub id:snd_ctl_elem_id,pub indirect:u32,pub value:[u8;1024],pub reserved:[u8;128] }
#[repr(C)] pub struct snd_ctl_tlv { pub numid:u32,pub length:u32,pub tlv:[u32;0] }
#[repr(C)] pub struct snd_ctl_event { pub type_:i32,pub data:[u8;60] }
pub const SNDRV_CHMAP_POSITION_MASK:u32=0xffff; pub const SNDRV_CHMAP_PHASE_INVERSE:u32=0x10000; pub const SNDRV_CHMAP_DRIVER_SPEC:u32=0x20000;
pub const SNDRV_RAWMIDI_INFO_OUTPUT:u32=1; pub const SNDRV_RAWMIDI_INFO_INPUT:u32=2; pub const SNDRV_RAWMIDI_INFO_DUPLEX:u32=4; pub const SNDRV_RAWMIDI_INFO_UMP:u32=8; pub const SNDRV_RAWMIDI_INFO_STREAM_INACTIVE:u32=16;
pub const SNDRV_TIMER_CLASS_NONE:i32=-1; pub const SNDRV_TIMER_CLASS_SLAVE:i32=0; pub const SNDRV_TIMER_CLASS_GLOBAL:i32=1; pub const SNDRV_TIMER_CLASS_CARD:i32=2; pub const SNDRV_TIMER_CLASS_PCM:i32=3; pub const SNDRV_TIMER_CLASS_LAST:i32=3;
pub const SNDRV_TIMER_SCLASS_NONE:i32=0; pub const SNDRV_TIMER_SCLASS_APPLICATION:i32=1; pub const SNDRV_TIMER_SCLASS_SEQUENCER:i32=2; pub const SNDRV_TIMER_SCLASS_OSS_SEQUENCER:i32=3; pub const SNDRV_TIMER_SCLASS_LAST:i32=3;
pub const SNDRV_TIMER_GLOBAL_SYSTEM:i32=0; pub const SNDRV_TIMER_GLOBAL_RTC:i32=1; pub const SNDRV_TIMER_GLOBAL_HPET:i32=2; pub const SNDRV_TIMER_GLOBAL_HRTIMER:i32=3; pub const SNDRV_TIMER_GLOBAL_UDRIVEN:i32=4;
#[repr(C)] pub struct snd_timer_ginfo { pub tid:snd_timer_id,pub flags:u32,pub card:i32,pub id:[u8;64],pub name:[u8;80],pub reserved0:usize,pub resolution:usize,pub resolution_min:usize,pub resolution_max:usize,pub clients:u32,pub reserved:[u8;32] }
#[repr(C)] pub struct snd_timer_gparams { pub tid:snd_timer_id,pub period_num:usize,pub period_den:usize,pub reserved:[u8;32] }
#[repr(C)] pub struct snd_timer_gstatus { pub tid:snd_timer_id,pub resolution:usize,pub resolution_num:usize,pub resolution_den:usize,pub reserved:[u8;32] }
#[repr(C)] pub struct snd_timer_info { pub flags:u32,pub card:i32,pub id:[u8;64],pub name:[u8;80],pub reserved0:usize,pub resolution:usize,pub reserved:[u8;64] }
#[repr(C)] pub struct snd_timer_params { pub flags:u32,pub ticks:u32,pub queue_size:u32,pub reserved0:u32,pub filter:u32,pub reserved:[u8;60] }
#[repr(C)] pub struct snd_timer_uinfo { pub resolution:u64,pub fd:i32,pub id:u32,pub reserved:[u8;16] }
#[repr(C)] pub struct snd_timer_status { pub resolution:u32,pub lost:u32,pub overrun:u32,pub queue:u32,pub reserved:[u8;64] }

/* Ioctl encodings are supplied by the target platform's UAPI support. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
