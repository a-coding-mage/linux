// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ALSA driver for Echoaudio soundcards.
 *  Copyright (C) 2003-2004 Giuliano Pochini <pochini@shiny.it>
 */

pub const ECHO24_FAMILY: bool = true;
pub const ECHOCARD_MONA: bool = true;
pub const ECHOCARD_NAME: &str = "Mona";
pub const ECHOCARD_HAS_MONITOR: bool = true;
pub const ECHOCARD_HAS_ASIC: bool = true;
pub const ECHOCARD_HAS_SUPER_INTERLEAVE: bool = true;
pub const ECHOCARD_HAS_DIGITAL_IO: bool = true;
pub const ECHOCARD_HAS_DIGITAL_IN_AUTOMUTE: bool = true;
pub const ECHOCARD_HAS_DIGITAL_MODE_SWITCH: bool = true;
pub const ECHOCARD_HAS_EXTERNAL_CLOCK: bool = true;
pub const ECHOCARD_HAS_ADAT: i32 = 6;
pub const ECHOCARD_HAS_STEREO_BIG_ENDIAN32: bool = true;

/* Pipe indexes */
pub const PX_ANALOG_OUT: i32 = 0; /* 6 */
pub const PX_DIGITAL_OUT: i32 = 6; /* 8 */
pub const PX_ANALOG_IN: i32 = 14; /* 4 */
pub const PX_DIGITAL_IN: i32 = 18; /* 8 */
pub const PX_NUM: i32 = 26;

/* Bus indexes */
pub const BX_ANALOG_OUT: i32 = 0; /* 6 */
pub const BX_DIGITAL_OUT: i32 = 6; /* 8 */
pub const BX_ANALOG_IN: i32 = 14; /* 4 */
pub const BX_DIGITAL_IN: i32 = 18; /* 8 */
pub const BX_NUM: i32 = 26;

// C dependencies removed from executable Rust:
// linux/delay.h, linux/init.h, linux/interrupt.h, linux/pci.h,
// linux/module.h, linux/firmware.h, linux/slab.h, linux/io.h,
// sound/core.h, sound/info.h, sound/control.h, sound/tlv.h,
// sound/pcm.h, sound/pcm_params.h, sound/asoundef.h, sound/initval.h,
// linux/atomic.h, and echoaudio.h.

// MODULE_FIRMWARE("ea/loader_dsp.fw");
// MODULE_FIRMWARE("ea/mona_301_dsp.fw");
// MODULE_FIRMWARE("ea/mona_361_dsp.fw");
// MODULE_FIRMWARE("ea/mona_301_1_asic_48.fw");
// MODULE_FIRMWARE("ea/mona_301_1_asic_96.fw");
// MODULE_FIRMWARE("ea/mona_361_1_asic_48.fw");
// MODULE_FIRMWARE("ea/mona_361_1_asic_96.fw");
// MODULE_FIRMWARE("ea/mona_2_asic.fw");

pub const FW_361_LOADER: usize = 0;
pub const FW_MONA_301_DSP: usize = 1;
pub const FW_MONA_361_DSP: usize = 2;
pub const FW_MONA_301_1_ASIC48: usize = 3;
pub const FW_MONA_301_1_ASIC96: usize = 4;
pub const FW_MONA_361_1_ASIC48: usize = 5;
pub const FW_MONA_361_1_ASIC96: usize = 6;
pub const FW_MONA_2_ASIC: usize = 7;

static card_fw: [firmware; 8] = [
    firmware {
        size: 0,
        data: b"loader_dsp.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"mona_301_dsp.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"mona_361_dsp.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"mona_301_1_asic_48.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"mona_301_1_asic_96.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"mona_361_1_asic_48.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"mona_361_1_asic_96.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"mona_2_asic.fw\0".as_ptr(),
    },
];

static snd_echo_ids: [pci_device_id; 7] = [
    /* DSP 56301 Mona rev.0 */
    PCI_DEVICE_SUB!(0x1057, 0x1801, 0xECC0, 0x0070),
    /* DSP 56301 Mona rev.1 */
    PCI_DEVICE_SUB!(0x1057, 0x1801, 0xECC0, 0x0071),
    /* DSP 56301 Mona rev.2 */
    PCI_DEVICE_SUB!(0x1057, 0x1801, 0xECC0, 0x0072),
    /* DSP 56361 Mona rev.0 */
    PCI_DEVICE_SUB!(0x1057, 0x3410, 0xECC0, 0x0070),
    /* DSP 56361 Mona rev.1 */
    PCI_DEVICE_SUB!(0x1057, 0x3410, 0xECC0, 0x0071),
    /* DSP 56361 Mona rev.2 */
    PCI_DEVICE_SUB!(0x1057, 0x3410, 0xECC0, 0x0072),
    pci_device_id::default(),
];

static pcm_hardware_skel: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_SYNC_START,
    formats: SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S24_3LE
        | SNDRV_PCM_FMTBIT_S32_LE
        | SNDRV_PCM_FMTBIT_S32_BE,
    rates: SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000,
    rate_min: 8000,
    rate_max: 96000,
    channels_min: 1,
    channels_max: 8,
    buffer_bytes_max: 262144,
    period_bytes_min: 32,
    period_bytes_max: 131072,
    periods_min: 2,
    periods_max: 220,
    /* One page (4k) contains 512 instructions. I don't know if the hw
    supports lists longer than this. In this case periods_max=220 is a
    safe limit to make sure the list never exceeds 512 instructions. */
};

include!("mona_dsp.rs");
include!("echoaudio_dsp.rs");
include!("echoaudio_gml.rs");
include!("echoaudio.rs");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
