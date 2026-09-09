/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of <linux/usb/audio.h>. */

pub const UAC_VERSION_1: u8 = 0x00; pub const UAC_VERSION_2: u8 = 0x20; pub const UAC_VERSION_3: u8 = 0x30;
pub const USB_SUBCLASS_AUDIOCONTROL: u8 = 0x01; pub const USB_SUBCLASS_AUDIOSTREAMING: u8 = 0x02; pub const USB_SUBCLASS_MIDISTREAMING: u8 = 0x03;
pub const UAC_HEADER: u8 = 0x01; pub const UAC_INPUT_TERMINAL: u8 = 0x02; pub const UAC_OUTPUT_TERMINAL: u8 = 0x03; pub const UAC_MIXER_UNIT: u8 = 0x04; pub const UAC_SELECTOR_UNIT: u8 = 0x05; pub const UAC_FEATURE_UNIT: u8 = 0x06; pub const UAC1_PROCESSING_UNIT: u8 = 0x07; pub const UAC1_EXTENSION_UNIT: u8 = 0x08;
pub const UAC_AS_GENERAL: u8 = 1; pub const UAC_FORMAT_TYPE: u8 = 2; pub const UAC_FORMAT_SPECIFIC: u8 = 3;
pub const UAC_PROCESS_UNDEFINED: u8=0; pub const UAC_PROCESS_UP_DOWNMIX:u8=1; pub const UAC_PROCESS_DOLBY_PROLOGIC:u8=2; pub const UAC_PROCESS_STEREO_EXTENDER:u8=3; pub const UAC_PROCESS_REVERB:u8=4; pub const UAC_PROCESS_CHORUS:u8=5; pub const UAC_PROCESS_DYN_RANGE_COMP:u8=6; pub const UAC_EP_GENERAL:u8=1;
pub const UAC_SET_:u8=0; pub const UAC_GET_:u8=0x80; pub const UAC__CUR:u8=1; pub const UAC__MIN:u8=2; pub const UAC__MAX:u8=3; pub const UAC__RES:u8=4; pub const UAC__MEM:u8=5;
pub const UAC_SET_CUR:u8=1; pub const UAC_GET_CUR:u8=0x81; pub const UAC_SET_MIN:u8=2; pub const UAC_GET_MIN:u8=0x82; pub const UAC_SET_MAX:u8=3; pub const UAC_GET_MAX:u8=0x83; pub const UAC_SET_RES:u8=4; pub const UAC_GET_RES:u8=0x84; pub const UAC_SET_MEM:u8=5; pub const UAC_GET_MEM:u8=0x85; pub const UAC_GET_STAT:u8=0xff;
pub const UAC_TERM_COPY_PROTECT:u8=1; pub const UAC_FU_MUTE:u8=1; pub const UAC_FU_VOLUME:u8=2; pub const UAC_FU_BASS:u8=3; pub const UAC_FU_MID:u8=4; pub const UAC_FU_TREBLE:u8=5; pub const UAC_FU_GRAPHIC_EQUALIZER:u8=6; pub const UAC_FU_AUTOMATIC_GAIN:u8=7; pub const UAC_FU_DELAY:u8=8; pub const UAC_FU_BASS_BOOST:u8=9; pub const UAC_FU_LOUDNESS:u8=10;
#[inline] pub const fn UAC_CONTROL_BIT(cs:u8)->u32 { 1u32 << (cs-1) }
pub const UAC_UD_ENABLE:u8=1; pub const UAC_UD_MODE_SELECT:u8=2; pub const UAC_DP_ENABLE:u8=1; pub const UAC_DP_MODE_SELECT:u8=2; pub const UAC_3D_ENABLE:u8=1; pub const UAC_3D_SPACE:u8=2; pub const UAC_REVERB_ENABLE:u8=1; pub const UAC_REVERB_LEVEL:u8=2; pub const UAC_REVERB_TIME:u8=3; pub const UAC_REVERB_FEEDBACK:u8=4; pub const UAC_CHORUS_ENABLE:u8=1; pub const UAC_CHORUS_LEVEL:u8=2; pub const UAC_CHORUS_RATE:u8=3; pub const UAC_CHORUS_DEPTH:u8=4; pub const UAC_DCR_ENABLE:u8=1; pub const UAC_DCR_RATE:u8=2; pub const UAC_DCR_MAXAMPL:u8=3; pub const UAC_DCR_THRESHOLD:u8=4; pub const UAC_DCR_ATTACK_TIME:u8=5; pub const UAC_DCR_RELEASE_TIME:u8=6; pub const UAC_XU_ENABLE:u8=1;
pub const UAC_MS_HEADER:u8=1; pub const UAC_MIDI_IN_JACK:u8=2; pub const UAC_MIDI_OUT_JACK:u8=3; pub const UAC_MS_GENERAL:u8=1;
pub const UAC_TERMINAL_UNDEFINED:u16=0x100; pub const UAC_TERMINAL_STREAMING:u16=0x101; pub const UAC_TERMINAL_VENDOR_SPEC:u16=0x1ff;

#[repr(C, packed)] pub struct uac1_ac_header_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bcdADC:u16,pub wTotalLength:u16,pub bInCollection:u8,pub baInterfaceNr:[u8;0] }
pub const fn UAC_DT_AC_HEADER_SIZE(n:usize)->usize {8+n}
#[repr(C, packed)] pub struct uac_input_terminal_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bTerminalID:u8,pub wTerminalType:u16,pub bAssocTerminal:u8,pub bNrChannels:u8,pub wChannelConfig:u16,pub iChannelNames:u8,pub iTerminal:u8 }
pub const UAC_DT_INPUT_TERMINAL_SIZE:usize=12;
pub const UAC_INPUT_TERMINAL_UNDEFINED:u16=0x200; pub const UAC_INPUT_TERMINAL_MICROPHONE:u16=0x201; pub const UAC_INPUT_TERMINAL_DESKTOP_MICROPHONE:u16=0x202; pub const UAC_INPUT_TERMINAL_PERSONAL_MICROPHONE:u16=0x203; pub const UAC_INPUT_TERMINAL_OMNI_DIR_MICROPHONE:u16=0x204; pub const UAC_INPUT_TERMINAL_MICROPHONE_ARRAY:u16=0x205; pub const UAC_INPUT_TERMINAL_PROC_MICROPHONE_ARRAY:u16=0x206; pub const UAC_TERMINAL_CS_COPY_PROTECT_CONTROL:u8=1;
#[repr(C, packed)] pub struct uac1_output_terminal_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bTerminalID:u8,pub wTerminalType:u16,pub bAssocTerminal:u8,pub bSourceID:u8,pub iTerminal:u8 }
pub const UAC_DT_OUTPUT_TERMINAL_SIZE:usize=9;
pub const UAC_OUTPUT_TERMINAL_UNDEFINED:u16=0x300; pub const UAC_OUTPUT_TERMINAL_SPEAKER:u16=0x301; pub const UAC_OUTPUT_TERMINAL_HEADPHONES:u16=0x302; pub const UAC_OUTPUT_TERMINAL_HEAD_MOUNTED_DISPLAY_AUDIO:u16=0x303; pub const UAC_OUTPUT_TERMINAL_DESKTOP_SPEAKER:u16=0x304; pub const UAC_OUTPUT_TERMINAL_ROOM_SPEAKER:u16=0x305; pub const UAC_OUTPUT_TERMINAL_COMMUNICATION_SPEAKER:u16=0x306; pub const UAC_OUTPUT_TERMINAL_LOW_FREQ_EFFECTS_SPEAKER:u16=0x307;
pub const UAC_BIDIR_TERMINAL_UNDEFINED:u16=0x400; pub const UAC_BIDIR_TERMINAL_HANDSET:u16=0x401; pub const UAC_BIDIR_TERMINAL_HEADSET:u16=0x402; pub const UAC_BIDIR_TERMINAL_SPEAKER_PHONE:u16=0x403; pub const UAC_BIDIR_TERMINAL_ECHO_SUPPRESSING:u16=0x404; pub const UAC_BIDIR_TERMINAL_ECHO_CANCELING:u16=0x405;
pub const fn UAC_DT_FEATURE_UNIT_SIZE(ch:usize)->usize { 7+(ch+1)*2 }

#[repr(C, packed)] pub struct uac_mixer_unit_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bUnitID:u8,pub bNrInPins:u8,pub baSourceID:[u8;0] }
#[inline] pub unsafe fn uac_mixer_unit_bNrChannels(d:*mut uac_mixer_unit_descriptor)->u8 { *((d as *mut u8).add(5+(*d).bNrInPins as usize)) }
#[inline] pub unsafe fn uac_mixer_unit_wChannelConfig(d:*mut uac_mixer_unit_descriptor,p:i32)->u32 { let b=d as *mut u8; let n=(*d).bNrInPins as usize; if p==0x00 { ((*b.add(5+n+2) as u32)<<8)|*b.add(5+n+1) as u32 } else { ((*b.add(5+n+4) as u32)<<24)|((*b.add(5+n+3) as u32)<<16)|((*b.add(5+n+2) as u32)<<8)|*b.add(5+n+1) as u32 } }
#[inline] pub unsafe fn uac_mixer_unit_iChannelNames(d:*mut uac_mixer_unit_descriptor,p:i32)->u8 { let b=d as *mut u8; let n=(*d).bNrInPins as usize; *b.add(5+n+if p==0 {3}else{5}) }
#[inline] pub unsafe fn uac_mixer_unit_bmControls(d:*mut uac_mixer_unit_descriptor,p:i32)->*mut u8 { let b=d as *mut u8; let n=(*d).bNrInPins as usize; match p {0=>b.add(5+n+4),0x20=>b.add(5+n+6),0x30=>b.add(5+n+2),_=>core::ptr::null_mut()} }
#[inline] pub unsafe fn uac3_mixer_unit_wClusterDescrID(d:*mut uac_mixer_unit_descriptor)->u16 { let b=d as *mut u8; let n=(*d).bNrInPins as usize; ((*b.add(5+n+1) as u16)<<8)|*b.add(5+n) as u16 }
#[inline] pub unsafe fn uac_mixer_unit_iMixer(d:*mut uac_mixer_unit_descriptor)->u8 { *((d as *mut u8).add((*d).bLength as usize-1)) }

#[repr(C, packed)] pub struct uac_selector_unit_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bUintID:u8,pub bNrInPins:u8,pub baSourceID:[u8;0] }
#[inline] pub unsafe fn uac_selector_unit_iSelector(d:*mut uac_selector_unit_descriptor)->u8 { *((d as *mut u8).add((*d).bLength as usize-1)) }
#[repr(C, packed)] pub struct uac_feature_unit_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bUnitID:u8,pub bSourceID:u8,pub bControlSize:u8,pub bmaControls:[u8;0] }
#[inline] pub unsafe fn uac_feature_unit_iFeature(d:*mut uac_feature_unit_descriptor)->u8 { *((d as *mut u8).add((*d).bLength as usize-1)) }
#[repr(C, packed)] pub struct uac_processing_unit_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bUnitID:u8,pub wProcessType:u16,pub bNrInPins:u8,pub baSourceID:[u8;0] }
#[inline] pub unsafe fn uac_processing_unit_bNrChannels(d:*mut uac_processing_unit_descriptor)->u8 { *((d as *mut u8).add(7+(*d).bNrInPins as usize)) }
#[inline] pub unsafe fn uac_processing_unit_wChannelConfig(d:*mut uac_processing_unit_descriptor,p:i32)->u32 { let b=d as *mut u8; let n=(*d).bNrInPins as usize; if p==0 { ((*b.add(7+n+2) as u32)<<8)|*b.add(7+n+1) as u32 } else { ((*b.add(7+n+4) as u32)<<24)|((*b.add(7+n+3) as u32)<<16)|((*b.add(7+n+2) as u32)<<8)|*b.add(7+n+1) as u32 } }
#[inline] pub unsafe fn uac_processing_unit_iChannelNames(d:*mut uac_processing_unit_descriptor,p:i32)->u8 { let b=d as *mut u8; let n=(*d).bNrInPins as usize; *b.add(7+n+if p==0 {3}else{5}) }
#[inline] pub unsafe fn uac_processing_unit_bControlSize(d:*mut uac_processing_unit_descriptor,p:i32)->u8 { let b=d as *mut u8; let n=(*d).bNrInPins as usize; match p {0=>*b.add(7+n+4),0x20=>2,0x30=>4,_=>1} }
#[inline] pub unsafe fn uac_processing_unit_bmControls(d:*mut uac_processing_unit_descriptor,p:i32)->*mut u8 { let b=d as *mut u8; let n=(*d).bNrInPins as usize; match p {0=>b.add(7+n+5),0x20=>b.add(7+n+6),0x30=>b.add(7+n+2),_=>core::ptr::null_mut()} }
#[inline] pub unsafe fn uac_processing_unit_iProcessing(d:*mut uac_processing_unit_descriptor,p:i32)->u8 { if p==0x30 {0} else {*uac_processing_unit_bmControls(d,p).add(uac_processing_unit_bControlSize(d,p) as usize)} }
#[inline] pub unsafe fn uac_processing_unit_specific(d:*mut uac_processing_unit_descriptor,p:i32)->*mut u8 { let c=uac_processing_unit_bControlSize(d,p) as usize; if p==0x30 {uac_processing_unit_bmControls(d,p).add(c)} else {uac_processing_unit_bmControls(d,p).add(c+1)} }
#[inline] pub unsafe fn uac_extension_unit_bControlSize(d:*mut uac_processing_unit_descriptor,p:i32)->u8 { let b=d as *mut u8; let n=(*d).bNrInPins as usize; match p {0=>*b.add(7+n+4),0x20=>1,0x30=>4,_=>1} }
#[inline] pub unsafe fn uac_extension_unit_iExtension(d:*mut uac_processing_unit_descriptor,p:i32)->u8 { if p==0x30 {0} else {*uac_processing_unit_bmControls(d,p).add(uac_extension_unit_bControlSize(d,p) as usize)} }

#[repr(C, packed)] pub struct uac1_as_header_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bTerminalLink:u8,pub bDelay:u8,pub wFormatTag:u16 }
pub const UAC_DT_AS_HEADER_SIZE:usize=7; pub const UAC_FORMAT_TYPE_I_UNDEFINED:u16=0; pub const UAC_FORMAT_TYPE_I_PCM:u16=1; pub const UAC_FORMAT_TYPE_I_PCM8:u16=2; pub const UAC_FORMAT_TYPE_I_IEEE_FLOAT:u16=3; pub const UAC_FORMAT_TYPE_I_ALAW:u16=4; pub const UAC_FORMAT_TYPE_I_MULAW:u16=5;
#[repr(C, packed)] pub struct uac_format_type_i_continuous_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bFormatType:u8,pub bNrChannels:u8,pub bSubframeSize:u8,pub bBitResolution:u8,pub bSamFreqType:u8,pub tLowerSamFreq:[u8;3],pub tUpperSamFreq:[u8;3] }
pub const UAC_FORMAT_TYPE_I_CONTINUOUS_DESC_SIZE:usize=14;
#[repr(C, packed)] pub struct uac_format_type_i_discrete_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bFormatType:u8,pub bNrChannels:u8,pub bSubframeSize:u8,pub bBitResolution:u8,pub bSamFreqType:u8,pub tSamFreq:[[u8;3];0] }
pub const fn UAC_FORMAT_TYPE_I_DISCRETE_DESC_SIZE(n:usize)->usize {8+n*3}
#[repr(C, packed)] pub struct uac_format_type_i_ext_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bFormatType:u8,pub bSubslotSize:u8,pub bBitResolution:u8,pub bHeaderLength:u8,pub bControlSize:u8,pub bSideBandProtocol:u8 }
pub const UAC_FORMAT_TYPE_II_MPEG:u16=0x1001; pub const UAC_FORMAT_TYPE_II_AC3:u16=0x1002; pub const UAC_FORMAT_TYPE_III_IEC1937_AC3:u16=0x2001; pub const UAC_FORMAT_TYPE_III_IEC1937_MPEG1_LAYER1:u16=0x2002; pub const UAC_FORMAT_TYPE_III_IEC1937_MPEG2_NOEXT:u16=0x2003; pub const UAC_FORMAT_TYPE_III_IEC1937_MPEG2_EXT:u16=0x2004; pub const UAC_FORMAT_TYPE_III_IEC1937_MPEG2_LAYER1_LS:u16=0x2005; pub const UAC_FORMAT_TYPE_III_IEC1937_MPEG2_LAYER23_LS:u16=0x2006;
pub const UAC_FORMAT_TYPE_UNDEFINED:u8=0; pub const UAC_FORMAT_TYPE_I:u8=1; pub const UAC_FORMAT_TYPE_II:u8=2; pub const UAC_FORMAT_TYPE_III:u8=3; pub const UAC_EXT_FORMAT_TYPE_I:u8=0x81; pub const UAC_EXT_FORMAT_TYPE_II:u8=0x82; pub const UAC_EXT_FORMAT_TYPE_III:u8=0x83;
#[repr(C, packed)] pub struct uac_format_type_ii_discrete_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bFormatType:u8,pub wMaxBitRate:u16,pub wSamplesPerFrame:u16,pub bSamFreqType:u8,pub tSamFreq:[[u8;3];0] }
#[repr(C, packed)] pub struct uac_format_type_ii_ext_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bFormatType:u8,pub wMaxBitRate:u16,pub wSamplesPerFrame:u16,pub bHeaderLength:u8,pub bSideBandProtocol:u8 }
#[repr(C, packed)] pub struct uac_iso_endpoint_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubtype:u8,pub bmAttributes:u8,pub bLockDelayUnits:u8,pub wLockDelay:u16 }
pub const UAC_ISO_ENDPOINT_DESC_SIZE:usize=7; pub const UAC_EP_CS_ATTR_SAMPLE_RATE:u8=1; pub const UAC_EP_CS_ATTR_PITCH_CONTROL:u8=2; pub const UAC_EP_CS_ATTR_FILL_MAX:u8=0x80;
pub const UAC1_STATUS_TYPE_ORIG_MASK:u8=0x0f; pub const UAC1_STATUS_TYPE_ORIG_AUDIO_CONTROL_IF:u8=0; pub const UAC1_STATUS_TYPE_ORIG_AUDIO_STREAM_IF:u8=1; pub const UAC1_STATUS_TYPE_ORIG_AUDIO_STREAM_EP:u8=2; pub const UAC1_STATUS_TYPE_IRQ_PENDING:u8=1<<7; pub const UAC1_STATUS_TYPE_MEM_CHANGED:u8=1<<6;
#[repr(C, packed)] pub struct uac1_status_word { pub bStatusType:u8,pub bOriginator:u8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
