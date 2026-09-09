/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* USB Communications Device Class (CDC) definitions. */

/* C dependency: linux/types.h supplies __u8, __u16, __le16, and __le32. */

pub const USB_CDC_SUBCLASS_ACM: u8 = 0x02;
pub const USB_CDC_SUBCLASS_ETHERNET: u8 = 0x06;
pub const USB_CDC_SUBCLASS_WHCM: u8 = 0x08;
pub const USB_CDC_SUBCLASS_DMM: u8 = 0x09;
pub const USB_CDC_SUBCLASS_MDLM: u8 = 0x0a;
pub const USB_CDC_SUBCLASS_OBEX: u8 = 0x0b;
pub const USB_CDC_SUBCLASS_EEM: u8 = 0x0c;
pub const USB_CDC_SUBCLASS_NCM: u8 = 0x0d;
pub const USB_CDC_SUBCLASS_MBIM: u8 = 0x0e;
pub const USB_CDC_PROTO_NONE: u8 = 0;
pub const USB_CDC_ACM_PROTO_AT_V25TER: u8 = 1;
pub const USB_CDC_ACM_PROTO_AT_PCCA101: u8 = 2;
pub const USB_CDC_ACM_PROTO_AT_PCCA101_WAKE: u8 = 3;
pub const USB_CDC_ACM_PROTO_AT_GSM: u8 = 4;
pub const USB_CDC_ACM_PROTO_AT_3G: u8 = 5;
pub const USB_CDC_ACM_PROTO_AT_CDMA: u8 = 6;
pub const USB_CDC_ACM_PROTO_VENDOR: u8 = 0xff;
pub const USB_CDC_PROTO_EEM: u8 = 7;
pub const USB_CDC_NCM_PROTO_NTB: u8 = 1;
pub const USB_CDC_MBIM_PROTO_NTB: u8 = 2;

pub const USB_CDC_HEADER_TYPE: u8 = 0x00;
pub const USB_CDC_CALL_MANAGEMENT_TYPE: u8 = 0x01;
pub const USB_CDC_ACM_TYPE: u8 = 0x02;
pub const USB_CDC_UNION_TYPE: u8 = 0x06;
pub const USB_CDC_COUNTRY_TYPE: u8 = 0x07;
pub const USB_CDC_NETWORK_TERMINAL_TYPE: u8 = 0x0a;
pub const USB_CDC_ETHERNET_TYPE: u8 = 0x0f;
pub const USB_CDC_WHCM_TYPE: u8 = 0x11;
pub const USB_CDC_MDLM_TYPE: u8 = 0x12;
pub const USB_CDC_MDLM_DETAIL_TYPE: u8 = 0x13;
pub const USB_CDC_DMM_TYPE: u8 = 0x14;
pub const USB_CDC_OBEX_TYPE: u8 = 0x15;
pub const USB_CDC_NCM_TYPE: u8 = 0x1a;
pub const USB_CDC_MBIM_TYPE: u8 = 0x1b;
pub const USB_CDC_MBIM_EXTENDED_TYPE: u8 = 0x1c;

#[repr(C, packed)]
pub struct usb_cdc_header_desc { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub bcdCDC: __le16 }
#[repr(C, packed)]
pub struct usb_cdc_call_mgmt_descriptor { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub bmCapabilities: u8, pub bDataInterface: u8 }
pub const USB_CDC_CALL_MGMT_CAP_CALL_MGMT: u8 = 0x01;
pub const USB_CDC_CALL_MGMT_CAP_DATA_INTF: u8 = 0x02;
#[repr(C, packed)]
pub struct usb_cdc_acm_descriptor { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub bmCapabilities: u8 }
pub const USB_CDC_COMM_FEATURE: u8 = 0x01;
pub const USB_CDC_CAP_LINE: u8 = 0x02;
pub const USB_CDC_CAP_BRK: u8 = 0x04;
pub const USB_CDC_CAP_NOTIFY: u8 = 0x08;

#[repr(C)]
pub union usb_cdc_union_desc__bindgen_ty_1 { pub bSlaveInterface0: u8, pub bSlaveInterfaces: [u8; 0] }
#[repr(C, packed)]
pub struct usb_cdc_union_desc { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub bMasterInterface0: u8, pub __bindgen_anon_1: usb_cdc_union_desc__bindgen_ty_1 }
#[repr(C)]
pub union usb_cdc_country_functional_desc__bindgen_ty_1 { pub wCountryCode0: __le16, pub wCountryCodes: [__le16; 0] }
#[repr(C, packed)]
pub struct usb_cdc_country_functional_desc { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub iCountryCodeRelDate: u8, pub __bindgen_anon_1: usb_cdc_country_functional_desc__bindgen_ty_1 }

#[repr(C, packed)]
pub struct usb_cdc_network_terminal_desc { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub bEntityId: u8, pub iName: u8, pub bChannelIndex: u8, pub bPhysicalInterface: u8 }
#[repr(C, packed)]
pub struct usb_cdc_ether_desc { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub iMACAddress: u8, pub bmEthernetStatistics: __le32, pub wMaxSegmentSize: __le16, pub wNumberMCFilters: __le16, pub bNumberPowerFilters: u8 }
#[repr(C, packed)]
pub struct usb_cdc_dmm_desc { pub bFunctionLength: u8, pub bDescriptorType: u8, pub bDescriptorSubtype: u8, pub bcdVersion: u16, pub wMaxCommand: __le16 }
#[repr(C, packed)]
pub struct usb_cdc_mdlm_desc { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub bcdVersion: __le16, pub bGUID: [u8; 16] }
#[repr(C, packed)]
pub struct usb_cdc_mdlm_detail_desc { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub bGuidDescriptorType: u8, pub bDetailData: [u8; 0] }
#[repr(C, packed)]
pub struct usb_cdc_obex_desc { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub bcdVersion: __le16 }
#[repr(C, packed)]
pub struct usb_cdc_ncm_desc { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub bcdNcmVersion: __le16, pub bmNetworkCapabilities: u8 }
#[repr(C, packed)]
pub struct usb_cdc_mbim_desc { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub bcdMBIMVersion: __le16, pub wMaxControlMessage: __le16, pub bNumberFilters: u8, pub bMaxFilterSize: u8, pub wMaxSegmentSize: __le16, pub bmNetworkCapabilities: u8 }
#[repr(C, packed)]
pub struct usb_cdc_mbim_extended_desc { pub bLength: u8, pub bDescriptorType: u8, pub bDescriptorSubType: u8, pub bcdMBIMExtendedVersion: __le16, pub bMaxOutstandingCommandMessages: u8, pub wMTU: __le16 }

pub const USB_CDC_SEND_ENCAPSULATED_COMMAND: u8 = 0x00;
pub const USB_CDC_GET_ENCAPSULATED_RESPONSE: u8 = 0x01;
pub const USB_CDC_REQ_SET_LINE_CODING: u8 = 0x20;
pub const USB_CDC_REQ_GET_LINE_CODING: u8 = 0x21;
pub const USB_CDC_REQ_SET_CONTROL_LINE_STATE: u8 = 0x22;
pub const USB_CDC_REQ_SEND_BREAK: u8 = 0x23;
pub const USB_CDC_SET_ETHERNET_MULTICAST_FILTERS: u8 = 0x40;
pub const USB_CDC_SET_ETHERNET_PM_PATTERN_FILTER: u8 = 0x41;
pub const USB_CDC_GET_ETHERNET_PM_PATTERN_FILTER: u8 = 0x42;
pub const USB_CDC_SET_ETHERNET_PACKET_FILTER: u8 = 0x43;
pub const USB_CDC_GET_ETHERNET_STATISTIC: u8 = 0x44;
pub const USB_CDC_GET_NTB_PARAMETERS: u8 = 0x80;
pub const USB_CDC_GET_NET_ADDRESS: u8 = 0x81;
pub const USB_CDC_SET_NET_ADDRESS: u8 = 0x82;
pub const USB_CDC_GET_NTB_FORMAT: u8 = 0x83;
pub const USB_CDC_SET_NTB_FORMAT: u8 = 0x84;
pub const USB_CDC_GET_NTB_INPUT_SIZE: u8 = 0x85;
pub const USB_CDC_SET_NTB_INPUT_SIZE: u8 = 0x86;
pub const USB_CDC_GET_MAX_DATAGRAM_SIZE: u8 = 0x87;
pub const USB_CDC_SET_MAX_DATAGRAM_SIZE: u8 = 0x88;
pub const USB_CDC_GET_CRC_MODE: u8 = 0x89;
pub const USB_CDC_SET_CRC_MODE: u8 = 0x8a;

#[repr(C, packed)]
pub struct usb_cdc_line_coding { pub dwDTERate: __le32, pub bCharFormat: u8, pub bParityType: u8, pub bDataBits: u8 }
pub const USB_CDC_1_STOP_BITS: u8 = 0; pub const USB_CDC_1_5_STOP_BITS: u8 = 1; pub const USB_CDC_2_STOP_BITS: u8 = 2;
pub const USB_CDC_NO_PARITY: u8 = 0; pub const USB_CDC_ODD_PARITY: u8 = 1; pub const USB_CDC_EVEN_PARITY: u8 = 2; pub const USB_CDC_MARK_PARITY: u8 = 3; pub const USB_CDC_SPACE_PARITY: u8 = 4;
pub const USB_CDC_CTRL_DTR: u8 = 1 << 0; pub const USB_CDC_CTRL_RTS: u8 = 1 << 1;
pub const USB_CDC_PACKET_TYPE_PROMISCUOUS: u8 = 1 << 0; pub const USB_CDC_PACKET_TYPE_ALL_MULTICAST: u8 = 1 << 1; pub const USB_CDC_PACKET_TYPE_DIRECTED: u8 = 1 << 2; pub const USB_CDC_PACKET_TYPE_BROADCAST: u8 = 1 << 3; pub const USB_CDC_PACKET_TYPE_MULTICAST: u8 = 1 << 4;

pub const USB_CDC_NOTIFY_NETWORK_CONNECTION: u8 = 0x00; pub const USB_CDC_NOTIFY_RESPONSE_AVAILABLE: u8 = 0x01; pub const USB_CDC_NOTIFY_SERIAL_STATE: u8 = 0x20; pub const USB_CDC_NOTIFY_SPEED_CHANGE: u8 = 0x2a;
#[repr(C, packed)]
pub struct usb_cdc_notification { pub bmRequestType: u8, pub bNotificationType: u8, pub wValue: __le16, pub wIndex: __le16, pub wLength: __le16 }
pub const USB_CDC_SERIAL_STATE_DCD: u8 = 1 << 0; pub const USB_CDC_SERIAL_STATE_DSR: u8 = 1 << 1; pub const USB_CDC_SERIAL_STATE_BREAK: u8 = 1 << 2; pub const USB_CDC_SERIAL_STATE_RING_SIGNAL: u8 = 1 << 3; pub const USB_CDC_SERIAL_STATE_FRAMING: u8 = 1 << 4; pub const USB_CDC_SERIAL_STATE_PARITY: u8 = 1 << 5; pub const USB_CDC_SERIAL_STATE_OVERRUN: u8 = 1 << 6;
#[repr(C, packed)]
pub struct usb_cdc_speed_change { pub DLBitRRate: __le32, pub ULBitRate: __le32 }

#[repr(C, packed)]
pub struct usb_cdc_ncm_ntb_parameters { pub wLength: __le16, pub bmNtbFormatsSupported: __le16, pub dwNtbInMaxSize: __le32, pub wNdpInDivisor: __le16, pub wNdpInPayloadRemainder: __le16, pub wNdpInAlignment: __le16, pub wPadding1: __le16, pub dwNtbOutMaxSize: __le32, pub wNdpOutDivisor: __le16, pub wNdpOutPayloadRemainder: __le16, pub wNdpOutAlignment: __le16, pub wNtbOutMaxDatagrams: __le16 }
pub const USB_CDC_NCM_NTH16_SIGN: u32 = 0x484D434E; pub const USB_CDC_NCM_NTH32_SIGN: u32 = 0x686D636E;
#[repr(C, packed)] pub struct usb_cdc_ncm_nth16 { pub dwSignature: __le32, pub wHeaderLength: __le16, pub wSequence: __le16, pub wBlockLength: __le16, pub wNdpIndex: __le16 }
#[repr(C, packed)] pub struct usb_cdc_ncm_nth32 { pub dwSignature: __le32, pub wHeaderLength: __le16, pub wSequence: __le16, pub dwBlockLength: __le32, pub dwNdpIndex: __le32 }
pub const USB_CDC_NCM_NDP16_CRC_SIGN: u32 = 0x314D434E; pub const USB_CDC_NCM_NDP16_NOCRC_SIGN: u32 = 0x304D434E; pub const USB_CDC_NCM_NDP32_CRC_SIGN: u32 = 0x316D636E; pub const USB_CDC_NCM_NDP32_NOCRC_SIGN: u32 = 0x306D636E;
pub const USB_CDC_MBIM_NDP16_IPS_SIGN: u32 = 0x00535049; pub const USB_CDC_MBIM_NDP32_IPS_SIGN: u32 = 0x00737069; pub const USB_CDC_MBIM_NDP16_DSS_SIGN: u32 = 0x00535344; pub const USB_CDC_MBIM_NDP32_DSS_SIGN: u32 = 0x00737364;
#[repr(C, packed)] pub struct usb_cdc_ncm_dpe16 { pub wDatagramIndex: __le16, pub wDatagramLength: __le16 }
#[repr(C, packed)] pub struct usb_cdc_ncm_ndp16 { pub dwSignature: __le32, pub wLength: __le16, pub wNextNdpIndex: __le16, pub dpe16: [usb_cdc_ncm_dpe16; 0] }
#[repr(C, packed)] pub struct usb_cdc_ncm_dpe32 { pub dwDatagramIndex: __le32, pub dwDatagramLength: __le32 }
#[repr(C, packed)] pub struct usb_cdc_ncm_ndp32 { pub dwSignature: __le32, pub wLength: __le16, pub wReserved6: __le16, pub dwNextNdpIndex: __le32, pub dwReserved12: __le32, pub dpe32: [usb_cdc_ncm_dpe32; 0] }

pub const USB_CDC_NCM_NDP16_INDEX_MIN: u16 = 0x000C; pub const USB_CDC_NCM_NDP32_INDEX_MIN: u16 = 0x0010; pub const USB_CDC_NCM_DATAGRAM_FORMAT_CRC: u8 = 0x30; pub const USB_CDC_NCM_DATAGRAM_FORMAT_NOCRC: u8 = 0x31; pub const USB_CDC_NCM_PROTO_CODE_NO_ENCAP_COMMANDS: u8 = 0x00; pub const USB_CDC_NCM_PROTO_CODE_EXTERN_PROTO: u8 = 0xFE;
pub const USB_CDC_NCM_NCAP_ETH_FILTER: u8 = 1 << 0; pub const USB_CDC_NCM_NCAP_NET_ADDRESS: u8 = 1 << 1; pub const USB_CDC_NCM_NCAP_ENCAP_COMMAND: u8 = 1 << 2; pub const USB_CDC_NCM_NCAP_MAX_DATAGRAM_SIZE: u8 = 1 << 3; pub const USB_CDC_NCM_NCAP_CRC_MODE: u8 = 1 << 4; pub const USB_CDC_NCM_NCAP_NTB_INPUT_SIZE: u8 = 1 << 5;
pub const USB_CDC_NCM_NTB16_SUPPORTED: u8 = 1 << 0; pub const USB_CDC_NCM_NTB32_SUPPORTED: u8 = 1 << 1; pub const USB_CDC_NCM_NDP_ALIGN_MIN_SIZE: u8 = 0x04; pub const USB_CDC_NCM_NTB_MAX_LENGTH: u8 = 0x1C; pub const USB_CDC_NCM_NTB16_FORMAT: u8 = 0x00; pub const USB_CDC_NCM_NTB32_FORMAT: u8 = 0x01; pub const USB_CDC_NCM_NTB_MIN_IN_SIZE: u16 = 2048; pub const USB_CDC_NCM_NTB_MIN_OUT_SIZE: u16 = 2048;
#[repr(C, packed)] pub struct usb_cdc_ncm_ndp_input_size { pub dwNtbInMaxSize: __le32, pub wNtbInMaxDatagrams: __le16, pub wReserved: __le16 }
pub const USB_CDC_NCM_CRC_NOT_APPENDED: u8 = 0x00; pub const USB_CDC_NCM_CRC_APPENDED: u8 = 0x01;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
