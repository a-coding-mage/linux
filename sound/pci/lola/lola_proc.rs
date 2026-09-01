// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Support for Digigram Lola PCI-e boards
 *
 *  Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lola_bar {
    pub remap_addr: *mut c_void,
}

#[repr(C)]
pub struct lola_pin {
    pub num_pins: c_int,
}

#[repr(C)]
pub struct lola_pcm {
    pub num_streams: c_int,
}

#[repr(C)]
pub struct lola {
    pub card: *mut snd_card,
    pub bar: [lola_bar; 2],
    pub pin: [lola_pin; 2],
    pub pcm: [lola_pcm; 2],
    pub lola_caps: c_uint,
    pub debug_res: c_uint,
    pub debug_res_ex: c_uint,
}

type c_uchar = u8;

unsafe extern "C" {
    static CAPT: c_int;
    static PLAY: c_int;
    static BAR0: c_int;
    static BAR1: c_int;
    static LOLA_PAR_VENDOR_ID: c_uint;
    static LOLA_PAR_FUNCTION_TYPE: c_uint;
    static LOLA_PAR_SPECIFIC_CAPS: c_uint;
    static LOLA_PAR_AUDIO_WIDGET_CAP: c_uint;
    static LOLA_PAR_STREAM_FORMATS: c_uint;
    static LOLA_PAR_AMP_IN_CAP: c_uint;
    static LOLA_PAR_AMP_OUT_CAP: c_uint;
    static LOLA_VERB_GET_MAX_LEVEL: c_uint;
    static LOLA_VERB_GET_CLOCK_LIST: c_uint;
    static LOLA_CLOCK_TYPE_INTERNAL: c_uchar;
    static LOLA_CLOCK_TYPE_VIDEO: c_uchar;
    static STS: c_int;
    static LPIB: c_int;
    static CTL: c_int;
    static LVI: c_int;
    static BDPL: c_int;
    static BDPU: c_int;

    fn LOLA_AMP_MUTE_CAPABLE(val: c_uint) -> c_uint;
    fn LOLA_AMP_STEP_SIZE(val: c_uint) -> c_uint;
    fn LOLA_AMP_NUM_STEPS(val: c_uint) -> c_uint;
    fn LOLA_AMP_OFFSET(val: c_uint) -> c_uint;
    fn LOLA_AFG_CLOCK_WIDGET_PRESENT(val: c_uint) -> bool;
    fn LOLA_AFG_MIXER_WIDGET_PRESENT(val: c_uint) -> bool;
    fn lola_read_param(chip: *mut lola, nid: c_int, param: c_uint, val: *mut c_uint) -> c_int;
    fn lola_codec_read(
        chip: *mut lola,
        nid: c_uint,
        verb: c_uint,
        data: c_uint,
        extdata: c_uint,
        val: *mut c_uint,
        extval: *mut c_uint,
    ) -> c_int;
    fn lola_sample_rate_convert(coded: c_uint) -> c_uint;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn readl(addr: *const c_void) -> c_uint;
    fn lola_dsd_read(chip: *mut lola, dsd: c_int, reg: c_int) -> c_uint;
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut lola,
        read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer),
    ) -> c_int;
    fn snd_card_rw_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut lola,
        read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer),
        write: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer),
    ) -> c_int;
}

unsafe extern "C" fn print_audio_widget(
    buffer: *mut snd_info_buffer,
    chip: *mut lola,
    nid: c_int,
    name: *const c_char,
) {
    let mut val: c_uint = 0;

    unsafe {
        lola_read_param(chip, nid, LOLA_PAR_AUDIO_WIDGET_CAP, &mut val);
        snd_iprintf(buffer, c"Node 0x%02x %s wcaps 0x%x\n".as_ptr(), nid, name, val);
        lola_read_param(chip, nid, LOLA_PAR_STREAM_FORMATS, &mut val);
        snd_iprintf(buffer, c"  Formats: 0x%x\n".as_ptr(), val);
    }
}

unsafe extern "C" fn print_pin_widget(
    buffer: *mut snd_info_buffer,
    chip: *mut lola,
    nid: c_int,
    ampcap: c_uint,
    name: *const c_char,
) {
    let mut val: c_uint = 0;

    unsafe {
        lola_read_param(chip, nid, LOLA_PAR_AUDIO_WIDGET_CAP, &mut val);
        snd_iprintf(buffer, c"Node 0x%02x %s wcaps 0x%x\n".as_ptr(), nid, name, val);
        if val == 0x00400200 {
            return;
        }
        lola_read_param(chip, nid, ampcap, &mut val);
        snd_iprintf(buffer, c"  Amp-Caps: 0x%x\n".as_ptr(), val);
        snd_iprintf(
            buffer,
            c"    mute=%d, step-size=%d, steps=%d, ofs=%d\n".as_ptr(),
            LOLA_AMP_MUTE_CAPABLE(val),
            LOLA_AMP_STEP_SIZE(val),
            LOLA_AMP_NUM_STEPS(val),
            LOLA_AMP_OFFSET(val),
        );
        lola_codec_read(
            chip,
            nid as c_uint,
            LOLA_VERB_GET_MAX_LEVEL,
            0,
            0,
            &mut val,
            core::ptr::null_mut(),
        );
        snd_iprintf(buffer, c"  Max-level: 0x%x\n".as_ptr(), val);
    }
}

unsafe extern "C" fn print_clock_widget(
    buffer: *mut snd_info_buffer,
    chip: *mut lola,
    nid: c_int,
) {
    let mut i: c_int;
    let mut j: c_int;
    let num_clocks: c_int;
    let mut val: c_uint = 0;

    unsafe {
        lola_read_param(chip, nid, LOLA_PAR_AUDIO_WIDGET_CAP, &mut val);
        snd_iprintf(buffer, c"Node 0x%02x [Clock] wcaps 0x%x\n".as_ptr(), nid, val);
        num_clocks = (val & 0xff) as c_int;
        i = 0;
        while i < num_clocks {
            let mut res_ex: c_uint = 0;
            let mut items: [u16; 4] = [0; 4];
            let name: *const c_char;

            lola_codec_read(
                chip,
                nid as c_uint,
                LOLA_VERB_GET_CLOCK_LIST,
                i as c_uint,
                0,
                &mut val,
                &mut res_ex,
            );
            items[0] = (val & 0xfff) as u16;
            items[1] = ((val >> 16) & 0xfff) as u16;
            items[2] = (res_ex & 0xfff) as u16;
            items[3] = ((res_ex >> 16) & 0xfff) as u16;
            j = 0;
            while j < 4 {
                let type_ = (items[j as usize] >> 8) as c_uchar;
                let mut freq = (items[j as usize] & 0xff) as c_uint;
                if i + j >= num_clocks {
                    break;
                }
                if type_ == LOLA_CLOCK_TYPE_INTERNAL {
                    name = c"Internal".as_ptr();
                    freq = lola_sample_rate_convert(freq);
                } else if type_ == LOLA_CLOCK_TYPE_VIDEO {
                    name = c"Video".as_ptr();
                    freq = lola_sample_rate_convert(freq);
                } else {
                    name = c"Other".as_ptr();
                }
                snd_iprintf(
                    buffer,
                    c"  Clock %d: Type %d:%s, freq=%d\n".as_ptr(),
                    i + j,
                    type_ as c_int,
                    name,
                    freq,
                );
                j += 1;
            }
            i += 4;
        }
    }
}

unsafe extern "C" fn print_mixer_widget(
    buffer: *mut snd_info_buffer,
    chip: *mut lola,
    nid: c_int,
) {
    let mut val: c_uint = 0;

    unsafe {
        lola_read_param(chip, nid, LOLA_PAR_AUDIO_WIDGET_CAP, &mut val);
        snd_iprintf(buffer, c"Node 0x%02x [Mixer] wcaps 0x%x\n".as_ptr(), nid, val);
    }
}

unsafe extern "C" fn lola_proc_codec_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let chip = unsafe { (*entry).private_data as *mut lola };
    let mut val: c_uint = 0;
    let mut i: c_int;
    let mut nid: c_int;

    unsafe {
        lola_read_param(chip, 0, LOLA_PAR_VENDOR_ID, &mut val);
        snd_iprintf(buffer, c"Vendor: 0x%08x\n".as_ptr(), val);
        lola_read_param(chip, 1, LOLA_PAR_FUNCTION_TYPE, &mut val);
        snd_iprintf(buffer, c"Function Type: %d\n".as_ptr(), val);
        lola_read_param(chip, 1, LOLA_PAR_SPECIFIC_CAPS, &mut val);
        snd_iprintf(buffer, c"Specific-Caps: 0x%08x\n".as_ptr(), val);
        snd_iprintf(
            buffer,
            c"  Pins-In %d, Pins-Out %d\n".as_ptr(),
            (*chip).pin[CAPT as usize].num_pins,
            (*chip).pin[PLAY as usize].num_pins,
        );
        nid = 2;
        i = 0;
        while i < (*chip).pcm[CAPT as usize].num_streams {
            print_audio_widget(buffer, chip, nid, c"[Audio-In]".as_ptr());
            i += 1;
            nid += 1;
        }
        i = 0;
        while i < (*chip).pcm[PLAY as usize].num_streams {
            print_audio_widget(buffer, chip, nid, c"[Audio-Out]".as_ptr());
            i += 1;
            nid += 1;
        }
        i = 0;
        while i < (*chip).pin[CAPT as usize].num_pins {
            print_pin_widget(buffer, chip, nid, LOLA_PAR_AMP_IN_CAP, c"[Pin-In]".as_ptr());
            i += 1;
            nid += 1;
        }
        i = 0;
        while i < (*chip).pin[PLAY as usize].num_pins {
            print_pin_widget(buffer, chip, nid, LOLA_PAR_AMP_OUT_CAP, c"[Pin-Out]".as_ptr());
            i += 1;
            nid += 1;
        }
        if LOLA_AFG_CLOCK_WIDGET_PRESENT((*chip).lola_caps) {
            print_clock_widget(buffer, chip, nid);
            nid += 1;
        }
        if LOLA_AFG_MIXER_WIDGET_PRESENT((*chip).lola_caps) {
            print_mixer_widget(buffer, chip, nid);
            nid += 1;
        }
    }
}

/* direct codec access for debugging */
unsafe extern "C" fn lola_proc_codec_rw_write(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let chip = unsafe { (*entry).private_data as *mut lola };
    let mut line: [c_char; 64] = [0; 64];
    let mut id: c_uint = 0;
    let mut verb: c_uint = 0;
    let mut data: c_uint = 0;
    let mut extdata: c_uint = 0;
    unsafe {
        while snd_info_get_line(buffer, line.as_mut_ptr(), line.len() as c_int) == 0 {
            if sscanf(
                line.as_ptr(),
                c"%u %u %u %u".as_ptr(),
                &mut id,
                &mut verb,
                &mut data,
                &mut extdata,
            ) != 4
            {
                continue;
            }
            lola_codec_read(
                chip,
                id,
                verb,
                data,
                extdata,
                &mut (*chip).debug_res,
                &mut (*chip).debug_res_ex,
            );
        }
    }
}

unsafe extern "C" fn lola_proc_codec_rw_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let chip = unsafe { (*entry).private_data as *mut lola };
    unsafe {
        snd_iprintf(
            buffer,
            c"0x%x 0x%x\n".as_ptr(),
            (*chip).debug_res,
            (*chip).debug_res_ex,
        );
    }
}

/*
 * dump some registers
 */
unsafe extern "C" fn lola_proc_regs_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let chip = unsafe { (*entry).private_data as *mut lola };
    let mut i: c_int;

    unsafe {
        i = 0;
        while i < 0x40 {
            snd_iprintf(
                buffer,
                c"BAR0 %02x: %08x\n".as_ptr(),
                i,
                readl((*chip).bar[BAR0 as usize].remap_addr.byte_offset(i as isize)),
            );
            i += 4;
        }
        snd_iprintf(buffer, c"\n".as_ptr());
        i = 0;
        while i < 0x30 {
            snd_iprintf(
                buffer,
                c"BAR1 %02x: %08x\n".as_ptr(),
                i,
                readl((*chip).bar[BAR1 as usize].remap_addr.byte_offset(i as isize)),
            );
            i += 4;
        }
        snd_iprintf(buffer, c"\n".as_ptr());
        i = 0x80;
        while i < 0xa0 {
            snd_iprintf(
                buffer,
                c"BAR1 %02x: %08x\n".as_ptr(),
                i,
                readl((*chip).bar[BAR1 as usize].remap_addr.byte_offset(i as isize)),
            );
            i += 4;
        }
        snd_iprintf(buffer, c"\n".as_ptr());
        i = 0;
        while i < 32 {
            snd_iprintf(
                buffer,
                c"DSD %02x STS  %08x\n".as_ptr(),
                i,
                lola_dsd_read(chip, i, STS),
            );
            snd_iprintf(
                buffer,
                c"DSD %02x LPIB %08x\n".as_ptr(),
                i,
                lola_dsd_read(chip, i, LPIB),
            );
            snd_iprintf(
                buffer,
                c"DSD %02x CTL  %08x\n".as_ptr(),
                i,
                lola_dsd_read(chip, i, CTL),
            );
            snd_iprintf(
                buffer,
                c"DSD %02x LVIL %08x\n".as_ptr(),
                i,
                lola_dsd_read(chip, i, LVI),
            );
            snd_iprintf(
                buffer,
                c"DSD %02x BDPL %08x\n".as_ptr(),
                i,
                lola_dsd_read(chip, i, BDPL),
            );
            snd_iprintf(
                buffer,
                c"DSD %02x BDPU %08x\n".as_ptr(),
                i,
                lola_dsd_read(chip, i, BDPU),
            );
            i += 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn lola_proc_debug_new(chip: *mut lola) {
    unsafe {
        snd_card_ro_proc_new((*chip).card, c"codec".as_ptr(), chip, lola_proc_codec_read);
        snd_card_rw_proc_new(
            (*chip).card,
            c"codec_rw".as_ptr(),
            chip,
            lola_proc_codec_rw_read,
            lola_proc_codec_rw_write,
        );
        snd_card_ro_proc_new((*chip).card, c"regs".as_ptr(), chip, lola_proc_regs_read);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
