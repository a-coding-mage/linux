/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Rust translation of uapi/sound/asequencer.h; dependencies are supplied externally. */

pub const SNDRV_SEQ_VERSION: u32 = SNDRV_PROTOCOL_VERSION(1, 0, 5);

pub type snd_seq_event_type_t = u8;
pub const SNDRV_SEQ_EVENT_SYSTEM: u32 = 0; pub const SNDRV_SEQ_EVENT_RESULT: u32 = 1;
pub const SNDRV_SEQ_EVENT_NOTE: u32 = 5; pub const SNDRV_SEQ_EVENT_NOTEON: u32 = 6;
pub const SNDRV_SEQ_EVENT_NOTEOFF: u32 = 7; pub const SNDRV_SEQ_EVENT_KEYPRESS: u32 = 8;
pub const SNDRV_SEQ_EVENT_CONTROLLER: u32 = 10; pub const SNDRV_SEQ_EVENT_PGMCHANGE: u32 = 11;
pub const SNDRV_SEQ_EVENT_CHANPRESS: u32 = 12; pub const SNDRV_SEQ_EVENT_PITCHBEND: u32 = 13;
pub const SNDRV_SEQ_EVENT_CONTROL14: u32 = 14; pub const SNDRV_SEQ_EVENT_NONREGPARAM: u32 = 15; pub const SNDRV_SEQ_EVENT_REGPARAM: u32 = 16;
pub const SNDRV_SEQ_EVENT_SONGPOS: u32 = 20; pub const SNDRV_SEQ_EVENT_SONGSEL: u32 = 21; pub const SNDRV_SEQ_EVENT_QFRAME: u32 = 22; pub const SNDRV_SEQ_EVENT_TIMESIGN: u32 = 23; pub const SNDRV_SEQ_EVENT_KEYSIGN: u32 = 24;
pub const SNDRV_SEQ_EVENT_START: u32 = 30; pub const SNDRV_SEQ_EVENT_CONTINUE: u32 = 31; pub const SNDRV_SEQ_EVENT_STOP: u32 = 32; pub const SNDRV_SEQ_EVENT_SETPOS_TICK: u32 = 33; pub const SNDRV_SEQ_EVENT_SETPOS_TIME: u32 = 34; pub const SNDRV_SEQ_EVENT_TEMPO: u32 = 35; pub const SNDRV_SEQ_EVENT_CLOCK: u32 = 36; pub const SNDRV_SEQ_EVENT_TICK: u32 = 37; pub const SNDRV_SEQ_EVENT_QUEUE_SKEW: u32 = 38;
pub const SNDRV_SEQ_EVENT_TUNE_REQUEST: u32 = 40; pub const SNDRV_SEQ_EVENT_RESET: u32 = 41; pub const SNDRV_SEQ_EVENT_SENSING: u32 = 42; pub const SNDRV_SEQ_EVENT_ECHO: u32 = 50; pub const SNDRV_SEQ_EVENT_OSS: u32 = 51;
pub const SNDRV_SEQ_EVENT_CLIENT_START: u32 = 60; pub const SNDRV_SEQ_EVENT_CLIENT_EXIT: u32 = 61; pub const SNDRV_SEQ_EVENT_CLIENT_CHANGE: u32 = 62; pub const SNDRV_SEQ_EVENT_PORT_START: u32 = 63; pub const SNDRV_SEQ_EVENT_PORT_EXIT: u32 = 64; pub const SNDRV_SEQ_EVENT_PORT_CHANGE: u32 = 65; pub const SNDRV_SEQ_EVENT_PORT_SUBSCRIBED: u32 = 66; pub const SNDRV_SEQ_EVENT_PORT_UNSUBSCRIBED: u32 = 67; pub const SNDRV_SEQ_EVENT_UMP_EP_CHANGE: u32 = 68; pub const SNDRV_SEQ_EVENT_UMP_BLOCK_CHANGE: u32 = 69;
pub const SNDRV_SEQ_EVENT_USR0: u32 = 90; pub const SNDRV_SEQ_EVENT_USR1: u32 = 91; pub const SNDRV_SEQ_EVENT_USR2: u32 = 92; pub const SNDRV_SEQ_EVENT_USR3: u32 = 93; pub const SNDRV_SEQ_EVENT_USR4: u32 = 94; pub const SNDRV_SEQ_EVENT_USR5: u32 = 95; pub const SNDRV_SEQ_EVENT_USR6: u32 = 96; pub const SNDRV_SEQ_EVENT_USR7: u32 = 97; pub const SNDRV_SEQ_EVENT_USR8: u32 = 98; pub const SNDRV_SEQ_EVENT_USR9: u32 = 99;
pub const SNDRV_SEQ_EVENT_SYSEX: u32 = 130; pub const SNDRV_SEQ_EVENT_BOUNCE: u32 = 131; pub const SNDRV_SEQ_EVENT_USR_VAR0: u32 = 135; pub const SNDRV_SEQ_EVENT_USR_VAR1: u32 = 136; pub const SNDRV_SEQ_EVENT_USR_VAR2: u32 = 137; pub const SNDRV_SEQ_EVENT_USR_VAR3: u32 = 138; pub const SNDRV_SEQ_EVENT_USR_VAR4: u32 = 139; pub const SNDRV_SEQ_EVENT_KERNEL_ERROR: u32 = 150; pub const SNDRV_SEQ_EVENT_KERNEL_QUOTE: u32 = 151; pub const SNDRV_SEQ_EVENT_NONE: u32 = 255;

#[repr(C)] #[derive(Copy, Clone)] pub struct snd_seq_addr { pub client: u8, pub port: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_seq_connect { pub sender: snd_seq_addr, pub dest: snd_seq_addr }
pub const SNDRV_SEQ_ADDRESS_UNKNOWN: u32 = 253; pub const SNDRV_SEQ_ADDRESS_SUBSCRIBERS: u32 = 254; pub const SNDRV_SEQ_ADDRESS_BROADCAST: u32 = 255; pub const SNDRV_SEQ_QUEUE_DIRECT: u32 = 253;
pub const SNDRV_SEQ_TIME_STAMP_TICK: u32 = 0; pub const SNDRV_SEQ_TIME_STAMP_REAL: u32 = 1; pub const SNDRV_SEQ_TIME_STAMP_MASK: u32 = 1; pub const SNDRV_SEQ_TIME_MODE_ABS: u32 = 0; pub const SNDRV_SEQ_TIME_MODE_REL: u32 = 2; pub const SNDRV_SEQ_TIME_MODE_MASK: u32 = 2; pub const SNDRV_SEQ_EVENT_LENGTH_FIXED: u32 = 0; pub const SNDRV_SEQ_EVENT_LENGTH_VARIABLE: u32 = 4; pub const SNDRV_SEQ_EVENT_LENGTH_VARUSR: u32 = 8; pub const SNDRV_SEQ_EVENT_LENGTH_MASK: u32 = 12; pub const SNDRV_SEQ_PRIORITY_NORMAL: u32 = 0; pub const SNDRV_SEQ_PRIORITY_HIGH: u32 = 16; pub const SNDRV_SEQ_PRIORITY_MASK: u32 = 16; pub const SNDRV_SEQ_EVENT_UMP: u32 = 32;

#[repr(C)] pub struct snd_seq_ev_note { pub channel:u8,pub note:u8,pub velocity:u8,pub off_velocity:u8,pub duration:u32 }
#[repr(C)] pub struct snd_seq_ev_ctrl { pub channel:u8,pub unused1:u8,pub unused2:u8,pub unused3:u8,pub param:u32,pub value:i32 }
#[repr(C)] pub struct snd_seq_ev_raw8 { pub d:[u8;12] }
#[repr(C)] pub struct snd_seq_ev_raw32 { pub d:[u32;3] }
#[repr(C, packed)] pub struct snd_seq_ev_ext { pub len:u32,pub ptr:*mut core::ffi::c_void }
#[repr(C)] pub struct snd_seq_result { pub event:i32,pub result:i32 }
#[repr(C)] pub struct snd_seq_real_time { pub tv_sec:u32,pub tv_nsec:u32 }
pub type snd_seq_tick_time_t = u32;
#[repr(C)] pub union snd_seq_timestamp { pub tick:snd_seq_tick_time_t,pub time:snd_seq_real_time }
#[repr(C)] pub struct snd_seq_queue_skew { pub value:u32,pub base:u32 }
#[repr(C)] pub union snd_seq_queue_control_param { pub value:i32,pub time:snd_seq_timestamp,pub position:u32,pub skew:snd_seq_queue_skew,pub d32:[u32;2],pub d8:[u8;8] }
#[repr(C)] pub struct snd_seq_ev_queue_control { pub queue:u8,pub pad:[u8;3],pub param:snd_seq_queue_control_param }
#[repr(C, packed)] pub struct snd_seq_ev_quote { pub origin:snd_seq_addr,pub value:u16,pub event:*mut snd_seq_event }
#[repr(C)] pub struct snd_seq_ev_ump_notify { pub client:u8,pub block:u8 }
#[repr(C)] pub union snd_seq_event_data { pub note:snd_seq_ev_note,pub control:snd_seq_ev_ctrl,pub raw8:snd_seq_ev_raw8,pub raw32:snd_seq_ev_raw32,pub ext:snd_seq_ev_ext,pub queue:snd_seq_ev_queue_control,pub time:snd_seq_timestamp,pub addr:snd_seq_addr,pub connect:snd_seq_connect,pub result:snd_seq_result,pub quote:snd_seq_ev_quote,pub ump_notify:snd_seq_ev_ump_notify }
#[repr(C)] pub struct snd_seq_event { pub type_:snd_seq_event_type_t,pub flags:u8,pub tag:i8,pub queue:u8,pub time:snd_seq_timestamp,pub source:snd_seq_addr,pub dest:snd_seq_addr,pub data:snd_seq_event_data }
#[repr(C)] pub union snd_seq_ump_event_data { pub data:snd_seq_event_data,pub ump:[u32;4] }
#[repr(C)] pub struct snd_seq_ump_event { pub type_:snd_seq_event_type_t,pub flags:u8,pub tag:i8,pub queue:u8,pub time:snd_seq_timestamp,pub source:snd_seq_addr,pub dest:snd_seq_addr,pub data:snd_seq_ump_event_data }

#[repr(C)] pub struct snd_seq_system_info { pub queues:i32,pub clients:i32,pub ports:i32,pub channels:i32,pub cur_clients:i32,pub cur_queues:i32,pub reserved:[i8;24] }
#[repr(C)] pub struct snd_seq_running_info { pub client:u8,pub big_endian:u8,pub cpu_mode:u8,pub pad:u8,pub reserved:[u8;12] }
pub const SNDRV_SEQ_CLIENT_SYSTEM:i32=0; pub const SNDRV_SEQ_CLIENT_DUMMY:i32=14; pub const SNDRV_SEQ_CLIENT_OSS:i32=15; pub type snd_seq_client_type_t=i32; pub const NO_CLIENT:i32=0; pub const USER_CLIENT:i32=1; pub const KERNEL_CLIENT:i32=2;

#[repr(C)] pub struct snd_seq_client_info { pub client:i32,pub type_:i32,pub name:[i8;64],pub filter:u32,pub multicast_filter:[u8;8],pub event_filter:[u8;32],pub num_ports:i32,pub event_lost:i32,pub card:i32,pub pid:i32,pub midi_version:u32,pub group_filter:u32,pub reserved:[i8;48] }
pub const SNDRV_SEQ_CLIENT_LEGACY_MIDI:u32=0; pub const SNDRV_SEQ_CLIENT_UMP_MIDI_1_0:u32=1; pub const SNDRV_SEQ_CLIENT_UMP_MIDI_2_0:u32=2;
#[repr(C)] pub struct snd_seq_client_pool { pub client:i32,pub output_pool:i32,pub input_pool:i32,pub output_room:i32,pub output_free:i32,pub input_free:i32,pub reserved:[i8;64] }
#[repr(C)] pub struct snd_seq_remove_events { pub remove_mode:u32,pub time:snd_seq_timestamp,pub queue:u8,pub dest:snd_seq_addr,pub channel:u8,pub type_:i32,pub tag:i8,pub reserved:[i32;10] }

pub const SNDRV_SEQ_PORT_SYSTEM_TIMER:i32=0; pub const SNDRV_SEQ_PORT_SYSTEM_ANNOUNCE:i32=1;
pub const SNDRV_SEQ_PORT_CAP_READ:u32=1; pub const SNDRV_SEQ_PORT_CAP_WRITE:u32=2; pub const SNDRV_SEQ_PORT_CAP_SYNC_READ:u32=4; pub const SNDRV_SEQ_PORT_CAP_SYNC_WRITE:u32=8; pub const SNDRV_SEQ_PORT_CAP_DUPLEX:u32=16; pub const SNDRV_SEQ_PORT_CAP_SUBS_READ:u32=32; pub const SNDRV_SEQ_PORT_CAP_SUBS_WRITE:u32=64; pub const SNDRV_SEQ_PORT_CAP_NO_EXPORT:u32=128; pub const SNDRV_SEQ_PORT_CAP_INACTIVE:u32=256; pub const SNDRV_SEQ_PORT_CAP_UMP_ENDPOINT:u32=512;
pub const SNDRV_SEQ_PORT_TYPE_SPECIFIC:u32=1; pub const SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC:u32=2; pub const SNDRV_SEQ_PORT_TYPE_MIDI_GM:u32=4; pub const SNDRV_SEQ_PORT_TYPE_MIDI_GS:u32=8; pub const SNDRV_SEQ_PORT_TYPE_MIDI_XG:u32=16; pub const SNDRV_SEQ_PORT_TYPE_MIDI_MT32:u32=32; pub const SNDRV_SEQ_PORT_TYPE_MIDI_GM2:u32=64; pub const SNDRV_SEQ_PORT_TYPE_MIDI_UMP:u32=128; pub const SNDRV_SEQ_PORT_TYPE_SYNTH:u32=1024; pub const SNDRV_SEQ_PORT_TYPE_DIRECT_SAMPLE:u32=2048; pub const SNDRV_SEQ_PORT_TYPE_SAMPLE:u32=4096; pub const SNDRV_SEQ_PORT_TYPE_HARDWARE:u32=65536; pub const SNDRV_SEQ_PORT_TYPE_SOFTWARE:u32=131072; pub const SNDRV_SEQ_PORT_TYPE_SYNTHESIZER:u32=262144; pub const SNDRV_SEQ_PORT_TYPE_PORT:u32=524288; pub const SNDRV_SEQ_PORT_TYPE_APPLICATION:u32=1048576;
pub const SNDRV_SEQ_PORT_FLG_GIVEN_PORT:u32=1; pub const SNDRV_SEQ_PORT_FLG_TIMESTAMP:u32=2; pub const SNDRV_SEQ_PORT_FLG_TIME_REAL:u32=4; pub const SNDRV_SEQ_PORT_FLG_IS_MIDI1:u32=8; pub const SNDRV_SEQ_PORT_DIR_UNKNOWN:u8=0; pub const SNDRV_SEQ_PORT_DIR_INPUT:u8=1; pub const SNDRV_SEQ_PORT_DIR_OUTPUT:u8=2; pub const SNDRV_SEQ_PORT_DIR_BIDIRECTION:u8=3;
#[repr(C)] pub struct snd_seq_port_info { pub addr:snd_seq_addr,pub name:[i8;64],pub capability:u32,pub type_:u32,pub midi_channels:i32,pub midi_voices:i32,pub synth_voices:i32,pub read_use:i32,pub write_use:i32,pub kernel:*mut core::ffi::c_void,pub flags:u32,pub time_queue:u8,pub direction:u8,pub ump_group:u8,pub reserved:[i8;57] }
pub const SNDRV_SEQ_QUEUE_FLG_SYNC:u32=1;
#[repr(C)] pub struct snd_seq_queue_info { pub queue:i32,pub owner:i32,pub locked:u32,pub name:[i8;64],pub flags:u32,pub reserved:[i8;60] }
#[repr(C)] pub struct snd_seq_queue_status { pub queue:i32,pub events:i32,pub tick:u32,pub time:snd_seq_real_time,pub running:i32,pub flags:i32,pub reserved:[i8;64] }
#[repr(C)] pub struct snd_seq_queue_tempo { pub queue:i32,pub tempo:u32,pub ppq:i32,pub skew_value:u32,pub skew_base:u32,pub tempo_base:u16,pub reserved:[i8;22] }
pub const SNDRV_SEQ_TIMER_ALSA:i32=0; pub const SNDRV_SEQ_TIMER_MIDI_CLOCK:i32=1; pub const SNDRV_SEQ_TIMER_MIDI_TICK:i32=2;
#[repr(C)] pub struct snd_seq_queue_timer { pub queue:i32,pub type_:i32,pub u:snd_seq_queue_timer_u,pub reserved:[i8;64] }
#[repr(C)] pub union snd_seq_queue_timer_u { pub alsa:snd_seq_queue_timer_alsa }
#[repr(C)] pub struct snd_seq_queue_timer_alsa { pub id:snd_timer_id,pub resolution:u32 }
#[repr(C)] pub struct snd_seq_queue_client { pub queue:i32,pub client:i32,pub used:i32,pub reserved:[i8;64] }
pub const SNDRV_SEQ_PORT_SUBS_EXCLUSIVE:u32=1; pub const SNDRV_SEQ_PORT_SUBS_TIMESTAMP:u32=2; pub const SNDRV_SEQ_PORT_SUBS_TIME_REAL:u32=4;
#[repr(C)] pub struct snd_seq_port_subscribe { pub sender:snd_seq_addr,pub dest:snd_seq_addr,pub voices:u32,pub flags:u32,pub queue:u8,pub pad:[u8;3],pub reserved:[i8;64] }
pub const SNDRV_SEQ_QUERY_SUBS_READ:i32=0; pub const SNDRV_SEQ_QUERY_SUBS_WRITE:i32=1;
#[repr(C)] pub struct snd_seq_query_subs { pub root:snd_seq_addr,pub type_:i32,pub index:i32,pub num_subs:i32,pub addr:snd_seq_addr,pub queue:u8,pub flags:u32,pub reserved:[i8;64] }
pub const SNDRV_SEQ_CLIENT_UMP_INFO_ENDPOINT:i32=0; pub const SNDRV_SEQ_CLIENT_UMP_INFO_BLOCK:i32=1;
#[repr(C, packed)] pub struct snd_seq_client_ump_info { pub client:i32,pub type_:i32,pub info:[u8;512] }

/* ioctl encodings are supplied by the asound dependency. */
pub const SNDRV_SEQ_IOCTL_PVERSION: u32 = _IOR('S',0x00,i32);
pub const SNDRV_SEQ_IOCTL_CLIENT_ID: u32 = _IOR('S',0x01,i32);
pub const SNDRV_SEQ_IOCTL_SYSTEM_INFO: u32 = _IOWR('S',0x02,snd_seq_system_info);
pub const SNDRV_SEQ_IOCTL_RUNNING_MODE: u32 = _IOWR('S',0x03,snd_seq_running_info);
pub const SNDRV_SEQ_IOCTL_USER_PVERSION: u32 = _IOW('S',0x04,i32);
pub const SNDRV_SEQ_IOCTL_GET_CLIENT_INFO:u32=_IOWR('S',0x10,snd_seq_client_info); pub const SNDRV_SEQ_IOCTL_SET_CLIENT_INFO:u32=_IOW('S',0x11,snd_seq_client_info); pub const SNDRV_SEQ_IOCTL_GET_CLIENT_UMP_INFO:u32=_IOWR('S',0x12,snd_seq_client_ump_info); pub const SNDRV_SEQ_IOCTL_SET_CLIENT_UMP_INFO:u32=_IOWR('S',0x13,snd_seq_client_ump_info);
pub const SNDRV_SEQ_IOCTL_CREATE_PORT:u32=_IOWR('S',0x20,snd_seq_port_info); pub const SNDRV_SEQ_IOCTL_DELETE_PORT:u32=_IOW('S',0x21,snd_seq_port_info); pub const SNDRV_SEQ_IOCTL_GET_PORT_INFO:u32=_IOWR('S',0x22,snd_seq_port_info); pub const SNDRV_SEQ_IOCTL_SET_PORT_INFO:u32=_IOW('S',0x23,snd_seq_port_info);
pub const SNDRV_SEQ_IOCTL_SUBSCRIBE_PORT:u32=_IOW('S',0x30,snd_seq_port_subscribe); pub const SNDRV_SEQ_IOCTL_UNSUBSCRIBE_PORT:u32=_IOW('S',0x31,snd_seq_port_subscribe); pub const SNDRV_SEQ_IOCTL_CREATE_QUEUE:u32=_IOWR('S',0x32,snd_seq_queue_info); pub const SNDRV_SEQ_IOCTL_DELETE_QUEUE:u32=_IOW('S',0x33,snd_seq_queue_info); pub const SNDRV_SEQ_IOCTL_GET_QUEUE_INFO:u32=_IOWR('S',0x34,snd_seq_queue_info); pub const SNDRV_SEQ_IOCTL_SET_QUEUE_INFO:u32=_IOWR('S',0x35,snd_seq_queue_info); pub const SNDRV_SEQ_IOCTL_GET_NAMED_QUEUE:u32=_IOWR('S',0x36,snd_seq_queue_info);
pub const SNDRV_SEQ_IOCTL_GET_QUEUE_STATUS:u32=_IOWR('S',0x40,snd_seq_queue_status); pub const SNDRV_SEQ_IOCTL_GET_QUEUE_TEMPO:u32=_IOWR('S',0x41,snd_seq_queue_tempo); pub const SNDRV_SEQ_IOCTL_SET_QUEUE_TEMPO:u32=_IOW('S',0x42,snd_seq_queue_tempo); pub const SNDRV_SEQ_IOCTL_GET_QUEUE_TIMER:u32=_IOWR('S',0x45,snd_seq_queue_timer); pub const SNDRV_SEQ_IOCTL_SET_QUEUE_TIMER:u32=_IOW('S',0x46,snd_seq_queue_timer); pub const SNDRV_SEQ_IOCTL_GET_QUEUE_CLIENT:u32=_IOWR('S',0x49,snd_seq_queue_client); pub const SNDRV_SEQ_IOCTL_SET_QUEUE_CLIENT:u32=_IOW('S',0x4a,snd_seq_queue_client); pub const SNDRV_SEQ_IOCTL_GET_CLIENT_POOL:u32=_IOWR('S',0x4b,snd_seq_client_pool); pub const SNDRV_SEQ_IOCTL_SET_CLIENT_POOL:u32=_IOW('S',0x4c,snd_seq_client_pool); pub const SNDRV_SEQ_IOCTL_REMOVE_EVENTS:u32=_IOW('S',0x4e,snd_seq_remove_events); pub const SNDRV_SEQ_IOCTL_QUERY_SUBS:u32=_IOWR('S',0x4f,snd_seq_query_subs); pub const SNDRV_SEQ_IOCTL_GET_SUBSCRIPTION:u32=_IOWR('S',0x50,snd_seq_port_subscribe); pub const SNDRV_SEQ_IOCTL_QUERY_NEXT_CLIENT:u32=_IOWR('S',0x51,snd_seq_client_info); pub const SNDRV_SEQ_IOCTL_QUERY_NEXT_PORT:u32=_IOWR('S',0x52,snd_seq_port_info);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
