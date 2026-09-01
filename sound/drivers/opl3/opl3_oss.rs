// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Interface for OSS sequencer emulation
 *
 *  Copyright (C) 2000 Uros Bizjak <uros@kss-loka.si>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

// Dependencies originally supplied by:
// #include <linux/export.h>
// #include "opl3_voice.h"

const SBFM_MAXINSTR: c_int = 256;

static oss_callback: snd_seq_oss_callback = snd_seq_oss_callback {
    owner: THIS_MODULE,
    open: Some(snd_opl3_open_seq_oss),
    close: Some(snd_opl3_close_seq_oss),
    ioctl: Some(snd_opl3_ioctl_seq_oss),
    load_patch: Some(snd_opl3_load_patch_seq_oss),
    reset: Some(snd_opl3_reset_seq_oss),
};

unsafe extern "C" fn snd_opl3_oss_event_input(
    ev: *mut snd_seq_event,
    direct: c_int,
    private_data: *mut c_void,
    atomic: c_int,
    hop: c_int,
) -> c_int {
    let opl3 = private_data as *mut snd_opl3;

    if (*ev).type_ != SNDRV_SEQ_EVENT_OSS {
        snd_midi_process_event(&opl3_ops, ev, (*opl3).oss_chset);
    }
    0
}

/* ------------------------------ */

unsafe extern "C" fn snd_opl3_oss_free_port(private_data: *mut c_void) {
    let opl3 = private_data as *mut snd_opl3;

    snd_midi_channel_free_set((*opl3).oss_chset);
}

unsafe fn snd_opl3_oss_create_port(opl3: *mut snd_opl3) -> c_int {
    let mut callbacks: snd_seq_port_callback = core::mem::zeroed();
    let mut name: [c_char; 32] = [0; 32];
    let voices: c_int;
    let opl_ver: c_int;

    voices = if (*opl3).hardware < OPL3_HW_OPL3 {
        MAX_OPL2_VOICES
    } else {
        MAX_OPL3_VOICES
    };
    (*opl3).oss_chset = snd_midi_channel_alloc_set(voices);
    if (*opl3).oss_chset.is_null() {
        return -ENOMEM;
    }
    (*(*opl3).oss_chset).private_data = opl3 as *mut c_void;

    callbacks.owner = THIS_MODULE;
    callbacks.event_input = Some(snd_opl3_oss_event_input);
    callbacks.private_free = Some(snd_opl3_oss_free_port);
    callbacks.private_data = opl3 as *mut c_void;

    opl_ver = (((*opl3).hardware & OPL3_HW_MASK) >> 8) as c_int;
    sprintf(name.as_mut_ptr(), c"OPL%i OSS Port".as_ptr(), opl_ver);

    (*(*opl3).oss_chset).client = (*opl3).seq_client;
    (*(*opl3).oss_chset).port = snd_seq_event_port_attach(
        (*opl3).seq_client,
        &mut callbacks,
        SNDRV_SEQ_PORT_CAP_WRITE,
        SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC
            | SNDRV_SEQ_PORT_TYPE_MIDI_GM
            | SNDRV_SEQ_PORT_TYPE_HARDWARE
            | SNDRV_SEQ_PORT_TYPE_SYNTHESIZER,
        voices,
        voices,
        name.as_mut_ptr(),
    );
    if (*(*opl3).oss_chset).port < 0 {
        let port: c_int;
        port = (*(*opl3).oss_chset).port;
        snd_midi_channel_free_set((*opl3).oss_chset);
        return port;
    }
    0
}

/* ------------------------------ */

/* register OSS synth */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_init_seq_oss(opl3: *mut snd_opl3, name: *mut c_char) {
    let mut arg: *mut snd_seq_oss_reg;
    let mut dev: *mut snd_seq_device = null_mut();

    if snd_seq_device_new(
        (*opl3).card,
        0,
        SNDRV_SEQ_DEV_ID_OSS,
        size_of::<snd_seq_oss_reg>(),
        &mut dev,
    ) < 0
    {
        return;
    }

    (*opl3).oss_seq_dev = dev;
    strscpy((*dev).name.as_mut_ptr(), name, size_of_val(&(*dev).name));
    arg = SNDRV_SEQ_DEVICE_ARGPTR(dev);
    (*arg).type_ = SYNTH_TYPE_FM;
    if (*opl3).hardware < OPL3_HW_OPL3 {
        (*arg).subtype = FM_TYPE_ADLIB;
        (*arg).nvoices = MAX_OPL2_VOICES;
    } else {
        (*arg).subtype = FM_TYPE_OPL3;
        (*arg).nvoices = MAX_OPL3_VOICES;
    }
    (*arg).oper = oss_callback;
    (*arg).private_data = opl3 as *mut c_void;

    if snd_opl3_oss_create_port(opl3) != 0 {
        /* register to OSS synth table */
        snd_device_register((*opl3).card, dev);
    }
}

/* unregister */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_free_seq_oss(opl3: *mut snd_opl3) {
    if !(*opl3).oss_seq_dev.is_null() {
        /* The instance should have been released in prior */
        (*opl3).oss_seq_dev = null_mut();
    }
}

/* ------------------------------ */

/* open OSS sequencer */
unsafe extern "C" fn snd_opl3_open_seq_oss(
    arg: *mut snd_seq_oss_arg,
    closure: *mut c_void,
) -> c_int {
    let opl3 = closure as *mut snd_opl3;
    let mut err: c_int;

    if snd_BUG_ON(arg.is_null() as c_int) != 0 {
        return -ENXIO;
    }

    err = snd_opl3_synth_setup(opl3);
    if err < 0 {
        return err;
    }

    /* fill the argument data */
    (*arg).private_data = opl3 as *mut c_void;
    (*arg).addr.client = (*(*opl3).oss_chset).client;
    (*arg).addr.port = (*(*opl3).oss_chset).port;

    err = snd_opl3_synth_use_inc(opl3);
    if err < 0 {
        return err;
    }

    (*opl3).synth_mode = SNDRV_OPL3_MODE_SYNTH;
    0
}

/* close OSS sequencer */
unsafe extern "C" fn snd_opl3_close_seq_oss(arg: *mut snd_seq_oss_arg) -> c_int {
    let opl3: *mut snd_opl3;

    if snd_BUG_ON(arg.is_null() as c_int) != 0 {
        return -ENXIO;
    }
    opl3 = (*arg).private_data as *mut snd_opl3;

    snd_opl3_synth_cleanup(opl3);

    snd_opl3_synth_use_dec(opl3);
    0
}

/* load patch */

/* from sound_config.h */
unsafe extern "C" fn snd_opl3_load_patch_seq_oss(
    arg: *mut snd_seq_oss_arg,
    format: c_int,
    buf: *const c_char,
    offs: c_int,
    count: c_int,
) -> c_int {
    let opl3: *mut snd_opl3;
    let mut sbi: sbi_instrument = core::mem::zeroed();
    let mut name: [c_char; 32] = [0; 32];
    let err: c_int;
    let type_: c_int;

    if snd_BUG_ON(arg.is_null() as c_int) != 0 {
        return -ENXIO;
    }
    opl3 = (*arg).private_data as *mut snd_opl3;

    if format == FM_PATCH {
        type_ = FM_PATCH_OPL2;
    } else if format == OPL3_PATCH {
        type_ = FM_PATCH_OPL3;
    } else {
        return -EINVAL;
    }

    if count < size_of::<sbi_instrument>() as c_int {
        dev_err(
            (*(*opl3).card).dev,
            c"FM Error: Patch record too short\n".as_ptr(),
        );
        return -EINVAL;
    }
    if copy_from_user(
        &mut sbi as *mut sbi_instrument as *mut c_void,
        buf as *const c_void,
        size_of::<sbi_instrument>(),
    ) != 0
    {
        return -EFAULT;
    }

    if sbi.channel < 0 || sbi.channel >= SBFM_MAXINSTR {
        dev_err(
            (*(*opl3).card).dev,
            c"FM Error: Invalid instrument number %d\n".as_ptr(),
            sbi.channel,
        );
        return -EINVAL;
    }

    name.fill(0);
    sprintf(name.as_mut_ptr(), c"Chan%d".as_ptr(), sbi.channel);

    err = snd_opl3_load_patch(
        opl3,
        sbi.channel,
        127,
        type_,
        name.as_mut_ptr(),
        null(),
        sbi.operators.as_mut_ptr(),
    );
    if err < 0 {
        return err;
    }

    size_of::<sbi_instrument>() as c_int
}

/* ioctl */
unsafe extern "C" fn snd_opl3_ioctl_seq_oss(
    arg: *mut snd_seq_oss_arg,
    cmd: c_uint,
    ioarg: c_ulong,
) -> c_int {
    let opl3: *mut snd_opl3;

    if snd_BUG_ON(arg.is_null() as c_int) != 0 {
        return -ENXIO;
    }
    opl3 = (*arg).private_data as *mut snd_opl3;
    match cmd {
        SNDCTL_FM_LOAD_INSTR => {
            dev_err(
                (*(*opl3).card).dev,
                c"OPL3: Obsolete ioctl(SNDCTL_FM_LOAD_INSTR) used. Fix the program.\n".as_ptr(),
            );
            -EINVAL
        }

        SNDCTL_SYNTH_MEMAVL => 0x7fffffff,

        SNDCTL_FM_4OP_ENABLE => {
            // handled automatically by OPL instrument type
            0
        }

        _ => -EINVAL,
    }
}

/* reset device */
unsafe extern "C" fn snd_opl3_reset_seq_oss(arg: *mut snd_seq_oss_arg) -> c_int {
    if snd_BUG_ON(arg.is_null() as c_int) != 0 {
        return -ENXIO;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
