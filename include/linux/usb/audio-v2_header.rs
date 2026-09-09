// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2010 Daniel Mack <daniel@caiaq.de>
 *
 * This file holds USB constants and structures defined by the USB Device Class
 * Definition for Audio Devices in version 2.0.
 */

// Definitions shared with audio.h are supplied by the surrounding translation.

#[inline]
pub fn uac_v2v3_control_is_readable(bm_controls: u32, control: u8) -> bool {
    ((bm_controls >> ((control.wrapping_sub(1) as u32) * 2)) & 0x1) != 0
}

#[inline]
pub fn uac_v2v3_control_is_writeable(bm_controls: u32, control: u8) -> bool {
    ((bm_controls >> ((control.wrapping_sub(1) as u32) * 2)) & 0x2) != 0
}

#[repr(C, packed)]
pub struct uac2_ac_header_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bcdADC:u16, pub bCategory:u8, pub wTotalLength:u16, pub bmControls:u8 }
#[repr(C, packed)]
pub struct uac2_format_type_i_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bFormatType:u8, pub bSubslotSize:u8, pub bBitResolution:u8 }
#[repr(C, packed)]
pub struct uac_clock_source_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bClockID:u8, pub bmAttributes:u8, pub bmControls:u8, pub bAssocTerminal:u8, pub iClockSource:u8 }
#[repr(C, packed)]
pub struct uac_clock_selector_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bClockID:u8, pub bNrInPins:u8, pub baCSourceID:[u8; 0] }
#[repr(C, packed)]
pub struct uac_clock_multiplier_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bClockID:u8, pub bCSourceID:u8, pub bmControls:u8, pub iClockMultiplier:u8 }
#[repr(C, packed)]
pub struct uac2_input_terminal_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bTerminalID:u8, pub wTerminalType:u16, pub bAssocTerminal:u8, pub bCSourceID:u8, pub bNrChannels:u8, pub bmChannelConfig:u32, pub iChannelNames:u8, pub bmControls:u16, pub iTerminal:u8 }
#[repr(C, packed)]
pub struct uac2_output_terminal_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bTerminalID:u8, pub wTerminalType:u16, pub bAssocTerminal:u8, pub bSourceID:u8, pub bCSourceID:u8, pub bmControls:u16, pub iTerminal:u8 }
#[repr(C, packed)]
pub struct uac2_feature_unit_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bUnitID:u8, pub bSourceID:u8, pub bmaControls:[u8; 0] }
pub const fn UAC2_DT_FEATURE_UNIT_SIZE(ch: usize) -> usize { 6 + (ch + 1) * 4 }
#[repr(C, packed)]
pub struct uac2_effect_unit_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bUnitID:u8, pub wEffectType:u16, pub bSourceID:u8, pub bmaControls:[u8; 0] }
#[repr(C, packed)]
pub struct uac2_as_header_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bTerminalLink:u8, pub bmControls:u8, pub bFormatType:u8, pub bmFormats:u32, pub bNrChannels:u8, pub bmChannelConfig:u32, pub iChannelNames:u8 }
#[repr(C, packed)]
pub struct uac2_iso_endpoint_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bmAttributes:u8, pub bmControls:u8, pub bLockDelayUnits:u8, pub wLockDelay:u16 }
#[repr(C, packed)]
pub struct uac2_connectors_ctl_blk { pub bNrChannels:u8, pub bmChannelConfig:u32, pub iChannelNames:u8 }
#[repr(C, packed)]
pub struct uac2_interrupt_data_msg { pub bInfo:u8, pub bAttribute:u8, pub wValue:u16, pub wIndex:u16 }

pub const UAC_CLOCK_SOURCE_TYPE_EXT:u8=0x0; pub const UAC_CLOCK_SOURCE_TYPE_INT_FIXED:u8=0x1; pub const UAC_CLOCK_SOURCE_TYPE_INT_VAR:u8=0x2; pub const UAC_CLOCK_SOURCE_TYPE_INT_PROG:u8=0x3; pub const UAC_CLOCK_SOURCE_SYNCED_TO_SOF:u8=1<<2;
pub const UAC2_FORMAT_TYPE_I_RAW_DATA:u32=1<<31;
pub const UAC2_CONTROL_PITCH:u8=3<<0; pub const UAC2_CONTROL_DATA_OVERRUN:u8=3<<2; pub const UAC2_CONTROL_DATA_UNDERRUN:u8=3<<4;
pub const UAC2_INTERRUPT_DATA_MSG_VENDOR:u8=1<<0; pub const UAC2_INTERRUPT_DATA_MSG_EP:u8=1<<1;

pub const UAC2_FUNCTION_SUBCLASS_UNDEFINED:u8=0x00; pub const UAC2_FUNCTION_DESKTOP_SPEAKER:u8=0x01; pub const UAC2_FUNCTION_HOME_THEATER:u8=0x02; pub const UAC2_FUNCTION_MICROPHONE:u8=0x03; pub const UAC2_FUNCTION_HEADSET:u8=0x04; pub const UAC2_FUNCTION_TELEPHONE:u8=0x05; pub const UAC2_FUNCTION_CONVERTER:u8=0x06; pub const UAC2_FUNCTION_SOUND_RECORDER:u8=0x07; pub const UAC2_FUNCTION_IO_BOX:u8=0x08; pub const UAC2_FUNCTION_MUSICAL_INSTRUMENT:u8=0x09; pub const UAC2_FUNCTION_PRO_AUDIO:u8=0x0a; pub const UAC2_FUNCTION_AUDIO_VIDEO:u8=0x0b; pub const UAC2_FUNCTION_CONTROL_PANEL:u8=0x0c; pub const UAC2_FUNCTION_OTHER:u8=0xff;
pub const UAC2_EFFECT_UNIT:u8=7; pub const UAC2_PROCESSING_UNIT_V2:u8=8; pub const UAC2_EXTENSION_UNIT_V2:u8=9; pub const UAC2_CLOCK_SOURCE:u8=0x0a; pub const UAC2_CLOCK_SELECTOR:u8=0x0b; pub const UAC2_CLOCK_MULTIPLIER:u8=0x0c; pub const UAC2_SAMPLE_RATE_CONVERTER:u8=0x0d; pub const UAC2_ENCODER:u8=3; pub const UAC2_DECODER:u8=4;
pub const UAC2_EFFECT_UNDEFINED:u8=0; pub const UAC2_EFFECT_PARAM_EQ:u8=1; pub const UAC2_EFFECT_REVERB:u8=2; pub const UAC2_EFFECT_MOD_DELAY:u8=3; pub const UAC2_EFFECT_DYN_RANGE_COMP:u8=4;
pub const UAC2_PROCESS_UNDEFINED:u8=0; pub const UAC2_PROCESS_UP_DOWNMIX:u8=1; pub const UAC2_PROCESS_DOLBY_PROLOCIC:u8=2; pub const UAC2_PROCESS_STEREO_EXTENDER:u8=3; pub const UAC2_CS_CUR:u8=1; pub const UAC2_CS_RANGE:u8=2; pub const UAC2_CS_MEM:u8=3;
pub const UAC2_ENCODER_UNDEFINED:u8=0; pub const UAC2_ENCODER_OTHER:u8=1; pub const UAC2_ENCODER_MPEG:u8=2; pub const UAC2_ENCODER_AC3:u8=3; pub const UAC2_ENCODER_WMA:u8=4; pub const UAC2_ENCODER_DTS:u8=5; pub const UAC2_DECODER_UNDEFINED:u8=0; pub const UAC2_DECODER_OTHER:u8=1; pub const UAC2_DECODER_MPEG:u8=2; pub const UAC2_DECODER_AC3:u8=3; pub const UAC2_DECODER_WMA:u8=4; pub const UAC2_DECODER_DTS:u8=5;
pub const UAC2_CS_UNDEFINED:u8=0; pub const UAC2_CS_CONTROL_SAM_FREQ:u8=1; pub const UAC2_CS_CONTROL_CLOCK_VALID:u8=2; pub const UAC2_CX_UNDEFINED:u8=0; pub const UAC2_CX_CLOCK_SELECTOR:u8=1; pub const UAC2_CM_UNDEFINED:u8=0; pub const UAC2_CM_NUMERATOR:u8=1; pub const UAC2_CM_DENOMINTATOR:u8=2;
pub const UAC2_TE_UNDEFINED:u8=0; pub const UAC2_TE_COPY_PROTECT:u8=1; pub const UAC2_TE_CONNECTOR:u8=2; pub const UAC2_TE_OVERLOAD:u8=3; pub const UAC2_TE_CLUSTER:u8=4; pub const UAC2_TE_UNDERFLOW:u8=5; pub const UAC2_TE_OVERFLOW:u8=6; pub const UAC2_TE_LATENCY:u8=7;
pub const UAC2_MU_UNDEFINED:u8=0; pub const UAC2_MU_MIXER:u8=1; pub const UAC2_MU_CLUSTER:u8=2; pub const UAC2_MU_UNDERFLOW:u8=3; pub const UAC2_MU_OVERFLOW:u8=4; pub const UAC2_MU_LATENCY:u8=5; pub const UAC2_SU_UNDEFINED:u8=0; pub const UAC2_SU_SELECTOR:u8=1; pub const UAC2_SU_LATENCY:u8=2;
pub const UAC2_FU_INPUT_GAIN:u8=0x0b; pub const UAC2_FU_INPUT_GAIN_PAD:u8=0x0c; pub const UAC2_FU_PHASE_INVERTER:u8=0x0d; pub const UAC2_FU_UNDERFLOW:u8=0x0e; pub const UAC2_FU_OVERFLOW:u8=0x0f; pub const UAC2_FU_LATENCY:u8=0x10;
pub const UAC2_PE_UNDEFINED:u8=0; pub const UAC2_PE_ENABLE:u8=1; pub const UAC2_PE_CENTERFREQ:u8=2; pub const UAC2_PE_QFACTOR:u8=3; pub const UAC2_PE_GAIN:u8=4; pub const UAC2_PE_UNDERFLOW:u8=5; pub const UAC2_PE_OVERFLOW:u8=6; pub const UAC2_PE_LATENCY:u8=7;
pub const UAC2_RV_UNDEFINED:u8=0; pub const UAC2_RV_ENABLE:u8=1; pub const UAC2_RV_TYPE:u8=2; pub const UAC2_RV_LEVEL:u8=3; pub const UAC2_RV_TIME:u8=4; pub const UAC2_RV_FEEDBACK:u8=5; pub const UAC2_RV_PREDELAY:u8=6; pub const UAC2_RV_DENSITY:u8=7; pub const UAC2_RV_HIFREQ_ROLLOFF:u8=8; pub const UAC2_RV_UNDERFLOW:u8=9; pub const UAC2_RV_OVERFLOW:u8=10; pub const UAC2_RV_LATENCY:u8=11;
pub const UAC2_MD_UNDEFINED:u8=0; pub const UAC2_MD_ENABLE:u8=1; pub const UAC2_MD_BALANCE:u8=2; pub const UAC2_MD_RATE:u8=3; pub const UAC2_MD_DEPTH:u8=4; pub const UAC2_MD_TIME:u8=5; pub const UAC2_MD_FEEDBACK:u8=6; pub const UAC2_MD_UNDERFLOW:u8=7; pub const UAC2_MD_OVERFLOW:u8=8; pub const UAC2_MD_LATENCY:u8=9;
pub const UAC2_DR_UNDEFINED:u8=0; pub const UAC2_DR_ENABLE:u8=1; pub const UAC2_DR_COMPRESSION_RATE:u8=2; pub const UAC2_DR_MAXAMPL:u8=3; pub const UAC2_DR_THRESHOLD:u8=4; pub const UAC2_DR_ATTACK_TIME:u8=5; pub const UAC2_DR_RELEASE_TIME:u8=6; pub const UAC2_DR_UNDEFLOW:u8=7; pub const UAC2_DR_OVERFLOW:u8=8; pub const UAC2_DR_LATENCY:u8=9;
pub const UAC2_UD_UNDEFINED:u8=0; pub const UAC2_UD_ENABLE:u8=1; pub const UAC2_UD_MODE_SELECT:u8=2; pub const UAC2_UD_CLUSTER:u8=3; pub const UAC2_UD_UNDERFLOW:u8=4; pub const UAC2_UD_OVERFLOW:u8=5; pub const UAC2_UD_LATENCY:u8=6; pub const UAC2_DP_UNDEFINED:u8=0; pub const UAC2_DP_ENABLE:u8=1; pub const UAC2_DP_MODE_SELECT:u8=2; pub const UAC2_DP_CLUSTER:u8=3; pub const UAC2_DP_UNDERFFLOW:u8=4; pub const UAC2_DP_OVERFLOW:u8=5; pub const UAC2_DP_LATENCY:u8=6;
pub const UAC2_ST_EXT_UNDEFINED:u8=0; pub const UAC2_ST_EXT_ENABLE:u8=1; pub const UAC2_ST_EXT_WIDTH:u8=2; pub const UAC2_ST_EXT_UNDEFLOW:u8=3; pub const UAC2_ST_EXT_OVERFLOW:u8=4; pub const UAC2_ST_EXT_LATENCY:u8=5; pub const UAC2_XU_UNDEFINED:u8=0; pub const UAC2_XU_ENABLE:u8=1; pub const UAC2_XU_CLUSTER:u8=2; pub const UAC2_XU_UNDERFLOW:u8=3; pub const UAC2_XU_OVERFLOW:u8=4; pub const UAC2_XU_LATENCY:u8=5;
pub const UAC2_AS_UNDEFINED:u8=0; pub const UAC2_AS_ACT_ALT_SETTING:u8=1; pub const UAC2_AS_VAL_ALT_SETTINGS:u8=2; pub const UAC2_AS_AUDIO_DATA_FORMAT:u8=3;
pub const UAC2_EP_CS_UNDEFINED:u8=0; pub const UAC2_EP_CS_PITCH:u8=1; pub const UAC2_EP_CS_DATA_OVERRUN:u8=2; pub const UAC2_EP_CS_DATA_UNDERRUN:u8=3;

#[macro_export]
macro_rules! DECLARE_UAC2_FEATURE_UNIT_DESCRIPTOR { ($name:ident, $ch:expr) => {
    #[repr(C, packed)] pub struct $name { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubtype:u8, pub bUnitID:u8, pub bSourceID:u8, pub bmaControls:[u32; $ch + 1], pub iFeature:u8 }
} }
pub const UAC2_EN_UNDEFINED:u8=0; pub const UAC2_EN_BIT_RATE:u8=1; pub const UAC2_EN_QUALITY:u8=2; pub const UAC2_EN_VBR:u8=3; pub const UAC2_EN_TYPE:u8=4; pub const UAC2_EN_UNDERFLOW:u8=5; pub const UAC2_EN_OVERFLOW:u8=6; pub const UAC2_EN_ENCODER_ERROR:u8=7; pub const UAC2_EN_PARAM1:u8=8; pub const UAC2_EN_PARAM2:u8=9; pub const UAC2_EN_PARAM3:u8=10; pub const UAC2_EN_PARAM4:u8=11; pub const UAC2_EN_PARAM5:u8=12; pub const UAC2_EN_PARAM6:u8=13; pub const UAC2_EN_PARAM7:u8=14; pub const UAC2_EN_PARAM8:u8=15;
pub const UAC2_MPEG_UNDEFINED:u8=0; pub const UAC2_MPEG_DUAL_CHANNEL:u8=1; pub const UAC2_MPEG_SECOND_STEREO:u8=2; pub const UAC2_MPEG_MULTILINGUAL:u8=3; pub const UAC2_MPEG_DYN_RANGE:u8=4; pub const UAC2_MPEG_SCALING:u8=5; pub const UAC2_MPEG_HILO_SCALING:u8=6; pub const UAC2_MPEG_UNDERFLOW:u8=7; pub const UAC2_MPEG_OVERFLOW:u8=8; pub const UAC2_MPEG_DECODER_ERROR:u8=9;
pub const UAC2_AC3_UNDEFINED:u8=0; pub const UAC2_AC3_MODE:u8=1; pub const UAC2_AC3_DYN_RANGE:u8=2; pub const UAC2_AC3_SCALING:u8=3; pub const UAC2_AC3_HILO_SCALING:u8=4; pub const UAC2_AC3_UNDERFLOW:u8=5; pub const UAC2_AC3_OVERFLOW:u8=6; pub const UAC2_AC3_DECODER_ERROR:u8=7; pub const UAC2_WMA_UNDEFINED:u8=0; pub const UAC2_WMA_UNDERFLOW:u8=1; pub const UAC2_WMA_OVERFLOW:u8=2; pub const UAC2_WMA_DECODER_ERROR:u8=3; pub const UAC2_DTS_UNDEFINED:u8=0; pub const UAC2_DTS_UNDERFLOW:u8=1; pub const UAC2_DTS_OVERFLOW:u8=2; pub const UAC2_DTS_DECODER_ERROR:u8=3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
