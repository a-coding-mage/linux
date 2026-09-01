// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Generic routines and proc interface for ELD(EDID Like Data) information
 *
 * Copyright(c) 2008 Intel Corporation.
 * Copyright (c) 2013 Anssi Hannula <anssi.hannula@iki.fi>
 *
 * Authors:
 * 		Wu Fengguang <wfg@linux.intel.com>
 */

use core::ffi::{c_char, c_int, c_longlong, c_uchar, c_uint};

// C includes translated as external dependencies:
// <linux/init.h>, <linux/slab.h>, <sound/core.h>, <sound/hda_chmap.h>,
// <sound/hda_codec.h>, "hda_local.h"

pub const CEA_EDID_VER_NONE: c_int = 0;
pub const CEA_EDID_VER_CEA861: c_int = 1;
pub const CEA_EDID_VER_CEA861A: c_int = 2;
pub const CEA_EDID_VER_CEA861BCD: c_int = 3;
pub const CEA_EDID_VER_RESERVED: c_int = 4;

/*
 * The following two lists are shared between
 * 	- HDMI audio InfoFrame (source to sink)
 * 	- CEA E-EDID Extension (sink to source)
 */

unsafe extern "C" {
    fn snd_hda_codec_read(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    ) -> c_uint;
    fn codec_info(codec: *mut hda_codec, fmt: *const c_char, ...);
    fn codec_dbg(codec: *mut hda_codec, fmt: *const c_char, ...);

    #[cfg(CONFIG_SND_PROC_FS)]
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    #[cfg(CONFIG_SND_PROC_FS)]
    fn snd_print_eld_info(e: *mut snd_parsed_hdmi_eld, buffer: *mut snd_info_buffer);
    #[cfg(CONFIG_SND_PROC_FS)]
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
    #[cfg(CONFIG_SND_PROC_FS)]
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    #[cfg(CONFIG_SND_PROC_FS)]
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    #[cfg(CONFIG_SND_PROC_FS)]
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

#[inline]
unsafe fn hdmi_get_eld_data(codec: *mut hda_codec, nid: hda_nid_t, byte_index: c_int) -> c_uint {
    let mut val: c_uint;

    val = unsafe {
        snd_hda_codec_read(
            codec,
            nid,
            0,
            AC_VERB_GET_HDMI_ELDD,
            byte_index as c_uint,
        )
    };
    #[cfg(BE_PARANOID)]
    unsafe {
        codec_info(
            codec,
            b"HDMI: ELD data byte %d: 0x%x\n\0".as_ptr() as *const c_char,
            byte_index,
            val,
        );
    }
    val
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdmi_get_eld_size(codec: *mut hda_codec, nid: hda_nid_t) -> c_int {
    unsafe {
        snd_hda_codec_read(
            codec,
            nid,
            0,
            AC_VERB_GET_HDMI_DIP_SIZE,
            AC_DIPSIZE_ELD_BUF,
        ) as c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdmi_get_eld(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    buf: *mut c_uchar,
    eld_size: *mut c_int,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int = 0;
    let mut size: c_int;

    /*
     * ELD size is initialized to zero in caller function. If no errors and
     * ELD is valid, actual eld_size is assigned.
     */

    size = unsafe { snd_hdmi_get_eld_size(codec, nid) };
    if size == 0 {
        /* wfg: workaround for ASUS P5E-VM HDMI board */
        unsafe {
            codec_info(
                codec,
                b"HDMI: ELD buf size is 0, force 128\n\0".as_ptr() as *const c_char,
            );
        }
        size = 128;
    }
    if size < ELD_FIXED_BYTES as c_int || size > ELD_MAX_SIZE as c_int {
        unsafe {
            codec_info(
                codec,
                b"HDMI: invalid ELD buf size %d\n\0".as_ptr() as *const c_char,
                size,
            );
        }
        return -ERANGE;
    }

    /* set ELD buffer */
    i = 0;
    while i < size {
        let mut val: c_uint = unsafe { hdmi_get_eld_data(codec, nid, i) };
        /*
         * Graphics driver might be writing to ELD buffer right now.
         * Just abort. The caller will repoll after a while.
         */
        if (val & AC_ELDD_ELD_VALID) == 0 {
            unsafe {
                codec_info(
                    codec,
                    b"HDMI: invalid ELD data byte %d\n\0".as_ptr() as *const c_char,
                    i,
                );
            }
            ret = -EINVAL;
            break;
        }
        val &= AC_ELDD_ELD_DATA;
        /*
         * The first byte cannot be zero. This can happen on some DVI
         * connections. Some Intel chips may also need some 250ms delay
         * to return non-zero ELD data, even when the graphics driver
         * correctly writes ELD content before setting ELD_valid bit.
         */
        if val == 0 && i == 0 {
            unsafe {
                codec_dbg(
                    codec,
                    b"HDMI: 0 ELD data\n\0".as_ptr() as *const c_char,
                );
            }
            ret = -EINVAL;
            break;
        }
        unsafe {
            *buf.add(i as usize) = val as c_uchar;
        }
        i += 1;
    }

    if ret == 0 {
        unsafe {
            *eld_size = size;
        }
    }
    ret
}

#[cfg(CONFIG_SND_PROC_FS)]
#[no_mangle]
pub unsafe extern "C" fn snd_hdmi_print_eld_info(
    eld: *mut hdmi_eld,
    buffer: *mut snd_info_buffer,
    pin_nid: hda_nid_t,
    dev_id: c_int,
    cvt_nid: hda_nid_t,
) {
    unsafe {
        snd_iprintf(
            buffer,
            b"monitor_present\t\t%d\n\0".as_ptr() as *const c_char,
            (*eld).monitor_present,
        );
        snd_iprintf(
            buffer,
            b"eld_valid\t\t%d\n\0".as_ptr() as *const c_char,
            (*eld).eld_valid,
        );
        snd_iprintf(
            buffer,
            b"codec_pin_nid\t\t0x%x\n\0".as_ptr() as *const c_char,
            pin_nid,
        );
        snd_iprintf(
            buffer,
            b"codec_dev_id\t\t0x%x\n\0".as_ptr() as *const c_char,
            dev_id,
        );
        snd_iprintf(
            buffer,
            b"codec_cvt_nid\t\t0x%x\n\0".as_ptr() as *const c_char,
            cvt_nid,
        );

        if (*eld).eld_valid == 0 {
            return;
        }

        snd_print_eld_info(&mut (*eld).info, buffer);
    }
}

#[cfg(CONFIG_SND_PROC_FS)]
#[no_mangle]
pub unsafe extern "C" fn snd_hdmi_write_eld_info(
    eld: *mut hdmi_eld,
    buffer: *mut snd_info_buffer,
) {
    let e: *mut snd_parsed_hdmi_eld = unsafe { &mut (*eld).info };
    let mut line: [c_char; 64] = [0; 64];
    let mut name: [c_char; 64] = [0; 64];
    let mut sname: *mut c_char;
    let mut val: c_longlong = 0;
    let mut n: c_uint;

    while unsafe { snd_info_get_line(buffer, line.as_mut_ptr(), line.len() as c_int) } == 0 {
        if unsafe {
            sscanf(
                line.as_ptr(),
                b"%s %llx\0".as_ptr() as *const c_char,
                name.as_mut_ptr(),
                &mut val,
            )
        } != 2
        {
            continue;
        }
        /*
         * We don't allow modification to these fields:
         * 	monitor_name manufacture_id product_id
         * 	eld_version edid_version
         */
        unsafe {
            if strcmp(name.as_ptr(), b"monitor_present\0".as_ptr() as *const c_char) == 0 {
                (*eld).monitor_present = val as _;
            } else if strcmp(name.as_ptr(), b"eld_valid\0".as_ptr() as *const c_char) == 0 {
                (*eld).eld_valid = val as _;
            } else if strcmp(name.as_ptr(), b"connection_type\0".as_ptr() as *const c_char) == 0 {
                (*e).conn_type = val as _;
            } else if strcmp(name.as_ptr(), b"port_id\0".as_ptr() as *const c_char) == 0 {
                (*e).port_id = val as _;
            } else if strcmp(name.as_ptr(), b"support_hdcp\0".as_ptr() as *const c_char) == 0 {
                (*e).support_hdcp = val as _;
            } else if strcmp(name.as_ptr(), b"support_ai\0".as_ptr() as *const c_char) == 0 {
                (*e).support_ai = val as _;
            } else if strcmp(name.as_ptr(), b"audio_sync_delay\0".as_ptr() as *const c_char) == 0 {
                (*e).aud_synch_delay = val as _;
            } else if strcmp(name.as_ptr(), b"speakers\0".as_ptr() as *const c_char) == 0 {
                (*e).spk_alloc = val as _;
            } else if strcmp(name.as_ptr(), b"sad_count\0".as_ptr() as *const c_char) == 0 {
                (*e).sad_count = val as _;
            } else if strncmp(name.as_ptr(), b"sad\0".as_ptr() as *const c_char, 3) == 0 {
                sname = name.as_mut_ptr().add(4);
                n = (name[3] - b'0' as c_char) as c_uint;
                if name[4] >= b'0' as c_char && name[4] <= b'9' as c_char {
                    sname = sname.add(1);
                    n = 10 * n + (name[4] - b'0' as c_char) as c_uint;
                }
                if n >= ELD_MAX_SAD {
                    continue;
                }
                if strcmp(sname, b"_coding_type\0".as_ptr() as *const c_char) == 0 {
                    (*e).sad[n as usize].format = val as _;
                } else if strcmp(sname, b"_channels\0".as_ptr() as *const c_char) == 0 {
                    (*e).sad[n as usize].channels = val as _;
                } else if strcmp(sname, b"_rates\0".as_ptr() as *const c_char) == 0 {
                    (*e).sad[n as usize].rates = val as _;
                } else if strcmp(sname, b"_bits\0".as_ptr() as *const c_char) == 0 {
                    (*e).sad[n as usize].sample_bits = val as _;
                } else if strcmp(sname, b"_max_bitrate\0".as_ptr() as *const c_char) == 0 {
                    (*e).sad[n as usize].max_bitrate = val as _;
                } else if strcmp(sname, b"_profile\0".as_ptr() as *const c_char) == 0 {
                    (*e).sad[n as usize].profile = val as _;
                }
                if n >= (*e).sad_count {
                    (*e).sad_count = n + 1;
                }
            }
        }
    }
}

/* update PCM info based on ELD */
#[no_mangle]
pub unsafe extern "C" fn snd_hdmi_eld_update_pcm_info(
    e: *mut snd_parsed_hdmi_eld,
    hinfo: *mut hda_pcm_stream,
) {
    let mut rates: u32;
    let mut formats: u64;
    let mut maxbps: c_uint;
    let mut channels_max: c_uint;
    let mut i: c_int;

    /* assume basic audio support (the basic audio flag is not in ELD;
     * however, all audio capable sinks are required to support basic
     * audio) */
    rates = SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
    formats = SNDRV_PCM_FMTBIT_S16_LE;
    maxbps = 16;
    channels_max = 2;
    i = 0;
    while unsafe { i < (*e).sad_count as c_int } {
        let a: *mut snd_cea_sad = unsafe { &mut (*e).sad[i as usize] };
        unsafe {
            rates |= (*a).rates;
            if (*a).channels > channels_max {
                channels_max = (*a).channels;
            }
            if (*a).format == AUDIO_CODING_TYPE_LPCM {
                if ((*a).sample_bits & ELD_PCM_BITS_20) != 0 {
                    formats |= SNDRV_PCM_FMTBIT_S32_LE;
                    if maxbps < 20 {
                        maxbps = 20;
                    }
                }
                if ((*a).sample_bits & ELD_PCM_BITS_24) != 0 {
                    formats |= SNDRV_PCM_FMTBIT_S32_LE;
                    if maxbps < 24 {
                        maxbps = 24;
                    }
                }
            }
        }
        i += 1;
    }

    /* restrict the parameters by the values the codec provides */
    unsafe {
        (*hinfo).rates &= rates;
        (*hinfo).formats &= formats;
        (*hinfo).maxbps = core::cmp::min((*hinfo).maxbps, maxbps);
        (*hinfo).channels_max = core::cmp::min((*hinfo).channels_max, channels_max);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
