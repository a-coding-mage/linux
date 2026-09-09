/* SPDX-License-Identifier: GPL-2.0 */
/*
 * USB IrDA Bridge Device Definition
 */

/* This device should use Application-specific class */
pub const USB_SUBCLASS_IRDA: u8 = 0x02;

/* Class-Specific requests (bRequest field) */
pub const USB_REQ_CS_IRDA_RECEIVING: u8 = 1;
pub const USB_REQ_CS_IRDA_CHECK_MEDIA_BUSY: u8 = 3;
pub const USB_REQ_CS_IRDA_RATE_SNIFF: u8 = 4;
pub const USB_REQ_CS_IRDA_UNICAST_LIST: u8 = 5;
pub const USB_REQ_CS_IRDA_GET_CLASS_DESC: u8 = 6;

/* Class-Specific descriptor */
pub const USB_DT_CS_IRDA: u8 = 0x21;

/* Data sizes */
pub const USB_IRDA_DS_2048: u8 = 1 << 5;
pub const USB_IRDA_DS_1024: u8 = 1 << 4;
pub const USB_IRDA_DS_512: u8 = 1 << 3;
pub const USB_IRDA_DS_256: u8 = 1 << 2;
pub const USB_IRDA_DS_128: u8 = 1 << 1;
pub const USB_IRDA_DS_64: u8 = 1 << 0;

/* Window sizes */
pub const USB_IRDA_WS_7: u8 = 1 << 6;
pub const USB_IRDA_WS_6: u8 = 1 << 5;
pub const USB_IRDA_WS_5: u8 = 1 << 4;
pub const USB_IRDA_WS_4: u8 = 1 << 3;
pub const USB_IRDA_WS_3: u8 = 1 << 2;
pub const USB_IRDA_WS_2: u8 = 1 << 1;
pub const USB_IRDA_WS_1: u8 = 1 << 0;

/* Min turnaround times in usecs */
pub const USB_IRDA_MTT_0: u8 = 1 << 7;
pub const USB_IRDA_MTT_10: u8 = 1 << 6;
pub const USB_IRDA_MTT_50: u8 = 1 << 5;
pub const USB_IRDA_MTT_100: u8 = 1 << 4;
pub const USB_IRDA_MTT_500: u8 = 1 << 3;
pub const USB_IRDA_MTT_1000: u8 = 1 << 2;
pub const USB_IRDA_MTT_5000: u8 = 1 << 1;
pub const USB_IRDA_MTT_10000: u8 = 1 << 0;

/* Baud rates */
pub const USB_IRDA_BR_4000000: u16 = 1 << 8;
pub const USB_IRDA_BR_1152000: u8 = 1 << 7;
pub const USB_IRDA_BR_576000: u8 = 1 << 6;
pub const USB_IRDA_BR_115200: u8 = 1 << 5;
pub const USB_IRDA_BR_57600: u8 = 1 << 4;
pub const USB_IRDA_BR_38400: u8 = 1 << 3;
pub const USB_IRDA_BR_19200: u8 = 1 << 2;
pub const USB_IRDA_BR_9600: u8 = 1 << 1;
pub const USB_IRDA_BR_2400: u8 = 1 << 0;

/* Additional BOFs */
pub const USB_IRDA_AB_0: u8 = 1 << 7;
pub const USB_IRDA_AB_1: u8 = 1 << 6;
pub const USB_IRDA_AB_2: u8 = 1 << 5;
pub const USB_IRDA_AB_3: u8 = 1 << 4;
pub const USB_IRDA_AB_6: u8 = 1 << 3;
pub const USB_IRDA_AB_12: u8 = 1 << 2;
pub const USB_IRDA_AB_24: u8 = 1 << 1;
pub const USB_IRDA_AB_48: u8 = 1 << 0;

/* IRDA Rate Sniff */
pub const USB_IRDA_RATE_SNIFF: u8 = 1;

#[repr(C, packed)]
pub struct usb_irda_cs_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdSpecRevision: u16,
    pub bmDataSize: u8,
    pub bmWindowSize: u8,
    pub bmMinTurnaroundTime: u8,
    pub wBaudRate: u16,
    pub bmAdditionalBOFs: u8,
    pub bIrdaRateSniff: u8,
    pub bMaxUnicastList: u8,
}

/* Data Format */
pub const USB_IRDA_STATUS_MEDIA_BUSY: u8 = 1 << 7;

/* The following is a 4-bit value used for both inbound and outbound headers. */
pub const USB_IRDA_STATUS_LINK_SPEED: u8 = 0x0f;

pub const USB_IRDA_LS_NO_CHANGE: u8 = 0;
pub const USB_IRDA_LS_2400: u8 = 1;
pub const USB_IRDA_LS_9600: u8 = 2;
pub const USB_IRDA_LS_19200: u8 = 3;
pub const USB_IRDA_LS_38400: u8 = 4;
pub const USB_IRDA_LS_57600: u8 = 5;
pub const USB_IRDA_LS_115200: u8 = 6;
pub const USB_IRDA_LS_576000: u8 = 7;
pub const USB_IRDA_LS_1152000: u8 = 8;
pub const USB_IRDA_LS_4000000: u8 = 9;

/* The following is a 4-bit value used only for outbound header. */
pub const USB_IRDA_EXTRA_BOFS: u8 = 0xf0;

#[repr(C)]
pub struct usb_irda_inbound_header {
    pub bmStatus: u8,
}

#[repr(C)]
pub struct usb_irda_outbound_header {
    pub bmChange: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
