/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * <linux/usb/midi.h> -- USB MIDI definitions.
 *
 * Copyright (C) 2006 Thumtronics Pty Ltd.
 * Developed for Thumtronics by Grey Innovation
 * Ben Williamson <ben.williamson@greyinnovation.com>
 *
 * This software is distributed under the terms of the GNU General Public
 * License ("GPL"), as published by the Free Software Foundation.
 *
 * This file holds USB constants and structures defined
 * by the USB Device Class Definition for MIDI Devices.
 */

// Dependency supplied by linux/types.h in the original header.

/* A.1  MS Class-Specific Interface Descriptor Subtypes */
pub const USB_MS_HEADER: __u8 = 0x01;
pub const USB_MS_MIDI_IN_JACK: __u8 = 0x02;
pub const USB_MS_MIDI_OUT_JACK: __u8 = 0x03;
pub const USB_MS_ELEMENT: __u8 = 0x04;

/* A.2  MS Class-Specific Endpoint Descriptor Subtypes */
pub const USB_MS_GENERAL: __u8 = 0x01;

/* A.3  MS MIDI IN and OUT Jack Types */
pub const USB_MS_EMBEDDED: __u8 = 0x01;
pub const USB_MS_EXTERNAL: __u8 = 0x02;

/* 6.1.2.1  Class-Specific MS Interface Header Descriptor */
#[repr(C, packed)]
pub struct usb_ms_header_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bcdMSC: __le16,
    pub wTotalLength: __le16,
}

pub const USB_DT_MS_HEADER_SIZE: usize = 7;

/* 6.1.2.2  MIDI IN Jack Descriptor */
#[repr(C, packed)]
pub struct usb_midi_in_jack_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8, /* USB_DT_CS_INTERFACE */
    pub bDescriptorSubtype: __u8, /* USB_MS_MIDI_IN_JACK */
    pub bJackType: __u8, /* USB_MS_EMBEDDED/EXTERNAL */
    pub bJackID: __u8,
    pub iJack: __u8,
}

pub const USB_DT_MIDI_IN_SIZE: usize = 6;

#[repr(C, packed)]
pub struct usb_midi_source_pin {
    pub baSourceID: __u8,
    pub baSourcePin: __u8,
}

/* 6.1.2.3  MIDI OUT Jack Descriptor */
#[repr(C, packed)]
pub struct usb_midi_out_jack_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8, /* USB_DT_CS_INTERFACE */
    pub bDescriptorSubtype: __u8, /* USB_MS_MIDI_OUT_JACK */
    pub bJackType: __u8, /* USB_MS_EMBEDDED/EXTERNAL */
    pub bJackID: __u8,
    pub bNrInputPins: __u8, /* p */
    pub pins: [usb_midi_source_pin; 0], /* [p], flexible array member */
}

pub const fn USB_DT_MIDI_OUT_SIZE(p: usize) -> usize { 7 + 2 * p }

/* As above, but more useful for defining your own descriptors: */
#[macro_export]
macro_rules! DECLARE_USB_MIDI_OUT_JACK_DESCRIPTOR {
    ($p:literal) => {
        #[repr(C, packed)]
        pub struct usb_midi_out_jack_descriptor_$p {
            pub bLength: __u8,
            pub bDescriptorType: __u8,
            pub bDescriptorSubtype: __u8,
            pub bJackType: __u8,
            pub bJackID: __u8,
            pub bNrInputPins: __u8,
            pub pins: [usb_midi_source_pin; $p],
            pub iJack: __u8,
        }
    };
}

/* 6.2.2  Class-Specific MS Bulk Data Endpoint Descriptor */
#[repr(C, packed)]
pub struct usb_ms_endpoint_descriptor {
    pub bLength: __u8, /* 4+n */
    pub bDescriptorType: __u8, /* USB_DT_CS_ENDPOINT */
    pub bDescriptorSubtype: __u8, /* USB_MS_GENERAL */
    pub bNumEmbMIDIJack: __u8, /* n */
    pub baAssocJackID: [__u8; 0], /* [n], flexible array member */
}

pub const fn USB_DT_MS_ENDPOINT_SIZE(n: usize) -> usize { 4 + n }

/* As above, but more useful for defining your own descriptors: */
#[macro_export]
macro_rules! DECLARE_USB_MS_ENDPOINT_DESCRIPTOR {
    ($n:literal) => {
        #[repr(C, packed)]
        pub struct usb_ms_endpoint_descriptor_$n {
            pub bLength: __u8,
            pub bDescriptorType: __u8,
            pub bDescriptorSubtype: __u8,
            pub bNumEmbMIDIJack: __u8,
            pub baAssocJackID: [__u8; $n],
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
