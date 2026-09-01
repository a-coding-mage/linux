// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) 2004 James Courtier-Dutton <James@superbug.demon.co.uk>
 *  Driver CA0106 chips. e.g. Sound Blaster Audigy LS and Live 24bit
 *  Version: 0.0.18
 *
 *  FEATURES currently supported:
 *    See ca0106_main.c for features.
 *
 *  Changelog:
 *    Support interrupts per period.
 *    Removed noise from Center/LFE channel when in Analog mode.
 *    Rename and remove mixer controls.
 *  0.0.6
 *    Use separate card based DMA buffer for periods table list.
 *  0.0.7
 *    Change remove and rename ctrls into lists.
 *  0.0.8
 *    Try to fix capture sources.
 *  0.0.9
 *    Fix AC3 output.
 *    Enable S32_LE format support.
 *  0.0.10
 *    Enable playback 48000 and 96000 rates. (Rates other that these do not work, even with "plug:front".)
 *  0.0.11
 *    Add Model name recognition.
 *  0.0.12
 *    Correct interrupt timing. interrupt at end of period, instead of in the middle of a playback period.
 *    Remove redundent "voice" handling.
 *  0.0.13
 *    Single trigger call for multi channels.
 *  0.0.14
 *    Set limits based on what the sound card hardware can do.
 *    playback periods_min=2, periods_max=8
 *    capture hw constraints require period_size = n * 64 bytes.
 *    playback hw constraints require period_size = n * 64 bytes.
 *  0.0.15
 *    Separate ca0106.c into separate functional .c files.
 *  0.0.16
 *    Modified Copyright message.
 *  0.0.17
 *    Add iec958 file in proc file system to show status of SPDIF in.
 *  0.0.18
 *    Implement support for Line-in capture on SB Live 24bit.
 *
 *  This code was initially based on code from ALSA's emu10k1x.c which is:
 *  Copyright (c) by Francisco Moraes <fmoraes@nc.rr.com>
 */

// C dependencies removed from executable Rust:
// linux/delay.h, linux/init.h, linux/interrupt.h, linux/moduleparam.h,
// linux/io.h, sound/core.h, sound/initval.h, sound/pcm.h,
// sound/ac97_codec.h, sound/info.h, sound/asoundef.h, and "ca0106.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u32 = c_uint;

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ca0106 {
    pub card: *mut snd_card,
    pub port: c_ulong,
    pub emu_lock: c_void,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut snd_ca0106,
}

#[repr(C)]
struct snd_ca0106_category_str {
    val: c_int,
    name: *const c_char,
}

unsafe extern "C" {
    static IEC958_AES1_CON_DAT: c_int;
    static IEC958_AES1_CON_VCR: c_int;
    static IEC958_AES1_CON_MICROPHONE: c_int;
    static IEC958_AES1_CON_SYNTHESIZER: c_int;
    static IEC958_AES1_CON_RATE_CONVERTER: c_int;
    static IEC958_AES1_CON_MIXER: c_int;
    static IEC958_AES1_CON_SAMPLER: c_int;
    static IEC958_AES1_CON_PCM_CODER: c_int;
    static IEC958_AES1_CON_IEC908_CD: c_int;
    static IEC958_AES1_CON_NON_IEC908_CD: c_int;
    static IEC958_AES1_CON_GENERAL: c_int;

    static IEC958_AES0_PROFESSIONAL: u32;
    static IEC958_AES0_NONAUDIO: u32;
    static IEC958_AES3_CON_FS: u32;
    static IEC958_AES3_CON_FS_44100: u32;
    static IEC958_AES3_CON_FS_48000: u32;
    static IEC958_AES3_CON_FS_32000: u32;
    static IEC958_AES0_CON_NOT_COPYRIGHT: u32;
    static IEC958_AES0_CON_EMPHASIS: u32;
    static IEC958_AES0_CON_EMPHASIS_5015: u32;
    static IEC958_AES1_CON_CATEGORY: u32;
    static IEC958_AES1_CON_ORIGINAL: u32;
    static IEC958_AES3_CON_CLOCK: u32;
    static IEC958_AES3_CON_CLOCK_1000PPM: u32;
    static IEC958_AES3_CON_CLOCK_50PPM: u32;
    static IEC958_AES3_CON_CLOCK_VARIABLE: u32;
    static IEC958_AES0_PRO_FS: u32;
    static IEC958_AES0_PRO_FS_44100: u32;
    static IEC958_AES0_PRO_FS_48000: u32;
    static IEC958_AES0_PRO_FS_32000: u32;
    static IEC958_AES0_PRO_FREQ_UNLOCKED: u32;
    static IEC958_AES0_PRO_EMPHASIS: u32;
    static IEC958_AES0_PRO_EMPHASIS_CCITT: u32;
    static IEC958_AES0_PRO_EMPHASIS_NONE: u32;
    static IEC958_AES0_PRO_EMPHASIS_5015: u32;
    static IEC958_AES0_PRO_EMPHASIS_NOTID: u32;
    static IEC958_AES1_PRO_MODE: u32;
    static IEC958_AES1_PRO_MODE_STEREOPHONIC: u32;
    static IEC958_AES1_PRO_USERBITS: u32;
    static IEC958_AES1_PRO_USERBITS_192: u32;
    static IEC958_AES1_PRO_USERBITS_UDEF: u32;
    static IEC958_AES2_PRO_SBITS: u32;
    static IEC958_AES2_PRO_SBITS_20: u32;
    static IEC958_AES2_PRO_SBITS_24: u32;
    static IEC958_AES2_PRO_SBITS_UDEF: u32;
    static IEC958_AES2_PRO_WORDLEN: u32;
    static IEC958_AES2_PRO_WORDLEN_22_18: u32;
    static IEC958_AES2_PRO_WORDLEN_23_19: u32;
    static IEC958_AES2_PRO_WORDLEN_24_20: u32;
    static IEC958_AES2_PRO_WORDLEN_20_16: u32;
    static SAMPLE_RATE_TRACKER_STATUS: c_uint;
    static SPDIF_INPUT_STATUS: c_uint;

    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_ca0106_ptr_read(emu: *mut snd_ca0106, reg: c_uint, chn: c_uint) -> c_ulong;
    fn snd_ca0106_ptr_write(emu: *mut snd_ca0106, reg: c_uint, chn: c_uint, data: c_uint);
    fn snd_ca0106_i2c_write(emu: *mut snd_ca0106, reg: c_uint, value: c_uint);
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn outl(value: c_uint, port: c_ulong);
    fn inl(port: c_ulong) -> c_ulong;
    fn inw(port: c_ulong) -> c_uint;
    fn inb(port: c_ulong) -> c_uint;
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut snd_ca0106,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn snd_card_rw_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut snd_ca0106,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
        write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
}

unsafe fn lock_emu_irqsave(_emu: *mut snd_ca0106) {}
unsafe fn unlock_emu_irqrestore(_emu: *mut snd_ca0106) {}

static SND_CA0106_CON_CATEGORY: [snd_ca0106_category_str; 11] = [
    snd_ca0106_category_str { val: unsafe { IEC958_AES1_CON_DAT }, name: c"DAT".as_ptr() },
    snd_ca0106_category_str { val: unsafe { IEC958_AES1_CON_VCR }, name: c"VCR".as_ptr() },
    snd_ca0106_category_str { val: unsafe { IEC958_AES1_CON_MICROPHONE }, name: c"microphone".as_ptr() },
    snd_ca0106_category_str { val: unsafe { IEC958_AES1_CON_SYNTHESIZER }, name: c"synthesizer".as_ptr() },
    snd_ca0106_category_str { val: unsafe { IEC958_AES1_CON_RATE_CONVERTER }, name: c"rate converter".as_ptr() },
    snd_ca0106_category_str { val: unsafe { IEC958_AES1_CON_MIXER }, name: c"mixer".as_ptr() },
    snd_ca0106_category_str { val: unsafe { IEC958_AES1_CON_SAMPLER }, name: c"sampler".as_ptr() },
    snd_ca0106_category_str { val: unsafe { IEC958_AES1_CON_PCM_CODER }, name: c"PCM coder".as_ptr() },
    snd_ca0106_category_str { val: unsafe { IEC958_AES1_CON_IEC908_CD }, name: c"CD".as_ptr() },
    snd_ca0106_category_str { val: unsafe { IEC958_AES1_CON_NON_IEC908_CD }, name: c"non-IEC908 CD".as_ptr() },
    snd_ca0106_category_str { val: unsafe { IEC958_AES1_CON_GENERAL }, name: c"general".as_ptr() },
];

unsafe extern "C" fn snd_ca0106_proc_dump_iec958(buffer: *mut snd_info_buffer, value: u32) {
    let mut i: usize;
    let mut status: [u32; 4] = [0; 4];
    status[0] = value & 0xff;
    status[1] = (value >> 8) & 0xff;
    status[2] = (value >> 16) & 0xff;
    status[3] = (value >> 24) & 0xff;

    if !(status[0] & IEC958_AES0_PROFESSIONAL != 0) {
        /* consumer */
        snd_iprintf(buffer, c"Mode: consumer\n".as_ptr());
        snd_iprintf(buffer, c"Data: ".as_ptr());
        if !(status[0] & IEC958_AES0_NONAUDIO != 0) {
            snd_iprintf(buffer, c"audio\n".as_ptr());
        } else {
            snd_iprintf(buffer, c"non-audio\n".as_ptr());
        }
        snd_iprintf(buffer, c"Rate: ".as_ptr());
        match status[3] & IEC958_AES3_CON_FS {
            x if x == IEC958_AES3_CON_FS_44100 => snd_iprintf(buffer, c"44100 Hz\n".as_ptr()),
            x if x == IEC958_AES3_CON_FS_48000 => snd_iprintf(buffer, c"48000 Hz\n".as_ptr()),
            x if x == IEC958_AES3_CON_FS_32000 => snd_iprintf(buffer, c"32000 Hz\n".as_ptr()),
            _ => snd_iprintf(buffer, c"unknown\n".as_ptr()),
        }
        snd_iprintf(buffer, c"Copyright: ".as_ptr());
        if status[0] & IEC958_AES0_CON_NOT_COPYRIGHT != 0 {
            snd_iprintf(buffer, c"permitted\n".as_ptr());
        } else {
            snd_iprintf(buffer, c"protected\n".as_ptr());
        }
        snd_iprintf(buffer, c"Emphasis: ".as_ptr());
        if (status[0] & IEC958_AES0_CON_EMPHASIS) != IEC958_AES0_CON_EMPHASIS_5015 {
            snd_iprintf(buffer, c"none\n".as_ptr());
        } else {
            snd_iprintf(buffer, c"50/15us\n".as_ptr());
        }
        snd_iprintf(buffer, c"Category: ".as_ptr());
        i = 0;
        while i < SND_CA0106_CON_CATEGORY.len() {
            if (status[1] & IEC958_AES1_CON_CATEGORY) == SND_CA0106_CON_CATEGORY[i].val as u32 {
                snd_iprintf(buffer, c"%s\n".as_ptr(), SND_CA0106_CON_CATEGORY[i].name);
                break;
            }
            i += 1;
        }
        if i >= SND_CA0106_CON_CATEGORY.len() {
            snd_iprintf(buffer, c"unknown 0x%x\n".as_ptr(), status[1] & IEC958_AES1_CON_CATEGORY);
        }
        snd_iprintf(buffer, c"Original: ".as_ptr());
        if status[1] & IEC958_AES1_CON_ORIGINAL != 0 {
            snd_iprintf(buffer, c"original\n".as_ptr());
        } else {
            snd_iprintf(buffer, c"1st generation\n".as_ptr());
        }
        snd_iprintf(buffer, c"Clock: ".as_ptr());
        match status[3] & IEC958_AES3_CON_CLOCK {
            x if x == IEC958_AES3_CON_CLOCK_1000PPM => snd_iprintf(buffer, c"1000 ppm\n".as_ptr()),
            x if x == IEC958_AES3_CON_CLOCK_50PPM => snd_iprintf(buffer, c"50 ppm\n".as_ptr()),
            x if x == IEC958_AES3_CON_CLOCK_VARIABLE => snd_iprintf(buffer, c"variable pitch\n".as_ptr()),
            _ => snd_iprintf(buffer, c"unknown\n".as_ptr()),
        }
    } else {
        snd_iprintf(buffer, c"Mode: professional\n".as_ptr());
        snd_iprintf(buffer, c"Data: ".as_ptr());
        if !(status[0] & IEC958_AES0_NONAUDIO != 0) {
            snd_iprintf(buffer, c"audio\n".as_ptr());
        } else {
            snd_iprintf(buffer, c"non-audio\n".as_ptr());
        }
        snd_iprintf(buffer, c"Rate: ".as_ptr());
        match status[0] & IEC958_AES0_PRO_FS {
            x if x == IEC958_AES0_PRO_FS_44100 => snd_iprintf(buffer, c"44100 Hz\n".as_ptr()),
            x if x == IEC958_AES0_PRO_FS_48000 => snd_iprintf(buffer, c"48000 Hz\n".as_ptr()),
            x if x == IEC958_AES0_PRO_FS_32000 => snd_iprintf(buffer, c"32000 Hz\n".as_ptr()),
            _ => snd_iprintf(buffer, c"unknown\n".as_ptr()),
        }
        snd_iprintf(buffer, c"Rate Locked: ".as_ptr());
        if status[0] & IEC958_AES0_PRO_FREQ_UNLOCKED != 0 {
            snd_iprintf(buffer, c"no\n".as_ptr());
        } else {
            snd_iprintf(buffer, c"yes\n".as_ptr());
        }
        snd_iprintf(buffer, c"Emphasis: ".as_ptr());
        match status[0] & IEC958_AES0_PRO_EMPHASIS {
            x if x == IEC958_AES0_PRO_EMPHASIS_CCITT => snd_iprintf(buffer, c"CCITT J.17\n".as_ptr()),
            x if x == IEC958_AES0_PRO_EMPHASIS_NONE => snd_iprintf(buffer, c"none\n".as_ptr()),
            x if x == IEC958_AES0_PRO_EMPHASIS_5015 => snd_iprintf(buffer, c"50/15us\n".as_ptr()),
            x if x == IEC958_AES0_PRO_EMPHASIS_NOTID => snd_iprintf(buffer, c"unknown\n".as_ptr()),
            _ => snd_iprintf(buffer, c"unknown\n".as_ptr()),
        }
        snd_iprintf(buffer, c"Stereophonic: ".as_ptr());
        if (status[1] & IEC958_AES1_PRO_MODE) == IEC958_AES1_PRO_MODE_STEREOPHONIC {
            snd_iprintf(buffer, c"stereo\n".as_ptr());
        } else {
            snd_iprintf(buffer, c"not indicated\n".as_ptr());
        }
        snd_iprintf(buffer, c"Userbits: ".as_ptr());
        match status[1] & IEC958_AES1_PRO_USERBITS {
            x if x == IEC958_AES1_PRO_USERBITS_192 => snd_iprintf(buffer, c"192bit\n".as_ptr()),
            x if x == IEC958_AES1_PRO_USERBITS_UDEF => snd_iprintf(buffer, c"user-defined\n".as_ptr()),
            _ => snd_iprintf(buffer, c"unknown\n".as_ptr()),
        }
        snd_iprintf(buffer, c"Sample Bits: ".as_ptr());
        match status[2] & IEC958_AES2_PRO_SBITS {
            x if x == IEC958_AES2_PRO_SBITS_20 => snd_iprintf(buffer, c"20 bit\n".as_ptr()),
            x if x == IEC958_AES2_PRO_SBITS_24 => snd_iprintf(buffer, c"24 bit\n".as_ptr()),
            x if x == IEC958_AES2_PRO_SBITS_UDEF => snd_iprintf(buffer, c"user defined\n".as_ptr()),
            _ => snd_iprintf(buffer, c"unknown\n".as_ptr()),
        }
        snd_iprintf(buffer, c"Word Length: ".as_ptr());
        match status[2] & IEC958_AES2_PRO_WORDLEN {
            x if x == IEC958_AES2_PRO_WORDLEN_22_18 => snd_iprintf(buffer, c"22 bit or 18 bit\n".as_ptr()),
            x if x == IEC958_AES2_PRO_WORDLEN_23_19 => snd_iprintf(buffer, c"23 bit or 19 bit\n".as_ptr()),
            x if x == IEC958_AES2_PRO_WORDLEN_24_20 => snd_iprintf(buffer, c"24 bit or 20 bit\n".as_ptr()),
            x if x == IEC958_AES2_PRO_WORDLEN_20_16 => snd_iprintf(buffer, c"20 bit or 16 bit\n".as_ptr()),
            _ => snd_iprintf(buffer, c"unknown\n".as_ptr()),
        }
    }
}

unsafe extern "C" fn snd_ca0106_proc_iec958(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let emu: *mut snd_ca0106 = (*entry).private_data;
    let mut value: u32;

    value = snd_ca0106_ptr_read(emu, SAMPLE_RATE_TRACKER_STATUS, 0) as u32;
    snd_iprintf(
        buffer,
        c"Status: %s, %s, %s\n".as_ptr(),
        if value & 0x100000 != 0 { c"Rate Locked".as_ptr() } else { c"Not Rate Locked".as_ptr() },
        if value & 0x200000 != 0 { c"SPDIF Locked".as_ptr() } else { c"No SPDIF Lock".as_ptr() },
        if value & 0x400000 != 0 { c"Audio Valid".as_ptr() } else { c"No valid audio".as_ptr() },
    );
    snd_iprintf(
        buffer,
        c"Estimated sample rate: %u\n".as_ptr(),
        ((value & 0xfffff).wrapping_mul(48000)) / 0x8000,
    );
    if value & 0x200000 != 0 {
        snd_iprintf(buffer, c"IEC958/SPDIF input status:\n".as_ptr());
        value = snd_ca0106_ptr_read(emu, SPDIF_INPUT_STATUS, 0) as u32;
        snd_ca0106_proc_dump_iec958(buffer, value);
    }

    snd_iprintf(buffer, c"\n".as_ptr());
}

unsafe extern "C" fn snd_ca0106_proc_reg_write32(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let emu: *mut snd_ca0106 = (*entry).private_data;
    let mut line: [c_char; 64] = [0; 64];
    let mut reg: u32 = 0;
    let mut val: u32 = 0;
    while snd_info_get_line(buffer, line.as_mut_ptr(), line.len() as c_int) == 0 {
        if sscanf(line.as_ptr(), c"%x %x".as_ptr(), &mut reg, &mut val) != 2 {
            continue;
        }
        if reg < 0x40 && val <= 0xffffffff {
            lock_emu_irqsave(emu);
            outl(val, (*emu).port + (reg & 0xfffffffc) as c_ulong);
            unlock_emu_irqrestore(emu);
        }
    }
}

unsafe extern "C" fn snd_ca0106_proc_reg_read32(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let emu: *mut snd_ca0106 = (*entry).private_data;
    let mut value: c_ulong;
    let mut i: c_int;

    snd_iprintf(buffer, c"Registers:\n\n".as_ptr());
    i = 0;
    while i < 0x20 {
        lock_emu_irqsave(emu);
        value = inl((*emu).port + i as c_ulong);
        unlock_emu_irqrestore(emu);
        snd_iprintf(buffer, c"Register %02X: %08lX\n".as_ptr(), i, value);
        i += 4;
    }
}

unsafe extern "C" fn snd_ca0106_proc_reg_read16(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let emu: *mut snd_ca0106 = (*entry).private_data;
    let mut value: c_uint;
    let mut i: c_int;

    snd_iprintf(buffer, c"Registers:\n\n".as_ptr());
    i = 0;
    while i < 0x20 {
        lock_emu_irqsave(emu);
        value = inw((*emu).port + i as c_ulong);
        unlock_emu_irqrestore(emu);
        snd_iprintf(buffer, c"Register %02X: %04X\n".as_ptr(), i, value);
        i += 2;
    }
}

unsafe extern "C" fn snd_ca0106_proc_reg_read8(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let emu: *mut snd_ca0106 = (*entry).private_data;
    let mut value: c_uint;
    let mut i: c_int;

    snd_iprintf(buffer, c"Registers:\n\n".as_ptr());
    i = 0;
    while i < 0x20 {
        lock_emu_irqsave(emu);
        value = inb((*emu).port + i as c_ulong);
        unlock_emu_irqrestore(emu);
        snd_iprintf(buffer, c"Register %02X: %02X\n".as_ptr(), i, value);
        i += 1;
    }
}

unsafe extern "C" fn snd_ca0106_proc_reg_read1(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let emu: *mut snd_ca0106 = (*entry).private_data;
    let mut value: c_ulong;
    let mut i: c_int;
    let mut j: c_int;

    snd_iprintf(buffer, c"Registers\n".as_ptr());
    i = 0;
    while i < 0x40 {
        snd_iprintf(buffer, c"%02X: ".as_ptr(), i);
        j = 0;
        while j < 4 {
            value = snd_ca0106_ptr_read(emu, i as c_uint, j as c_uint);
            snd_iprintf(buffer, c"%08lX ".as_ptr(), value);
            j += 1;
        }
        snd_iprintf(buffer, c"\n".as_ptr());
        i += 1;
    }
}

unsafe extern "C" fn snd_ca0106_proc_reg_read2(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let emu: *mut snd_ca0106 = (*entry).private_data;
    let mut value: c_ulong;
    let mut i: c_int;
    let mut j: c_int;

    snd_iprintf(buffer, c"Registers\n".as_ptr());
    i = 0x40;
    while i < 0x80 {
        snd_iprintf(buffer, c"%02X: ".as_ptr(), i);
        j = 0;
        while j < 4 {
            value = snd_ca0106_ptr_read(emu, i as c_uint, j as c_uint);
            snd_iprintf(buffer, c"%08lX ".as_ptr(), value);
            j += 1;
        }
        snd_iprintf(buffer, c"\n".as_ptr());
        i += 1;
    }
}

unsafe extern "C" fn snd_ca0106_proc_reg_write(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let emu: *mut snd_ca0106 = (*entry).private_data;
    let mut line: [c_char; 64] = [0; 64];
    let mut reg: c_uint = 0;
    let mut channel_id: c_uint = 0;
    let mut val: c_uint = 0;
    while snd_info_get_line(buffer, line.as_mut_ptr(), line.len() as c_int) == 0 {
        if sscanf(
            line.as_ptr(),
            c"%x %x %x".as_ptr(),
            &mut reg,
            &mut channel_id,
            &mut val,
        ) != 3
        {
            continue;
        }
        if reg < 0x80 && val <= 0xffffffff && channel_id <= 3 {
            snd_ca0106_ptr_write(emu, reg, channel_id, val);
        }
    }
}

unsafe extern "C" fn snd_ca0106_proc_i2c_write(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let emu: *mut snd_ca0106 = (*entry).private_data;
    let mut line: [c_char; 64] = [0; 64];
    let mut reg: c_uint = 0;
    let mut val: c_uint = 0;
    while snd_info_get_line(buffer, line.as_mut_ptr(), line.len() as c_int) == 0 {
        if sscanf(line.as_ptr(), c"%x %x".as_ptr(), &mut reg, &mut val) != 2 {
            continue;
        }
        if (reg <= 0x7f) || (val <= 0x1ff) {
            snd_ca0106_i2c_write(emu, reg, val);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_ca0106_proc_init(emu: *mut snd_ca0106) -> c_int {
    snd_card_ro_proc_new(
        (*emu).card,
        c"iec958".as_ptr(),
        emu,
        Some(snd_ca0106_proc_iec958),
    );
    snd_card_rw_proc_new(
        (*emu).card,
        c"ca0106_reg32".as_ptr(),
        emu,
        Some(snd_ca0106_proc_reg_read32),
        Some(snd_ca0106_proc_reg_write32),
    );
    snd_card_ro_proc_new(
        (*emu).card,
        c"ca0106_reg16".as_ptr(),
        emu,
        Some(snd_ca0106_proc_reg_read16),
    );
    snd_card_ro_proc_new(
        (*emu).card,
        c"ca0106_reg8".as_ptr(),
        emu,
        Some(snd_ca0106_proc_reg_read8),
    );
    snd_card_rw_proc_new(
        (*emu).card,
        c"ca0106_regs1".as_ptr(),
        emu,
        Some(snd_ca0106_proc_reg_read1),
        Some(snd_ca0106_proc_reg_write),
    );
    snd_card_rw_proc_new(
        (*emu).card,
        c"ca0106_i2c".as_ptr(),
        emu,
        None,
        Some(snd_ca0106_proc_i2c_write),
    );
    snd_card_ro_proc_new(
        (*emu).card,
        c"ca0106_regs2".as_ptr(),
        emu,
        Some(snd_ca0106_proc_reg_read2),
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
