/* SPDX-License-Identifier: GPL-2.0 */
/*
 * <linux/usb/midi-v2.h> -- USB MIDI 2.0 definitions.
 */

// Dependencies supplied by the corresponding Linux type and USB MIDI headers.

/* A.1 MS Class-Specific Interface Descriptor Types */
pub const USB_DT_CS_GR_TRM_BLOCK: u8 = 0x26;

/* A.1 MS Class-Specific Interface Descriptor Subtypes */
/* same as MIDI 1.0 */

/* A.2 MS Class-Specific Endpoint Descriptor Subtypes */
pub const USB_MS_GENERAL_2_0: u8 = 0x02;

/* A.3 MS Class-Specific Group Terminal Block Descriptor Subtypes */
pub const USB_MS_GR_TRM_BLOCK_UNDEFINED: u8 = 0x00;
pub const USB_MS_GR_TRM_BLOCK_HEADER: u8 = 0x01;
pub const USB_MS_GR_TRM_BLOCK: u8 = 0x02;

/* A.4 MS Interface Header MIDIStreaming Class Revision */
pub const USB_MS_REV_MIDI_1_0: u16 = 0x0100;
pub const USB_MS_REV_MIDI_2_0: u16 = 0x0200;

/* A.5 MS MIDI IN and OUT Jack Types */
/* same as MIDI 1.0 */

/* A.6 Group Terminal Block Types */
pub const USB_MS_GR_TRM_BLOCK_TYPE_BIDIRECTIONAL: u8 = 0x00;
pub const USB_MS_GR_TRM_BLOCK_TYPE_INPUT_ONLY: u8 = 0x01;
pub const USB_MS_GR_TRM_BLOCK_TYPE_OUTPUT_ONLY: u8 = 0x02;

/* A.7 Group Terminal Default MIDI Protocol */
pub const USB_MS_MIDI_PROTO_UNKNOWN: u8 = 0x00; /* Unknown (Use MIDI-CI) */
pub const USB_MS_MIDI_PROTO_1_0_64: u8 = 0x01; /* MIDI 1.0, UMP up to 64bits */
pub const USB_MS_MIDI_PROTO_1_0_64_JRTS: u8 = 0x02; /* MIDI 1.0, UMP up to 64bits, Jitter Reduction Timestamps */
pub const USB_MS_MIDI_PROTO_1_0_128: u8 = 0x03; /* MIDI 1.0, UMP up to 128bits */
pub const USB_MS_MIDI_PROTO_1_0_128_JRTS: u8 = 0x04; /* MIDI 1.0, UMP up to 128bits, Jitter Reduction Timestamps */
pub const USB_MS_MIDI_PROTO_2_0: u8 = 0x11; /* MIDI 2.0 */
pub const USB_MS_MIDI_PROTO_2_0_JRTS: u8 = 0x12; /* MIDI 2.0, Jitter Reduction Timestamps */

/* 5.2.2.1 Class-Specific MS Interface Header Descriptor */
/* Same as MIDI 1.0, use struct usb_ms_header_descriptor */

/* 5.3.2 Class-Specific MIDI Streaming Data Endpoint Descriptor */
#[repr(C, packed)]
pub struct usb_ms20_endpoint_descriptor {
    pub bLength: u8, // 4+n
    pub bDescriptorType: u8, // USB_DT_CS_ENDPOINT
    pub bDescriptorSubtype: u8, // USB_MS_GENERAL_2_0
    pub bNumGrpTrmBlock: u8, // Number of Group Terminal Blocks: n
    pub baAssoGrpTrmBlkID: [u8; 0], // ID of the Group Terminal Blocks [n]
}

#[inline]
pub const fn USB_DT_MS20_ENDPOINT_SIZE(n: usize) -> usize {
    4 + n
}

/* As above, but more useful for defining your own descriptors: */
macro_rules! DECLARE_USB_MS20_ENDPOINT_DESCRIPTOR {
    ($name:ident, $n:literal) => {
        #[repr(C, packed)]
        pub struct $name {
            pub bLength: u8,
            pub bDescriptorType: u8,
            pub bDescriptorSubtype: u8,
            pub bNumGrpTrmBlock: u8,
            pub baAssoGrpTrmBlkID: [u8; $n],
        }
    };
}

/* 5.4.1 Class-Specific Group Terminal Block Header Descriptor */
#[repr(C, packed)]
pub struct usb_ms20_gr_trm_block_header_descriptor {
    pub bLength: u8, // 5
    pub bDescriptorType: u8, // USB_DT_CS_GR_TRM_BLOCK
    pub bDescriptorSubtype: u8, // USB_MS_GR_TRM_BLOCK_HEADER
    pub wTotalLength: u16, // Total number of bytes
}

/* 5.4.2.1 Group Terminal Block Descriptor */
#[repr(C, packed)]
pub struct usb_ms20_gr_trm_block_descriptor {
    pub bLength: u8, // 13
    pub bDescriptorType: u8, // USB_DT_CS_GR_TRM_BLOCK
    pub bDescriptorSubtype: u8, // USB_MS_GR_TRM_BLOCK
    pub bGrpTrmBlkID: u8, // ID of this Group Terminal Block
    pub bGrpTrmBlkType: u8, // Group Terminal Block Type
    pub nGroupTrm: u8, // The first member Group Terminal in this block
    pub nNumGroupTrm: u8, // Number of member Group Terminals spanned
    pub iBlockItem: u8, // String ID of Block item
    pub bMIDIProtocol: u8, // Default MIDI protocol
    pub wMaxInputBandwidth: u16, // Max input bandwidth capability in 4kB/s
    pub wMaxOutputBandwidth: u16, // Max output bandwidth capability in 4kB/s
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
