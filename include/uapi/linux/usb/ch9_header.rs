/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translation of uapi/linux/usb/ch9.h. */

pub const USB_DIR_OUT: u32 = 0;
pub const USB_DIR_IN: u32 = 0x80;
pub const USB_TYPE_MASK: u32 = 0x03 << 5;
pub const USB_TYPE_STANDARD: u32 = 0x00 << 5;
pub const USB_TYPE_CLASS: u32 = 0x01 << 5;
pub const USB_TYPE_VENDOR: u32 = 0x02 << 5;
pub const USB_TYPE_RESERVED: u32 = 0x03 << 5;
pub const USB_RECIP_MASK: u32 = 0x1f;
pub const USB_RECIP_DEVICE: u32 = 0;
pub const USB_RECIP_INTERFACE: u32 = 1;
pub const USB_RECIP_ENDPOINT: u32 = 2;
pub const USB_RECIP_OTHER: u32 = 3;
pub const USB_RECIP_PORT: u32 = 4;
pub const USB_RECIP_RPIPE: u32 = 5;

pub const USB_REQ_GET_STATUS: u32 = 0x00;
pub const USB_REQ_CLEAR_FEATURE: u32 = 0x01;
pub const USB_REQ_SET_FEATURE: u32 = 0x03;
pub const USB_REQ_SET_ADDRESS: u32 = 0x05;
pub const USB_REQ_GET_DESCRIPTOR: u32 = 0x06;
pub const USB_REQ_SET_DESCRIPTOR: u32 = 0x07;
pub const USB_REQ_GET_CONFIGURATION: u32 = 0x08;
pub const USB_REQ_SET_CONFIGURATION: u32 = 0x09;
pub const USB_REQ_GET_INTERFACE: u32 = 0x0a;
pub const USB_REQ_SET_INTERFACE: u32 = 0x0b;
pub const USB_REQ_SYNCH_FRAME: u32 = 0x0c;
pub const USB_REQ_SET_SEL: u32 = 0x30;
pub const USB_REQ_SET_ISOCH_DELAY: u32 = 0x31;
pub const USB_REQ_SET_ENCRYPTION: u32 = 0x0d;
pub const USB_REQ_GET_ENCRYPTION: u32 = 0x0e;
pub const USB_REQ_RPIPE_ABORT: u32 = 0x0e;
pub const USB_REQ_SET_HANDSHAKE: u32 = 0x0f;
pub const USB_REQ_RPIPE_RESET: u32 = 0x0f;
pub const USB_REQ_GET_HANDSHAKE: u32 = 0x10;
pub const USB_REQ_SET_CONNECTION: u32 = 0x11;
pub const USB_REQ_SET_SECURITY_DATA: u32 = 0x12;
pub const USB_REQ_GET_SECURITY_DATA: u32 = 0x13;
pub const USB_REQ_SET_WUSB_DATA: u32 = 0x14;
pub const USB_REQ_LOOPBACK_DATA_WRITE: u32 = 0x15;
pub const USB_REQ_LOOPBACK_DATA_READ: u32 = 0x16;
pub const USB_REQ_SET_INTERFACE_DS: u32 = 0x17;
pub const USB_REQ_AUTH_IN: u32 = 0x18;
pub const USB_REQ_AUTH_OUT: u32 = 0x19;
pub const USB_REQ_GET_PARTNER_PDO: u32 = 20;
pub const USB_REQ_GET_BATTERY_STATUS: u32 = 21;
pub const USB_REQ_SET_PDO: u32 = 22;
pub const USB_REQ_GET_VDM: u32 = 23;
pub const USB_REQ_SEND_VDM: u32 = 24;

pub const USB_DEVICE_SELF_POWERED: u32 = 0;
pub const USB_DEVICE_REMOTE_WAKEUP: u32 = 1;
pub const USB_DEVICE_TEST_MODE: u32 = 2;
pub const USB_DEVICE_BATTERY: u32 = 2;
pub const USB_DEVICE_B_HNP_ENABLE: u32 = 3;
pub const USB_DEVICE_WUSB_DEVICE: u32 = 3;
pub const USB_DEVICE_A_HNP_SUPPORT: u32 = 4;
pub const USB_DEVICE_A_ALT_HNP_SUPPORT: u32 = 5;
pub const USB_DEVICE_DEBUG_MODE: u32 = 6;
pub const USB_DEVICE_BULK_MAX_PACKET_UPDATE: u32 = 8;
pub const USB_TEST_J: u32 = 1; pub const USB_TEST_K: u32 = 2;
pub const USB_TEST_SE0_NAK: u32 = 3; pub const USB_TEST_PACKET: u32 = 4;
pub const USB_TEST_FORCE_ENABLE: u32 = 5;
pub const USB_STATUS_TYPE_STANDARD: u32 = 0; pub const USB_STATUS_TYPE_PTM: u32 = 1;
pub const USB_DEVICE_U1_ENABLE: u32 = 48; pub const USB_DEVICE_U2_ENABLE: u32 = 49;
pub const USB_DEVICE_LTM_ENABLE: u32 = 50; pub const USB_INTRF_FUNC_SUSPEND: u32 = 0;
pub const USB_INTR_FUNC_SUSPEND_OPT_MASK: u32 = 0xff00;
pub const USB_INTRF_FUNC_SUSPEND_LP: u32 = 1 << 8; pub const USB_INTRF_FUNC_SUSPEND_RW: u32 = 1 << 9;
pub const USB_INTRF_STAT_FUNC_RW_CAP: u32 = 1; pub const USB_INTRF_STAT_FUNC_RW: u32 = 2;
pub const USB_ENDPOINT_HALT: u32 = 0;
pub const USB_DEV_STAT_U1_ENABLED: u32 = 2; pub const USB_DEV_STAT_U2_ENABLED: u32 = 3; pub const USB_DEV_STAT_LTM_ENABLED: u32 = 4;
pub const USB_DEVICE_BATTERY_WAKE_MASK: u32 = 40; pub const USB_DEVICE_OS_IS_PD_AWARE: u32 = 41;
pub const USB_DEVICE_POLICY_MODE: u32 = 42; pub const USB_PORT_PR_SWAP: u32 = 43;
pub const USB_PORT_GOTO_MIN: u32 = 44; pub const USB_PORT_RETURN_POWER: u32 = 45;
pub const USB_PORT_ACCEPT_PD_REQUEST: u32 = 46; pub const USB_PORT_REJECT_PD_REQUEST: u32 = 47;
pub const USB_PORT_PORT_PD_RESET: u32 = 48; pub const USB_PORT_C_PORT_PD_CHANGE: u32 = 49;
pub const USB_PORT_CABLE_PD_RESET: u32 = 50; pub const USB_DEVICE_CHARGING_POLICY: u32 = 54;

pub const USB_DT_DEVICE: u32 = 1; pub const USB_DT_CONFIG: u32 = 2; pub const USB_DT_STRING: u32 = 3;
pub const USB_DT_INTERFACE: u32 = 4; pub const USB_DT_ENDPOINT: u32 = 5; pub const USB_DT_DEVICE_QUALIFIER: u32 = 6;
pub const USB_DT_OTHER_SPEED_CONFIG: u32 = 7; pub const USB_DT_INTERFACE_POWER: u32 = 8;
pub const USB_DT_OTG: u32 = 9; pub const USB_DT_DEBUG: u32 = 0x0a; pub const USB_DT_INTERFACE_ASSOCIATION: u32 = 0x0b;
pub const USB_DT_SECURITY: u32 = 0x0c; pub const USB_DT_KEY: u32 = 0x0d; pub const USB_DT_ENCRYPTION_TYPE: u32 = 0x0e;
pub const USB_DT_BOS: u32 = 0x0f; pub const USB_DT_DEVICE_CAPABILITY: u32 = 0x10;
pub const USB_DT_WIRELESS_ENDPOINT_COMP: u32 = 0x11; pub const USB_DT_EUSB2_ISOC_ENDPOINT_COMP: u32 = 0x12;
pub const USB_DT_WIRE_ADAPTER: u32 = 0x21; pub const USB_DT_DFU_FUNCTIONAL: u32 = 0x21;
pub const USB_DT_RPIPE: u32 = 0x22; pub const USB_DT_CS_RADIO_CONTROL: u32 = 0x23; pub const USB_DT_PIPE_USAGE: u32 = 0x24;
pub const USB_DT_SS_ENDPOINT_COMP: u32 = 0x30; pub const USB_DT_SSP_ISOC_ENDPOINT_COMP: u32 = 0x31;
pub const USB_DT_CS_DEVICE: u32 = USB_TYPE_CLASS | USB_DT_DEVICE; pub const USB_DT_CS_CONFIG: u32 = USB_TYPE_CLASS | USB_DT_CONFIG;
pub const USB_DT_CS_STRING: u32 = USB_TYPE_CLASS | USB_DT_STRING; pub const USB_DT_CS_INTERFACE: u32 = USB_TYPE_CLASS | USB_DT_INTERFACE;
pub const USB_DT_CS_ENDPOINT: u32 = USB_TYPE_CLASS | USB_DT_ENDPOINT;

#[repr(C, packed)] pub struct usb_ctrlrequest { pub bRequestType: __u8, pub bRequest: __u8, pub wValue: __le16, pub wIndex: __le16, pub wLength: __le16 }
#[repr(C, packed)] pub struct usb_descriptor_header { pub bLength: __u8, pub bDescriptorType: __u8 }
#[repr(C, packed)] pub struct usb_device_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bcdUSB: __le16, pub bDeviceClass: __u8, pub bDeviceSubClass: __u8, pub bDeviceProtocol: __u8, pub bMaxPacketSize0: __u8, pub idVendor: __le16, pub idProduct: __le16, pub bcdDevice: __le16, pub iManufacturer: __u8, pub iProduct: __u8, pub iSerialNumber: __u8, pub bNumConfigurations: __u8 }
pub const USB_DT_DEVICE_SIZE: u32 = 18;
pub const USB_CLASS_PER_INTERFACE: u32 = 0; pub const USB_CLASS_AUDIO: u32 = 1; pub const USB_CLASS_COMM: u32 = 2; pub const USB_CLASS_HID: u32 = 3; pub const USB_CLASS_PHYSICAL: u32 = 5; pub const USB_CLASS_STILL_IMAGE: u32 = 6; pub const USB_CLASS_PRINTER: u32 = 7; pub const USB_CLASS_MASS_STORAGE: u32 = 8; pub const USB_CLASS_HUB: u32 = 9; pub const USB_CLASS_CDC_DATA: u32 = 0x0a; pub const USB_CLASS_CSCID: u32 = 0x0b; pub const USB_CLASS_CONTENT_SEC: u32 = 0x0d; pub const USB_CLASS_VIDEO: u32 = 0x0e; pub const USB_CLASS_PERSONAL_HEALTHCARE: u32 = 0x0f; pub const USB_CLASS_AUDIO_VIDEO: u32 = 0x10; pub const USB_CLASS_BILLBOARD: u32 = 0x11; pub const USB_CLASS_USB_TYPE_C_BRIDGE: u32 = 0x12; pub const USB_CLASS_MCTP: u32 = 0x14; pub const USB_CLASS_WIRELESS_CONTROLLER: u32 = 0xe0; pub const USB_CLASS_MISC: u32 = 0xef; pub const USB_CLASS_APP_SPEC: u32 = 0xfe; pub const USB_CLASS_VENDOR_SPEC: u32 = 0xff; pub const USB_SUBCLASS_DFU: u32 = 1; pub const USB_SUBCLASS_VENDOR_SPEC: u32 = 0xff;

#[repr(C, packed)] pub struct usb_config_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub wTotalLength: __le16, pub bNumInterfaces: __u8, pub bConfigurationValue: __u8, pub iConfiguration: __u8, pub bmAttributes: __u8, pub bMaxPower: __u8 }
pub const USB_DT_CONFIG_SIZE: u32 = 9; pub const USB_CONFIG_ATT_ONE: u32 = 1 << 7; pub const USB_CONFIG_ATT_SELFPOWER: u32 = 1 << 6; pub const USB_CONFIG_ATT_WAKEUP: u32 = 1 << 5; pub const USB_CONFIG_ATT_BATTERY: u32 = 1 << 4;
pub const USB_MAX_STRING_LEN: u32 = 126;
#[repr(C)] pub union usb_string_descriptor_data { pub legacy_padding: __le16, pub wData: [__le16; 0] }
#[repr(C, packed)] pub struct usb_string_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub data: usb_string_descriptor_data }
#[repr(C, packed)] pub struct usb_interface_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bInterfaceNumber: __u8, pub bAlternateSetting: __u8, pub bNumEndpoints: __u8, pub bInterfaceClass: __u8, pub bInterfaceSubClass: __u8, pub bInterfaceProtocol: __u8, pub iInterface: __u8 }
pub const USB_DT_INTERFACE_SIZE: u32 = 9;
#[repr(C, packed)] pub struct usb_endpoint_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bEndpointAddress: __u8, pub bmAttributes: __u8, pub wMaxPacketSize: __le16, pub bInterval: __u8, pub bRefresh: __u8, pub bSynchAddress: __u8 }
pub const USB_DT_ENDPOINT_SIZE: u32 = 7; pub const USB_DT_ENDPOINT_AUDIO_SIZE: u32 = 9;
pub const USB_ENDPOINT_NUMBER_MASK: u32 = 0x0f; pub const USB_ENDPOINT_DIR_MASK: u32 = 0x80; pub const USB_ENDPOINT_XFERTYPE_MASK: u32 = 3; pub const USB_ENDPOINT_XFER_CONTROL: u32 = 0; pub const USB_ENDPOINT_XFER_ISOC: u32 = 1; pub const USB_ENDPOINT_XFER_BULK: u32 = 2; pub const USB_ENDPOINT_XFER_INT: u32 = 3; pub const USB_ENDPOINT_MAX_ADJUSTABLE: u32 = 0x80; pub const USB_ENDPOINT_MAXP_MASK: u32 = 0x07ff; pub const USB_EP_MAXP_MULT_SHIFT: u32 = 11; pub const USB_EP_MAXP_MULT_MASK: u32 = 3 << 11;
pub const USB_ENDPOINT_INTRTYPE: u32 = 0x30; pub const USB_ENDPOINT_INTR_PERIODIC: u32 = 0; pub const USB_ENDPOINT_INTR_NOTIFICATION: u32 = 1 << 4; pub const USB_ENDPOINT_SYNCTYPE: u32 = 0x0c; pub const USB_ENDPOINT_SYNC_NONE: u32 = 0; pub const USB_ENDPOINT_SYNC_ASYNC: u32 = 1 << 2; pub const USB_ENDPOINT_SYNC_ADAPTIVE: u32 = 2 << 2; pub const USB_ENDPOINT_SYNC_SYNC: u32 = 3 << 2; pub const USB_ENDPOINT_USAGE_MASK: u32 = 0x30; pub const USB_ENDPOINT_USAGE_DATA: u32 = 0; pub const USB_ENDPOINT_USAGE_FEEDBACK: u32 = 0x10; pub const USB_ENDPOINT_USAGE_IMPLICIT_FB: u32 = 0x20;
#[inline] pub const fn USB_EP_MAXP_MULT(m: u32) -> u32 { (m & USB_EP_MAXP_MULT_MASK) >> USB_EP_MAXP_MULT_SHIFT }

pub unsafe fn usb_endpoint_num(epd: *const usb_endpoint_descriptor) -> i32 { ((*epd).bEndpointAddress as u32 & USB_ENDPOINT_NUMBER_MASK) as i32 }
pub unsafe fn usb_endpoint_type(epd: *const usb_endpoint_descriptor) -> i32 { ((*epd).bmAttributes as u32 & USB_ENDPOINT_XFERTYPE_MASK) as i32 }
pub unsafe fn usb_endpoint_dir_in(epd: *const usb_endpoint_descriptor) -> i32 { (((*epd).bEndpointAddress as u32 & USB_ENDPOINT_DIR_MASK) == USB_DIR_IN) as i32 }
pub unsafe fn usb_endpoint_dir_out(epd: *const usb_endpoint_descriptor) -> i32 { (((*epd).bEndpointAddress as u32 & USB_ENDPOINT_DIR_MASK) == USB_DIR_OUT) as i32 }
pub unsafe fn usb_endpoint_xfer_bulk(epd: *const usb_endpoint_descriptor) -> i32 { (((*epd).bmAttributes as u32 & 3) == USB_ENDPOINT_XFER_BULK) as i32 }
pub unsafe fn usb_endpoint_xfer_control(epd: *const usb_endpoint_descriptor) -> i32 { (((*epd).bmAttributes as u32 & 3) == 0) as i32 }
pub unsafe fn usb_endpoint_xfer_int(epd: *const usb_endpoint_descriptor) -> i32 { (((*epd).bmAttributes as u32 & 3) == USB_ENDPOINT_XFER_INT) as i32 }
pub unsafe fn usb_endpoint_xfer_isoc(epd: *const usb_endpoint_descriptor) -> i32 { (((*epd).bmAttributes as u32 & 3) == USB_ENDPOINT_XFER_ISOC) as i32 }
pub unsafe fn usb_endpoint_is_bulk_in(e: *const usb_endpoint_descriptor) -> i32 { (usb_endpoint_xfer_bulk(e) != 0 && usb_endpoint_dir_in(e) != 0) as i32 }
pub unsafe fn usb_endpoint_is_bulk_out(e: *const usb_endpoint_descriptor) -> i32 { (usb_endpoint_xfer_bulk(e) != 0 && usb_endpoint_dir_out(e) != 0) as i32 }
pub unsafe fn usb_endpoint_is_int_in(e: *const usb_endpoint_descriptor) -> i32 { (usb_endpoint_xfer_int(e) != 0 && usb_endpoint_dir_in(e) != 0) as i32 }
pub unsafe fn usb_endpoint_is_int_out(e: *const usb_endpoint_descriptor) -> i32 { (usb_endpoint_xfer_int(e) != 0 && usb_endpoint_dir_out(e) != 0) as i32 }
pub unsafe fn usb_endpoint_is_isoc_in(e: *const usb_endpoint_descriptor) -> i32 { (usb_endpoint_xfer_isoc(e) != 0 && usb_endpoint_dir_in(e) != 0) as i32 }
pub unsafe fn usb_endpoint_is_isoc_out(e: *const usb_endpoint_descriptor) -> i32 { (usb_endpoint_xfer_isoc(e) != 0 && usb_endpoint_dir_out(e) != 0) as i32 }
pub unsafe fn usb_endpoint_maxp(epd: *const usb_endpoint_descriptor) -> i32 { (__le16_to_cpu((*epd).wMaxPacketSize) as u32 & USB_ENDPOINT_MAXP_MASK) as i32 }
pub unsafe fn usb_endpoint_maxp_mult(epd: *const usb_endpoint_descriptor) -> i32 { (USB_EP_MAXP_MULT(__le16_to_cpu((*epd).wMaxPacketSize) as u32) + 1) as i32 }
pub unsafe fn usb_endpoint_interrupt_type(epd: *const usb_endpoint_descriptor) -> i32 { (((*epd).bmAttributes as u32) & USB_ENDPOINT_INTRTYPE) as i32 }

#[repr(C, packed)] pub struct usb_eusb2_isoc_ep_comp_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub wMaxPacketSize: __le16, pub dwBytesPerInterval: __le32 }
pub const USB_DT_EUSB2_ISOC_EP_COMP_SIZE: u32 = 8;
#[repr(C, packed)] pub struct usb_ssp_isoc_ep_comp_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub wReseved: __le16, pub dwBytesPerInterval: __le32 }
pub const USB_DT_SSP_ISOC_EP_COMP_SIZE: u32 = 8;
#[repr(C, packed)] pub struct usb_ss_ep_comp_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bMaxBurst: __u8, pub bmAttributes: __u8, pub wBytesPerInterval: __le16 }
pub const USB_DT_SS_EP_COMP_SIZE: u32 = 6;
pub unsafe fn usb_ss_max_streams(comp: *const usb_ss_ep_comp_descriptor) -> i32 { if comp.is_null() { return 0; } let n = ((*comp).bmAttributes as i32) & 0x1f; if n == 0 { 0 } else { 1i32 << n } }
pub const USB_SS_MULT_MASK: u32 = 3; pub const USB_SS_SSP_ISOC_COMP_MASK: u32 = 1 << 7;
#[inline] pub const fn USB_SS_MULT(p: u32) -> u32 { 1 + (p & 3) }
#[inline] pub const fn USB_SS_SSP_ISOC_COMP(p: u32) -> u32 { p & (1 << 7) }

#[repr(C, packed)] pub struct usb_qualifier_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bcdUSB: __le16, pub bDeviceClass: __u8, pub bDeviceSubClass: __u8, pub bDeviceProtocol: __u8, pub bMaxPacketSize0: __u8, pub bNumConfigurations: __u8, pub bRESERVED: __u8 }
#[repr(C, packed)] pub struct usb_otg_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bmAttributes: __u8 }
#[repr(C, packed)] pub struct usb_otg20_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bmAttributes: __u8, pub bcdOTG: __le16 }
pub const USB_OTG_SRP: u32 = 1; pub const USB_OTG_HNP: u32 = 2; pub const USB_OTG_ADP: u32 = 4; pub const USB_OTG_RSP: u32 = 8; pub const OTG_STS_SELECTOR: u32 = 0xf000;
#[repr(C, packed)] pub struct usb_debug_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDebugInEndpoint: __u8, pub bDebugOutEndpoint: __u8 }
#[repr(C, packed)] pub struct usb_interface_assoc_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bFirstInterface: __u8, pub bInterfaceCount: __u8, pub bFunctionClass: __u8, pub bFunctionSubClass: __u8, pub bFunctionProtocol: __u8, pub iFunction: __u8 }
pub const USB_DT_INTERFACE_ASSOCIATION_SIZE: u32 = 8;
#[repr(C, packed)] pub struct usb_security_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub wTotalLength: __le16, pub bNumEncryptionTypes: __u8 }
#[repr(C, packed)] pub struct usb_key_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub tTKID: [__u8; 3], pub bReserved: __u8, pub bKeyData: [__u8; 0] }
#[repr(C, packed)] pub struct usb_encryption_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bEncryptionType: __u8, pub bEncryptionValue: __u8, pub bAuthKeyIndex: __u8 }
pub const USB_ENC_TYPE_UNSECURE: u32 = 0; pub const USB_ENC_TYPE_WIRED: u32 = 1; pub const USB_ENC_TYPE_CCM_1: u32 = 2; pub const USB_ENC_TYPE_RSA_1: u32 = 3;
#[repr(C, packed)] pub struct usb_bos_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub wTotalLength: __le16, pub bNumDeviceCaps: __u8 }
pub const USB_DT_BOS_SIZE: u32 = 5;
#[repr(C, packed)] pub struct usb_dev_cap_header { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8 }
pub const USB_CAP_TYPE_WIRELESS_USB: u32 = 1; pub const USB_CAP_TYPE_EXT: u32 = 2; pub const USB_SS_CAP_TYPE: u32 = 3; pub const CONTAINER_ID_TYPE: u32 = 4; pub const USB_PLAT_DEV_CAP_TYPE: u32 = 5; pub const USB_SSP_CAP_TYPE: u32 = 0x0a; pub const USB_PTM_CAP_TYPE: u32 = 0x0b;
#[repr(C, packed)] pub struct usb_wireless_cap_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8, pub bmAttributes: __u8, pub wPHYRates: __le16, pub bmTFITXPowerInfo: __u8, pub bmFFITXPowerInfo: __u8, pub bmBandGroup: __le16, pub bReserved: __u8 }
pub const USB_WIRELESS_P2P_DRD: u32 = 1 << 1; pub const USB_WIRELESS_BEACON_MASK: u32 = 3 << 2; pub const USB_WIRELESS_BEACON_SELF: u32 = 1 << 2; pub const USB_WIRELESS_BEACON_DIRECTED: u32 = 2 << 2; pub const USB_WIRELESS_BEACON_NONE: u32 = 3 << 2; pub const USB_WIRELESS_PHY_53: u32 = 1; pub const USB_WIRELESS_PHY_80: u32 = 1 << 1; pub const USB_WIRELESS_PHY_107: u32 = 1 << 2; pub const USB_WIRELESS_PHY_160: u32 = 1 << 3; pub const USB_WIRELESS_PHY_200: u32 = 1 << 4; pub const USB_WIRELESS_PHY_320: u32 = 1 << 5; pub const USB_WIRELESS_PHY_400: u32 = 1 << 6; pub const USB_WIRELESS_PHY_480: u32 = 1 << 7; pub const USB_DT_USB_WIRELESS_CAP_SIZE: u32 = 11;
#[repr(C, packed)] pub struct usb_ext_cap_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8, pub bmAttributes: __le32 }
pub const USB_LPM_SUPPORT: u32 = 1 << 1; pub const USB_BESL_SUPPORT: u32 = 1 << 2; pub const USB_BESL_BASELINE_VALID: u32 = 1 << 3; pub const USB_BESL_DEEP_VALID: u32 = 1 << 4; pub const USB_DT_USB_EXT_CAP_SIZE: u32 = 7; pub const USB_DT_USB_SS_CAP_SIZE: u32 = 10;
#[inline] pub const fn USB_SET_BESL_BASELINE(p: u32) -> u32 { (p & 0xf) << 8 }
#[inline] pub const fn USB_SET_BESL_DEEP(p: u32) -> u32 { (p & 0xf) << 12 }
#[inline] pub const fn USB_GET_BESL_BASELINE(p: u32) -> u32 { (p & (0xf << 8)) >> 8 }
#[inline] pub const fn USB_GET_BESL_DEEP(p: u32) -> u32 { (p & (0xf << 12)) >> 12 }
#[repr(C, packed)] pub struct usb_ss_cap_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8, pub bmAttributes: __u8, pub wSpeedSupported: __le16, pub bFunctionalitySupport: __u8, pub bU1devExitLat: __u8, pub bU2DevExitLat: __le16 }
pub const USB_LTM_SUPPORT: u32 = 1 << 1; pub const USB_LOW_SPEED_OPERATION: u32 = 1; pub const USB_FULL_SPEED_OPERATION: u32 = 1 << 1; pub const USB_HIGH_SPEED_OPERATION: u32 = 1 << 2; pub const USB_5GBPS_OPERATION: u32 = 1 << 3;
#[repr(C, packed)] pub struct usb_ss_container_id_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8, pub bReserved: __u8, pub ContainerID: [__u8; 16] }
pub const USB_DT_USB_SS_CONTN_ID_SIZE: u32 = 20;
#[repr(C, packed)] pub struct usb_plat_dev_cap_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8, pub bReserved: __u8, pub UUID: [__u8; 16], pub CapabilityData: [__u8; 0] }
#[inline] pub const fn USB_DT_USB_PLAT_DEV_CAP_SIZE(capability_data_size: u32) -> u32 { 20 + capability_data_size }
#[repr(C, packed)] pub struct usb_ssp_cap_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8, pub bReserved: __u8, pub bmAttributes: __le32, pub wFunctionalitySupport: __le16, pub wReserved: __le16, pub bmSublinkSpeedAttr: [__le32; 0] }
pub const USB_SSP_SUBLINK_SPEED_ATTRIBS: u32 = 0x1f; pub const USB_SSP_SUBLINK_SPEED_IDS: u32 = 0xf << 5; pub const USB_SSP_MIN_SUBLINK_SPEED_ATTRIBUTE_ID: u32 = 0xf; pub const USB_SSP_MIN_RX_LANE_COUNT: u32 = 0xf << 8; pub const USB_SSP_MIN_TX_LANE_COUNT: u32 = 0xf << 12; pub const USB_SSP_SUBLINK_SPEED_SSID: u32 = 0xf; pub const USB_SSP_SUBLINK_SPEED_LSE: u32 = 3 << 4; pub const USB_SSP_SUBLINK_SPEED_LSE_BPS: u32 = 0; pub const USB_SSP_SUBLINK_SPEED_LSE_KBPS: u32 = 1; pub const USB_SSP_SUBLINK_SPEED_LSE_MBPS: u32 = 2; pub const USB_SSP_SUBLINK_SPEED_LSE_GBPS: u32 = 3; pub const USB_SSP_SUBLINK_SPEED_ST: u32 = 3 << 6; pub const USB_SSP_SUBLINK_SPEED_ST_SYM_RX: u32 = 0; pub const USB_SSP_SUBLINK_SPEED_ST_ASYM_RX: u32 = 1; pub const USB_SSP_SUBLINK_SPEED_ST_SYM_TX: u32 = 2; pub const USB_SSP_SUBLINK_SPEED_ST_ASYM_TX: u32 = 3; pub const USB_SSP_SUBLINK_SPEED_RSVD: u32 = 0x3f << 8; pub const USB_SSP_SUBLINK_SPEED_LP: u32 = 3 << 14; pub const USB_SSP_SUBLINK_SPEED_LP_SS: u32 = 0; pub const USB_SSP_SUBLINK_SPEED_LP_SSP: u32 = 1; pub const USB_SSP_SUBLINK_SPEED_LSM: u32 = 0xff << 16;
#[inline] pub const fn USB_DT_USB_SSP_CAP_SIZE(ssac: u32) -> u32 { 12 + (ssac + 1) * 4 }
#[repr(C, packed)] pub struct usb_pd_cap_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8, pub bReserved: __u8, pub bmAttributes: __le32, pub bmProviderPorts: __le16, pub bmConsumerPorts: __le16, pub bcdBCVersion: __le16, pub bcdPDVersion: __le16, pub bcdUSBTypeCVersion: __le16 }
pub const USB_PD_POWER_DELIVERY_CAPABILITY: u32 = 6; pub const USB_PD_BATTERY_INFO_CAPABILITY: u32 = 7; pub const USB_PD_PD_CONSUMER_PORT_CAPABILITY: u32 = 8; pub const USB_PD_PD_PROVIDER_PORT_CAPABILITY: u32 = 9; pub const USB_PD_CAP_BATTERY_CHARGING: u32 = 1 << 1; pub const USB_PD_CAP_USB_PD: u32 = 1 << 2; pub const USB_PD_CAP_PROVIDER: u32 = 1 << 3; pub const USB_PD_CAP_CONSUMER: u32 = 1 << 4; pub const USB_PD_CAP_CHARGING_POLICY: u32 = 1 << 5; pub const USB_PD_CAP_TYPE_C_CURRENT: u32 = 1 << 6; pub const USB_PD_CAP_PWR_AC: u32 = 1 << 8; pub const USB_PD_CAP_PWR_BAT: u32 = 1 << 9; pub const USB_PD_CAP_PWR_USE_V_BUS: u32 = 1 << 14;
#[repr(C, packed)] pub struct usb_pd_cap_battery_info_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8, pub iBattery: __u8, pub iSerial: __u8, pub iManufacturer: __u8, pub bBatteryId: __u8, pub bReserved: __u8, pub dwChargedThreshold: __le32, pub dwWeakThreshold: __le32, pub dwBatteryDesignCapacity: __le32, pub dwBatteryLastFullchargeCapacity: __le32 }
#[repr(C, packed)] pub struct usb_pd_cap_consumer_port_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8, pub bReserved: __u8, pub bmCapabilities: __u8, pub wMinVoltage: __le16, pub wMaxVoltage: __le16, pub wReserved: __u16, pub dwMaxOperatingPower: __le32, pub dwMaxPeakPower: __le32, pub dwMaxPeakPowerTime: __le32 }
pub const USB_PD_CAP_CONSUMER_BC: u32 = 1; pub const USB_PD_CAP_CONSUMER_PD: u32 = 2; pub const USB_PD_CAP_CONSUMER_TYPE_C: u32 = 4; pub const USB_PD_CAP_CONSUMER_UNKNOWN_PEAK_POWER_TIME: u32 = 0xffff;
#[repr(C, packed)] pub struct usb_pd_cap_provider_port_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8, pub bReserved1: __u8, pub bmCapabilities: __u8, pub bNumOfPDObjects: __u8, pub bReserved2: __u8, pub wPowerDataObject: [__le32; 0] }
pub const USB_PD_CAP_PROVIDER_BC: u32 = 1; pub const USB_PD_CAP_PROVIDER_PD: u32 = 2; pub const USB_PD_CAP_PROVIDER_TYPE_C: u32 = 4;
#[repr(C, packed)] pub struct usb_ptm_cap_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bDevCapabilityType: __u8 }
#[repr(C, packed)] pub struct usb_authentication_capability_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bmAttributes: __u8, pub bcdProtocolVersion: __u8, pub bcdCapability: __u8 }
#[repr(C, packed)] pub struct usb_wireless_ep_comp_descriptor { pub bLength: __u8, pub bDescriptorType: __u8, pub bMaxBurst: __u8, pub bMaxSequence: __u8, pub wMaxStreamDelay: __le16, pub wOverTheAirPacketSize: __le16, pub bOverTheAirInterval: __u8, pub bmCompAttributes: __u8 }
pub const USB_DT_USB_PTM_ID_SIZE: u32 = 3; pub const USB_ENDPOINT_SWITCH_MASK: u32 = 3; pub const USB_ENDPOINT_SWITCH_NO: u32 = 0; pub const USB_ENDPOINT_SWITCH_SWITCH: u32 = 1; pub const USB_ENDPOINT_SWITCH_SCALE: u32 = 2;
#[repr(C, packed)] pub struct usb_handshake { pub bMessageNumber: __u8, pub bStatus: __u8, pub tTKID: [__u8; 3], pub bReserved: __u8, pub CDID: [__u8; 16], pub nonce: [__u8; 16], pub MIC: [__u8; 8] }
#[repr(C, packed)] pub struct usb_connection_context { pub CHID: [__u8; 16], pub CDID: [__u8; 16], pub CK: [__u8; 16] }

#[repr(C)] #[derive(Copy, Clone)] pub enum usb_device_speed { USB_SPEED_UNKNOWN = 0, USB_SPEED_LOW, USB_SPEED_FULL, USB_SPEED_HIGH, USB_SPEED_WIRELESS, USB_SPEED_SUPER, USB_SPEED_SUPER_PLUS }
#[repr(C)] #[derive(Copy, Clone)] pub enum usb_device_state { USB_STATE_NOTATTACHED = 0, USB_STATE_ATTACHED, USB_STATE_POWERED, USB_STATE_RECONNECTING, USB_STATE_UNAUTHENTICATED, USB_STATE_DEFAULT, USB_STATE_ADDRESS, USB_STATE_CONFIGURED, USB_STATE_SUSPENDED }
#[repr(C)] #[derive(Copy, Clone)] pub enum usb3_link_state { USB3_LPM_U0 = 0, USB3_LPM_U1, USB3_LPM_U2, USB3_LPM_U3 }
pub const USB3_LPM_DISABLED: u32 = 0; pub const USB3_LPM_U1_MAX_TIMEOUT: u32 = 0x7f; pub const USB3_LPM_U2_MAX_TIMEOUT: u32 = 0xfe; pub const USB3_LPM_DEVICE_INITIATED: u32 = 0xff;
#[repr(C, packed)] pub struct usb_set_sel_req { pub u1_sel: __u8, pub u1_pel: __u8, pub u2_sel: __le16, pub u2_pel: __le16 }
pub const USB3_LPM_MAX_U1_SEL_PEL: u32 = 0xff; pub const USB3_LPM_MAX_U2_SEL_PEL: u32 = 0xffff; pub const USB_SELF_POWER_VBUS_MAX_DRAW: u32 = 100;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
