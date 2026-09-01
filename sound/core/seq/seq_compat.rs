// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   32bit -> 64bit ioctl wrapper for sequencer API
 *   Copyright (c) by Takashi Iwai <tiwai@suse.de>
 */

/* This file included from seq.c */

use core::ffi::{c_long, c_uint, c_ulong, c_void};

use crate::*;

#[repr(C)]
pub struct snd_seq_port_info32 {
    pub addr: snd_seq_addr, /* client/port numbers */
    pub name: [i8; 64],     /* port name */

    pub capability: u32,   /* port capability bits */
    pub type_: u32,        /* port type bits */
    pub midi_channels: i32, /* channels per MIDI port */
    pub midi_voices: i32,  /* voices per MIDI port */
    pub synth_voices: i32, /* voices per SYNTH port */

    pub read_use: i32,  /* R/O: subscribers for output (from this port) */
    pub write_use: i32, /* R/O: subscribers for input (to this port) */

    pub kernel: u32,          /* reserved for kernel use (must be NULL) */
    pub flags: u32,           /* misc. conditioning */
    pub time_queue: u8,       /* queue # for timestamping */
    pub reserved: [i8; 59],   /* for future use */
}

unsafe fn snd_seq_call_port_info_ioctl(
    client: *mut snd_seq_client,
    cmd: c_uint,
    data32: *mut snd_seq_port_info32,
) -> c_int {
    let mut err: c_int;
    let data: *mut snd_seq_port_info = kmalloc_obj::<snd_seq_port_info>();

    if data.is_null() {
        return -ENOMEM;
    }

    if copy_from_user(
        data as *mut c_void,
        data32 as *const c_void,
        core::mem::size_of::<snd_seq_port_info32>(),
    ) != 0
        || get_user(&mut (*data).flags, &(*data32).flags) != 0
        || get_user(&mut (*data).time_queue, &(*data32).time_queue) != 0
    {
        kfree(data as *mut c_void);
        return -EFAULT;
    }
    (*data).kernel = core::ptr::null_mut();

    err = snd_seq_kernel_client_ctl((*client).number, cmd, data as *mut c_void);
    if err < 0 {
        kfree(data as *mut c_void);
        return err;
    }

    if copy_to_user(
        data32 as *mut c_void,
        data as *const c_void,
        core::mem::size_of::<snd_seq_port_info32>(),
    ) != 0
        || put_user((*data).flags, &mut (*data32).flags) != 0
        || put_user((*data).time_queue, &mut (*data32).time_queue) != 0
    {
        kfree(data as *mut c_void);
        return -EFAULT;
    }

    kfree(data as *mut c_void);
    err
}

/*
 */

/* ioctl numbers are the 32-bit compat forms of the native sequencer port ioctls. */
pub const SNDRV_SEQ_IOCTL_CREATE_PORT32: c_uint =
    _IOWR(b'S' as c_uint, 0x20, core::mem::size_of::<snd_seq_port_info32>()) as c_uint;
pub const SNDRV_SEQ_IOCTL_DELETE_PORT32: c_uint =
    _IOW(b'S' as c_uint, 0x21, core::mem::size_of::<snd_seq_port_info32>()) as c_uint;
pub const SNDRV_SEQ_IOCTL_GET_PORT_INFO32: c_uint =
    _IOWR(b'S' as c_uint, 0x22, core::mem::size_of::<snd_seq_port_info32>()) as c_uint;
pub const SNDRV_SEQ_IOCTL_SET_PORT_INFO32: c_uint =
    _IOW(b'S' as c_uint, 0x23, core::mem::size_of::<snd_seq_port_info32>()) as c_uint;
pub const SNDRV_SEQ_IOCTL_QUERY_NEXT_PORT32: c_uint =
    _IOWR(b'S' as c_uint, 0x52, core::mem::size_of::<snd_seq_port_info32>()) as c_uint;

unsafe fn snd_seq_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let client: *mut snd_seq_client = (*file).private_data as *mut snd_seq_client;
    let argp: *mut c_void = compat_ptr(arg);

    if snd_BUG_ON(client.is_null()) {
        return -ENXIO as c_long;
    }

    match cmd {
        SNDRV_SEQ_IOCTL_PVERSION
        | SNDRV_SEQ_IOCTL_USER_PVERSION
        | SNDRV_SEQ_IOCTL_CLIENT_ID
        | SNDRV_SEQ_IOCTL_SYSTEM_INFO
        | SNDRV_SEQ_IOCTL_GET_CLIENT_INFO
        | SNDRV_SEQ_IOCTL_SET_CLIENT_INFO
        | SNDRV_SEQ_IOCTL_GET_CLIENT_UMP_INFO
        | SNDRV_SEQ_IOCTL_SET_CLIENT_UMP_INFO
        | SNDRV_SEQ_IOCTL_SUBSCRIBE_PORT
        | SNDRV_SEQ_IOCTL_UNSUBSCRIBE_PORT
        | SNDRV_SEQ_IOCTL_CREATE_QUEUE
        | SNDRV_SEQ_IOCTL_DELETE_QUEUE
        | SNDRV_SEQ_IOCTL_GET_QUEUE_INFO
        | SNDRV_SEQ_IOCTL_SET_QUEUE_INFO
        | SNDRV_SEQ_IOCTL_GET_NAMED_QUEUE
        | SNDRV_SEQ_IOCTL_GET_QUEUE_STATUS
        | SNDRV_SEQ_IOCTL_GET_QUEUE_TEMPO
        | SNDRV_SEQ_IOCTL_SET_QUEUE_TEMPO
        | SNDRV_SEQ_IOCTL_GET_QUEUE_TIMER
        | SNDRV_SEQ_IOCTL_SET_QUEUE_TIMER
        | SNDRV_SEQ_IOCTL_GET_QUEUE_CLIENT
        | SNDRV_SEQ_IOCTL_SET_QUEUE_CLIENT
        | SNDRV_SEQ_IOCTL_GET_CLIENT_POOL
        | SNDRV_SEQ_IOCTL_SET_CLIENT_POOL
        | SNDRV_SEQ_IOCTL_REMOVE_EVENTS
        | SNDRV_SEQ_IOCTL_QUERY_SUBS
        | SNDRV_SEQ_IOCTL_GET_SUBSCRIPTION
        | SNDRV_SEQ_IOCTL_QUERY_NEXT_CLIENT
        | SNDRV_SEQ_IOCTL_RUNNING_MODE => snd_seq_ioctl(file, cmd, arg),
        SNDRV_SEQ_IOCTL_CREATE_PORT32 => snd_seq_call_port_info_ioctl(
            client,
            SNDRV_SEQ_IOCTL_CREATE_PORT,
            argp as *mut snd_seq_port_info32,
        ) as c_long,
        SNDRV_SEQ_IOCTL_DELETE_PORT32 => snd_seq_call_port_info_ioctl(
            client,
            SNDRV_SEQ_IOCTL_DELETE_PORT,
            argp as *mut snd_seq_port_info32,
        ) as c_long,
        SNDRV_SEQ_IOCTL_GET_PORT_INFO32 => snd_seq_call_port_info_ioctl(
            client,
            SNDRV_SEQ_IOCTL_GET_PORT_INFO,
            argp as *mut snd_seq_port_info32,
        ) as c_long,
        SNDRV_SEQ_IOCTL_SET_PORT_INFO32 => snd_seq_call_port_info_ioctl(
            client,
            SNDRV_SEQ_IOCTL_SET_PORT_INFO,
            argp as *mut snd_seq_port_info32,
        ) as c_long,
        SNDRV_SEQ_IOCTL_QUERY_NEXT_PORT32 => snd_seq_call_port_info_ioctl(
            client,
            SNDRV_SEQ_IOCTL_QUERY_NEXT_PORT,
            argp as *mut snd_seq_port_info32,
        ) as c_long,
        _ => -ENOIOCTLCMD as c_long,
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
