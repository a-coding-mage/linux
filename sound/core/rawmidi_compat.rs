// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   32bit -> 64bit ioctl wrapper for raw MIDI API
 *   Copyright (c) by Takashi Iwai <tiwai@suse.de>
 */

/* This file included from rawmidi.c */

/* C dependency: #include <linux/compat.h> */

#[repr(C, packed)]
pub struct snd_rawmidi_params32 {
    pub stream: i32,
    pub buffer_size: u32,
    pub avail_min: u32,
    pub no_active_sensing: ::core::ffi::c_uint, /* avoid bit-field */
    pub mode: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uchar; 12],
}

unsafe fn snd_rawmidi_ioctl_params_compat(
    rfile: *mut snd_rawmidi_file,
    src: *mut snd_rawmidi_params32,
) -> ::core::ffi::c_int {
    let mut params: snd_rawmidi_params = unsafe { ::core::mem::zeroed() };
    let mut val: ::core::ffi::c_uint = 0;

    if unsafe {
        get_user(
            ::core::ptr::addr_of_mut!(params.stream),
            ::core::ptr::addr_of!((*src).stream),
        ) != 0
            || get_user(
                ::core::ptr::addr_of_mut!(params.buffer_size),
                ::core::ptr::addr_of!((*src).buffer_size),
            ) != 0
            || get_user(
                ::core::ptr::addr_of_mut!(params.avail_min),
                ::core::ptr::addr_of!((*src).avail_min),
            ) != 0
            || get_user(
                ::core::ptr::addr_of_mut!(params.mode),
                ::core::ptr::addr_of!((*src).mode),
            ) != 0
            || get_user(
                ::core::ptr::addr_of_mut!(val),
                ::core::ptr::addr_of!((*src).no_active_sensing),
            ) != 0
    } {
        return -EFAULT;
    }
    params.no_active_sensing = val;
    match params.stream {
        SNDRV_RAWMIDI_STREAM_OUTPUT => {
            if unsafe { (*rfile).output.is_null() } {
                return -EINVAL;
            }
            return unsafe {
                snd_rawmidi_output_params((*rfile).output, ::core::ptr::addr_of_mut!(params))
            };
        }
        SNDRV_RAWMIDI_STREAM_INPUT => {
            if unsafe { (*rfile).input.is_null() } {
                return -EINVAL;
            }
            return unsafe {
                snd_rawmidi_input_params((*rfile).input, ::core::ptr::addr_of_mut!(params))
            };
        }
        _ => {}
    }
    -EINVAL
}

#[repr(C, packed)]
pub struct compat_snd_rawmidi_status64 {
    pub stream: i32,
    pub rsvd: [u8; 4], /* alignment */
    pub tstamp_sec: i64,
    pub tstamp_nsec: i64,
    pub avail: u32,
    pub xruns: u32,
    pub reserved: [::core::ffi::c_uchar; 16],
}

unsafe fn snd_rawmidi_ioctl_status_compat64(
    rfile: *mut snd_rawmidi_file,
    src: *mut compat_snd_rawmidi_status64,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int;
    let mut status: snd_rawmidi_status64 = unsafe { ::core::mem::zeroed() };
    let mut compat_status: compat_snd_rawmidi_status64 = unsafe { ::core::mem::zeroed() };

    if unsafe {
        get_user(
            ::core::ptr::addr_of_mut!(status.stream),
            ::core::ptr::addr_of!((*src).stream),
        ) != 0
    } {
        return -EFAULT;
    }

    match status.stream {
        SNDRV_RAWMIDI_STREAM_OUTPUT => {
            if unsafe { (*rfile).output.is_null() } {
                return -EINVAL;
            }
            err = unsafe { snd_rawmidi_output_status((*rfile).output, ::core::ptr::addr_of_mut!(status)) };
        }
        SNDRV_RAWMIDI_STREAM_INPUT => {
            if unsafe { (*rfile).input.is_null() } {
                return -EINVAL;
            }
            err = unsafe { snd_rawmidi_input_status((*rfile).input, ::core::ptr::addr_of_mut!(status)) };
        }
        _ => {
            return -EINVAL;
        }
    }
    if err < 0 {
        return err;
    }

    compat_status = compat_snd_rawmidi_status64 {
        stream: status.stream,
        rsvd: [0; 4],
        tstamp_sec: status.tstamp_sec,
        tstamp_nsec: status.tstamp_nsec,
        avail: status.avail,
        xruns: status.xruns,
        reserved: [0; 16],
    };

    if unsafe {
        copy_to_user(
            src as *mut ::core::ffi::c_void,
            ::core::ptr::addr_of!(compat_status) as *const ::core::ffi::c_void,
            ::core::mem::size_of_val(unsafe { &*src }),
        ) != 0
    } {
        return -EFAULT;
    }

    0
}

pub const SNDRV_RAWMIDI_IOCTL_PARAMS32: ::core::ffi::c_uint =
    _IOWR::<snd_rawmidi_params32>(b'W' as ::core::ffi::c_uint, 0x10);
pub const SNDRV_RAWMIDI_IOCTL_STATUS_COMPAT32: ::core::ffi::c_uint =
    _IOWR::<snd_rawmidi_status32>(b'W' as ::core::ffi::c_uint, 0x20);
pub const SNDRV_RAWMIDI_IOCTL_STATUS_COMPAT64: ::core::ffi::c_uint =
    _IOWR::<compat_snd_rawmidi_status64>(b'W' as ::core::ffi::c_uint, 0x20);

unsafe fn snd_rawmidi_ioctl_compat(
    file: *mut file,
    cmd: ::core::ffi::c_uint,
    arg: ::core::ffi::c_ulong,
) -> ::core::ffi::c_long {
    let rfile: *mut snd_rawmidi_file;
    let argp: *mut ::core::ffi::c_void = unsafe { compat_ptr(arg) };

    rfile = unsafe { (*file).private_data as *mut snd_rawmidi_file };
    match cmd {
        SNDRV_RAWMIDI_IOCTL_PVERSION
        | SNDRV_RAWMIDI_IOCTL_INFO
        | SNDRV_RAWMIDI_IOCTL_DROP
        | SNDRV_RAWMIDI_IOCTL_DRAIN => {
            return unsafe { snd_rawmidi_ioctl(file, cmd, argp as ::core::ffi::c_ulong) };
        }
        /*
         * C conditional:
         * #if IS_ENABLED(CONFIG_SND_UMP)
         * case SNDRV_UMP_IOCTL_ENDPOINT_INFO:
         * case SNDRV_UMP_IOCTL_BLOCK_INFO:
         * #endif
         */
        SNDRV_UMP_IOCTL_ENDPOINT_INFO | SNDRV_UMP_IOCTL_BLOCK_INFO => {
            return unsafe { snd_rawmidi_ioctl(file, cmd, argp as ::core::ffi::c_ulong) };
        }
        SNDRV_RAWMIDI_IOCTL_PARAMS32 => {
            return unsafe { snd_rawmidi_ioctl_params_compat(rfile, argp as *mut snd_rawmidi_params32) }
                as ::core::ffi::c_long;
        }
        SNDRV_RAWMIDI_IOCTL_STATUS_COMPAT32 => {
            return unsafe { snd_rawmidi_ioctl_status32(rfile, argp) } as ::core::ffi::c_long;
        }
        SNDRV_RAWMIDI_IOCTL_STATUS_COMPAT64 => {
            return unsafe {
                snd_rawmidi_ioctl_status_compat64(
                    rfile,
                    argp as *mut compat_snd_rawmidi_status64,
                )
            } as ::core::ffi::c_long;
        }
        _ => {}
    }
    -ENOIOCTLCMD as ::core::ffi::c_long
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
