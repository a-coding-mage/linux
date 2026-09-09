/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* USB Video Class definitions. Translated from the Linux UVC header. */

// Dependency intent: __u8, __u16, __u32, __le16 and __le32 are supplied by linux/types.h.

pub const UVC_SC_UNDEFINED: u8 = 0x00;
pub const UVC_SC_VIDEOCONTROL: u8 = 0x01;
pub const UVC_SC_VIDEOSTREAMING: u8 = 0x02;
pub const UVC_SC_VIDEO_INTERFACE_COLLECTION: u8 = 0x03;
pub const UVC_PC_PROTOCOL_UNDEFINED: u8 = 0x00;
pub const UVC_PC_PROTOCOL_15: u8 = 0x01;
pub const UVC_VC_DESCRIPTOR_UNDEFINED: u8 = 0x00;
pub const UVC_VC_HEADER: u8 = 0x01;
pub const UVC_VC_INPUT_TERMINAL: u8 = 0x02;
pub const UVC_VC_OUTPUT_TERMINAL: u8 = 0x03;
pub const UVC_VC_SELECTOR_UNIT: u8 = 0x04;
pub const UVC_VC_PROCESSING_UNIT: u8 = 0x05;
pub const UVC_VC_EXTENSION_UNIT: u8 = 0x06;
pub const UVC_VS_UNDEFINED: u8 = 0x00;
pub const UVC_VS_INPUT_HEADER: u8 = 0x01;
pub const UVC_VS_OUTPUT_HEADER: u8 = 0x02;
pub const UVC_VS_STILL_IMAGE_FRAME: u8 = 0x03;
pub const UVC_VS_FORMAT_UNCOMPRESSED: u8 = 0x04;
pub const UVC_VS_FRAME_UNCOMPRESSED: u8 = 0x05;
pub const UVC_VS_FORMAT_MJPEG: u8 = 0x06;
pub const UVC_VS_FRAME_MJPEG: u8 = 0x07;
pub const UVC_VS_FORMAT_MPEG2TS: u8 = 0x0a;
pub const UVC_VS_FORMAT_DV: u8 = 0x0c;
pub const UVC_VS_COLORFORMAT: u8 = 0x0d;
pub const UVC_VS_FORMAT_FRAME_BASED: u8 = 0x10;
pub const UVC_VS_FRAME_FRAME_BASED: u8 = 0x11;
pub const UVC_VS_FORMAT_STREAM_BASED: u8 = 0x12;
pub const UVC_EP_UNDEFINED: u8 = 0x00;
pub const UVC_EP_GENERAL: u8 = 0x01;
pub const UVC_EP_ENDPOINT: u8 = 0x02;
pub const UVC_EP_INTERRUPT: u8 = 0x03;
pub const UVC_RC_UNDEFINED: u8 = 0x00;
pub const UVC_SET_CUR: u8 = 0x01;
pub const UVC_GET_CUR: u8 = 0x81;
pub const UVC_GET_MIN: u8 = 0x82;
pub const UVC_GET_MAX: u8 = 0x83;
pub const UVC_GET_RES: u8 = 0x84;
pub const UVC_GET_LEN: u8 = 0x85;
pub const UVC_GET_INFO: u8 = 0x86;
pub const UVC_GET_DEF: u8 = 0x87;

pub const UVC_VC_CONTROL_UNDEFINED: u8 = 0;
pub const UVC_VC_VIDEO_POWER_MODE_CONTROL: u8 = 1;
pub const UVC_VC_REQUEST_ERROR_CODE_CONTROL: u8 = 2;
pub const UVC_TE_CONTROL_UNDEFINED: u8 = 0;
pub const UVC_SU_CONTROL_UNDEFINED: u8 = 0;
pub const UVC_SU_INPUT_SELECT_CONTROL: u8 = 1;
pub const UVC_CT_CONTROL_UNDEFINED: u8 = 0;
pub const UVC_CT_SCANNING_MODE_CONTROL: u8 = 1;
pub const UVC_CT_AE_MODE_CONTROL: u8 = 2;
pub const UVC_CT_AE_PRIORITY_CONTROL: u8 = 3;
pub const UVC_CT_EXPOSURE_TIME_ABSOLUTE_CONTROL: u8 = 4;
pub const UVC_CT_EXPOSURE_TIME_RELATIVE_CONTROL: u8 = 5;
pub const UVC_CT_FOCUS_ABSOLUTE_CONTROL: u8 = 6;
pub const UVC_CT_FOCUS_RELATIVE_CONTROL: u8 = 7;
pub const UVC_CT_FOCUS_AUTO_CONTROL: u8 = 8;
pub const UVC_CT_IRIS_ABSOLUTE_CONTROL: u8 = 9;
pub const UVC_CT_IRIS_RELATIVE_CONTROL: u8 = 10;
pub const UVC_CT_ZOOM_ABSOLUTE_CONTROL: u8 = 11;
pub const UVC_CT_ZOOM_RELATIVE_CONTROL: u8 = 12;
pub const UVC_CT_PANTILT_ABSOLUTE_CONTROL: u8 = 13;
pub const UVC_CT_PANTILT_RELATIVE_CONTROL: u8 = 14;
pub const UVC_CT_ROLL_ABSOLUTE_CONTROL: u8 = 15;
pub const UVC_CT_ROLL_RELATIVE_CONTROL: u8 = 16;
pub const UVC_CT_PRIVACY_CONTROL: u8 = 17;
pub const UVC_CT_REGION_OF_INTEREST_CONTROL: u8 = 20;
pub const UVC_PU_CONTROL_UNDEFINED: u8 = 0;
pub const UVC_PU_BACKLIGHT_COMPENSATION_CONTROL: u8 = 1;
pub const UVC_PU_BRIGHTNESS_CONTROL: u8 = 2;
pub const UVC_PU_CONTRAST_CONTROL: u8 = 3;
pub const UVC_PU_GAIN_CONTROL: u8 = 4;
pub const UVC_PU_POWER_LINE_FREQUENCY_CONTROL: u8 = 5;
pub const UVC_PU_HUE_CONTROL: u8 = 6;
pub const UVC_PU_SATURATION_CONTROL: u8 = 7;
pub const UVC_PU_SHARPNESS_CONTROL: u8 = 8;
pub const UVC_PU_GAMMA_CONTROL: u8 = 9;
pub const UVC_PU_WHITE_BALANCE_TEMPERATURE_CONTROL: u8 = 10;
pub const UVC_PU_WHITE_BALANCE_TEMPERATURE_AUTO_CONTROL: u8 = 11;
pub const UVC_PU_WHITE_BALANCE_COMPONENT_CONTROL: u8 = 12;
pub const UVC_PU_WHITE_BALANCE_COMPONENT_AUTO_CONTROL: u8 = 13;
pub const UVC_PU_DIGITAL_MULTIPLIER_CONTROL: u8 = 14;
pub const UVC_PU_DIGITAL_MULTIPLIER_LIMIT_CONTROL: u8 = 15;
pub const UVC_PU_HUE_AUTO_CONTROL: u8 = 16;
pub const UVC_PU_ANALOG_VIDEO_STANDARD_CONTROL: u8 = 17;
pub const UVC_PU_ANALOG_LOCK_STATUS_CONTROL: u8 = 18;
pub const UVC_VS_CONTROL_UNDEFINED: u8 = 0;
pub const UVC_VS_PROBE_CONTROL: u8 = 1;
pub const UVC_VS_COMMIT_CONTROL: u8 = 2;
pub const UVC_VS_STILL_PROBE_CONTROL: u8 = 3;
pub const UVC_VS_STILL_COMMIT_CONTROL: u8 = 4;
pub const UVC_VS_STILL_IMAGE_TRIGGER_CONTROL: u8 = 5;
pub const UVC_VS_STREAM_ERROR_CODE_CONTROL: u8 = 6;
pub const UVC_VS_GENERATE_KEY_FRAME_CONTROL: u8 = 7;
pub const UVC_VS_UPDATE_FRAME_SEGMENT_CONTROL: u8 = 8;
pub const UVC_VS_SYNC_DELAY_CONTROL: u8 = 9;

pub const UVC_TT_VENDOR_SPECIFIC: u16 = 0x0100;
pub const UVC_TT_STREAMING: u16 = 0x0101;
pub const UVC_ITT_VENDOR_SPECIFIC: u16 = 0x0200;
pub const UVC_ITT_CAMERA: u16 = 0x0201;
pub const UVC_ITT_MEDIA_TRANSPORT_INPUT: u16 = 0x0202;
pub const UVC_OTT_VENDOR_SPECIFIC: u16 = 0x0300;
pub const UVC_OTT_DISPLAY: u16 = 0x0301;
pub const UVC_OTT_MEDIA_TRANSPORT_OUTPUT: u16 = 0x0302;
pub const UVC_EXTERNAL_VENDOR_SPECIFIC: u16 = 0x0400;
pub const UVC_COMPOSITE_CONNECTOR: u16 = 0x0401;
pub const UVC_SVIDEO_CONNECTOR: u16 = 0x0402;
pub const UVC_COMPONENT_CONNECTOR: u16 = 0x0403;
pub const UVC_STATUS_TYPE_CONTROL: u8 = 1;
pub const UVC_STATUS_TYPE_STREAMING: u8 = 2;
pub const UVC_STREAM_EOH: u8 = 1 << 7;
pub const UVC_STREAM_ERR: u8 = 1 << 6;
pub const UVC_STREAM_STI: u8 = 1 << 5;
pub const UVC_STREAM_RES: u8 = 1 << 4;
pub const UVC_STREAM_SCR: u8 = 1 << 3;
pub const UVC_STREAM_PTS: u8 = 1 << 2;
pub const UVC_STREAM_EOF: u8 = 1 << 1;
pub const UVC_STREAM_FID: u8 = 1;
pub const UVC_CONTROL_CAP_GET: u8 = 1;
pub const UVC_CONTROL_CAP_SET: u8 = 1 << 1;
pub const UVC_CONTROL_CAP_DISABLED: u8 = 1 << 2;
pub const UVC_CONTROL_CAP_AUTOUPDATE: u8 = 1 << 3;
pub const UVC_CONTROL_CAP_ASYNCHRONOUS: u8 = 1 << 4;

#[repr(u32)]
pub enum uvc_color_primaries_values { UVC_COLOR_PRIMARIES_UNSPECIFIED, UVC_COLOR_PRIMARIES_BT_709_SRGB, UVC_COLOR_PRIMARIES_BT_470_2_M, UVC_COLOR_PRIMARIES_BT_470_2_B_G, UVC_COLOR_PRIMARIES_SMPTE_170M, UVC_COLOR_PRIMARIES_SMPTE_240M }
#[repr(u32)]
pub enum uvc_transfer_characteristics_values { UVC_TRANSFER_CHARACTERISTICS_UNSPECIFIED, UVC_TRANSFER_CHARACTERISTICS_BT_709, UVC_TRANSFER_CHARACTERISTICS_BT_470_2_M, UVC_TRANSFER_CHARACTERISTICS_BT_470_2_B_G, UVC_TRANSFER_CHARACTERISTICS_SMPTE_170M, UVC_TRANSFER_CHARACTERISTICS_SMPTE_240M, UVC_TRANSFER_CHARACTERISTICS_LINEAR, UVC_TRANSFER_CHARACTERISTICS_SRGB }
#[repr(u32)]
pub enum uvc_matrix_coefficients { UVC_MATRIX_COEFFICIENTS_UNSPECIFIED, UVC_MATRIX_COEFFICIENTS_BT_709, UVC_MATRIX_COEFFICIENTS_FCC, UVC_MATRIX_COEFFICIENTS_BT_470_2_B_G, UVC_MATRIX_COEFFICIENTS_SMPTE_170M, UVC_MATRIX_COEFFICIENTS_SMPTE_240M }

#[repr(C, packed)]
pub struct uvc_descriptor_header { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8 }
#[repr(C, packed)]
pub struct uvc_header_descriptor { pub bLength:u8, pub bDescriptorType:u8, pub bDescriptorSubType:u8, pub bcdUVC:u16, pub wTotalLength:u16, pub dwClockFrequency:u32, pub bInCollection:u8, pub baInterfaceNr:[u8; 0] }
pub const UVC_DT_HEADER_SIZE: fn(usize) -> usize = |n| 12+n;
pub const UVC_DT_INPUT_TERMINAL_SIZE: usize = 8;
pub const UVC_DT_OUTPUT_TERMINAL_SIZE: usize = 9;
pub const UVC_DT_COLOR_MATCHING_SIZE: usize = 6;

#[repr(C, packed)] pub struct uvc_input_terminal_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bTerminalID:u8,pub wTerminalType:u16,pub bAssocTerminal:u8,pub iTerminal:u8 }
#[repr(C, packed)] pub struct uvc_output_terminal_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bTerminalID:u8,pub wTerminalType:u16,pub bAssocTerminal:u8,pub bSourceID:u8,pub iTerminal:u8 }
#[repr(C, packed)] pub struct uvc_camera_terminal_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bTerminalID:u8,pub wTerminalType:u16,pub bAssocTerminal:u8,pub iTerminal:u8,pub wObjectiveFocalLengthMin:u16,pub wObjectiveFocalLengthMax:u16,pub wOcularFocalLength:u16,pub bControlSize:u8,pub bmControls:[u8;3] }
#[repr(C, packed)] pub struct uvc_selector_unit_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bUnitID:u8,pub bNrInPins:u8,pub baSourceID:[u8;0],pub iSelector:u8 }
#[repr(C, packed)] pub struct uvc_processing_unit_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bUnitID:u8,pub bSourceID:u8,pub wMaxMultiplier:u16,pub bControlSize:u8,pub bmControls:[u8;2],pub iProcessing:u8,pub bmVideoStandards:u8 }
#[repr(C, packed)] pub struct uvc_extension_unit_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bUnitID:u8,pub guidExtensionCode:[u8;16],pub bNumControls:u8,pub bNrInPins:u8,pub baSourceID:[u8;0],pub bControlSize:u8,pub bmControls:[u8;0],pub iExtension:u8 }
#[repr(C, packed)] pub struct uvc_control_endpoint_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub wMaxTransferSize:u16 }
#[repr(C, packed)] pub struct uvc_input_header_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bNumFormats:u8,pub wTotalLength:u16,pub bEndpointAddress:u8,pub bmInfo:u8,pub bTerminalLink:u8,pub bStillCaptureMethod:u8,pub bTriggerSupport:u8,pub bTriggerUsage:u8,pub bControlSize:u8,pub bmaControls:[u8;0] }
#[repr(C, packed)] pub struct uvc_output_header_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bNumFormats:u8,pub wTotalLength:u16,pub bEndpointAddress:u8,pub bTerminalLink:u8,pub bControlSize:u8,pub bmaControls:[u8;0] }
#[repr(C, packed)] pub struct uvc_color_matching_descriptor { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bColorPrimaries:u8,pub bTransferCharacteristics:u8,pub bMatrixCoefficients:u8 }

#[repr(C, packed)] pub struct uvc_streaming_control { pub bmHint:u16,pub bFormatIndex:u8,pub bFrameIndex:u8,pub dwFrameInterval:u32,pub wKeyFrameRate:u16,pub wPFrameRate:u16,pub wCompQuality:u16,pub wCompWindowSize:u16,pub wDelay:u16,pub dwMaxVideoFrameSize:u32,pub dwMaxPayloadTransferSize:u32,pub dwClockFrequency:u32,pub bmFramingInfo:u8,pub bPreferedVersion:u8,pub bMinVersion:u8,pub bMaxVersion:u8 }
#[repr(C, packed)] pub struct uvc_format_uncompressed { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bFormatIndex:u8,pub bNumFrameDescriptors:u8,pub guidFormat:[u8;16],pub bBitsPerPixel:u8,pub bDefaultFrameIndex:u8,pub bAspectRatioX:u8,pub bAspectRatioY:u8,pub bmInterlaceFlags:u8,pub bCopyProtect:u8 }
#[repr(C, packed)] pub struct uvc_frame_uncompressed { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bFrameIndex:u8,pub bmCapabilities:u8,pub wWidth:u16,pub wHeight:u16,pub dwMinBitRate:u32,pub dwMaxBitRate:u32,pub dwMaxVideoFrameBufferSize:u32,pub dwDefaultFrameInterval:u32,pub bFrameIntervalType:u8,pub dwFrameInterval:[u32;0] }
#[repr(C, packed)] pub struct uvc_format_mjpeg { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bFormatIndex:u8,pub bNumFrameDescriptors:u8,pub bmFlags:u8,pub bDefaultFrameIndex:u8,pub bAspectRatioX:u8,pub bAspectRatioY:u8,pub bmInterlaceFlags:u8,pub bCopyProtect:u8 }
#[repr(C, packed)] pub struct uvc_frame_mjpeg { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bFrameIndex:u8,pub bmCapabilities:u8,pub wWidth:u16,pub wHeight:u16,pub dwMinBitRate:u32,pub dwMaxBitRate:u32,pub dwMaxVideoFrameBufferSize:u32,pub dwDefaultFrameInterval:u32,pub bFrameIntervalType:u8,pub dwFrameInterval:[u32;0] }
#[repr(C, packed)] pub struct uvc_format_framebased { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bFormatIndex:u8,pub bNumFrameDescriptors:u8,pub guidFormat:[u8;16],pub bBitsPerPixel:u8,pub bDefaultFrameIndex:u8,pub bAspectRatioX:u8,pub bAspectRatioY:u8,pub bmInterfaceFlags:u8,pub bCopyProtect:u8,pub bVariableSize:u8 }
#[repr(C, packed)] pub struct uvc_frame_framebased { pub bLength:u8,pub bDescriptorType:u8,pub bDescriptorSubType:u8,pub bFrameIndex:u8,pub bmCapabilities:u8,pub wWidth:u16,pub wHeight:u16,pub dwMinBitRate:u32,pub dwMaxBitRate:u32,pub dwDefaultFrameInterval:u32,pub bFrameIntervalType:u8,pub dwBytesPerLine:u32,pub dwFrameInterval:[u32;0] }

pub const UVC_DT_CAMERA_TERMINAL_SIZE: fn(usize)->usize = |n| 15+n;
pub const UVC_DT_SELECTOR_UNIT_SIZE: fn(usize)->usize = |n| 6+n;
pub const UVC_DT_PROCESSING_UNIT_SIZE: fn(usize)->usize = |n| 10+n;
pub const UVC_DT_EXTENSION_UNIT_SIZE: fn(usize,usize)->usize = |p,n| 24+p+n;
pub const UVC_DT_CONTROL_ENDPOINT_SIZE: usize = 5;
pub const UVC_DT_INPUT_HEADER_SIZE: fn(usize,usize)->usize = |n,p| 13+n*p;
pub const UVC_DT_OUTPUT_HEADER_SIZE: fn(usize,usize)->usize = |n,p| 9+n*p;
pub const UVC_DT_FORMAT_UNCOMPRESSED_SIZE: usize = 27;
pub const UVC_DT_FRAME_UNCOMPRESSED_SIZE: fn(usize)->usize = |n| 26+4*n;
pub const UVC_DT_FORMAT_MJPEG_SIZE: usize = 11;
pub const UVC_DT_FRAME_MJPEG_SIZE: fn(usize)->usize = |n| 26+4*n;
pub const UVC_DT_FORMAT_FRAMEBASED_SIZE: usize = 28;
pub const UVC_DT_FRAME_FRAMEBASED_SIZE: fn(usize)->usize = |n| 26+4*n;

// C parameterized descriptor macros are represented as Rust declarative macros.
#[macro_export] macro_rules! UVC_HEADER_DESCRIPTOR { ($n:ident) => { $n } }
#[macro_export] macro_rules! UVC_SELECTOR_UNIT_DESCRIPTOR { ($n:ident) => { $n } }
#[macro_export] macro_rules! UVC_EXTENSION_UNIT_DESCRIPTOR { ($n:ident,$p:ident) => { $n } }
#[macro_export] macro_rules! UVC_INPUT_HEADER_DESCRIPTOR { ($n:ident,$p:ident) => { $n } }
#[macro_export] macro_rules! UVC_OUTPUT_HEADER_DESCRIPTOR { ($n:ident,$p:ident) => { $n } }
#[macro_export] macro_rules! UVC_FRAME_UNCOMPRESSED { ($n:ident) => { $n } }
#[macro_export] macro_rules! UVC_FRAME_MJPEG { ($n:ident) => { $n } }
#[macro_export] macro_rules! UVC_FRAME_FRAMEBASED { ($n:ident) => { $n } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
