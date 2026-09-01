// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PCM DRM helpers
 *
 * C dependencies removed from executable Rust:
 * <linux/export.h>, <linux/types.h>, <sound/asoundef.h>, <sound/pcm.h>,
 * <sound/pcm_params.h>, <sound/pcm_iec958.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type u8 = core::ffi::c_uchar;
pub type uint = core::ffi::c_uint;
pub type size_t = usize;

unsafe extern "C" {
    fn snd_pcm_format_width(format: core::ffi::c_int) -> core::ffi::c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> uint;
}

/*
 * External declarations supplied by translated kernel/ALSA headers.
 */
#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: uint,
    pub format: core::ffi::c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

unsafe extern "C" {
    static EINVAL: core::ffi::c_int;
    static IEC958_AES0_CON_NOT_COPYRIGHT: u8;
    static IEC958_AES0_CON_EMPHASIS_NONE: u8;
    static IEC958_AES1_CON_GENERAL: u8;
    static IEC958_AES2_CON_SOURCE_UNSPEC: u8;
    static IEC958_AES2_CON_CHANNEL_UNSPEC: u8;
    static IEC958_AES3_CON_CLOCK_1000PPM: u8;
    static IEC958_AES3_CON_FS_NOTID: u8;
    static IEC958_AES4_CON_WORDLEN_NOTID: u8;
    static IEC958_AES3_CON_FS: u8;
    static IEC958_AES3_CON_FS_32000: u8;
    static IEC958_AES3_CON_FS_44100: u8;
    static IEC958_AES3_CON_FS_48000: u8;
    static IEC958_AES3_CON_FS_88200: u8;
    static IEC958_AES3_CON_FS_96000: u8;
    static IEC958_AES3_CON_FS_176400: u8;
    static IEC958_AES3_CON_FS_192000: u8;
    static IEC958_AES4_CON_WORDLEN: u8;
    static IEC958_AES4_CON_WORDLEN_20_16: u8;
    static IEC958_AES4_CON_WORDLEN_22_18: u8;
    static IEC958_AES4_CON_MAX_WORDLEN_24: u8;
    static IEC958_AES4_CON_WORDLEN_24_20: u8;
}

/**
 * snd_pcm_create_iec958_consumer_default - create default consumer format IEC958 channel status
 * @cs: channel status buffer, at least four bytes
 * @len: length of channel status buffer
 *
 * Create the consumer format channel status data in @cs of maximum size
 * @len. When relevant, the configuration-dependant bits will be set as
 * unspecified.
 *
 * Drivers should then call einter snd_pcm_fill_iec958_consumer() or
 * snd_pcm_fill_iec958_consumer_hw_params() to replace these unspecified
 * bits by their actual values.
 *
 * Drivers may wish to tweak the contents of the buffer after creation.
 *
 * Returns: length of buffer, or negative error code if something failed.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_create_iec958_consumer_default(
    cs: *mut u8,
    len: size_t,
) -> core::ffi::c_int {
    if len < 4 {
        return -EINVAL;
    }

    core::ptr::write_bytes(cs, 0, len);

    *cs.add(0) = IEC958_AES0_CON_NOT_COPYRIGHT | IEC958_AES0_CON_EMPHASIS_NONE;
    *cs.add(1) = IEC958_AES1_CON_GENERAL;
    *cs.add(2) = IEC958_AES2_CON_SOURCE_UNSPEC | IEC958_AES2_CON_CHANNEL_UNSPEC;
    *cs.add(3) = IEC958_AES3_CON_CLOCK_1000PPM | IEC958_AES3_CON_FS_NOTID;

    if len > 4 {
        *cs.add(4) = IEC958_AES4_CON_WORDLEN_NOTID;
    }

    len as core::ffi::c_int
}
/* EXPORT_SYMBOL_GPL(snd_pcm_create_iec958_consumer_default); */

unsafe fn fill_iec958_consumer(
    rate: uint,
    sample_width: uint,
    cs: *mut u8,
    len: size_t,
) -> core::ffi::c_int {
    if len < 4 {
        return -EINVAL;
    }

    if (*cs.add(3) & IEC958_AES3_CON_FS) == IEC958_AES3_CON_FS_NOTID {
        let fs: uint;

        match rate {
            32000 => {
                fs = IEC958_AES3_CON_FS_32000 as uint;
            }
            44100 => {
                fs = IEC958_AES3_CON_FS_44100 as uint;
            }
            48000 => {
                fs = IEC958_AES3_CON_FS_48000 as uint;
            }
            88200 => {
                fs = IEC958_AES3_CON_FS_88200 as uint;
            }
            96000 => {
                fs = IEC958_AES3_CON_FS_96000 as uint;
            }
            176400 => {
                fs = IEC958_AES3_CON_FS_176400 as uint;
            }
            192000 => {
                fs = IEC958_AES3_CON_FS_192000 as uint;
            }
            _ => {
                return -EINVAL;
            }
        }

        *cs.add(3) &= !IEC958_AES3_CON_FS;
        *cs.add(3) |= fs as u8;
    }

    if len > 4
        && (*cs.add(4) & IEC958_AES4_CON_WORDLEN) == IEC958_AES4_CON_WORDLEN_NOTID
    {
        let ws: uint;

        match sample_width {
            16 => {
                ws = IEC958_AES4_CON_WORDLEN_20_16 as uint;
            }
            18 => {
                ws = IEC958_AES4_CON_WORDLEN_22_18 as uint;
            }
            20 => {
                ws = (IEC958_AES4_CON_WORDLEN_20_16 | IEC958_AES4_CON_MAX_WORDLEN_24) as uint;
            }
            24 | 32 => {
                /* Assume 24-bit width for 32-bit samples. */
                ws = (IEC958_AES4_CON_WORDLEN_24_20 | IEC958_AES4_CON_MAX_WORDLEN_24) as uint;
            }

            _ => {
                return -EINVAL;
            }
        }

        *cs.add(4) &= !IEC958_AES4_CON_WORDLEN;
        *cs.add(4) |= ws as u8;
    }

    len as core::ffi::c_int
}

/**
 * snd_pcm_fill_iec958_consumer - Fill consumer format IEC958 channel status
 * @runtime: pcm runtime structure with ->rate filled in
 * @cs: channel status buffer, at least four bytes
 * @len: length of channel status buffer
 *
 * Fill the unspecified bits in an IEC958 status bits array using the
 * parameters of the PCM runtime @runtime.
 *
 * Drivers may wish to tweak the contents of the buffer after its been
 * filled.
 *
 * Returns: length of buffer, or negative error code if something failed.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_fill_iec958_consumer(
    runtime: *mut snd_pcm_runtime,
    cs: *mut u8,
    len: size_t,
) -> core::ffi::c_int {
    fill_iec958_consumer(
        (*runtime).rate,
        snd_pcm_format_width((*runtime).format) as uint,
        cs,
        len,
    )
}
/* EXPORT_SYMBOL_GPL(snd_pcm_fill_iec958_consumer); */

/**
 * snd_pcm_fill_iec958_consumer_hw_params - Fill consumer format IEC958 channel status
 * @params: the hw_params instance for extracting rate and sample format
 * @cs: channel status buffer, at least four bytes
 * @len: length of channel status buffer
 *
 * Fill the unspecified bits in an IEC958 status bits array using the
 * parameters of the PCM hardware parameters @params.
 *
 * Drivers may wish to tweak the contents of the buffer after its been
 * filled..
 *
 * Returns: length of buffer, or negative error code if something failed.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_fill_iec958_consumer_hw_params(
    params: *mut snd_pcm_hw_params,
    cs: *mut u8,
    len: size_t,
) -> core::ffi::c_int {
    fill_iec958_consumer(params_rate(params), params_width(params), cs, len)
}
/* EXPORT_SYMBOL_GPL(snd_pcm_fill_iec958_consumer_hw_params); */

/**
 * snd_pcm_create_iec958_consumer - create consumer format IEC958 channel status
 * @runtime: pcm runtime structure with ->rate filled in
 * @cs: channel status buffer, at least four bytes
 * @len: length of channel status buffer
 *
 * Create the consumer format channel status data in @cs of maximum size
 * @len corresponding to the parameters of the PCM runtime @runtime.
 *
 * Drivers may wish to tweak the contents of the buffer after creation.
 *
 * Returns: length of buffer, or negative error code if something failed.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_create_iec958_consumer(
    runtime: *mut snd_pcm_runtime,
    cs: *mut u8,
    len: size_t,
) -> core::ffi::c_int {
    let ret: core::ffi::c_int;

    ret = snd_pcm_create_iec958_consumer_default(cs, len);
    if ret < 0 {
        return ret;
    }

    snd_pcm_fill_iec958_consumer(runtime, cs, len)
}
/* EXPORT_SYMBOL(snd_pcm_create_iec958_consumer); */

/**
 * snd_pcm_create_iec958_consumer_hw_params - create IEC958 channel status
 * @params: the hw_params instance for extracting rate and sample format
 * @cs: channel status buffer, at least four bytes
 * @len: length of channel status buffer
 *
 * Create the consumer format channel status data in @cs of maximum size
 * @len corresponding to the parameters of the PCM runtime @runtime.
 *
 * Drivers may wish to tweak the contents of the buffer after creation.
 *
 * Returns: length of buffer, or negative error code if something failed.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_create_iec958_consumer_hw_params(
    params: *mut snd_pcm_hw_params,
    cs: *mut u8,
    len: size_t,
) -> core::ffi::c_int {
    let ret: core::ffi::c_int;

    ret = snd_pcm_create_iec958_consumer_default(cs, len);
    if ret < 0 {
        return ret;
    }

    fill_iec958_consumer(params_rate(params), params_width(params), cs, len)
}
/* EXPORT_SYMBOL(snd_pcm_create_iec958_consumer_hw_params); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
