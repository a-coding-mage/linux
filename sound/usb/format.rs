// SPDX-License-Identifier: GPL-2.0-or-later
// Depends on: linux/init.h, linux/slab.h, linux/usb.h, linux/usb/audio.h,
// linux/usb/audio-v2.h, linux/usb/audio-v3.h, sound/core.h, sound/pcm.h,
// usbaudio.h, card.h, quirks.h, helper.h, clock.h, format.h

// Constants and types from external dependencies
type SndUsbAudio = std::ffi::c_void;
type Audioformat = std::ffi::c_void;
type SndPcmFormatT = u32;
type UsbDevice = std::ffi::c_void;
type Usb_interface = std::ffi::c_void;
type UsbHostInterface = std::ffi::c_void;

#[repr(C)]
struct UacFormatTypeIDiscreteDescriptor {
    b_length: u8,
    b_descriptor_type: u8,
    b_descriptor_subtype: u8,
    b_format_type: u8,
    b_bit_resolution: u8,
    b_subframe_size: u8,
}

#[repr(C)]
struct UacFormatTypeIExtDescriptor {
    b_length: u8,
    b_descriptor_type: u8,
    b_descriptor_subtype: u8,
    b_format_type: u8,
    b_bit_resolution: u8,
    b_subslot_size: u8,
}

#[repr(C)]
struct Uac3AsHeaderDescriptor {
    b_length: u8,
    b_descriptor_type: u8,
    b_descriptor_subtype: u8,
    b_terminal_link: u8,
    b_controls: u32,
    b_format_type: u8,
    bm_formats: u64,
    b_nr_channels: u8,
    bm_channel_config: u32,
    b_bit_resolution: u8,
    b_subslot_size: u8,
}

#[repr(C)]
struct Uac2AsHeaderDescriptor {
    b_length: u8,
    b_descriptor_type: u8,
    b_descriptor_subtype: u8,
    b_terminal_link: u8,
    bm_controls: u32,
}

#[repr(C)]
struct UacFormatTypeIiDiscreteDescriptor {
    b_length: u8,
    b_descriptor_type: u8,
    b_descriptor_subtype: u8,
    b_format_type: u8,
    w_max_bit_rate: u16,
    w_samples_per_frame: u16,
}

#[repr(C)]
struct UacFormatTypeIiExtDescriptor {
    b_length: u8,
    b_descriptor_type: u8,
    b_descriptor_subtype: u8,
    b_format_type: u8,
    w_max_bit_rate: u16,
    w_samples_per_frame: u16,
}

#[repr(C)]
struct UacFormatTypeIContinuousDescriptor {
    b_length: u8,
    b_descriptor_type: u8,
    b_descriptor_subtype: u8,
    b_format_type: u8,
}

// External function declarations
extern "C" {
    fn usb_audio_info(chip: *const SndUsbAudio, fmt: *const u8, ...);
    fn usb_audio_err(chip: *const SndUsbAudio, fmt: *const u8, ...);
    fn usb_audio_dbg(chip: *const SndUsbAudio, fmt: *const u8, ...);
    fn snd_usb_interface_dsd_format_quirks(chip: *const SndUsbAudio, fp: *const Audioformat, sample_bytes: i32) -> u64;
    fn snd_usb_is_big_endian_format(chip: *const SndUsbAudio, fp: *const Audioformat) -> bool;
    fn kfree(ptr: *mut std::ffi::c_void);
    fn kmalloc(size: usize, flags: i32) -> *mut std::ffi::c_void;
    fn kmalloc_array(n: usize, size: usize, flags: i32) -> *mut std::ffi::c_void;
    fn kcalloc(n: usize, size: usize, flags: i32) -> *mut std::ffi::c_void;
    fn kzalloc(size: usize, flags: i32) -> *mut std::ffi::c_void;
    fn combine_triple(p: *const u8) -> u32;
    fn combine_quad(p: *const u8) -> i32;
    fn le16_to_cpu(x: u16) -> u16;
    fn le32_to_cpu(x: u32) -> u32;
    fn le64_to_cpu(x: u64) -> u64;
    fn snd_pcm_rate_to_rate_bit(rate: u32) -> u64;
    fn pcm_format_to_bits(fmt: SndPcmFormatT) -> u64;
    fn snd_usb_get_host_interface(chip: *const SndUsbAudio, iface: u32, alt: i32) -> *mut UsbHostInterface;
    fn snd_usb_find_csint_desc(extra: *const u8, extralen: i32, last: *const u8, type_: u8) -> *mut u8;
    fn uac_v2v3_control_is_readable(controls: u32, channel: u8) -> bool;
    fn usb_ifnum_to_if(dev: *const UsbDevice, ifnum: u32) -> *mut Usb_interface;
    fn snd_usb_clock_find_source(chip: *const SndUsbAudio, fp: *const Audioformat, validate: bool) -> i32;
    fn snd_usb_find_ctrl_interface(chip: *const SndUsbAudio, iface: u32) -> *mut UsbHostInterface;
    fn snd_usb_ctl_msg(dev: *const UsbDevice, pipe: u32, request: u8, requesttype: u8, value: u16, index: u16, data: *mut std::ffi::c_void, size: usize) -> i32;
    fn usb_rcvctrlpipe(dev: *const UsbDevice, endpoint: u8) -> u32;
    fn usb_set_interface(dev: *const UsbDevice, ifnum: u32, alternate: i32) -> i32;
    fn snd_usb_set_sample_rate_v2v3(chip: *const SndUsbAudio, fp: *const Audioformat, clock: i32, rate: i32) -> i32;
    fn snd_usb_ctrl_intf(alts: *const UsbHostInterface) -> u8;
    fn dev_err(dev: *const std::ffi::c_void, fmt: *const u8, ...);
    fn dev_info(dev: *const std::ffi::c_void, fmt: *const u8, ...);
    fn snd_BUG_ON(cond: bool) -> bool;
}

const GFP_KERNEL: i32 = 0xd0;
const ENOMEM: i32 = -12;
const EINVAL: i32 = -22;
const ENODEV: i32 = -19;
const ENOTSUPP: i32 = -95;

const UAC_VERSION_1: u32 = 0x0100;
const UAC_VERSION_2: u32 = 0x0200;
const UAC_VERSION_3: u32 = 0x0300;

const UAC_FORMAT_TYPE_I_UNDEFINED: u64 = 0;
const UAC_FORMAT_TYPE_I_PCM: u64 = 1;
const UAC_FORMAT_TYPE_I_PCM8: u64 = 2;
const UAC_FORMAT_TYPE_I_IEEE_FLOAT: u64 = 3;
const UAC_FORMAT_TYPE_I_ALAW: u64 = 4;
const UAC_FORMAT_TYPE_I_MULAW: u64 = 5;
const UAC_FORMAT_TYPE_III: u8 = 3;

const UAC2_FORMAT_TYPE_I_RAW_DATA: u64 = 0x80000000;
const UAC3_FORMAT_TYPE_I_RAW_DATA: u64 = 0x1000000;

const UAC_FORMAT_TYPE_II_AC3: u64 = 0;
const UAC_FORMAT_TYPE_II_MPEG: u64 = 1;

const SNDRV_PCM_FMTBIT_SPECIAL: u64 = 1 << 31;
const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S16_BE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << 5;
const SNDRV_PCM_FMTBIT_S24_3BE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
const SNDRV_PCM_FMTBIT_U8: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_FLOAT_LE: u64 = 1 << 14;
const SNDRV_PCM_FMTBIT_A_LAW: u64 = 1 << 11;
const SNDRV_PCM_FMTBIT_MU_LAW: u64 = 1 << 12;
const SNDRV_PCM_FMTBIT_MPEG: u64 = 1 << 23;

const SNDRV_PCM_RATE_CONTINUOUS: u64 = 1 << 30;
const SNDRV_PCM_RATE_48000: u64 = 1 << 7;
const SNDRV_PCM_RATE_96000: u64 = 1 << 8;

const SNDRV_PCM_FORMAT_S16_BE: u32 = 3;
const SNDRV_PCM_FORMAT_S16_LE: u32 = 2;

const UAC_AS_GENERAL: u8 = 1;
const UAC_FORMAT_TYPE: u8 = 2;

const MAX_NR_RATES: i32 = 1024;

const QUIRK_FLAG_VALIDATE_RATES: u64 = 1 << 1;

macro_rules! USB_ID {
    ($vendor:expr, $product:expr) => {
        (($vendor as u32) << 16) | ($product as u32)
    };
}

macro_rules! USB_ID_VENDOR {
    ($id:expr) => {
        (($id as u32) >> 16) & 0xffff
    };
}

macro_rules! BIT {
    ($x:expr) => {
        1u64 << ($x as u64)
    };
}

// Parse the audio format type I descriptor and return the corresponding PCM format
fn parse_audio_format_i_type(
    chip: *const SndUsbAudio,
    fp: *mut Audioformat,
    format: u64,
    _fmt: *mut std::ffi::c_void,
) -> u64 {
    let mut sample_width: i32;
    let mut sample_bytes: i32;
    let mut pcm_formats: u64 = 0;
    let mut dsd_formats: u64 = 0;

    let protocol = unsafe { *(fp as *const u32) };

    match protocol {
        UAC_VERSION_1 => {
            let fmt = _fmt as *const UacFormatTypeIDiscreteDescriptor;
            if format >= 64 {
                unsafe {
                    usb_audio_info(
                        chip,
                        b"%u:%d: invalid format type 0x%llx is detected, processed as PCM\n".as_ptr(),
                    );
                }
                let format = UAC_FORMAT_TYPE_I_PCM;
            }
            sample_width = unsafe { (*fmt).b_bit_resolution as i32 };
            sample_bytes = unsafe { (*fmt).b_subframe_size as i32 };
            let format = 1u64 << format;
        }
        UAC_VERSION_2 => {
            let fmt = _fmt as *const UacFormatTypeIExtDescriptor;
            sample_width = unsafe { (*fmt).b_bit_resolution as i32 };
            sample_bytes = unsafe { (*fmt).b_subslot_size as i32 };

            if format & UAC2_FORMAT_TYPE_I_RAW_DATA != 0 {
                pcm_formats |= SNDRV_PCM_FMTBIT_SPECIAL;
                let dsd_raw = unsafe { &mut *(fp as *mut u8).add(std::mem::size_of::<*mut std::ffi::c_void>() * 10) };
                *dsd_raw = 1;
                let format = format & !UAC2_FORMAT_TYPE_I_RAW_DATA;
            }

            let format = format << 1;
        }
        UAC_VERSION_3 => {
            let as_ = _fmt as *const Uac3AsHeaderDescriptor;

            sample_width = unsafe { (*as_).b_bit_resolution as i32 };
            sample_bytes = unsafe { (*as_).b_subslot_size as i32 };

            if format & UAC3_FORMAT_TYPE_I_RAW_DATA != 0 {
                pcm_formats |= SNDRV_PCM_FMTBIT_SPECIAL;
                let format = format & !UAC3_FORMAT_TYPE_I_RAW_DATA;
            }

            let format = format << 1;
        }
        _ => {
            let fmt = _fmt as *const UacFormatTypeIDiscreteDescriptor;
            if format >= 64 {
                unsafe {
                    usb_audio_info(
                        chip,
                        b"%u:%d: invalid format type 0x%llx is detected, processed as PCM\n".as_ptr(),
                    );
                }
                let format = UAC_FORMAT_TYPE_I_PCM;
            }
            sample_width = unsafe { (*fmt).b_bit_resolution as i32 };
            sample_bytes = unsafe { (*fmt).b_subframe_size as i32 };
            let format = 1u64 << format;
        }
    }

    unsafe {
        let fp_fmt_bits = (fp as *mut u32).add(5);
        *fp_fmt_bits = sample_width as u32;
        let fp_fmt_sz = (fp as *mut i32).add(6);
        *fp_fmt_sz = sample_bytes;
    }

    if (pcm_formats == 0) && (format == 0 || format == BIT!(UAC_FORMAT_TYPE_I_UNDEFINED)) {
        unsafe {
            usb_audio_info(
                chip,
                b"%u:%d : format type 0 is detected, processed as PCM\n".as_ptr(),
            );
        }
        let format = BIT!(UAC_FORMAT_TYPE_I_PCM);
    }

    if format & BIT!(UAC_FORMAT_TYPE_I_PCM) != 0 {
        let usb_id = unsafe { *(chip as *const u32).add(1) };
        if ((usb_id == USB_ID!(0x0582, 0x0016)) || (usb_id == USB_ID!(0x0582, 0x000c)))
            && sample_width == 24
            && sample_bytes == 2
        {
            sample_bytes = 3;
        } else if sample_width > sample_bytes * 8 {
            unsafe {
                usb_audio_info(
                    chip,
                    b"%u:%d : sample bitwidth %d in over sample bytes %d\n".as_ptr(),
                );
            }
        }

        match sample_bytes {
            1 => pcm_formats |= SNDRV_PCM_FMTBIT_S8,
            2 => {
                if unsafe { snd_usb_is_big_endian_format(chip, fp) } {
                    pcm_formats |= SNDRV_PCM_FMTBIT_S16_BE;
                } else {
                    pcm_formats |= SNDRV_PCM_FMTBIT_S16_LE;
                }
            }
            3 => {
                if unsafe { snd_usb_is_big_endian_format(chip, fp) } {
                    pcm_formats |= SNDRV_PCM_FMTBIT_S24_3BE;
                } else {
                    pcm_formats |= SNDRV_PCM_FMTBIT_S24_3LE;
                }
            }
            4 => pcm_formats |= SNDRV_PCM_FMTBIT_S32_LE,
            _ => {
                unsafe {
                    usb_audio_info(
                        chip,
                        b"%u:%d : unsupported sample bitwidth %d in %d bytes\n".as_ptr(),
                    );
                }
            }
        }
    }

    if format & BIT!(UAC_FORMAT_TYPE_I_PCM8) != 0 {
        let usb_id = unsafe { *(chip as *const u32).add(1) };
        if usb_id == USB_ID!(0x04fa, 0x4201) {
            pcm_formats |= SNDRV_PCM_FMTBIT_S8;
        } else {
            pcm_formats |= SNDRV_PCM_FMTBIT_U8;
        }
    }

    if format & BIT!(UAC_FORMAT_TYPE_I_IEEE_FLOAT) != 0 {
        pcm_formats |= SNDRV_PCM_FMTBIT_FLOAT_LE;
    }

    if format & BIT!(UAC_FORMAT_TYPE_I_ALAW) != 0 {
        pcm_formats |= SNDRV_PCM_FMTBIT_A_LAW;
    }

    if format & BIT!(UAC_FORMAT_TYPE_I_MULAW) != 0 {
        pcm_formats |= SNDRV_PCM_FMTBIT_MU_LAW;
    }

    if format & !0x3f != 0 {
        unsafe {
            usb_audio_info(
                chip,
                b"%u:%d : unsupported format bits %#llx\n".as_ptr(),
            );
        }
    }

    dsd_formats |= unsafe { snd_usb_interface_dsd_format_quirks(chip, fp, sample_bytes) };
    let dsd_dop = unsafe { *(fp as *const u8).add(std::mem::size_of::<*mut std::ffi::c_void>() * 11) };
    if dsd_formats != 0 && dsd_dop == 0 {
        pcm_formats = dsd_formats;
    }

    pcm_formats
}

fn set_fixed_rate(fp: *mut Audioformat, rate: i32, rate_bits: u64) -> i32 {
    unsafe {
        let rate_table = (fp as *mut *mut i32).add(4);
        kfree(*rate_table as *mut std::ffi::c_void);
        *rate_table = kmalloc(std::mem::size_of::<i32>(), GFP_KERNEL) as *mut i32;
        if (*rate_table).is_null() {
            return -ENOMEM;
        }
        let nr_rates = (fp as *mut i32).add(5);
        *nr_rates = 1;
        let rate_min = (fp as *mut i32).add(6);
        *rate_min = rate;
        let rate_max = (fp as *mut i32).add(7);
        *rate_max = rate;
        let rates = (fp as *mut u64).add(8);
        *rates = rate_bits;
        (**rate_table.offset(0)) = rate;
    }
    0
}

fn set_rate_table_min_max(fp: *mut Audioformat) {
    let mut rate: u32;
    unsafe {
        let rate_min = (fp as *mut u32).add(6);
        *rate_min = i32::MAX as u32;
        let rate_max = (fp as *mut u32).add(7);
        *rate_max = 0;
        let rates = (fp as *mut u64).add(8);
        *rates = 0;
        let nr_rates = *(fp as *const i32).add(5);
        let rate_table = *(fp as *const *mut u32).add(4);
        for i in 0..nr_rates {
            rate = *rate_table.add(i as usize);
            let rm = &mut *rate_min;
            *rm = (*rm).min(rate);
            let rmax = &mut *rate_max;
            *rmax = (*rmax).max(rate);
            *rates |= snd_pcm_rate_to_rate_bit(rate);
        }
    }
}

fn parse_audio_format_rates_v1(
    chip: *const SndUsbAudio,
    fp: *mut Audioformat,
    fmt: *const u8,
    offset: i32,
) -> i32 {
    let nr_rates = unsafe { *fmt.add(offset as usize) as i32 };

    if unsafe { *fmt as i32 } < offset + 1 + 3 * (if nr_rates != 0 { nr_rates } else { 2 }) {
        unsafe {
            usb_audio_err(
                chip,
                b"%u:%d : invalid UAC_FORMAT_TYPE desc\n".as_ptr(),
            );
        }
        return -EINVAL;
    }

    if nr_rates != 0 {
        unsafe {
            let rate_table = (fp as *mut *mut i32).add(4);
            *rate_table = kmalloc_array(nr_rates as usize, std::mem::size_of::<i32>(), GFP_KERNEL) as *mut i32;
            if (*rate_table).is_null() {
                return -ENOMEM;
            }

            let nr_rates_ptr = (fp as *mut i32).add(5);
            *nr_rates_ptr = 0;
            let mut idx = offset + 1;
            for _r in 0..nr_rates {
                let rate_val = combine_triple(&*fmt.add(idx as usize));
                if rate_val == 0 {
                    idx += 3;
                    continue;
                }

                let mut rate = rate_val;

                if rate == 48000
                    && nr_rates == 1
                    && (*(chip as *const u32).add(1) == USB_ID!(0x0d8c, 0x0201)
                        || *(chip as *const u32).add(1) == USB_ID!(0x0d8c, 0x0102)
                        || *(chip as *const u32).add(1) == USB_ID!(0x0d8c, 0x0078)
                        || *(chip as *const u32).add(1) == USB_ID!(0x0ccd, 0x00b1))
                    && *(fp as *const i32).add(2) == 5
                    && *(fp as *const i32).add(3) == 392
                {
                    rate = 96000;
                }

                if rate == 16000
                    && (*(chip as *const u32).add(1) == USB_ID!(0x041e, 0x4064)
                        || *(chip as *const u32).add(1) == USB_ID!(0x041e, 0x4068))
                {
                    rate = 8000;
                }

                let rt = *rate_table;
                let nr = *nr_rates_ptr;
                *rt.add(nr as usize) = rate as i32;
                *nr_rates_ptr = nr + 1;
                idx += 3;
            }

            if *nr_rates_ptr == 0 {
                usb_audio_info(
                    chip,
                    b"%u:%d: All rates were zero\n".as_ptr(),
                );
                return -EINVAL;
            }
            set_rate_table_min_max(fp);
        }
    } else {
        unsafe {
            let rates = (fp as *mut u64).add(8);
            *rates = SNDRV_PCM_RATE_CONTINUOUS;
            let rate_min = (fp as *mut u32).add(6);
            *rate_min = combine_triple(&*fmt.add((offset + 1) as usize));
            let rate_max = (fp as *mut u32).add(7);
            *rate_max = combine_triple(&*fmt.add((offset + 4) as usize));
        }
    }

    let usb_id = unsafe { *(chip as *const u32).add(1) };
    if usb_id == USB_ID!(0x0b0e, 0x030b) || usb_id == USB_ID!(0x0b0e, 0x030c) {
        let nr_rates = unsafe { *(fp as *const i32).add(5) };
        if nr_rates != 1 {
            return set_fixed_rate(fp, 48000, SNDRV_PCM_RATE_48000);
        }
    }

    0
}

fn s1810c_valid_sample_rate(fp: *const Audioformat, rate: u32) -> bool {
    let altsetting = unsafe { *(fp as *const i32).add(2) };
    match altsetting {
        1 => rate <= 48000,
        2 => rate == 88200 || rate == 96000,
        3 => rate >= 176400,
        _ => false,
    }
}

fn focusrite_rate_pair(rate: u32, max_rate: u32) -> bool {
    match max_rate {
        48000 => rate == 44100 || rate == 48000,
        96000 => rate == 88200 || rate == 96000,
        192000 => rate == 176400 || rate == 192000,
        _ => true,
    }
}

fn focusrite_valid_sample_rate(
    chip: *const SndUsbAudio,
    fp: *const Audioformat,
    rate: u32,
) -> bool {
    let iface = unsafe { *(fp as *const i32) };
    let altsetting = unsafe { *(fp as *const i32).add(2) };

    let alts = unsafe { snd_usb_get_host_interface(chip, iface as u32, altsetting) };
    if alts.is_null() {
        return true;
    }

    let extra = unsafe { *(alts as *const *const u8) };
    let extralen = unsafe { *(alts as *const i32).add(1) };

    let fmt = unsafe { snd_usb_find_csint_desc(extra, extralen, std::ptr::null(), UAC_FORMAT_TYPE) };
    if fmt.is_null() {
        return true;
    }

    let as_ = unsafe { snd_usb_find_csint_desc(extra, extralen, std::ptr::null(), UAC_AS_GENERAL) };
    if as_.is_null() {
        return true;
    }

    let val_alt = unsafe { uac_v2v3_control_is_readable(*(as_ as *const u32), 0) };

    if unsafe { *fmt as i32 } == 10 {
        let max_rate = unsafe { combine_quad(&*fmt.add(6)) as u32 };

        if val_alt {
            return focusrite_rate_pair(rate, max_rate);
        }

        match max_rate {
            192000 => {
                if rate == 176400 || rate == 192000 {
                    return true;
                }
            }
            96000 => {
                if rate == 88200 || rate == 96000 {
                    return true;
                }
            }
            48000 => {
                return rate == 44100 || rate == 48000;
            }
            _ => {
                unsafe {
                    usb_audio_info(
                        chip,
                        b"%u:%d : unexpected max rate: %u\n".as_ptr(),
                    );
                }
                return true;
            }
        }
        return false;
    }

    if !val_alt {
        return true;
    }

    let dev = unsafe { *(chip as *const *const UsbDevice) };
    let iface_ptr = unsafe { usb_ifnum_to_if(dev, iface as u32) };
    if iface_ptr.is_null() {
        return true;
    }

    let num_altsetting = unsafe { *(iface_ptr as *const i32).add(1) };
    if num_altsetting <= 2 {
        return true;
    }

    let max_rate = match altsetting {
        1 => 48000,
        2 => 96000,
        3 => 192000,
        _ => return true,
    };

    focusrite_rate_pair(rate, max_rate)
}

fn parse_uac2_sample_rate_range(
    chip: *const SndUsbAudio,
    fp: *mut Audioformat,
    nr_triplets: i32,
    data: *const u8,
) -> i32 {
    let mut nr_rates = 0;

    for i in 0..nr_triplets {
        let offset = (2 + 12 * i) as usize;
        let min = unsafe { combine_quad(data.add(offset)) };
        let max = unsafe { combine_quad(data.add(offset + 4)) };
        let res = unsafe { combine_quad(data.add(offset + 8)) };

        if max < 0 || min < 0 || res < 0 || max < min {
            continue;
        }

        if res == 1 {
            unsafe {
                let rate_min = (fp as *mut i32).add(6);
                *rate_min = min;
                let rate_max = (fp as *mut i32).add(7);
                *rate_max = max;
                let rates = (fp as *mut u64).add(8);
                *rates = SNDRV_PCM_RATE_CONTINUOUS;
            }
            return 0;
        }

        let mut rate = min as u32;
        while rate <= max as u32 {
            let usb_id = unsafe { *(chip as *const u32).add(1) };
            if usb_id == USB_ID!(0x194f, 0x010c)
                && !s1810c_valid_sample_rate(fp, rate)
            {
                rate += res as u32;
                continue;
            }
            if usb_id == USB_ID!(0x194f, 0x010d)
                && !s1810c_valid_sample_rate(fp, rate)
            {
                rate += res as u32;
                continue;
            }
            if usb_id == USB_ID!(0x194f, 0x0107)
                && !s1810c_valid_sample_rate(fp, rate)
            {
                rate += res as u32;
                continue;
            }

            if USB_ID_VENDOR!(usb_id) == 0x1235 && !focusrite_valid_sample_rate(chip, fp, rate) {
                rate += res as u32;
                continue;
            }

            let rate_table = unsafe { *(fp as *const *mut i32).add(4) };
            if !rate_table.is_null() {
                unsafe { *rate_table.add(nr_rates as usize) = rate as i32 };
            }
            nr_rates += 1;
            if nr_rates >= MAX_NR_RATES {
                unsafe {
                    usb_audio_err(chip, b"invalid uac2 rates\n".as_ptr());
                }
                return nr_rates;
            }

            if res == 0 {
                break;
            }
            rate += res as u32;
        }
    }

    nr_rates
}

fn line6_parse_audio_format_rates_quirk(chip: *const SndUsbAudio, fp: *mut Audioformat) -> i32 {
    let usb_id = unsafe { *(chip as *const u32).add(1) };
    match usb_id {
        USB_ID!(0x0e41, 0x4241)
        | USB_ID!(0x0e41, 0x4242)
        | USB_ID!(0x0e41, 0x4244)
        | USB_ID!(0x0e41, 0x4246)
        | USB_ID!(0x0e41, 0x4253)
        | USB_ID!(0x0e41, 0x4247)
        | USB_ID!(0x0e41, 0x4248)
        | USB_ID!(0x0e41, 0x4249)
        | USB_ID!(0x0e41, 0x424a)
        | USB_ID!(0x0e41, 0x424b)
        | USB_ID!(0x19f7, 0x0011) => set_fixed_rate(fp, 48000, SNDRV_PCM_RATE_48000),
        _ => -ENODEV,
    }
}

fn check_valid_altsetting_v2v3(chip: *const SndUsbAudio, iface: i32, altsetting: i32) -> bool {
    let dev = unsafe { *(chip as *const *const UsbDevice) };
    let mut raw_data: u64 = 0;

    if unsafe { snd_BUG_ON((altsetting >= 64 - 8) as bool) } {
        return false;
    }

    let err = unsafe {
        snd_usb_ctl_msg(
            dev,
            usb_rcvctrlpipe(dev, 0),
            0x82, // UAC2_CS_CUR
            0xc0, // USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_IN
            0x0102u16, // UAC2_AS_VAL_ALT_SETTINGS << 8
            iface as u16,
            &mut raw_data as *mut u64 as *mut std::ffi::c_void,
            std::mem::size_of::<u64>(),
        )
    };
    if err < 0 {
        return false;
    }

    let data = unsafe { le64_to_cpu(raw_data) };
    if ((data & 0xff) as i32) * 8 < altsetting {
        return false;
    }
    if (data & (1u64 << (altsetting + 8))) != 0 {
        return true;
    }

    false
}

fn validate_sample_rate_table_v2v3(
    chip: *const SndUsbAudio,
    fp: *mut Audioformat,
    clock: i32,
) -> i32 {
    let dev = unsafe { *(chip as *const *const UsbDevice) };
    let iface = unsafe { *(fp as *const i32) };
    let altsetting = unsafe { *(fp as *const i32).add(2) };

    let quirk_flags = unsafe { *(chip as *const u64).add(10) };
    if (quirk_flags & QUIRK_FLAG_VALIDATE_RATES) == 0 {
        return 0;
    }

    let alts = unsafe { snd_usb_get_host_interface(chip, iface as u32, altsetting) };
    if alts.is_null() {
        return 0;
    }

    let protocol = unsafe { *(fp as *const u32) };
    let bm_controls = if protocol == UAC_VERSION_3 {
        let as_ = unsafe { snd_usb_find_csint_desc(
            *(alts as *const *const u8),
            *(alts as *const i32).add(1),
            std::ptr::null(),
            UAC_AS_GENERAL,
        ) as *const Uac3AsHeaderDescriptor };
        if as_.is_null() {
            return 0;
        }
        unsafe { le32_to_cpu((*as_).b_controls) }
    } else {
        let as_ = unsafe { snd_usb_find_csint_desc(
            *(alts as *const *const u8),
            *(alts as *const i32).add(1),
            std::ptr::null(),
            UAC_AS_GENERAL,
        ) as *const Uac2AsHeaderDescriptor };
        if as_.is_null() {
            return 0;
        }
        unsafe { (*as_).bm_controls }
    };

    if !unsafe { uac_v2v3_control_is_readable(bm_controls, 0) } {
        return 0;
    }

    let nr_rates_orig = unsafe { *(fp as *const i32).add(5) };
    let table = unsafe { kcalloc(nr_rates_orig as usize, std::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32 };
    if table.is_null() {
        return -ENOMEM;
    }

    unsafe { usb_set_interface(dev, iface as u32, 0) };

    let mut nr_rates = 0;
    let rate_table = unsafe { *(fp as *const *const i32).add(4) };
    for i in 0..nr_rates_orig {
        let err = unsafe { snd_usb_set_sample_rate_v2v3(chip, fp, clock, *rate_table.add(i as usize)) };
        if err < 0 {
            continue;
        }

        if check_valid_altsetting_v2v3(chip, iface, altsetting) {
            unsafe { *table.add(nr_rates as usize) = *rate_table.add(i as usize) as u32 };
            nr_rates += 1;
        }
    }

    if nr_rates == 0 {
        unsafe {
            usb_audio_dbg(
                chip,
                b"No valid sample rate available for %d:%d, assuming a firmware bug\n".as_ptr(),
            );
        }
        nr_rates = nr_rates_orig;
    }

    if nr_rates_orig == nr_rates {
        unsafe { kfree(table as *mut std::ffi::c_void) };
        return 0;
    }

    unsafe {
        kfree(rate_table as *mut std::ffi::c_void);
        let fp_rate_table = (fp as *mut *mut i32).add(4);
        *fp_rate_table = table as *mut i32;
        let fp_nr_rates = (fp as *mut i32).add(5);
        *fp_nr_rates = nr_rates;
    }
    0
}

fn parse_audio_format_rates_v2v3(chip: *const SndUsbAudio, fp: *mut Audioformat) -> i32 {
    let dev = unsafe { *(chip as *const *const UsbDevice) };
    let mut tmp: [u8; 2] = [0; 2];
    let iface = unsafe { *(fp as *const i32) };

    let ctrl_intf = unsafe { snd_usb_find_ctrl_interface(chip, iface as u32) };
    let clock = unsafe { snd_usb_clock_find_source(chip, fp, false) };

    if clock < 0 {
        unsafe {
            dev_err(
                &(*dev) as *const _ as *const std::ffi::c_void,
                b"%s(): unable to find clock source (clock %d)\n".as_ptr(),
            );
        }
        return -1;
    }

    let mut ret = unsafe {
        snd_usb_ctl_msg(
            dev,
            usb_rcvctrlpipe(dev, 0),
            0x82, // UAC2_CS_RANGE
            0xc0, // USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_IN
            0x0100u16, // UAC2_CS_CONTROL_SAM_FREQ << 8
            unsafe { snd_usb_ctrl_intf(ctrl_intf) as u16 } | ((clock as u16) << 8),
            tmp.as_mut_ptr() as *mut std::ffi::c_void,
            std::mem::size_of::<[u8; 2]>(),
        )
    };

    if ret < 0 {
        let ret_l6 = line6_parse_audio_format_rates_quirk(chip, fp);
        if ret_l6 == -ENODEV {
            unsafe {
                dev_err(
                    &(*dev) as *const _ as *const std::ffi::c_void,
                    b"%s(): unable to retrieve number of sample rates (clock %d)\n".as_ptr(),
                );
            }
            return -1;
        }
        if ret_l6 == 0 {
            unsafe {
                dev_info(
                    &(*dev) as *const _ as *const std::ffi::c_void,
                    b"%s(): unable to retrieve number of sample rates: set it to a predefined value (clock %d).\n".as_ptr(),
                );
            }
            return 0;
        }
        ret = ret_l6;
        return ret;
    }

    let nr_triplets = ((tmp[1] as i32) << 8) | (tmp[0] as i32);
    let data_size = (2 + 12 * nr_triplets) as usize;
    let data = unsafe { kzalloc(data_size, GFP_KERNEL) as *mut u8 };
    if data.is_null() {
        return -ENOMEM;
    }

    ret = unsafe {
        snd_usb_ctl_msg(
            dev,
            usb_rcvctrlpipe(dev, 0),
            0x82, // UAC2_CS_RANGE
            0xc0, // USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_IN
            0x0100u16, // UAC2_CS_CONTROL_SAM_FREQ << 8
            unsafe { snd_usb_ctrl_intf(ctrl_intf) as u16 } | ((clock as u16) << 8),
            data as *mut std::ffi::c_void,
            data_size,
        )
    };

    if ret < 0 {
        unsafe {
            dev_err(
                &(*dev) as *const _ as *const std::ffi::c_void,
                b"%s(): unable to retrieve sample rate range (clock %d)\n".as_ptr(),
            );
        }
        unsafe { kfree(data as *mut std::ffi::c_void) };
        return -EINVAL;
    }

    unsafe {
        kfree((*(fp as *const *mut i32).add(4)) as *mut std::ffi::c_void);
        let fp_rate_table = (fp as *mut *mut i32).add(4);
        *fp_rate_table = std::ptr::null_mut();
        let fp_nr_rates = (fp as *mut i32).add(5);
        *fp_nr_rates = parse_uac2_sample_rate_range(chip, fp, nr_triplets, data);
    }

    let nr_rates = unsafe { *(fp as *const i32).add(5) };
    if nr_rates == 0 {
        unsafe { kfree(data as *mut std::ffi::c_void) };
        return 0;
    }

    unsafe {
        let fp_rate_table = (fp as *mut *mut i32).add(4);
        *fp_rate_table = kmalloc_array(nr_rates as usize, std::mem::size_of::<i32>(), GFP_KERNEL) as *mut i32;
        if (*fp_rate_table).is_null() {
            kfree(data as *mut std::ffi::c_void);
            return -ENOMEM;
        }
    }

    parse_uac2_sample_rate_range(chip, fp, nr_triplets, data);

    ret = validate_sample_rate_table_v2v3(chip, fp, clock);
    if ret < 0 {
        unsafe { kfree(data as *mut std::ffi::c_void) };
        return ret;
    }

    set_rate_table_min_max(fp);

    unsafe { kfree(data as *mut std::ffi::c_void) };
    0
}

fn parse_audio_format_i(
    chip: *const SndUsbAudio,
    fp: *mut Audioformat,
    format: u64,
    _fmt: *mut std::ffi::c_void,
) -> i32 {
    let protocol = unsafe { *(fp as *const u32) };
    let fmt_type = match protocol {
        UAC_VERSION_3 => unsafe { *(fp as *const u8).add(std::mem::size_of::<*mut std::ffi::c_void>() * 12) },
        _ => unsafe { *((_fmt as *const UacFormatTypeIContinuousDescriptor)).b_format_type },
    };

    if fmt_type == UAC_FORMAT_TYPE_III {
        let usb_id = unsafe { *(chip as *const u32).add(1) };
        let pcm_format = match usb_id {
            USB_ID!(0x0763, 0x2003) => {
                let setup = unsafe { *(chip as *const u32).add(5) };
                let altsetting = unsafe { *(fp as *const i32).add(2) };
                if setup == 0 && altsetting == 6 {
                    SNDRV_PCM_FORMAT_S16_BE
                } else {
                    SNDRV_PCM_FORMAT_S16_LE
                }
            }
            _ => SNDRV_PCM_FORMAT_S16_LE,
        };
        unsafe {
            let fp_formats = (fp as *mut u64).add(0);
            *fp_formats = pcm_format_to_bits(pcm_format);
        }
    } else {
        let formats = parse_audio_format_i_type(chip, fp, format, _fmt);
        if formats == 0 {
            return -EINVAL;
        }
        unsafe {
            let fp_formats = (fp as *mut u64).add(0);
            *fp_formats = formats;
        }
    }

    let ret = match protocol {
        UAC_VERSION_1 => {
            let fmt = _fmt as *const UacFormatTypeIContinuousDescriptor;
            unsafe {
                let fp_channels = (fp as *mut u8).add(8);
                *fp_channels = 0; // bNrChannels would be set here
                parse_audio_format_rates_v1(chip, fp, _fmt as *const u8, 7)
            }
        }
        UAC_VERSION_2 | UAC_VERSION_3 => parse_audio_format_rates_v2v3(chip, fp),
        _ => {
            let fmt = _fmt as *const UacFormatTypeIContinuousDescriptor;
            unsafe {
                let fp_channels = (fp as *mut u8).add(8);
                *fp_channels = 0; // bNrChannels would be set here
                parse_audio_format_rates_v1(chip, fp, _fmt as *const u8, 7)
            }
        }
    };

    let channels = unsafe { *(fp as *const u8).add(8) };
    if (channels as i32) < 1 {
        unsafe {
            usb_audio_err(
                chip,
                b"%u:%d : invalid channels %d\n".as_ptr(),
            );
        }
        return -EINVAL;
    }

    ret
}

fn parse_audio_format_ii(
    chip: *const SndUsbAudio,
    fp: *mut Audioformat,
    format: u64,
    _fmt: *mut std::ffi::c_void,
) -> i32 {
    match format {
        UAC_FORMAT_TYPE_II_AC3 => {
            unsafe {
                let fp_formats = (fp as *mut u64).add(0);
                *fp_formats = SNDRV_PCM_FMTBIT_U8;
            }
        }
        UAC_FORMAT_TYPE_II_MPEG => {
            unsafe {
                let fp_formats = (fp as *mut u64).add(0);
                *fp_formats = SNDRV_PCM_FMTBIT_MPEG;
            }
        }
        _ => {
            unsafe {
                usb_audio_info(
                    chip,
                    b"%u:%d : unknown format tag %#llx is detected.  processed as MPEG.\n".as_ptr(),
                );
                let fp_formats = (fp as *mut u64).add(0);
                *fp_formats = SNDRV_PCM_FMTBIT_MPEG;
            }
        }
    }

    unsafe {
        let fp_channels = (fp as *mut u8).add(8);
        *fp_channels = 1;
    }

    let protocol = unsafe { *(fp as *const u32) };
    match protocol {
        UAC_VERSION_1 => {
            let fmt = _fmt as *const UacFormatTypeIiDiscreteDescriptor;
            let brate = unsafe { le16_to_cpu((*fmt).w_max_bit_rate) };
            let framesize = unsafe { le16_to_cpu((*fmt).w_samples_per_frame) };
            unsafe {
                usb_audio_info(
                    chip,
                    b"found format II with max.bitrate = %d, frame size=%d\n".as_ptr(),
                );
                let fp_frame_size = (fp as *mut u16).add(9);
                *fp_frame_size = framesize;
            }
            parse_audio_format_rates_v1(chip, fp, _fmt as *const u8, 8)
        }
        UAC_VERSION_2 => {
            let fmt = _fmt as *const UacFormatTypeIiExtDescriptor;
            let brate = unsafe { le16_to_cpu((*fmt).w_max_bit_rate) };
            let framesize = unsafe { le16_to_cpu((*fmt).w_samples_per_frame) };
            unsafe {
                usb_audio_info(
                    chip,
                    b"found format II with max.bitrate = %d, frame size=%d\n".as_ptr(),
                );
                let fp_frame_size = (fp as *mut u16).add(9);
                *fp_frame_size = framesize;
            }
            parse_audio_format_rates_v2v3(chip, fp)
        }
        _ => {
            let fmt = _fmt as *const UacFormatTypeIiDiscreteDescriptor;
            let brate = unsafe { le16_to_cpu((*fmt).w_max_bit_rate) };
            let framesize = unsafe { le16_to_cpu((*fmt).w_samples_per_frame) };
            unsafe {
                usb_audio_info(
                    chip,
                    b"found format II with max.bitrate = %d, frame size=%d\n".as_ptr(),
                );
                let fp_frame_size = (fp as *mut u16).add(9);
                *fp_frame_size = framesize;
            }
            parse_audio_format_rates_v1(chip, fp, _fmt as *const u8, 8)
        }
    }
}

#[no_mangle]
pub extern "C" fn snd_usb_parse_audio_format(
    chip: *const SndUsbAudio,
    fp: *mut Audioformat,
    format: u64,
    fmt: *const UacFormatTypeIContinuousDescriptor,
    stream: i32,
) -> i32 {
    let fmt_type = unsafe { (*fmt).b_format_type };

    let err = match fmt_type {
        UAC_FORMAT_TYPE_III => parse_audio_format_i(chip, fp, format, fmt as *mut std::ffi::c_void),
        2 => {
            // UAC_FORMAT_TYPE_II
            parse_audio_format_ii(chip, fp, format, fmt as *mut std::ffi::c_void)
        }
        _ => {
            unsafe {
                usb_audio_info(
                    chip,
                    b"%u:%d : format type %d is not supported yet\n".as_ptr(),
                );
            }
            return -ENOTSUPP;
        }
    };

    unsafe {
        let fp_fmt_type = (fp as *mut u8).add(std::mem::size_of::<*mut std::ffi::c_void>() * 12);
        *fp_fmt_type = fmt_type;
    }

    if err < 0 {
        return err;
    }

    let usb_id = unsafe { *(chip as *const u32).add(1) };
    if (usb_id == USB_ID!(0x041e, 0x3000)
        || usb_id == USB_ID!(0x041e, 0x3020)
        || usb_id == USB_ID!(0x041e, 0x3061))
        && fmt_type == 1 // UAC_FORMAT_TYPE_I
    {
        let rates = unsafe { *(fp as *const u64).add(8) };
        if rates != SNDRV_PCM_RATE_48000 && rates != SNDRV_PCM_RATE_96000 {
            return -ENOTSUPP;
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn snd_usb_parse_audio_format_v3(
    chip: *const SndUsbAudio,
    fp: *mut Audioformat,
    as_: *const Uac3AsHeaderDescriptor,
    stream: i32,
) -> i32 {
    let format = unsafe { le64_to_cpu((*as_).bm_formats) };

    if format & 0x7f != 0 {
        unsafe {
            let fp_fmt_type = (fp as *mut u8).add(std::mem::size_of::<*mut std::ffi::c_void>() * 12);
            *fp_fmt_type = 1; // UAC_FORMAT_TYPE_I
        }
    } else {
        unsafe {
            let fp_fmt_type = (fp as *mut u8).add(std::mem::size_of::<*mut std::ffi::c_void>() * 12);
            *fp_fmt_type = 3; // UAC_FORMAT_TYPE_III
        }
    }

    let err = parse_audio_format_i(chip, fp, format, as_ as *mut std::ffi::c_void);
    if err < 0 {
        return err;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
