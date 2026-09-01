// SPDX-License-Identifier: GPL-2.0
/*
 * dice-extension.c - a part of driver for DICE based devices
 *
 * Copyright (c) 2018 Takashi Sakamoto
 */

// Translated from C source. The original included "dice.h"; declarations and
// constants supplied there are referenced here as external repository items.

use core::ffi::{c_int, c_uint, c_void};
use core::mem::size_of;

type __be32 = u32;

#[repr(C)]
pub struct snd_dice {
    pub unit: *mut c_void,
    pub clock_caps: c_uint,
    pub tx_pcm_chs: [[c_uint; 3]; MAX_STREAMS],
    pub tx_midi_ports: [c_uint; MAX_STREAMS],
    pub rx_pcm_chs: [[c_uint; 3]; MAX_STREAMS],
    pub rx_midi_ports: [c_uint; MAX_STREAMS],
}

unsafe extern "C" {
    fn snd_fw_transaction(
        unit: *mut c_void,
        tcode: c_uint,
        offset: u64,
        buffer: *mut c_void,
        length: usize,
        flags: c_uint,
    ) -> c_int;
    fn be32_to_cpu(value: __be32) -> u32;
    fn kmalloc_array(n: usize, size: usize, flags: c_uint) -> *mut __be32;
    fn kfree(ptr: *mut __be32);
}

/* For TCD2210/2220, TCAT defines extension of application protocol. */

const DICE_EXT_APP_SPACE: u64 = 0xffffe0200000u64;

const DICE_EXT_APP_CAPS_OFFSET: u32 = 0x00;
const DICE_EXT_APP_CAPS_SIZE: u32 = 0x04;
const DICE_EXT_APP_CMD_OFFSET: u32 = 0x08;
const DICE_EXT_APP_CMD_SIZE: u32 = 0x0c;
const DICE_EXT_APP_MIXER_OFFSET: u32 = 0x10;
const DICE_EXT_APP_MIXER_SIZE: u32 = 0x14;
const DICE_EXT_APP_PEAK_OFFSET: u32 = 0x18;
const DICE_EXT_APP_PEAK_SIZE: u32 = 0x1c;
const DICE_EXT_APP_ROUTER_OFFSET: u32 = 0x20;
const DICE_EXT_APP_ROUTER_SIZE: u32 = 0x24;
const DICE_EXT_APP_STREAM_OFFSET: u32 = 0x28;
const DICE_EXT_APP_STREAM_SIZE: u32 = 0x2c;
const DICE_EXT_APP_CURRENT_OFFSET: u32 = 0x30;
const DICE_EXT_APP_CURRENT_SIZE: u32 = 0x34;
const DICE_EXT_APP_STANDALONE_OFFSET: u32 = 0x38;
const DICE_EXT_APP_STANDALONE_SIZE: u32 = 0x3c;
const DICE_EXT_APP_APPLICATION_OFFSET: u32 = 0x40;
const DICE_EXT_APP_APPLICATION_SIZE: u32 = 0x44;

const EXT_APP_STREAM_TX_NUMBER: u32 = 0x0000;
const EXT_APP_STREAM_RX_NUMBER: u32 = 0x0004;
const EXT_APP_STREAM_ENTRIES: u32 = 0x0008;
const EXT_APP_STREAM_ENTRY_SIZE: u32 = 0x010c;
const EXT_APP_NUMBER_AUDIO: u32 = 0x0000;
const EXT_APP_NUMBER_MIDI: u32 = 0x0004;
const EXT_APP_NAMES: u32 = 0x0008;
const EXT_APP_NAMES_SIZE: u32 = 256;
const EXT_APP_AC3: u32 = 0x0108;

const EXT_APP_CONFIG_LOW_ROUTER: u32 = 0x0000;
const EXT_APP_CONFIG_LOW_STREAM: u32 = 0x1000;
const EXT_APP_CONFIG_MIDDLE_ROUTER: u32 = 0x2000;
const EXT_APP_CONFIG_MIDDLE_STREAM: u32 = 0x3000;
const EXT_APP_CONFIG_HIGH_ROUTER: u32 = 0x4000;
const EXT_APP_CONFIG_HIGH_STREAM: u32 = 0x5000;

#[inline]
unsafe fn read_transaction(
    dice: *mut snd_dice,
    section_addr: u64,
    offset: u32,
    buf: *mut c_void,
    len: usize,
) -> c_int {
    unsafe {
        snd_fw_transaction(
            (*dice).unit,
            if len == 4 {
                TCODE_READ_QUADLET_REQUEST
            } else {
                TCODE_READ_BLOCK_REQUEST
            },
            section_addr + offset as u64,
            buf,
            len,
            0,
        )
    }
}

unsafe fn read_stream_entries(
    dice: *mut snd_dice,
    section_addr: u64,
    base_offset: u32,
    stream_count: c_uint,
    mode: c_uint,
    pcm_channels: *mut [c_uint; 3],
    midi_ports: *mut c_uint,
) -> c_int {
    let mut entry_offset: u32;
    let mut reg: [__be32; 2] = [0; 2];
    let mut err: c_int;
    let mut i: c_int;

    i = 0;
    while i < stream_count as c_int {
        entry_offset = base_offset + i as u32 * EXT_APP_STREAM_ENTRY_SIZE;
        err = unsafe {
            read_transaction(
                dice,
                section_addr,
                entry_offset + EXT_APP_NUMBER_AUDIO,
                reg.as_mut_ptr() as *mut c_void,
                size_of::<[__be32; 2]>(),
            )
        };
        if err < 0 {
            return err;
        }
        unsafe {
            (*pcm_channels.add(i as usize))[mode as usize] = be32_to_cpu(reg[0]);
            *midi_ports.add(i as usize) =
                (*midi_ports.add(i as usize)).max(be32_to_cpu(reg[1]));
        }

        i += 1;
    }

    0
}

unsafe fn detect_stream_formats(dice: *mut snd_dice, section_addr: u64) -> c_int {
    let mut base_offset: u32;
    let mut reg: [__be32; 2] = [0; 2];
    let mut stream_count: c_uint;
    let mut mode: c_int;
    let mut err: c_int = 0;

    mode = 0;
    while mode < SND_DICE_RATE_MODE_COUNT as c_int {
        let cap: c_uint;

        /*
         * Some models report stream formats at highest mode, however
         * they don't support the mode. Check clock capabilities.
         */
        if mode == 2 {
            cap = CLOCK_CAP_RATE_176400 | CLOCK_CAP_RATE_192000;
        } else if mode == 1 {
            cap = CLOCK_CAP_RATE_88200 | CLOCK_CAP_RATE_96000;
        } else {
            cap = CLOCK_CAP_RATE_32000 | CLOCK_CAP_RATE_44100 | CLOCK_CAP_RATE_48000;
        }
        if (cap & unsafe { (*dice).clock_caps }) == 0 {
            mode += 1;
            continue;
        }

        base_offset = 0x2000 * mode as u32 + 0x1000;

        err = unsafe {
            read_transaction(
                dice,
                section_addr,
                base_offset + EXT_APP_STREAM_TX_NUMBER,
                reg.as_mut_ptr() as *mut c_void,
                size_of::<[__be32; 2]>(),
            )
        };
        if err < 0 {
            break;
        }

        base_offset += EXT_APP_STREAM_ENTRIES;
        stream_count = unsafe { be32_to_cpu(reg[0]) }.min(MAX_STREAMS as c_uint);
        err = unsafe {
            read_stream_entries(
                dice,
                section_addr,
                base_offset,
                stream_count,
                mode as c_uint,
                (*dice).tx_pcm_chs.as_mut_ptr(),
                (*dice).tx_midi_ports.as_mut_ptr(),
            )
        };
        if err < 0 {
            break;
        }

        base_offset += stream_count * EXT_APP_STREAM_ENTRY_SIZE;
        stream_count = unsafe { be32_to_cpu(reg[1]) }.min(MAX_STREAMS as c_uint);
        err = unsafe {
            read_stream_entries(
                dice,
                section_addr,
                base_offset,
                stream_count,
                mode as c_uint,
                (*dice).rx_pcm_chs.as_mut_ptr(),
                (*dice).rx_midi_ports.as_mut_ptr(),
            )
        };
        if err < 0 {
            break;
        }

        mode += 1;
    }

    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dice_detect_extension_formats(dice: *mut snd_dice) -> c_int {
    let pointers: *mut __be32;
    let mut i: c_uint;
    let section_addr: u64;
    let mut err: c_int;

    pointers = unsafe { kmalloc_array(9, size_of::<__be32>() * 2, GFP_KERNEL) };
    if pointers.is_null() {
        return -ENOMEM;
    }

    err = unsafe {
        snd_fw_transaction(
            (*dice).unit,
            TCODE_READ_BLOCK_REQUEST,
            DICE_EXT_APP_SPACE,
            pointers as *mut c_void,
            9 * size_of::<__be32>() * 2,
            0,
        )
    };
    if err < 0 {
        unsafe {
            kfree(pointers);
        }
        return err;
    }

    /* Check two of them for offset have the same value or not. */
    i = 0;
    while i < 9 {
        let mut j: c_int;

        j = i as c_int + 1;
        while j < 9 {
            if unsafe { *pointers.add(i as usize * 2) == *pointers.add(j as usize * 2) } {
                // Fallback to limited functionality.
                err = -ENXIO;
                unsafe {
                    kfree(pointers);
                }
                return err;
            }
            j += 1;
        }
        i += 1;
    }

    section_addr = DICE_EXT_APP_SPACE + unsafe { be32_to_cpu(*pointers.add(12)) } as u64 * 4;
    err = unsafe { detect_stream_formats(dice, section_addr) };

    unsafe {
        kfree(pointers);
    }
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
