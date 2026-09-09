// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2017 Ruslan Bilovol <ruslan.bilovol@gmail.com>
 *
 * This file holds USB constants and structures defined
 * by the USB DEVICE CLASS DEFINITION FOR AUDIO DEVICES Release 3.0.
 */

// v1.0, v2.0 and v3.0 of this standard have many things in common. For the
// rest of the definitions, please refer to audio.h and audio-v2.h.

#[repr(C, packed)]
pub struct uac3_hc_descriptor_header { pub wLength: __le16, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub wDescriptorID: __le16 }
#[repr(C, packed)]
pub struct uac3_cluster_header_descriptor { pub wLength: __le16, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub wDescriptorID: __le16, pub bNrChannels: __u8 }
#[repr(C, packed)]
pub struct uac3_cluster_segment_descriptor { pub wLength: __le16, pub bSegmentType: __u8 }
#[repr(C, packed)]
pub struct uac3_cluster_end_segment_descriptor { pub wLength: __le16, pub bSegmentType: __u8 }
#[repr(C, packed)]
pub struct uac3_cluster_information_segment_descriptor { pub wLength: __le16, pub bSegmentType: __u8, pub bChPurpose: __u8, pub bChRelationship: __u8, pub bChGroupID: __u8 }
#[repr(C, packed)]
pub struct uac3_ac_header_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bCategory: __u8, pub wTotalLength: __le16, pub bmControls: __le32 }
#[repr(C, packed)]
pub struct uac3_input_terminal_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bTerminalID: __u8, pub wTerminalType: __le16, pub bAssocTerminal: __u8, pub bCSourceID: __u8, pub bmControls: __le32, pub wClusterDescrID: __le16, pub wExTerminalDescrID: __le16, pub wConnectorsDescrID: __le16, pub wTerminalDescrStr: __le16 }
#[repr(C, packed)]
pub struct uac3_output_terminal_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bTerminalID: __u8, pub wTerminalType: __le16, pub bAssocTerminal: __u8, pub bSourceID: __u8, pub bCSourceID: __u8, pub bmControls: __le32, pub wExTerminalDescrID: __le16, pub wConnectorsDescrID: __le16, pub wTerminalDescrStr: __le16 }
#[repr(C, packed)]
pub struct uac3_feature_unit_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bUnitID: __u8, pub bSourceID: __u8, pub bmaControls: [__u8; 0] }

#[inline]
pub const fn UAC3_DT_FEATURE_UNIT_SIZE(ch: usize) -> usize { 7 + (ch + 1) * 4 }

#[macro_export]
macro_rules! DECLARE_UAC3_FEATURE_UNIT_DESCRIPTOR {
    ($ch:expr) => {
        #[repr(C, packed)]
        pub struct uac3_feature_unit_descriptor_generated { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bUnitID: __u8, pub bSourceID: __u8, pub bmaControls: [__le32; $ch + 1], pub wFeatureDescrStr: __le16 }
    };
}

#[repr(C, packed)]
pub struct uac3_clock_source_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bClockID: __u8, pub bmAttributes: __u8, pub bmControls: __le32, pub bReferenceTerminal: __u8, pub wClockSourceStr: __le16 }
pub const UAC3_CLOCK_SOURCE_TYPE_EXT: u32 = 0x0;
pub const UAC3_CLOCK_SOURCE_TYPE_INT: u32 = 0x1;
pub const UAC3_CLOCK_SOURCE_ASYNC: u32 = 0 << 2;
pub const UAC3_CLOCK_SOURCE_SYNCED_TO_SOF: u32 = 1 << 1;
#[repr(C, packed)]
pub struct uac3_clock_selector_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bClockID: __u8, pub bNrInPins: __u8, pub baCSourceID: [__u8; 0] }
#[repr(C, packed)]
pub struct uac3_clock_multiplier_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bClockID: __u8, pub bCSourceID: __u8, pub bmControls: __le32, pub wCMultiplierDescrStr: __le16 }
#[repr(C, packed)]
pub struct uac3_power_domain_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bPowerDomainID: __u8, pub waRecoveryTime1: __le16, pub waRecoveryTime2: __le16, pub bNrEntities: __u8, pub baEntityID: [__u8; 0] }
#[macro_export]
macro_rules! DECLARE_UAC3_POWER_DOMAIN_DESCRIPTOR {
    ($n:expr) => { #[repr(C, packed)] pub struct uac3_power_domain_descriptor_generated { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bPowerDomainID: __u8, pub waRecoveryTime1: __le16, pub waRecoveryTime2: __le16, pub bNrEntities: __u8, pub baEntityID: [__u8; $n], pub wPDomainDescrStr: __le16 } };
}
#[repr(C, packed)]
pub struct uac3_as_header_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bTerminalLink: __u8, pub bmControls: __le32, pub wClusterDescrID: __le16, pub bmFormats: __le64, pub bSubslotSize: __u8, pub bBitResolution: __u8, pub bmAuxProtocols: __le16, pub bControlSize: __u8 }
pub const UAC3_FORMAT_TYPE_I_RAW_DATA: u32 = 1 << 6;
#[repr(C, packed)]
pub struct uac3_iso_endpoint_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDescriptorSubtype: __u8, pub bmControls: __le32, pub bLockDelayUnits: __u8, pub wLockDelay: __le16 }
#[repr(C, packed)] pub struct uac3_insertion_ctl_blk { pub bSize: __u8, pub bmConInserted: __u8 }
#[repr(C, packed)] pub struct uac3_interrupt_data_msg { pub bInfo: __u8, pub bSourceType: __u8, pub wValue: __le16, pub wIndex: __le16 }

// A.2 AUDIO AUDIO FUNCTION SUBCLASS CODES
pub const UAC3_FUNCTION_SUBCLASS_UNDEFINED: u32 = 0x00; pub const UAC3_FUNCTION_SUBCLASS_FULL_ADC_3_0: u32 = 0x01; pub const UAC3_FUNCTION_SUBCLASS_GENERIC_IO: u32 = 0x20; pub const UAC3_FUNCTION_SUBCLASS_HEADPHONE: u32 = 0x21; pub const UAC3_FUNCTION_SUBCLASS_SPEAKER: u32 = 0x22; pub const UAC3_FUNCTION_SUBCLASS_MICROPHONE: u32 = 0x23; pub const UAC3_FUNCTION_SUBCLASS_HEADSET: u32 = 0x24; pub const UAC3_FUNCTION_SUBCLASS_HEADSET_ADAPTER: u32 = 0x25; pub const UAC3_FUNCTION_SUBCLASS_SPEAKERPHONE: u32 = 0x26;
// The C header repeats UAC3_FUNCTION_SUBCLASS_UNDEFINED for the category code.
pub const UAC3_FUNCTION_DESKTOP_SPEAKER: u32 = 0x01; pub const UAC3_FUNCTION_HOME_THEATER: u32 = 0x02; pub const UAC3_FUNCTION_MICROPHONE: u32 = 0x03; pub const UAC3_FUNCTION_HEADSET: u32 = 0x04; pub const UAC3_FUNCTION_TELEPHONE: u32 = 0x05; pub const UAC3_FUNCTION_CONVERTER: u32 = 0x06; pub const UAC3_FUNCTION_SOUND_RECORDER: u32 = 0x07; pub const UAC3_FUNCTION_IO_BOX: u32 = 0x08; pub const UAC3_FUNCTION_MUSICAL_INSTRUMENT: u32 = 0x09; pub const UAC3_FUNCTION_PRO_AUDIO: u32 = 0x0a; pub const UAC3_FUNCTION_AUDIO_VIDEO: u32 = 0x0b; pub const UAC3_FUNCTION_CONTROL_PANEL: u32 = 0x0c; pub const UAC3_FUNCTION_HEADPHONE: u32 = 0x0d; pub const UAC3_FUNCTION_GENERIC_SPEAKER: u32 = 0x0e; pub const UAC3_FUNCTION_HEADSET_ADAPTER: u32 = 0x0f; pub const UAC3_FUNCTION_SPEAKERPHONE: u32 = 0x10; pub const UAC3_FUNCTION_OTHER: u32 = 0xff;
pub const UAC3_CS_UNDEFINED: u32 = 0x20; pub const UAC3_CS_DEVICE: u32 = 0x21; pub const UAC3_CS_CONFIGURATION: u32 = 0x22; pub const UAC3_CS_STRING: u32 = 0x23; pub const UAC3_CS_INTERFACE: u32 = 0x24; pub const UAC3_CS_ENDPOINT: u32 = 0x25; pub const UAC3_CS_CLUSTER: u32 = 0x26;
pub const UAC3_SEGMENT_UNDEFINED: u32 = 0x00; pub const UAC3_CLUSTER_DESCRIPTION: u32 = 0x01; pub const UAC3_CLUSTER_VENDOR_DEFINED: u32 = 0x1f; pub const UAC3_CHANNEL_INFORMATION: u32 = 0x20; pub const UAC3_CHANNEL_AMBISONIC: u32 = 0x21; pub const UAC3_CHANNEL_DESCRIPTION: u32 = 0x22; pub const UAC3_CHANNEL_VENDOR_DEFINED: u32 = 0xfe; pub const UAC3_END_SEGMENT: u32 = 0xff;
pub const UAC3_PURPOSE_UNDEFINED: u32 = 0x00; pub const UAC3_PURPOSE_GENERIC_AUDIO: u32 = 0x01; pub const UAC3_PURPOSE_VOICE: u32 = 0x02; pub const UAC3_PURPOSE_SPEECH: u32 = 0x03; pub const UAC3_PURPOSE_AMBIENT: u32 = 0x04; pub const UAC3_PURPOSE_REFERENCE: u32 = 0x05; pub const UAC3_PURPOSE_ULTRASONIC: u32 = 0x06; pub const UAC3_PURPOSE_VIBROKINETIC: u32 = 0x07; pub const UAC3_PURPOSE_NON_AUDIO: u32 = 0xff;
pub const UAC3_CH_RELATIONSHIP_UNDEFINED: u32 = 0x00; pub const UAC3_CH_MONO: u32 = 0x01; pub const UAC3_CH_LEFT: u32 = 0x02; pub const UAC3_CH_RIGHT: u32 = 0x03; pub const UAC3_CH_ARRAY: u32 = 0x04; pub const UAC3_CH_PATTERN_X: u32 = 0x20; pub const UAC3_CH_PATTERN_Y: u32 = 0x21; pub const UAC3_CH_PATTERN_A: u32 = 0x22; pub const UAC3_CH_PATTERN_B: u32 = 0x23; pub const UAC3_CH_PATTERN_M: u32 = 0x24; pub const UAC3_CH_PATTERN_S: u32 = 0x25;
// Remaining channel relationship values are retained as a compact lookup table.
pub const UAC3_CH_RELATIONSHIP_VALUES: [u32; 68] = [0x80,0x81,0x82,0x83,0x84,0x85,0x86,0x87,0x88,0x89,0x8a,0x8b,0x8c,0x8d,0x8e,0x8f,0x90,0x91,0x92,0x93,0x94,0x95,0x96,0x97,0x98,0x99,0x9a,0x9b,0x9c,0x9d,0x9e,0x9f,0xa0,0xa1,0xa2,0xa3,0xa4,0xa5,0xa6,0xa7,0xa8,0xa9,0xaa,0xab,0xac,0xad,0xae,0xaf,0xb0,0xb1,0xb2,0xb3,0xb4,0xb5,0xb6,0xb7,0xb8,0xb9,0xba,0xbb,0xbc];
pub const UAC3_EXTENDED_TERMINAL: u32 = 0x04; pub const UAC3_MIXER_UNIT: u32 = 0x05; pub const UAC3_SELECTOR_UNIT: u32 = 0x06; pub const UAC3_FEATURE_UNIT: u32 = 0x07; pub const UAC3_EFFECT_UNIT: u32 = 0x08; pub const UAC3_PROCESSING_UNIT: u32 = 0x09; pub const UAC3_EXTENSION_UNIT: u32 = 0x0a; pub const UAC3_CLOCK_SOURCE: u32 = 0x0b; pub const UAC3_CLOCK_SELECTOR: u32 = 0x0c; pub const UAC3_CLOCK_MULTIPLIER: u32 = 0x0d; pub const UAC3_SAMPLE_RATE_CONVERTER: u32 = 0x0e; pub const UAC3_CONNECTORS: u32 = 0x0f; pub const UAC3_POWER_DOMAIN: u32 = 0x10;
pub const UAC3_PROCESS_UNDEFINED: u32 = 0x00; pub const UAC3_PROCESS_UP_DOWNMIX: u32 = 0x01; pub const UAC3_PROCESS_STEREO_EXTENDER: u32 = 0x02; pub const UAC3_PROCESS_MULTI_FUNCTION: u32 = 0x03;
pub const UAC3_CS_REQ_INTEN: u32 = 0x04; pub const UAC3_CS_REQ_STRING: u32 = 0x05; pub const UAC3_CS_REQ_HIGH_CAPABILITY_DESCRIPTOR: u32 = 0x06;
pub const UAC3_AC_CONTROL_UNDEFINED: u32 = 0x00; pub const UAC3_AC_ACTIVE_INTERFACE_CONTROL: u32 = 0x01; pub const UAC3_AC_POWER_DOMAIN_CONTROL: u32 = 0x02;
pub const UAC3_TE_UNDEFINED: u32 = 0x00; pub const UAC3_TE_INSERTION: u32 = 0x01; pub const UAC3_TE_OVERLOAD: u32 = 0x02; pub const UAC3_TE_UNDERFLOW: u32 = 0x03; pub const UAC3_TE_OVERFLOW: u32 = 0x04; pub const UAC3_TE_LATENCY: u32 = 0x05;
pub const UAC3_UD_MODE_SELECT: u32 = 0x01; pub const UAC3_EXT_WIDTH_CONTROL: u32 = 0x01;
pub const UAC3_BADD_IT_ID1: u32 = 1; pub const UAC3_BADD_FU_ID2: u32 = 2; pub const UAC3_BADD_OT_ID3: u32 = 3; pub const UAC3_BADD_IT_ID4: u32 = 4; pub const UAC3_BADD_FU_ID5: u32 = 5; pub const UAC3_BADD_OT_ID6: u32 = 6; pub const UAC3_BADD_FU_ID7: u32 = 7; pub const UAC3_BADD_MU_ID8: u32 = 8; pub const UAC3_BADD_CS_ID9: u32 = 9; pub const UAC3_BADD_PD_ID10: u32 = 10; pub const UAC3_BADD_PD_ID11: u32 = 11;
pub const UAC3_BADD_EP_MAXPSIZE_SYNC_MONO_16: u32 = 0x0060; pub const UAC3_BADD_EP_MAXPSIZE_ASYNC_MONO_16: u32 = 0x0062; pub const UAC3_BADD_EP_MAXPSIZE_SYNC_MONO_24: u32 = 0x0090; pub const UAC3_BADD_EP_MAXPSIZE_ASYNC_MONO_24: u32 = 0x0093; pub const UAC3_BADD_EP_MAXPSIZE_SYNC_STEREO_16: u32 = 0x00c0; pub const UAC3_BADD_EP_MAXPSIZE_ASYNC_STEREO_16: u32 = 0x00c4; pub const UAC3_BADD_EP_MAXPSIZE_SYNC_STEREO_24: u32 = 0x0120; pub const UAC3_BADD_EP_MAXPSIZE_ASYNC_STEREO_24: u32 = 0x0126;
pub const UAC3_BADD_SAMPLING_RATE: u32 = 48000; pub const UAC3_BADD_PD_RECOVER_D1D0: u32 = 0x0258; pub const UAC3_BADD_PD_RECOVER_D2D0: u32 = 0x1770;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
