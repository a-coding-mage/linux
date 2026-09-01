// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * OSS compatible i/o control
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

// C dependencies originally included here:
// "seq_oss_device.h", "seq_oss_readq.h", "seq_oss_writeq.h",
// "seq_oss_timer.h", "seq_oss_synth.h", "seq_oss_midi.h",
// "seq_oss_event.h"

use core::ffi::{c_int, c_ulong, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

unsafe fn snd_seq_oss_synth_info_user(
    dp: *mut seq_oss_devinfo,
    arg: *mut c_void,
) -> c_int {
    let mut info: synth_info = core::mem::zeroed();

    if copy_from_user(
        &mut info as *mut synth_info as *mut c_void,
        arg,
        size_of::<synth_info>(),
    ) != 0
    {
        return -EFAULT;
    }
    if snd_seq_oss_synth_make_info(dp, info.device, &mut info) < 0 {
        return -EINVAL;
    }
    if copy_to_user(
        arg,
        &info as *const synth_info as *const c_void,
        size_of::<synth_info>(),
    ) != 0
    {
        return -EFAULT;
    }
    0
}

unsafe fn snd_seq_oss_midi_info_user(
    dp: *mut seq_oss_devinfo,
    arg: *mut c_void,
) -> c_int {
    let mut info: midi_info = core::mem::zeroed();

    if copy_from_user(
        &mut info as *mut midi_info as *mut c_void,
        arg,
        size_of::<midi_info>(),
    ) != 0
    {
        return -EFAULT;
    }
    if snd_seq_oss_midi_make_info(dp, info.device, &mut info) < 0 {
        return -EINVAL;
    }
    if copy_to_user(
        arg,
        &info as *const midi_info as *const c_void,
        size_of::<midi_info>(),
    ) != 0
    {
        return -EFAULT;
    }
    0
}

unsafe fn snd_seq_oss_oob_user(dp: *mut seq_oss_devinfo, arg: *mut c_void) -> c_int {
    let mut ev: [u8; 8] = [0; 8];
    let mut tmpev: snd_seq_event = core::mem::zeroed();

    if copy_from_user(ev.as_mut_ptr() as *mut c_void, arg, 8) != 0 {
        return -EFAULT;
    }
    memset(
        &mut tmpev as *mut snd_seq_event as *mut c_void,
        0,
        size_of::<snd_seq_event>(),
    );
    snd_seq_oss_fill_addr(
        dp,
        &mut tmpev,
        (*dp).addr.client,
        (*dp).addr.port,
    );
    tmpev.time.tick = 0;

    // Original C uses: snd_use_lock_t *lock __free(seq_oss_use_lock) = NULL;
    let mut lock: *mut snd_use_lock_t = ptr::null_mut();

    if !snd_seq_oss_process_event(
        dp,
        ev.as_mut_ptr() as *mut evrec,
        &mut tmpev,
        &mut lock,
    ) {
        snd_seq_oss_dispatch(dp, &mut tmpev, 0, 0);
    }
    0
}

pub unsafe fn snd_seq_oss_ioctl(
    dp: *mut seq_oss_devinfo,
    cmd: c_uint,
    carg: c_ulong,
) -> c_int {
    let mut dev: c_int;
    let mut val: c_int;
    let arg: *mut c_void = carg as *mut c_void;
    let p: *mut c_int = arg as *mut c_int;

    match cmd {
        x if x == SNDCTL_TMR_TIMEBASE
            || x == SNDCTL_TMR_TEMPO
            || x == SNDCTL_TMR_START
            || x == SNDCTL_TMR_STOP
            || x == SNDCTL_TMR_CONTINUE
            || x == SNDCTL_TMR_METRONOME
            || x == SNDCTL_TMR_SOURCE
            || x == SNDCTL_TMR_SELECT
            || x == SNDCTL_SEQ_CTRLRATE =>
        {
            return snd_seq_oss_timer_ioctl((*dp).timer, cmd, arg);
        }

        x if x == SNDCTL_SEQ_PANIC => {
            snd_seq_oss_reset(dp);
            return -EINVAL;
        }

        x if x == SNDCTL_SEQ_SYNC => {
            if !is_write_mode((*dp).file_mode) || (*dp).writeq.is_null() {
                return 0;
            }
            while snd_seq_oss_writeq_sync((*dp).writeq) != 0 {}
            if signal_pending(current) != 0 {
                return -ERESTARTSYS;
            }
            return 0;
        }

        x if x == SNDCTL_SEQ_RESET => {
            snd_seq_oss_reset(dp);
            return 0;
        }

        x if x == SNDCTL_SEQ_TESTMIDI => {
            if get_user(&mut dev, p) != 0 {
                return -EFAULT;
            }
            return snd_seq_oss_midi_open(dp, dev, (*dp).file_mode);
        }

        x if x == SNDCTL_SEQ_GETINCOUNT => {
            if (*dp).readq.is_null() || !is_read_mode((*dp).file_mode) {
                return 0;
            }
            return if put_user((*(*dp).readq).qlen, p) != 0 {
                -EFAULT
            } else {
                0
            };
        }

        x if x == SNDCTL_SEQ_GETOUTCOUNT => {
            if !is_write_mode((*dp).file_mode) || (*dp).writeq.is_null() {
                return 0;
            }
            return if put_user(snd_seq_oss_writeq_get_free_size((*dp).writeq), p) != 0 {
                -EFAULT
            } else {
                0
            };
        }

        x if x == SNDCTL_SEQ_GETTIME => {
            return if put_user(snd_seq_oss_timer_cur_tick((*dp).timer), p) != 0 {
                -EFAULT
            } else {
                0
            };
        }

        x if x == SNDCTL_SEQ_RESETSAMPLES => {
            if get_user(&mut dev, p) != 0 {
                return -EFAULT;
            }
            return snd_seq_oss_synth_ioctl(dp, dev, cmd, carg);
        }

        x if x == SNDCTL_SEQ_NRSYNTHS => {
            return if put_user((*dp).max_synthdev, p) != 0 {
                -EFAULT
            } else {
                0
            };
        }

        x if x == SNDCTL_SEQ_NRMIDIS => {
            return if put_user((*dp).max_mididev, p) != 0 {
                -EFAULT
            } else {
                0
            };
        }

        x if x == SNDCTL_SYNTH_MEMAVL => {
            if get_user(&mut dev, p) != 0 {
                return -EFAULT;
            }
            val = snd_seq_oss_synth_ioctl(dp, dev, cmd, carg);
            return if put_user(val, p) != 0 { -EFAULT } else { 0 };
        }

        x if x == SNDCTL_FM_4OP_ENABLE => {
            if get_user(&mut dev, p) != 0 {
                return -EFAULT;
            }
            snd_seq_oss_synth_ioctl(dp, dev, cmd, carg);
            return 0;
        }

        x if x == SNDCTL_SYNTH_INFO || x == SNDCTL_SYNTH_ID => {
            return snd_seq_oss_synth_info_user(dp, arg);
        }

        x if x == SNDCTL_SEQ_OUTOFBAND => {
            return snd_seq_oss_oob_user(dp, arg);
        }

        x if x == SNDCTL_MIDI_INFO => {
            return snd_seq_oss_midi_info_user(dp, arg);
        }

        x if x == SNDCTL_SEQ_THRESHOLD => {
            if !is_write_mode((*dp).file_mode) {
                return 0;
            }
            if get_user(&mut val, p) != 0 {
                return -EFAULT;
            }
            if val < 1 {
                val = 1;
            }
            if val >= (*(*dp).writeq).maxlen {
                val = (*(*dp).writeq).maxlen - 1;
            }
            snd_seq_oss_writeq_set_output((*dp).writeq, val);
            return 0;
        }

        x if x == SNDCTL_MIDI_PRETIME => {
            if (*dp).readq.is_null() || !is_read_mode((*dp).file_mode) {
                return 0;
            }
            if get_user(&mut val, p) != 0 {
                return -EFAULT;
            }
            if val <= 0 {
                val = -1;
            } else {
                val = (HZ * val) / 10;
            }
            (*(*dp).readq).pre_event_timeout = val;
            return if put_user(val, p) != 0 { -EFAULT } else { 0 };
        }

        _ => {
            if !is_write_mode((*dp).file_mode) {
                return -EIO;
            }
            return snd_seq_oss_synth_ioctl(dp, 0, cmd, carg);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
