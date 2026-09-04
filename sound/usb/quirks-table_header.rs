// SPDX-License-Identifier: GPL-2.0-or-later
//
// ALSA USB Audio Driver
//
// Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>,
//                       Clemens Ladisch <clemens@ladisch.de>

// The contents of this file are part of the driver's id_table.
//
// In a perfect world, this file would be empty.

// Use this for devices where other interfaces are standard compliant,
// to prevent the quirk being applied to those interfaces. (To work with
// hotplugging, bDeviceClass must be set to USB_CLASS_PER_INTERFACE.)
macro_rules! USB_DEVICE_VENDOR_SPEC {
    ($vend:expr, $prod:expr) => {{
        // .match_flags = USB_DEVICE_ID_MATCH_VENDOR | USB_DEVICE_ID_MATCH_PRODUCT | USB_DEVICE_ID_MATCH_INT_CLASS,
        // .idVendor = $vend,
        // .idProduct = $prod,
        // .bInterfaceClass = USB_CLASS_VENDOR_SPEC
        (
            USB_DEVICE_ID_MATCH_VENDOR | USB_DEVICE_ID_MATCH_PRODUCT | USB_DEVICE_ID_MATCH_INT_CLASS,
            $vend,
            $prod,
            USB_CLASS_VENDOR_SPEC,
        )
    }};
}

// A standard entry matching with vid/pid and the audio class/subclass
macro_rules! USB_AUDIO_DEVICE {
    ($vend:expr, $prod:expr) => {{
        // .match_flags = USB_DEVICE_ID_MATCH_DEVICE | USB_DEVICE_ID_MATCH_INT_CLASS | USB_DEVICE_ID_MATCH_INT_SUBCLASS,
        // .idVendor = $vend,
        // .idProduct = $prod,
        // .bInterfaceClass = USB_CLASS_AUDIO,
        // .bInterfaceSubClass = USB_SUBCLASS_AUDIOCONTROL
        (
            USB_DEVICE_ID_MATCH_DEVICE | USB_DEVICE_ID_MATCH_INT_CLASS | USB_DEVICE_ID_MATCH_INT_SUBCLASS,
            $vend,
            $prod,
            USB_CLASS_AUDIO,
            USB_SUBCLASS_AUDIOCONTROL,
        )
    }};
}

// Quirk .driver_info, followed by the definition of the quirk entry;
// put like QUIRK_DRIVER_INFO { ... } in each entry of the quirk table
macro_rules! QUIRK_DRIVER_INFO {
    ($quirk:expr) => {{
        // .driver_info = (unsigned long)&(const struct snd_usb_audio_quirk)
        $quirk as *const _
    }};
}

// Macros for quirk data entries

// Quirk data entry for ignoring the interface
macro_rules! QUIRK_DATA_IGNORE {
    ($ifno:expr) => {{
        // .ifnum = ($ifno), .type = QUIRK_IGNORE_INTERFACE
        ($ifno, QUIRK_IGNORE_INTERFACE)
    }};
}

// Quirk data entry for a standard audio interface
macro_rules! QUIRK_DATA_STANDARD_AUDIO {
    ($ifno:expr) => {{
        // .ifnum = ($ifno), .type = QUIRK_AUDIO_STANDARD_INTERFACE
        ($ifno, QUIRK_AUDIO_STANDARD_INTERFACE)
    }};
}

// Quirk data entry for a standard MIDI interface
macro_rules! QUIRK_DATA_STANDARD_MIDI {
    ($ifno:expr) => {{
        // .ifnum = ($ifno), .type = QUIRK_MIDI_STANDARD_INTERFACE
        ($ifno, QUIRK_MIDI_STANDARD_INTERFACE)
    }};
}

// Quirk data entry for a standard mixer interface
macro_rules! QUIRK_DATA_STANDARD_MIXER {
    ($ifno:expr) => {{
        // .ifnum = ($ifno), .type = QUIRK_AUDIO_STANDARD_MIXER
        ($ifno, QUIRK_AUDIO_STANDARD_MIXER)
    }};
}

// Quirk data entry for Yamaha MIDI
macro_rules! QUIRK_DATA_MIDI_YAMAHA {
    ($ifno:expr) => {{
        // .ifnum = ($ifno), .type = QUIRK_MIDI_YAMAHA
        ($ifno, QUIRK_MIDI_YAMAHA)
    }};
}

// Quirk data entry for Edirol UAxx
macro_rules! QUIRK_DATA_EDIROL_UAXX {
    ($ifno:expr) => {{
        // .ifnum = ($ifno), .type = QUIRK_AUDIO_EDIROL_UAXX
        ($ifno, QUIRK_AUDIO_EDIROL_UAXX)
    }};
}

// Quirk data entry for raw bytes interface
macro_rules! QUIRK_DATA_RAW_BYTES {
    ($ifno:expr) => {{
        // .ifnum = ($ifno), .type = QUIRK_MIDI_RAW_BYTES
        ($ifno, QUIRK_MIDI_RAW_BYTES)
    }};
}

// Quirk composite array terminator
// { .ifnum = -1 }
const QUIRK_COMPOSITE_END: i32 = -1;

// Quirk data entry for composite quirks;
// followed by the quirk array that is terminated with QUIRK_COMPOSITE_END
// e.g. QUIRK_DATA_COMPOSITE { { quirk1 }, { quirk2 },..., QUIRK_COMPOSITE_END }
macro_rules! QUIRK_DATA_COMPOSITE {
    () => {{
        // .ifnum = QUIRK_ANY_INTERFACE, .type = QUIRK_COMPOSITE, .data = &(const struct snd_usb_audio_quirk[])
        (QUIRK_ANY_INTERFACE, QUIRK_COMPOSITE)
    }};
}

// Quirk data entry for a fixed audio endpoint;
// followed by audioformat definition
// e.g. QUIRK_DATA_AUDIOFORMAT(n) { .formats = xxx, ... }
macro_rules! QUIRK_DATA_AUDIOFORMAT {
    ($ifno:expr) => {{
        // .ifnum = ($ifno), .type = QUIRK_AUDIO_FIXED_ENDPOINT, .data = &(const struct audioformat)
        ($ifno, QUIRK_AUDIO_FIXED_ENDPOINT)
    }};
}

// Quirk data entry for a fixed MIDI endpoint;
// followed by snd_usb_midi_endpoint_info definition
// e.g. QUIRK_DATA_MIDI_FIXED_ENDPOINT(n) { .out_cables = x, .in_cables = y }
macro_rules! QUIRK_DATA_MIDI_FIXED_ENDPOINT {
    ($ifno:expr) => {{
        // .ifnum = ($ifno), .type = QUIRK_MIDI_FIXED_ENDPOINT, .data = &(const struct snd_usb_midi_endpoint_info)
        ($ifno, QUIRK_MIDI_FIXED_ENDPOINT)
    }};
}

// Quirk data entry for a MIDIMAN MIDI endpoint
macro_rules! QUIRK_DATA_MIDI_MIDIMAN {
    ($ifno:expr) => {{
        // .ifnum = ($ifno), .type = QUIRK_MIDI_MIDIMAN, .data = &(const struct snd_usb_midi_endpoint_info)
        ($ifno, QUIRK_MIDI_MIDIMAN)
    }};
}

// Quirk data entry for a EMAGIC MIDI endpoint
macro_rules! QUIRK_DATA_MIDI_EMAGIC {
    ($ifno:expr) => {{
        // .ifnum = ($ifno), .type = QUIRK_MIDI_EMAGIC, .data = &(const struct snd_usb_midi_endpoint_info)
        ($ifno, QUIRK_MIDI_EMAGIC)
    }};
}

// Device matching macros for convenience
macro_rules! USB_DEVICE {
    ($vend:expr, $prod:expr) => {
        (0x0582u16, $vend, $prod)
    };
}

macro_rules! USB_DEVICE_VER {
    ($vend:expr, $prod:expr, $lo:expr, $hi:expr) => {
        (0x0582u16, $vend, $prod, $lo, $hi)
    };
}

// Yamaha device macros
macro_rules! YAMAHA_DEVICE {
    ($id:expr, $name:expr) => {
        // { USB_DEVICE(0x0499, $id), QUIRK_DRIVER_INFO { ... } }
        (0x0499u16, $id, $name, QUIRK_ANY_INTERFACE)
    };
}

macro_rules! YAMAHA_INTERFACE {
    ($id:expr, $intf:expr, $name:expr) => {
        // { USB_DEVICE_VENDOR_SPEC(0x0499, $id), QUIRK_DRIVER_INFO { ... } }
        (0x0499u16, $id, $intf, $name)
    };
}

// Arturia AF16Rig macros
macro_rules! QUIRK_AF16RIG {
    (
        $channels:expr, $iface:expr, $low_rate:expr, $high_rate:expr,
        $pack_size:expr, $clock:expr, $ep_idx:expr, $ep:expr
    ) => {
        (
            $channels,
            $iface,
            $low_rate,
            $high_rate,
            $pack_size,
            $clock,
            $ep_idx,
            $ep,
        )
    };
}

macro_rules! QUIRK_AF16RIG_CLOCK {
    ($clock:expr) => {
        (
            QUIRK_AF16RIG!(34, 1, 44100, 48000, 0x3b8, $clock, 1, 0x01),
            QUIRK_AF16RIG!(34, 1, 44100, 48000, 0x3b8, $clock, 2, 0x81),
            QUIRK_AF16RIG!(18, 2, 88200, 96000, 0x3a8, $clock, 1, 0x01),
            QUIRK_AF16RIG!(18, 2, 88200, 96000, 0x3a8, $clock, 2, 0x81),
            QUIRK_AF16RIG!(10, 3, 176400, 192000, 0x3e8, $clock, 1, 0x01),
            QUIRK_AF16RIG!(10, 3, 176400, 192000, 0x3e8, $clock, 2, 0x81),
        )
    };
}

// Constants for interface numbers and quirk types (declared but not defined - from external dependencies)
extern "C" {
    // USB class and subclass constants
    static USB_CLASS_PER_INTERFACE: u8;
    static USB_CLASS_VENDOR_SPEC: u8;
    static USB_CLASS_AUDIO: u8;
    static USB_SUBCLASS_AUDIOCONTROL: u8;

    // Device ID match flags
    static USB_DEVICE_ID_MATCH_VENDOR: u32;
    static USB_DEVICE_ID_MATCH_PRODUCT: u32;
    static USB_DEVICE_ID_MATCH_INT_CLASS: u32;
    static USB_DEVICE_ID_MATCH_DEVICE: u32;
    static USB_DEVICE_ID_MATCH_INT_SUBCLASS: u32;
    static USB_DEVICE_ID_MATCH_DEV_SUBCLASS: u32;

    // Quirk type constants
    static QUIRK_IGNORE_INTERFACE: u32;
    static QUIRK_AUDIO_STANDARD_INTERFACE: u32;
    static QUIRK_MIDI_STANDARD_INTERFACE: u32;
    static QUIRK_AUDIO_STANDARD_MIXER: u32;
    static QUIRK_MIDI_YAMAHA: u32;
    static QUIRK_AUDIO_EDIROL_UAXX: u32;
    static QUIRK_MIDI_RAW_BYTES: u32;
    static QUIRK_COMPOSITE: u32;
    static QUIRK_AUDIO_FIXED_ENDPOINT: u32;
    static QUIRK_MIDI_FIXED_ENDPOINT: u32;
    static QUIRK_MIDI_MIDIMAN: u32;
    static QUIRK_MIDI_EMAGIC: u32;
    static QUIRK_MIDI_FTDI: u32;
    static QUIRK_AUTODETECT: u32;
    static QUIRK_NODEV_INTERFACE: u32;

    // Special interface numbers
    static QUIRK_ANY_INTERFACE: i32;

    // Audio format flags
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_3LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;

    // Sample rate constants and flags
    static SNDRV_PCM_RATE_CONTINUOUS: u32;
    static SNDRV_PCM_RATE_44100: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_RATE_88200: u32;
    static SNDRV_PCM_RATE_96000: u32;
    static SNDRV_PCM_RATE_176400: u32;
    static SNDRV_PCM_RATE_192000: u32;

    // USB endpoint transfer types
    static USB_ENDPOINT_XFER_ISOC: u32;

    // UAC attribute flags
    static UAC_EP_CS_ATTR_FILL_MAX: u32;
    static UAC_EP_CS_ATTR_SAMPLE_RATE: u32;
    static UAC_VERSION_2: u32;

    // Struct types (declared for reference)
    pub struct snd_usb_audio_quirk;
    pub struct audioformat;
    pub struct snd_usb_midi_endpoint_info;
}

// Here we go... the quirk table definition begins:
// The actual quirk table entries from the C source would be represented
// as constant data structures initialized with the macros above.
//
// NOTE: This header file is typically included in a C translation unit
// where these entries form an array/table. In Rust, the corresponding
// quirk table entries would be initialized as:
//
// const QUIRK_TABLE: &[QuirkEntry] = &[
//     // FTDI devices
//     QuirkEntry { usb_device: USB_DEVICE(0x0403, 0xb8d8), quirk_info: { ... } },
//     // ... etc
// ];
//
// The structure definitions for QuirkEntry, snd_usb_audio_quirk, audioformat,
// and snd_usb_midi_endpoint_info must be defined in other translation units
// that are linked with this one.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
