/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from the Linux UAPI sound/firewire.h header.

// External Linux types and ioctl encoding helpers are supplied by dependencies.

pub const SNDRV_FIREWIRE_EVENT_LOCK_STATUS: u32 = 0x0000_10cc;
pub const SNDRV_FIREWIRE_EVENT_DICE_NOTIFICATION: u32 = 0xd1ce_004e;
pub const SNDRV_FIREWIRE_EVENT_EFW_RESPONSE: u32 = 0x4e61_7475;
pub const SNDRV_FIREWIRE_EVENT_DIGI00X_MESSAGE: u32 = 0x746e_736c;
pub const SNDRV_FIREWIRE_EVENT_MOTU_NOTIFICATION: u32 = 0x6477_6479;
pub const SNDRV_FIREWIRE_EVENT_TASCAM_CONTROL: u32 = 0x7473_636d;
pub const SNDRV_FIREWIRE_EVENT_MOTU_REGISTER_DSP_CHANGE: u32 = 0x4d54_5244;
pub const SNDRV_FIREWIRE_EVENT_FF400_MESSAGE: u32 = 0x4f6c_6761;

#[repr(C)]
pub struct snd_firewire_event_common { pub type_: u32 }
#[repr(C)]
pub struct snd_firewire_event_lock_status { pub type_: u32, pub status: u32 }
#[repr(C)]
pub struct snd_firewire_event_dice_notification { pub type_: u32, pub notification: u32 }

pub const SND_EFW_TRANSACTION_USER_SEQNUM_MAX: u32 = (u16::MAX as u32) - 1;

#[repr(C)]
pub struct snd_efw_transaction {
    pub length: __be32, pub version: __be32, pub seqnum: __be32,
    pub category: __be32, pub command: __be32, pub status: __be32,
    pub params: [__be32; 0],
}
#[repr(C)]
pub struct snd_firewire_event_efw_response { pub type_: u32, pub response: [__be32; 0] }
#[repr(C)]
pub struct snd_firewire_event_digi00x_message { pub type_: u32, pub message: __u32 }
#[repr(C)]
pub struct snd_firewire_event_motu_notification { pub type_: u32, pub message: __u32 }
#[repr(C)]
pub struct snd_firewire_tascam_change { pub index: u32, pub before: __be32, pub after: __be32 }
#[repr(C)]
pub struct snd_firewire_event_tascam_control { pub type_: u32, pub changes: [snd_firewire_tascam_change; 0] }
#[repr(C)]
pub struct snd_firewire_event_motu_register_dsp_change {
    pub type_: u32, pub count: __u32, pub changes: [__u32; 0],
}
#[repr(C)]
pub struct snd_firewire_event_ff400_message_item { pub message: __u32, pub tstamp: __u32 }
#[repr(C)]
pub struct snd_firewire_event_ff400_message {
    pub type_: u32, pub message_count: u32,
    pub messages: [snd_firewire_event_ff400_message_item; 0],
}

#[repr(C)]
pub union snd_firewire_event {
    pub common: snd_firewire_event_common,
    pub lock_status: snd_firewire_event_lock_status,
    pub dice_notification: snd_firewire_event_dice_notification,
    pub efw_response: snd_firewire_event_efw_response,
    pub digi00x_message: snd_firewire_event_digi00x_message,
    pub tascam_control: snd_firewire_event_tascam_control,
    pub motu_notification: snd_firewire_event_motu_notification,
    pub motu_register_dsp_change: snd_firewire_event_motu_register_dsp_change,
    pub ff400_message: snd_firewire_event_ff400_message,
}

// SNDRV_FIREWIRE_IOCTL_* are _IO/_IOR encodings using the external ioctl helpers:
// GET_INFO=(_IOR('H',0xf8,snd_firewire_get_info)), LOCK=_IO('H',0xf9),
// UNLOCK=_IO('H',0xfa), TASCAM_STATE=_IOR('H',0xfb,snd_firewire_tascam_state),
// MOTU_REGISTER_DSP_METER=_IOR('H',0xfc,snd_firewire_motu_register_dsp_meter),
// MOTU_COMMAND_DSP_METER=_IOR('H',0xfd,snd_firewire_motu_command_dsp_meter),
// MOTU_REGISTER_DSP_PARAMETER=_IOR('H',0xfe,snd_firewire_motu_register_dsp_parameter).

pub const SNDRV_FIREWIRE_TYPE_DICE: u32 = 1;
pub const SNDRV_FIREWIRE_TYPE_FIREWORKS: u32 = 2;
pub const SNDRV_FIREWIRE_TYPE_BEBOB: u32 = 3;
pub const SNDRV_FIREWIRE_TYPE_OXFW: u32 = 4;
pub const SNDRV_FIREWIRE_TYPE_DIGI00X: u32 = 5;
pub const SNDRV_FIREWIRE_TYPE_TASCAM: u32 = 6;
pub const SNDRV_FIREWIRE_TYPE_MOTU: u32 = 7;
pub const SNDRV_FIREWIRE_TYPE_FIREFACE: u32 = 8;

#[repr(C)]
pub struct snd_firewire_get_info {
    pub type_: u32, pub card: u32, pub guid: [u8; 8], pub device_name: [core::ffi::c_char; 16],
}
pub const SNDRV_FIREWIRE_TASCAM_STATE_COUNT: usize = 64;
#[repr(C)]
pub struct snd_firewire_tascam_state { pub data: [__be32; SNDRV_FIREWIRE_TASCAM_STATE_COUNT] }

pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_INPUT_COUNT: usize = 24;
pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_OUTPUT_COUNT: usize = 24;
pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_COUNT: usize = 48;
#[repr(C)]
pub struct snd_firewire_motu_register_dsp_meter { pub data: [__u8; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_COUNT] }

pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT: usize = 4;
pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT: usize = 20;
pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_INPUT_COUNT: usize = 10;
pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_ALIGNED_INPUT_COUNT: usize = 12;

#[repr(C)]
pub struct snd_firewire_motu_register_dsp_parameter_mixer_source {
    pub gain: [__u8; 20], pub pan: [__u8; 20], pub flag: [__u8; 20],
    pub paired_balance: [__u8; 20], pub paired_width: [__u8; 20],
}
#[repr(C)]
pub struct snd_firewire_motu_register_dsp_parameter_mixer_output { pub paired_volume: [__u8; 4], pub paired_flag: [__u8; 4] }
#[repr(C)]
pub struct snd_firewire_motu_register_dsp_parameter_mixer {
    pub source: [snd_firewire_motu_register_dsp_parameter_mixer_source; 4],
    pub output: snd_firewire_motu_register_dsp_parameter_mixer_output,
}
#[repr(C)]
pub struct snd_firewire_motu_register_dsp_parameter_output {
    pub main_paired_volume: __u8, pub hp_paired_volume: __u8, pub hp_paired_assignment: __u8, pub reserved: [__u8; 5],
}
#[repr(C)]
pub struct snd_firewire_motu_register_dsp_parameter_line_input { pub boost_flag: __u8, pub nominal_level_flag: __u8, pub reserved: [__u8; 6] }
#[repr(C)]
pub struct snd_firewire_motu_register_dsp_parameter_input { pub gain_and_invert: [__u8; 12], pub flag: [__u8; 12] }
#[repr(C)]
pub struct snd_firewire_motu_register_dsp_parameter {
    pub mixer: snd_firewire_motu_register_dsp_parameter_mixer,
    pub output: snd_firewire_motu_register_dsp_parameter_output,
    pub line_input: snd_firewire_motu_register_dsp_parameter_line_input,
    pub input: snd_firewire_motu_register_dsp_parameter_input,
    pub reserved: [__u8; 64],
}

pub const SNDRV_FIREWIRE_MOTU_COMMAND_DSP_METER_COUNT: usize = 400;
#[repr(C)]
pub struct snd_firewire_motu_command_dsp_meter {
    // In kernel builds this field is __u32; in userspace builds it is float.
    pub data: [f32; SNDRV_FIREWIRE_MOTU_COMMAND_DSP_METER_COUNT],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
