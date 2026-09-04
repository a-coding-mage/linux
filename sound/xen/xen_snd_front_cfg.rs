// SPDX-License-Identifier: GPL-2.0 OR MIT

/*
 * Xen para-virtual sound device
 *
 * Copyright (C) 2016-2018 EPAM Systems Inc.
 *
 * Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>
 */

// Requires: xen/xenbus.h, xen/interface/io/sndif.h
// Requires: xen_snd_front.h, xen_snd_front_cfg.h

const VSND_MAX_STREAM: usize = 8;

#[repr(C)]
struct CfgHwSampleRate {
    name: *const i8,
    mask: u32,
    value: u32,
}

static CFG_HW_SUPPORTED_RATES: &[CfgHwSampleRate] = &[
    CfgHwSampleRate { name: b"5512\0" as *const i8, mask: 0x1, value: 5512 },     // SNDRV_PCM_RATE_5512
    CfgHwSampleRate { name: b"8000\0" as *const i8, mask: 0x2, value: 8000 },     // SNDRV_PCM_RATE_8000
    CfgHwSampleRate { name: b"11025\0" as *const i8, mask: 0x4, value: 11025 },   // SNDRV_PCM_RATE_11025
    CfgHwSampleRate { name: b"16000\0" as *const i8, mask: 0x8, value: 16000 },   // SNDRV_PCM_RATE_16000
    CfgHwSampleRate { name: b"22050\0" as *const i8, mask: 0x10, value: 22050 },  // SNDRV_PCM_RATE_22050
    CfgHwSampleRate { name: b"32000\0" as *const i8, mask: 0x20, value: 32000 },  // SNDRV_PCM_RATE_32000
    CfgHwSampleRate { name: b"44100\0" as *const i8, mask: 0x40, value: 44100 },  // SNDRV_PCM_RATE_44100
    CfgHwSampleRate { name: b"48000\0" as *const i8, mask: 0x80, value: 48000 },  // SNDRV_PCM_RATE_48000
    CfgHwSampleRate { name: b"64000\0" as *const i8, mask: 0x100, value: 64000 },    // SNDRV_PCM_RATE_64000
    CfgHwSampleRate { name: b"96000\0" as *const i8, mask: 0x200, value: 96000 },    // SNDRV_PCM_RATE_96000
    CfgHwSampleRate { name: b"176400\0" as *const i8, mask: 0x400, value: 176400 },  // SNDRV_PCM_RATE_176400
    CfgHwSampleRate { name: b"192000\0" as *const i8, mask: 0x800, value: 192000 },  // SNDRV_PCM_RATE_192000
];

#[repr(C)]
struct CfgHwSampleFormat {
    name: *const i8,
    mask: u64,
}

static CFG_HW_SUPPORTED_FORMATS: &[CfgHwSampleFormat] = &[
    CfgHwSampleFormat { name: b"U8\0" as *const i8, mask: 0x1 },                        // XENSND_PCM_FORMAT_U8_STR / SNDRV_PCM_FMTBIT_U8
    CfgHwSampleFormat { name: b"S8\0" as *const i8, mask: 0x2 },                        // XENSND_PCM_FORMAT_S8_STR / SNDRV_PCM_FMTBIT_S8
    CfgHwSampleFormat { name: b"U16_LE\0" as *const i8, mask: 0x4 },                    // XENSND_PCM_FORMAT_U16_LE_STR / SNDRV_PCM_FMTBIT_U16_LE
    CfgHwSampleFormat { name: b"U16_BE\0" as *const i8, mask: 0x8 },                    // XENSND_PCM_FORMAT_U16_BE_STR / SNDRV_PCM_FMTBIT_U16_BE
    CfgHwSampleFormat { name: b"S16_LE\0" as *const i8, mask: 0x10 },                   // XENSND_PCM_FORMAT_S16_LE_STR / SNDRV_PCM_FMTBIT_S16_LE
    CfgHwSampleFormat { name: b"S16_BE\0" as *const i8, mask: 0x20 },                   // XENSND_PCM_FORMAT_S16_BE_STR / SNDRV_PCM_FMTBIT_S16_BE
    CfgHwSampleFormat { name: b"U24_LE\0" as *const i8, mask: 0x40 },                   // XENSND_PCM_FORMAT_U24_LE_STR / SNDRV_PCM_FMTBIT_U24_LE
    CfgHwSampleFormat { name: b"U24_BE\0" as *const i8, mask: 0x80 },                   // XENSND_PCM_FORMAT_U24_BE_STR / SNDRV_PCM_FMTBIT_U24_BE
    CfgHwSampleFormat { name: b"S24_LE\0" as *const i8, mask: 0x100 },                  // XENSND_PCM_FORMAT_S24_LE_STR / SNDRV_PCM_FMTBIT_S24_LE
    CfgHwSampleFormat { name: b"S24_BE\0" as *const i8, mask: 0x200 },                  // XENSND_PCM_FORMAT_S24_BE_STR / SNDRV_PCM_FMTBIT_S24_BE
    CfgHwSampleFormat { name: b"U32_LE\0" as *const i8, mask: 0x400 },                  // XENSND_PCM_FORMAT_U32_LE_STR / SNDRV_PCM_FMTBIT_U32_LE
    CfgHwSampleFormat { name: b"U32_BE\0" as *const i8, mask: 0x800 },                  // XENSND_PCM_FORMAT_U32_BE_STR / SNDRV_PCM_FMTBIT_U32_BE
    CfgHwSampleFormat { name: b"S32_LE\0" as *const i8, mask: 0x1000 },                 // XENSND_PCM_FORMAT_S32_LE_STR / SNDRV_PCM_FMTBIT_S32_LE
    CfgHwSampleFormat { name: b"S32_BE\0" as *const i8, mask: 0x2000 },                 // XENSND_PCM_FORMAT_S32_BE_STR / SNDRV_PCM_FMTBIT_S32_BE
    CfgHwSampleFormat { name: b"A_LAW\0" as *const i8, mask: 0x4000 },                  // XENSND_PCM_FORMAT_A_LAW_STR / SNDRV_PCM_FMTBIT_A_LAW
    CfgHwSampleFormat { name: b"MU_LAW\0" as *const i8, mask: 0x8000 },                 // XENSND_PCM_FORMAT_MU_LAW_STR / SNDRV_PCM_FMTBIT_MU_LAW
    CfgHwSampleFormat { name: b"F32_LE\0" as *const i8, mask: 0x10000 },                // XENSND_PCM_FORMAT_F32_LE_STR / SNDRV_PCM_FMTBIT_FLOAT_LE
    CfgHwSampleFormat { name: b"F32_BE\0" as *const i8, mask: 0x20000 },                // XENSND_PCM_FORMAT_F32_BE_STR / SNDRV_PCM_FMTBIT_FLOAT_BE
    CfgHwSampleFormat { name: b"F64_LE\0" as *const i8, mask: 0x40000 },                // XENSND_PCM_FORMAT_F64_LE_STR / SNDRV_PCM_FMTBIT_FLOAT64_LE
    CfgHwSampleFormat { name: b"F64_BE\0" as *const i8, mask: 0x80000 },                // XENSND_PCM_FORMAT_F64_BE_STR / SNDRV_PCM_FMTBIT_FLOAT64_BE
    CfgHwSampleFormat { name: b"IEC958_SUBFRAME_LE\0" as *const i8, mask: 0x100000 },   // XENSND_PCM_FORMAT_IEC958_SUBFRAME_LE_STR / SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE
    CfgHwSampleFormat { name: b"IEC958_SUBFRAME_BE\0" as *const i8, mask: 0x200000 },   // XENSND_PCM_FORMAT_IEC958_SUBFRAME_BE_STR / SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_BE
    CfgHwSampleFormat { name: b"IMA_ADPCM\0" as *const i8, mask: 0x400000 },            // XENSND_PCM_FORMAT_IMA_ADPCM_STR / SNDRV_PCM_FMTBIT_IMA_ADPCM
    CfgHwSampleFormat { name: b"MPEG\0" as *const i8, mask: 0x800000 },                 // XENSND_PCM_FORMAT_MPEG_STR / SNDRV_PCM_FMTBIT_MPEG
    CfgHwSampleFormat { name: b"GSM\0" as *const i8, mask: 0x1000000 },                 // XENSND_PCM_FORMAT_GSM_STR / SNDRV_PCM_FMTBIT_GSM
];

const MAX_BUFFER_SIZE: usize = 64 * 1024;
const MIN_PERIOD_SIZE: usize = 64;
const MAX_PERIOD_SIZE: usize = MAX_BUFFER_SIZE;
const USE_FORMATS: u64 = 0x3;          // SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE
const USE_RATE: u32 = 0x60000;         // SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000
const USE_RATE_MIN: u32 = 5512;
const USE_RATE_MAX: u32 = 48000;
const USE_CHANNELS_MIN: u32 = 1;
const USE_CHANNELS_MAX: u32 = 2;
const USE_PERIODS_MIN: u32 = 2;
const USE_PERIODS_MAX: usize = MAX_BUFFER_SIZE / MIN_PERIOD_SIZE;

#[repr(C)]
struct SndDrvPcmHwDefault {
    info: u32,
    formats: u64,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: usize,
    period_bytes_min: usize,
    period_bytes_max: usize,
    periods_min: u32,
    periods_max: usize,
    fifo_size: u32,
}

static SND_DRV_PCM_HW_DEFAULT: SndDrvPcmHwDefault = SndDrvPcmHwDefault {
    info: 0x1 | 0x2 | 0x8 | 0x10,  // SNDRV_PCM_INFO_MMAP | INTERLEAVED | RESUME | MMAP_VALID
    formats: USE_FORMATS,
    rates: USE_RATE,
    rate_min: USE_RATE_MIN,
    rate_max: USE_RATE_MAX,
    channels_min: USE_CHANNELS_MIN,
    channels_max: USE_CHANNELS_MAX,
    buffer_bytes_max: MAX_BUFFER_SIZE,
    period_bytes_min: MIN_PERIOD_SIZE,
    period_bytes_max: MAX_PERIOD_SIZE,
    periods_min: USE_PERIODS_MIN,
    periods_max: USE_PERIODS_MAX,
    fifo_size: 0,
};

// External FFI declarations
extern "C" {
    fn xenbus_read_unsigned(path: *const i8, key: *const i8, default: u32) -> u32;
    fn xenbus_read(
        xbt: u64,
        path: *const i8,
        key: *const i8,
        len: *mut u32,
    ) -> *mut i8;
    fn xenbus_exists(xbt: u64, path: *const i8, key: *const i8) -> i32;
    fn kfree(ptr: *mut u8);
    fn devm_kcalloc(dev: *mut u8, n: usize, size: usize, flags: u32) -> *mut u8;
    fn devm_kasprintf(dev: *mut u8, flags: u32, fmt: *const i8, ...) -> *mut i8;
    fn kasprintf(flags: u32, fmt: *const i8, ...) -> *mut i8;
    fn strscpy(dest: *mut i8, src: *const i8, size: usize) -> isize;
    fn snprintf(str: *mut i8, size: usize, fmt: *const i8, ...) -> i32;
    fn scnprintf(str: *mut i8, size: usize, fmt: *const i8, ...) -> i32;
    fn strncasecmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
    fn strsep(stringp: *mut *mut i8, delim: *const i8) -> *mut i8;
}

// Helper macros for IS_ERR and PTR_ERR semantics
#[inline]
fn is_err(ptr: *const u8) -> bool {
    (ptr as usize) > -4096isize as usize
}

#[inline]
fn ptr_err(ptr: *const u8) -> i32 {
    (-(ptr as i32))
}

fn cfg_hw_rates(
    list: *mut i8,
    _len: u32,
    _path: *const i8,
    pcm_hw: *mut SndDrvPcmHwDefault,
) {
    let mut cur_rate: *mut i8;
    let mut cur_mask: u32;
    let mut cur_value: u32;
    let mut rates: u32 = 0;
    let mut rate_min: u32 = u32::MAX;
    let mut rate_max: u32 = 0;
    let mut i: usize;

    const XENSND_LIST_SEPARATOR: *const i8 = b",\0" as *const i8;
    const XENSND_SAMPLE_RATE_MAX_LEN: usize = 5;

    loop {
        cur_rate = unsafe { strsep(&mut (list as *mut *mut i8), XENSND_LIST_SEPARATOR) };
        if cur_rate.is_null() {
            break;
        }

        i = 0;
        while i < CFG_HW_SUPPORTED_RATES.len() {
            if unsafe {
                strncasecmp(
                    cur_rate,
                    CFG_HW_SUPPORTED_RATES[i].name,
                    XENSND_SAMPLE_RATE_MAX_LEN,
                )
            } == 0
            {
                cur_mask = CFG_HW_SUPPORTED_RATES[i].mask;
                cur_value = CFG_HW_SUPPORTED_RATES[i].value;
                rates |= cur_mask;
                if rate_min > cur_value {
                    rate_min = cur_value;
                }
                if rate_max < cur_value {
                    rate_max = cur_value;
                }
            }
            i += 1;
        }
    }

    if rates != 0 {
        unsafe {
            (*pcm_hw).rates = rates;
            (*pcm_hw).rate_min = rate_min;
            (*pcm_hw).rate_max = rate_max;
        }
    }
}

fn cfg_formats(
    list: *mut i8,
    _len: u32,
    _path: *const i8,
    pcm_hw: *mut SndDrvPcmHwDefault,
) {
    let mut formats: u64 = 0;
    let mut cur_format: *mut i8;
    let mut i: usize;

    const XENSND_LIST_SEPARATOR: *const i8 = b",\0" as *const i8;
    const XENSND_SAMPLE_FORMAT_MAX_LEN: usize = 19;

    loop {
        cur_format = unsafe { strsep(&mut (list as *mut *mut i8), XENSND_LIST_SEPARATOR) };
        if cur_format.is_null() {
            break;
        }

        i = 0;
        while i < CFG_HW_SUPPORTED_FORMATS.len() {
            if unsafe {
                strncasecmp(
                    cur_format,
                    CFG_HW_SUPPORTED_FORMATS[i].name,
                    XENSND_SAMPLE_FORMAT_MAX_LEN,
                )
            } == 0
            {
                formats |= CFG_HW_SUPPORTED_FORMATS[i].mask;
            }
            i += 1;
        }
    }

    if formats != 0 {
        unsafe {
            (*pcm_hw).formats = formats;
        }
    }
}

fn cfg_read_pcm_hw(
    path: *const i8,
    parent_pcm_hw: *const SndDrvPcmHwDefault,
    pcm_hw: *mut SndDrvPcmHwDefault,
) {
    let mut list: *mut i8;
    let mut val: u32;
    let mut len: u32;

    const XENSND_FIELD_CHANNELS_MIN: *const i8 = b"channels-min\0" as *const i8;
    const XENSND_FIELD_CHANNELS_MAX: *const i8 = b"channels-max\0" as *const i8;
    const XENSND_FIELD_SAMPLE_RATES: *const i8 = b"sample-rates\0" as *const i8;
    const XENSND_FIELD_SAMPLE_FORMATS: *const i8 = b"sample-formats\0" as *const i8;
    const XENSND_FIELD_BUFFER_SIZE: *const i8 = b"buffer-size\0" as *const i8;

    if !parent_pcm_hw.is_null() {
        unsafe {
            *pcm_hw = *parent_pcm_hw;
        }
    } else {
        unsafe {
            *pcm_hw = SND_DRV_PCM_HW_DEFAULT;
        }
    }

    val = unsafe { xenbus_read_unsigned(path, XENSND_FIELD_CHANNELS_MIN, 0) };
    if val != 0 {
        unsafe {
            (*pcm_hw).channels_min = val;
        }
    }

    val = unsafe { xenbus_read_unsigned(path, XENSND_FIELD_CHANNELS_MAX, 0) };
    if val != 0 {
        unsafe {
            (*pcm_hw).channels_max = val;
        }
    }

    len = 0;
    list = unsafe { xenbus_read(0, path, XENSND_FIELD_SAMPLE_RATES, &mut len) };
    if !is_err(list as *const u8) {
        cfg_hw_rates(list, len, path, pcm_hw);
        unsafe {
            kfree(list as *mut u8);
        }
    }

    len = 0;
    list = unsafe { xenbus_read(0, path, XENSND_FIELD_SAMPLE_FORMATS, &mut len) };
    if !is_err(list as *const u8) {
        cfg_formats(list, len, path, pcm_hw);
        unsafe {
            kfree(list as *mut u8);
        }
    }

    let buf_sz = unsafe { xenbus_read_unsigned(path, XENSND_FIELD_BUFFER_SIZE, 0) };
    if buf_sz != 0 {
        unsafe {
            (*pcm_hw).buffer_bytes_max = buf_sz as usize;
        }
    }

    unsafe {
        if (*pcm_hw).channels_min > (*pcm_hw).channels_max {
            (*pcm_hw).channels_min = (*pcm_hw).channels_max;
        }

        if (*pcm_hw).rate_min > (*pcm_hw).rate_max {
            (*pcm_hw).rate_min = (*pcm_hw).rate_max;
        }

        (*pcm_hw).period_bytes_max = (*pcm_hw).buffer_bytes_max;
        (*pcm_hw).periods_max =
            (*pcm_hw).period_bytes_max / (*pcm_hw).period_bytes_min;
    }
}

fn cfg_get_stream_type(
    path: *const i8,
    index: i32,
    num_pb: *mut i32,
    num_cap: *mut i32,
) -> i32 {
    let mut str_ptr: *mut i8 = std::ptr::null_mut();
    let mut stream_path: *mut i8;
    let ret: i32;

    const XENSND_FIELD_TYPE: *const i8 = b"type\0" as *const i8;
    const XENSND_STREAM_TYPE_PLAYBACK: *const i8 = b"playback\0" as *const i8;
    const XENSND_STREAM_TYPE_CAPTURE: *const i8 = b"capture\0" as *const i8;

    unsafe {
        *num_pb = 0;
        *num_cap = 0;
    }

    stream_path = unsafe { kasprintf(0x10, b"%s/%d\0" as *const i8, path, index) };
    if stream_path.is_null() {
        return -12; // -ENOMEM
    }

    str_ptr = unsafe { xenbus_read(0, stream_path, XENSND_FIELD_TYPE, std::ptr::null_mut()) };
    if is_err(str_ptr as *const u8) {
        ret = ptr_err(str_ptr as *const u8);
        str_ptr = std::ptr::null_mut();
    } else if unsafe {
        strncasecmp(str_ptr, XENSND_STREAM_TYPE_PLAYBACK, 8) == 0
    }
    {
        unsafe {
            *num_pb += 1;
        }
        ret = 0;
    } else if unsafe {
        strncasecmp(str_ptr, XENSND_STREAM_TYPE_CAPTURE, 7) == 0
    }
    {
        unsafe {
            *num_cap += 1;
        }
        ret = 0;
    } else {
        ret = -22; // -EINVAL
    }

    unsafe {
        kfree(stream_path as *mut u8);
        if !str_ptr.is_null() {
            kfree(str_ptr as *mut u8);
        }
    }
    ret
}

#[repr(C)]
struct XenFrontCfgStream {
    index: i32,
    xenstore_path: *mut i8,
    pcm_hw: SndDrvPcmHwDefault,
}

#[repr(C)]
struct XenFrontCfgPcmInstance {
    name: [i8; 32],
    device_id: i32,
    num_streams_pb: i32,
    num_streams_cap: i32,
    streams_pb: *mut XenFrontCfgStream,
    streams_cap: *mut XenFrontCfgStream,
    pcm_hw: SndDrvPcmHwDefault,
}

#[repr(C)]
struct XenSndFrontInfo {
    xb_dev: *mut u8,
    cfg: *mut u8,
}

fn cfg_stream(
    front_info: *mut XenSndFrontInfo,
    pcm_instance: *mut XenFrontCfgPcmInstance,
    path: *const i8,
    index: i32,
    cur_pb: *mut i32,
    cur_cap: *mut i32,
    stream_cnt: *mut i32,
) -> i32 {
    let mut str_ptr: *mut i8 = std::ptr::null_mut();
    let stream_path: *mut i8;
    let stream: *mut XenFrontCfgStream;
    let mut ret: i32;

    const XENSND_FIELD_TYPE: *const i8 = b"type\0" as *const i8;
    const XENSND_STREAM_TYPE_PLAYBACK: *const i8 = b"playback\0" as *const i8;
    const XENSND_STREAM_TYPE_CAPTURE: *const i8 = b"capture\0" as *const i8;

    unsafe {
        stream_path = devm_kasprintf(
            (*front_info).xb_dev,
            0x10,
            b"%s/%d\0" as *const i8,
            path,
            index,
        );
    }
    if stream_path.is_null() {
        ret = -12; // -ENOMEM
        return ret;
    }

    str_ptr = unsafe { xenbus_read(0, stream_path, XENSND_FIELD_TYPE, std::ptr::null_mut()) };
    if is_err(str_ptr as *const u8) {
        ret = ptr_err(str_ptr as *const u8);
        str_ptr = std::ptr::null_mut();
        return ret;
    }

    if unsafe { strncasecmp(str_ptr, XENSND_STREAM_TYPE_PLAYBACK, 8) == 0 } {
        unsafe {
            stream = &mut (*pcm_instance).streams_pb[(*cur_pb) as usize];
            *cur_pb += 1;
        }
    } else if unsafe { strncasecmp(str_ptr, XENSND_STREAM_TYPE_CAPTURE, 7) == 0 } {
        unsafe {
            stream = &mut (*pcm_instance).streams_cap[(*cur_cap) as usize];
            *cur_cap += 1;
        }
    } else {
        ret = -22; // -EINVAL
        unsafe {
            kfree(str_ptr as *mut u8);
        }
        return ret;
    }

    unsafe {
        (*stream).index = *stream_cnt;
        *stream_cnt += 1;
        (*stream).xenstore_path = stream_path;
        cfg_read_pcm_hw(
            stream_path,
            &(*pcm_instance).pcm_hw,
            &mut (*stream).pcm_hw,
        );
        kfree(str_ptr as *mut u8);
    }
    0
}

fn cfg_device(
    front_info: *mut XenSndFrontInfo,
    pcm_instance: *mut XenFrontCfgPcmInstance,
    parent_pcm_hw: *const SndDrvPcmHwDefault,
    path: *const i8,
    node_index: i32,
    stream_cnt: *mut i32,
) -> i32 {
    let mut str_ptr: *mut i8;
    let device_path: *mut i8;
    let mut ret: i32;
    let mut i: i32;
    let mut num_streams: i32;
    let mut num_pb: i32;
    let mut num_cap: i32;
    let mut cur_pb: i32;
    let mut cur_cap: i32;
    let mut node: [i8; 3] = [0; 3];

    const XENSND_FIELD_DEVICE_NAME: *const i8 = b"name\0" as *const i8;

    device_path = unsafe { kasprintf(0x10, b"%s/%d\0" as *const i8, path, node_index) };
    if device_path.is_null() {
        return -12; // -ENOMEM
    }

    str_ptr = unsafe { xenbus_read(0, device_path, XENSND_FIELD_DEVICE_NAME, std::ptr::null_mut()) };
    if !is_err(str_ptr as *const u8) {
        unsafe {
            strscpy(
                (*pcm_instance).name.as_mut_ptr(),
                str_ptr,
                (*pcm_instance).name.len(),
            );
            kfree(str_ptr as *mut u8);
        }
    }

    unsafe {
        (*pcm_instance).device_id = node_index;
    }

    cfg_read_pcm_hw(device_path, parent_pcm_hw, unsafe {
        &mut (*pcm_instance).pcm_hw
    });

    num_streams = 0;
    loop {
        unsafe {
            snprintf(
                node.as_mut_ptr(),
                node.len(),
                b"%d\0" as *const i8,
                num_streams,
            );
        }
        if unsafe { xenbus_exists(0, device_path, node.as_ptr()) } == 0 {
            break;
        }
        num_streams += 1;
        if num_streams >= VSND_MAX_STREAM as i32 {
            break;
        }
    }

    unsafe {
        (*pcm_instance).num_streams_pb = 0;
        (*pcm_instance).num_streams_cap = 0;
    }

    i = 0;
    while i < num_streams {
        num_pb = 0;
        num_cap = 0;
        ret = cfg_get_stream_type(device_path, i, &mut num_pb, &mut num_cap);
        if ret < 0 {
            unsafe {
                kfree(device_path as *mut u8);
            }
            return ret;
        }

        unsafe {
            (*pcm_instance).num_streams_pb += num_pb;
            (*pcm_instance).num_streams_cap += num_cap;
        }
        i += 1;
    }

    if unsafe { (*pcm_instance).num_streams_pb } > 0 {
        unsafe {
            (*pcm_instance).streams_pb = devm_kcalloc(
                (*front_info).xb_dev,
                (*pcm_instance).num_streams_pb as usize,
                std::mem::size_of::<XenFrontCfgStream>(),
                0x10,
            ) as *mut XenFrontCfgStream;
            if (*pcm_instance).streams_pb.is_null() {
                kfree(device_path as *mut u8);
                return -12; // -ENOMEM
            }
        }
    }

    if unsafe { (*pcm_instance).num_streams_cap } > 0 {
        unsafe {
            (*pcm_instance).streams_cap = devm_kcalloc(
                (*front_info).xb_dev,
                (*pcm_instance).num_streams_cap as usize,
                std::mem::size_of::<XenFrontCfgStream>(),
                0x10,
            ) as *mut XenFrontCfgStream;
            if (*pcm_instance).streams_cap.is_null() {
                kfree(device_path as *mut u8);
                return -12; // -ENOMEM
            }
        }
    }

    cur_pb = 0;
    cur_cap = 0;
    i = 0;
    while i < num_streams {
        ret = cfg_stream(
            front_info,
            pcm_instance,
            device_path,
            i,
            &mut cur_pb,
            &mut cur_cap,
            stream_cnt,
        );
        if ret < 0 {
            unsafe {
                kfree(device_path as *mut u8);
            }
            return ret;
        }
        i += 1;
    }

    unsafe {
        kfree(device_path as *mut u8);
    }
    0
}

#[repr(C)]
pub struct XenFrontCfgCard {
    pcm_instances: *mut XenFrontCfgPcmInstance,
    num_pcm_instances: i32,
    pcm_hw: SndDrvPcmHwDefault,
}

pub extern "C" fn xen_snd_front_cfg_card(
    front_info: *mut XenSndFrontInfo,
    stream_cnt: *mut i32,
) -> i32 {
    let mut xb_dev: *mut u8;
    let cfg: *mut XenFrontCfgCard;
    let mut ret: i32;
    let mut num_devices: i32;
    let mut i: i32;
    let mut node: [i8; 3] = [0; 3];

    const SNDRV_PCM_DEVICES: i32 = 8;

    unsafe {
        xb_dev = (*front_info).xb_dev;
        cfg = (*front_info).cfg as *mut XenFrontCfgCard;
        *stream_cnt = 0;
    }

    num_devices = 0;
    loop {
        unsafe {
            scnprintf(
                node.as_mut_ptr(),
                node.len(),
                b"%d\0" as *const i8,
                num_devices,
            );
        }
        if unsafe { xenbus_exists(0, unsafe { (xb_dev as *const u8) as *const i8 }, node.as_ptr()) }
            == 0
        {
            break;
        }
        num_devices += 1;
        if num_devices >= SNDRV_PCM_DEVICES {
            break;
        }
    }

    if num_devices == 0 {
        // dev_warn call - would output warning
        return -19; // -ENODEV
    }

    cfg_read_pcm_hw(unsafe { (xb_dev as *const u8) as *const i8 }, std::ptr::null(), unsafe {
        &mut (*cfg).pcm_hw
    });

    unsafe {
        (*cfg).pcm_instances = devm_kcalloc(
            xb_dev,
            num_devices as usize,
            std::mem::size_of::<XenFrontCfgPcmInstance>(),
            0x10,
        ) as *mut XenFrontCfgPcmInstance;
        if (*cfg).pcm_instances.is_null() {
            return -12; // -ENOMEM
        }
    }

    i = 0;
    while i < num_devices {
        ret = cfg_device(
            front_info,
            unsafe { &mut (*cfg).pcm_instances[i as usize] },
            unsafe { &(*cfg).pcm_hw },
            unsafe { (xb_dev as *const u8) as *const i8 },
            i,
            stream_cnt,
        );
        if ret < 0 {
            return ret;
        }
        i += 1;
    }

    unsafe {
        (*cfg).num_pcm_instances = num_devices;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
