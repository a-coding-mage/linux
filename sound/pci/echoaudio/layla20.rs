// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ALSA driver for Echoaudio soundcards.
 *  Copyright (C) 2003-2004 Giuliano Pochini <pochini@shiny.it>
 */

pub const ECHOGALS_FAMILY: bool = true;
pub const ECHOCARD_LAYLA20: bool = true;
pub const ECHOCARD_NAME: &str = "Layla20";
pub const ECHOCARD_HAS_MONITOR: bool = true;
pub const ECHOCARD_HAS_ASIC: bool = true;
pub const ECHOCARD_HAS_INPUT_GAIN: bool = true;
pub const ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL: bool = true;
pub const ECHOCARD_HAS_SUPER_INTERLEAVE: bool = true;
pub const ECHOCARD_HAS_DIGITAL_IO: bool = true;
pub const ECHOCARD_HAS_EXTERNAL_CLOCK: bool = true;
pub const ECHOCARD_HAS_ADAT: bool = false;
pub const ECHOCARD_HAS_OUTPUT_CLOCK_SWITCH: bool = true;
pub const ECHOCARD_HAS_MIDI: bool = true;

/* Pipe indexes */
pub const PX_ANALOG_OUT: u32 = 0; /* 10 */
pub const PX_DIGITAL_OUT: u32 = 10; /*  2 */
pub const PX_ANALOG_IN: u32 = 12; /*  8 */
pub const PX_DIGITAL_IN: u32 = 20; /*  2 */
pub const PX_NUM: u32 = 22;

/* Bus indexes */
pub const BX_ANALOG_OUT: u32 = 0; /* 10 */
pub const BX_DIGITAL_OUT: u32 = 10; /*  2 */
pub const BX_ANALOG_IN: u32 = 12; /*  8 */
pub const BX_DIGITAL_IN: u32 = 20; /*  2 */
pub const BX_NUM: u32 = 22;

/*
 * C dependencies:
 * linux/delay.h, linux/init.h, linux/interrupt.h, linux/pci.h,
 * linux/module.h, linux/firmware.h, linux/slab.h, linux/io.h,
 * sound/core.h, sound/info.h, sound/control.h, sound/tlv.h,
 * sound/pcm.h, sound/pcm_params.h, sound/asoundef.h, sound/initval.h,
 * sound/rawmidi.h, linux/atomic.h, and "echoaudio.h".
 */

// MODULE_FIRMWARE("ea/layla20_dsp.fw");
// MODULE_FIRMWARE("ea/layla20_asic.fw");

pub const FW_LAYLA20_DSP: usize = 0;
pub const FW_LAYLA20_ASIC: usize = 1;

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

static CARD_FW_NAME_0: &[u8] = b"layla20_dsp.fw\0";
static CARD_FW_NAME_1: &[u8] = b"layla20_asic.fw\0";

pub static CARD_FW: [firmware; 2] = [
    firmware {
        size: 0,
        data: CARD_FW_NAME_0.as_ptr(),
    },
    firmware {
        size: 0,
        data: CARD_FW_NAME_1.as_ptr(),
    },
];

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: usize,
}

pub const fn PCI_DEVICE_SUB(
    vendor: u32,
    device: u32,
    subvendor: u32,
    subdevice: u32,
) -> pci_device_id {
    pci_device_id {
        vendor,
        device,
        subvendor,
        subdevice,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }
}

pub static SND_ECHO_IDS: [pci_device_id; 3] = [
    PCI_DEVICE_SUB(0x1057, 0x1801, 0xECC0, 0x0030), /* DSP 56301 Layla20 rev.0 */
    PCI_DEVICE_SUB(0x1057, 0x1801, 0xECC0, 0x0031), /* DSP 56301 Layla20 rev.1 */
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
}

extern "C" {
    static SNDRV_PCM_INFO_MMAP: u32;
    static SNDRV_PCM_INFO_INTERLEAVED: u32;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: u32;
    static SNDRV_PCM_INFO_MMAP_VALID: u32;
    static SNDRV_PCM_INFO_PAUSE: u32;
    static SNDRV_PCM_INFO_SYNC_START: u32;
    static SNDRV_PCM_FMTBIT_U8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_3LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_BE: u64;
    static SNDRV_PCM_RATE_8000_48000: u32;
    static SNDRV_PCM_RATE_CONTINUOUS: u32;
}

pub static PCM_HARDWARE_SKEL: snd_pcm_hardware = unsafe {
    snd_pcm_hardware {
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
        rates: SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 8000,
        rate_max: 50000,
        channels_min: 1,
        channels_max: 10,
        buffer_bytes_max: 262144,
        period_bytes_min: 32,
        period_bytes_max: 131072,
        periods_min: 2,
        periods_max: 220,
        /*
         * One page (4k) contains 512 instructions. I don't know if the hw
         * supports lists longer than this. In this case periods_max=220 is a
         * safe limit to make sure the list never exceeds 512 instructions.
         */
    }
};

/*
 * C implementation dependencies included here:
 * "layla20_dsp.c"
 * "echoaudio_dsp.c"
 * "echoaudio.c"
 * "midi.c"
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
