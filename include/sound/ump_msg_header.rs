// SPDX-License-Identifier: GPL-2.0-or-later
/* Universal MIDI Packet (UMP): Message Definitions */

pub const UMP_MSG_STATUS_PER_NOTE_RCC: u32 = 0x0;
pub const UMP_MSG_STATUS_PER_NOTE_ACC: u32 = 0x1;
pub const UMP_MSG_STATUS_RPN: u32 = 0x2;
pub const UMP_MSG_STATUS_NRPN: u32 = 0x3;
pub const UMP_MSG_STATUS_RELATIVE_RPN: u32 = 0x4;
pub const UMP_MSG_STATUS_RELATIVE_NRPN: u32 = 0x5;
pub const UMP_MSG_STATUS_PER_NOTE_PITCH_BEND: u32 = 0x6;
pub const UMP_MSG_STATUS_NOTE_OFF: u32 = 0x8;
pub const UMP_MSG_STATUS_NOTE_ON: u32 = 0x9;
pub const UMP_MSG_STATUS_POLY_PRESSURE: u32 = 0xa;
pub const UMP_MSG_STATUS_CC: u32 = 0xb;
pub const UMP_MSG_STATUS_PROGRAM: u32 = 0xc;
pub const UMP_MSG_STATUS_CHANNEL_PRESSURE: u32 = 0xd;
pub const UMP_MSG_STATUS_PITCH_BEND: u32 = 0xe;
pub const UMP_MSG_STATUS_PER_NOTE_MGMT: u32 = 0xf;

pub const UMP_CC_BANK_SELECT: u32 = 0; pub const UMP_CC_MODULATION: u32 = 1;
pub const UMP_CC_BREATH: u32 = 2; pub const UMP_CC_FOOT: u32 = 4;
pub const UMP_CC_PORTAMENTO_TIME: u32 = 5; pub const UMP_CC_DATA: u32 = 6;
pub const UMP_CC_VOLUME: u32 = 7; pub const UMP_CC_BALANCE: u32 = 8;
pub const UMP_CC_PAN: u32 = 10; pub const UMP_CC_EXPRESSION: u32 = 11;
pub const UMP_CC_EFFECT_CONTROL_1: u32 = 12; pub const UMP_CC_EFFECT_CONTROL_2: u32 = 13;
pub const UMP_CC_GP_1: u32 = 16; pub const UMP_CC_GP_2: u32 = 17; pub const UMP_CC_GP_3: u32 = 18; pub const UMP_CC_GP_4: u32 = 19;
pub const UMP_CC_BANK_SELECT_LSB: u32 = 32; pub const UMP_CC_MODULATION_LSB: u32 = 33; pub const UMP_CC_BREATH_LSB: u32 = 34;
pub const UMP_CC_FOOT_LSB: u32 = 36; pub const UMP_CC_PORTAMENTO_TIME_LSB: u32 = 37; pub const UMP_CC_DATA_LSB: u32 = 38;
pub const UMP_CC_VOLUME_LSB: u32 = 39; pub const UMP_CC_BALANCE_LSB: u32 = 40; pub const UMP_CC_PAN_LSB: u32 = 42;
pub const UMP_CC_EXPRESSION_LSB: u32 = 43; pub const UMP_CC_EFFECT1_LSB: u32 = 44; pub const UMP_CC_EFFECT2_LSB: u32 = 45;
pub const UMP_CC_GP_1_LSB: u32 = 48; pub const UMP_CC_GP_2_LSB: u32 = 49; pub const UMP_CC_GP_3_LSB: u32 = 50; pub const UMP_CC_GP_4_LSB: u32 = 51;
pub const UMP_CC_SUSTAIN: u32 = 64; pub const UMP_CC_PORTAMENTO_SWITCH: u32 = 65; pub const UMP_CC_SOSTENUTO: u32 = 66; pub const UMP_CC_SOFT_PEDAL: u32 = 67; pub const UMP_CC_LEGATO: u32 = 68; pub const UMP_CC_HOLD_2: u32 = 69;
pub const UMP_CC_SOUND_CONTROLLER_1: u32 = 70; pub const UMP_CC_SOUND_CONTROLLER_2: u32 = 71; pub const UMP_CC_SOUND_CONTROLLER_3: u32 = 72; pub const UMP_CC_SOUND_CONTROLLER_4: u32 = 73; pub const UMP_CC_SOUND_CONTROLLER_5: u32 = 74; pub const UMP_CC_SOUND_CONTROLLER_6: u32 = 75; pub const UMP_CC_SOUND_CONTROLLER_7: u32 = 76; pub const UMP_CC_SOUND_CONTROLLER_8: u32 = 77; pub const UMP_CC_SOUND_CONTROLLER_9: u32 = 78; pub const UMP_CC_SOUND_CONTROLLER_10: u32 = 79;
pub const UMP_CC_GP_5: u32 = 80; pub const UMP_CC_GP_6: u32 = 81; pub const UMP_CC_GP_7: u32 = 82; pub const UMP_CC_GP_8: u32 = 83; pub const UMP_CC_PORTAMENTO_CONTROL: u32 = 84;
pub const UMP_CC_EFFECT_1: u32 = 91; pub const UMP_CC_EFFECT_2: u32 = 92; pub const UMP_CC_EFFECT_3: u32 = 93; pub const UMP_CC_EFFECT_4: u32 = 94; pub const UMP_CC_EFFECT_5: u32 = 95;
pub const UMP_CC_DATA_INC: u32 = 96; pub const UMP_CC_DATA_DEC: u32 = 97; pub const UMP_CC_NRPN_LSB: u32 = 98; pub const UMP_CC_NRPN_MSB: u32 = 99; pub const UMP_CC_RPN_LSB: u32 = 100; pub const UMP_CC_RPN_MSB: u32 = 101;
pub const UMP_CC_ALL_SOUND_OFF: u32 = 120; pub const UMP_CC_RESET_ALL: u32 = 121; pub const UMP_CC_LOCAL_CONTROL: u32 = 122; pub const UMP_CC_ALL_NOTES_OFF: u32 = 123; pub const UMP_CC_OMNI_OFF: u32 = 124; pub const UMP_CC_OMNI_ON: u32 = 125; pub const UMP_CC_POLY_OFF: u32 = 126; pub const UMP_CC_POLY_ON: u32 = 127;

pub const UMP_SYSTEM_STATUS_MIDI_TIME_CODE: u32 = 0xf1; pub const UMP_SYSTEM_STATUS_SONG_POSITION: u32 = 0xf2; pub const UMP_SYSTEM_STATUS_SONG_SELECT: u32 = 0xf3; pub const UMP_SYSTEM_STATUS_TUNE_REQUEST: u32 = 0xf6; pub const UMP_SYSTEM_STATUS_TIMING_CLOCK: u32 = 0xf8; pub const UMP_SYSTEM_STATUS_START: u32 = 0xfa; pub const UMP_SYSTEM_STATUS_CONTINUE: u32 = 0xfb; pub const UMP_SYSTEM_STATUS_STOP: u32 = 0xfc; pub const UMP_SYSTEM_STATUS_ACTIVE_SENSING: u32 = 0xfe; pub const UMP_SYSTEM_STATUS_RESET: u32 = 0xff;
pub const UMP_MIDI1_MSG_REALTIME: u32 = 0xf0; pub const UMP_MIDI1_MSG_SYSEX_START: u32 = 0xf0; pub const UMP_MIDI1_MSG_SYSEX_END: u32 = 0xf7;

// C bit-fields have no direct Rust syntax. These packed representations retain
// the declared members and their integer widths; bit-field order follows the
// source's __BIG_ENDIAN_BITFIELD conditional at the integration boundary.
#[repr(C, packed)] pub struct snd_ump_midi1_msg_note { pub velocity:u32, pub note:u32, pub channel:u32, pub status:u32, pub group:u32, pub r#type:u32 }
#[repr(C, packed)] pub struct snd_ump_midi1_msg_paf { pub data:u32, pub note:u32, pub channel:u32, pub status:u32, pub group:u32, pub r#type:u32 }
#[repr(C, packed)] pub struct snd_ump_midi1_msg_cc { pub data:u32, pub index:u32, pub channel:u32, pub status:u32, pub group:u32, pub r#type:u32 }
#[repr(C, packed)] pub struct snd_ump_midi1_msg_program { pub reserved:u32, pub program:u32, pub channel:u32, pub status:u32, pub group:u32, pub r#type:u32 }
#[repr(C, packed)] pub struct snd_ump_midi1_msg_caf { pub reserved:u32, pub data:u32, pub channel:u32, pub status:u32, pub group:u32, pub r#type:u32 }
#[repr(C, packed)] pub struct snd_ump_midi1_msg_pitchbend { pub data_msb:u32, pub data_lsb:u32, pub channel:u32, pub status:u32, pub group:u32, pub r#type:u32 }
#[repr(C, packed)] pub struct snd_ump_system_msg { pub parm2:u32, pub parm1:u32, pub status:u32, pub group:u32, pub r#type:u32 }

#[repr(C)] pub union snd_ump_midi1_msg { pub note:snd_ump_midi1_msg_note, pub paf:snd_ump_midi1_msg_paf, pub cc:snd_ump_midi1_msg_cc, pub pg:snd_ump_midi1_msg_program, pub caf:snd_ump_midi1_msg_caf, pub pb:snd_ump_midi1_msg_pitchbend, pub system:snd_ump_system_msg, pub raw:u32 }

macro_rules! midi2_struct { ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => { #[repr(C, packed)] pub struct $name { $(pub $field:$ty,)* } }; }
midi2_struct!(snd_ump_midi2_msg_note { attribute_type:u32, note:u32, channel:u32, status:u32, group:u32, r#type:u32, attribute_data:u32, velocity:u32 });
midi2_struct!(snd_ump_midi2_msg_paf { reserved:u32, note:u32, channel:u32, status:u32, group:u32, r#type:u32, data:u32 });
midi2_struct!(snd_ump_midi2_msg_pernote_cc { index:u32, note:u32, channel:u32, status:u32, group:u32, r#type:u32, data:u32 });
midi2_struct!(snd_ump_midi2_msg_pernote_mgmt { flags:u32, note:u32, channel:u32, status:u32, group:u32, r#type:u32, reserved:u32 });
midi2_struct!(snd_ump_midi2_msg_cc { reserved:u32, index:u32, channel:u32, status:u32, group:u32, r#type:u32, data:u32 });
midi2_struct!(snd_ump_midi2_msg_rpn { index:u32, bank:u32, channel:u32, status:u32, group:u32, r#type:u32, data:u32 });
midi2_struct!(snd_ump_midi2_msg_program { bank_valid:u32, reserved:u32, channel:u32, status:u32, group:u32, r#type:u32, program:u32, reserved2:u32, bank_msb:u32, bank_lsb:u32 });
midi2_struct!(snd_ump_midi2_msg_caf { reserved:u32, channel:u32, status:u32, group:u32, r#type:u32, data:u32 });
midi2_struct!(snd_ump_midi2_msg_pitchbend { reserved:u32, channel:u32, status:u32, group:u32, r#type:u32, data:u32 });
midi2_struct!(snd_ump_midi2_msg_pernote_pitchbend { reserved:u32, note:u32, channel:u32, status:u32, group:u32, r#type:u32, data:u32 });
#[repr(C)] pub union snd_ump_midi2_msg { pub note:snd_ump_midi2_msg_note, pub paf:snd_ump_midi2_msg_paf, pub pernote_cc:snd_ump_midi2_msg_pernote_cc, pub pernote_mgmt:snd_ump_midi2_msg_pernote_mgmt, pub cc:snd_ump_midi2_msg_cc, pub rpn:snd_ump_midi2_msg_rpn, pub pg:snd_ump_midi2_msg_program, pub caf:snd_ump_midi2_msg_caf, pub pb:snd_ump_midi2_msg_pitchbend, pub pernote_pb:snd_ump_midi2_msg_pernote_pitchbend, pub raw:[u32;2] }

#[repr(C, packed)] pub struct snd_ump_stream_msg_ep_discovery { pub r#type:u32, pub format:u32, pub status:u32, pub ump_version_major:u32, pub ump_version_minor:u32, pub reserved:u32, pub filter_bitmap:u32, pub reserved2:[u32;2] }
#[repr(C, packed)] pub struct snd_ump_stream_msg_ep_info { pub r#type:u32, pub format:u32, pub status:u32, pub ump_version_major:u32, pub ump_version_minor:u32, pub static_function_block:u32, pub num_function_blocks:u32, pub reserved:u32, pub protocol:u32, pub reserved2:u32, pub jrts:u32, pub reserved3:[u32;2] }
#[repr(C, packed)] pub struct snd_ump_stream_msg_device_info { pub r#type:u32, pub format:u32, pub status:u32, pub reserved:u32, pub manufacture_id:u32, pub family_lsb:u8, pub family_msb:u8, pub model_lsb:u8, pub model_msb:u8, pub sw_revision:u32 }
#[repr(C, packed)] pub struct snd_ump_stream_msg_stream_cfg { pub r#type:u32, pub format:u32, pub status:u32, pub protocol:u32, pub reserved:u32, pub jrts:u32, pub reserved2:[u32;3] }
#[repr(C, packed)] pub struct snd_ump_stream_msg_fb_discovery { pub r#type:u32, pub format:u32, pub status:u32, pub function_block_id:u32, pub filter:u32, pub reserved:[u32;3] }
#[repr(C, packed)] pub struct snd_ump_stream_msg_fb_info { pub r#type:u32, pub format:u32, pub status:u32, pub active:u32, pub function_block_id:u32, pub reserved:u32, pub ui_hint:u32, pub midi_10:u32, pub direction:u32, pub first_group:u32, pub num_groups:u32, pub midi_ci_version:u32, pub sysex8_streams:u32, pub reserved2:[u32;2] }
#[repr(C, packed)] pub struct snd_ump_stream_msg_fb_name { pub r#type:u16, pub format:u16, pub status:u16, pub function_block_id:u8, pub name0:u8, pub name:[u8;12] }
#[repr(C)] pub union snd_ump_stream_msg { pub ep_discovery:snd_ump_stream_msg_ep_discovery, pub ep_info:snd_ump_stream_msg_ep_info, pub device_info:snd_ump_stream_msg_device_info, pub stream_cfg:snd_ump_stream_msg_stream_cfg, pub fb_discovery:snd_ump_stream_msg_fb_discovery, pub fb_info:snd_ump_stream_msg_fb_info, pub fb_name:snd_ump_stream_msg_fb_name, pub raw:[u32;4] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
