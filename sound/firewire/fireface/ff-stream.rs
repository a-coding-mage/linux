// SPDX-License-Identifier: GPL-2.0-only
/*
 * ff-stream.c - a part of driver for RME Fireface series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto
 */

// Rust translation of implementation source; dependencies are supplied by surrounding driver code.

const READY_TIMEOUT_MS: u32 = 200;

pub unsafe fn snd_ff_stream_get_multiplier_mode(
    sfc: cip_sfc,
    mode: *mut snd_ff_stream_mode,
) -> c_int {
    static MODES: [snd_ff_stream_mode; CIP_SFC_COUNT as usize] = [
        SND_FF_STREAM_MODE_LOW,
        SND_FF_STREAM_MODE_LOW,
        SND_FF_STREAM_MODE_LOW,
        SND_FF_STREAM_MODE_MID,
        SND_FF_STREAM_MODE_MID,
        SND_FF_STREAM_MODE_HIGH,
        SND_FF_STREAM_MODE_HIGH,
    ];

    if sfc >= CIP_SFC_COUNT {
        return -EINVAL;
    }

    *mode = MODES[sfc as usize];

    0
}

#[inline]
unsafe fn finish_session(ff: *mut snd_ff) {
    ((*(*(*ff).spec).protocol).finish_session)(ff);
    ((*(*(*ff).spec).protocol).switch_fetching_mode)(ff, false);
}

unsafe fn init_stream(ff: *mut snd_ff, s: *mut amdtp_stream) -> c_int {
    let resources: *mut fw_iso_resources;
    let dir: amdtp_stream_direction;
    let mut err: c_int;

    if s == &mut (*ff).tx_stream as *mut amdtp_stream {
        resources = &mut (*ff).tx_resources;
        dir = AMDTP_IN_STREAM;
    } else {
        resources = &mut (*ff).rx_resources;
        dir = AMDTP_OUT_STREAM;
    }

    err = fw_iso_resources_init(resources, (*ff).unit);
    if err < 0 {
        return err;
    }

    err = amdtp_ff_init(s, (*ff).unit, dir);
    if err < 0 {
        fw_iso_resources_destroy(resources);
    }

    err
}

unsafe fn destroy_stream(ff: *mut snd_ff, s: *mut amdtp_stream) {
    amdtp_stream_destroy(s);

    if s == &mut (*ff).tx_stream as *mut amdtp_stream {
        fw_iso_resources_destroy(&mut (*ff).tx_resources);
    } else {
        fw_iso_resources_destroy(&mut (*ff).rx_resources);
    }
}

pub unsafe fn snd_ff_stream_init_duplex(ff: *mut snd_ff) -> c_int {
    let mut err: c_int;

    err = init_stream(ff, &mut (*ff).rx_stream);
    if err < 0 {
        return err;
    }

    err = init_stream(ff, &mut (*ff).tx_stream);
    if err < 0 {
        destroy_stream(ff, &mut (*ff).rx_stream);
        return err;
    }

    err = amdtp_domain_init(&mut (*ff).domain);
    if err < 0 {
        destroy_stream(ff, &mut (*ff).rx_stream);
        destroy_stream(ff, &mut (*ff).tx_stream);
    }

    err
}

/*
 * This function should be called before starting streams or after stopping
 * streams.
 */
pub unsafe fn snd_ff_stream_destroy_duplex(ff: *mut snd_ff) {
    amdtp_domain_destroy(&mut (*ff).domain);

    destroy_stream(ff, &mut (*ff).rx_stream);
    destroy_stream(ff, &mut (*ff).tx_stream);
}

pub unsafe fn snd_ff_stream_reserve_duplex(
    ff: *mut snd_ff,
    rate: c_uint,
    frames_per_period: c_uint,
    frames_per_buffer: c_uint,
) -> c_int {
    let mut curr_rate: c_uint = 0;
    let mut src: snd_ff_clock_src = core::mem::zeroed();
    let mut err: c_int;

    err = ((*(*(*ff).spec).protocol).get_clock)(ff, &mut curr_rate, &mut src);
    if err < 0 {
        return err;
    }

    if (*ff).substreams_counter == 0 || curr_rate != rate {
        let mut mode: snd_ff_stream_mode = core::mem::zeroed();
        let mut i: c_int;

        amdtp_domain_stop(&mut (*ff).domain);
        finish_session(ff);

        fw_iso_resources_free(&mut (*ff).tx_resources);
        fw_iso_resources_free(&mut (*ff).rx_resources);

        i = 0;
        while i < CIP_SFC_COUNT {
            if amdtp_rate_table[i as usize] == rate {
                break;
            }
            i += 1;
        }
        if i >= CIP_SFC_COUNT {
            return -EINVAL;
        }

        err = snd_ff_stream_get_multiplier_mode(i, &mut mode);
        if err < 0 {
            return err;
        }

        err = amdtp_ff_set_parameters(
            &mut (*ff).tx_stream,
            rate,
            (*(*ff).spec).pcm_capture_channels[mode as usize],
        );
        if err < 0 {
            return err;
        }

        err = amdtp_ff_set_parameters(
            &mut (*ff).rx_stream,
            rate,
            (*(*ff).spec).pcm_playback_channels[mode as usize],
        );
        if err < 0 {
            return err;
        }

        err = ((*(*(*ff).spec).protocol).allocate_resources)(ff, rate);
        if err < 0 {
            return err;
        }

        err = amdtp_domain_set_events_per_period(
            &mut (*ff).domain,
            frames_per_period,
            frames_per_buffer,
        );
        if err < 0 {
            fw_iso_resources_free(&mut (*ff).tx_resources);
            fw_iso_resources_free(&mut (*ff).rx_resources);
            return err;
        }
    }

    0
}

pub unsafe fn snd_ff_stream_start_duplex(ff: *mut snd_ff, rate: c_uint) -> c_int {
    let mut err: c_int;

    if (*ff).substreams_counter == 0 {
        return 0;
    }

    if amdtp_streaming_error(&mut (*ff).tx_stream) || amdtp_streaming_error(&mut (*ff).rx_stream) {
        amdtp_domain_stop(&mut (*ff).domain);
        finish_session(ff);
    }

    /*
     * Regardless of current source of clock signal, drivers transfer some
     * packets. Then, the device transfers packets.
     */
    if !amdtp_stream_running(&mut (*ff).rx_stream) {
        let spd: c_int = (*fw_parent_device((*ff).unit)).max_speed;

        err = ((*(*(*ff).spec).protocol).begin_session)(ff, rate);
        if err < 0 {
            goto_error(ff, err);
            return err;
        }

        err = amdtp_domain_add_stream(
            &mut (*ff).domain,
            &mut (*ff).rx_stream,
            (*ff).rx_resources.channel,
            spd,
        );
        if err < 0 {
            goto_error(ff, err);
            return err;
        }

        err = amdtp_domain_add_stream(
            &mut (*ff).domain,
            &mut (*ff).tx_stream,
            (*ff).tx_resources.channel,
            spd,
        );
        if err < 0 {
            goto_error(ff, err);
            return err;
        }

        // NOTE: The device doesn't transfer packets unless receiving any packet. The
        // sequence of tx packets includes cycle skip corresponding to empty packet or
        // NODATA packet in IEC 61883-1/6. The sequence of the number of data blocks per
        // packet is important for media clock recovery.
        err = amdtp_domain_start(&mut (*ff).domain, 0, true, true);
        if err < 0 {
            goto_error(ff, err);
            return err;
        }

        if !amdtp_domain_wait_ready(&mut (*ff).domain, READY_TIMEOUT_MS) {
            err = -ETIMEDOUT;
            goto_error(ff, err);
            return err;
        }

        err = ((*(*(*ff).spec).protocol).switch_fetching_mode)(ff, true);
        if err < 0 {
            goto_error(ff, err);
            return err;
        }
    }

    0
}

unsafe fn goto_error(ff: *mut snd_ff, err: c_int) -> c_int {
    amdtp_domain_stop(&mut (*ff).domain);
    finish_session(ff);

    err
}

pub unsafe fn snd_ff_stream_stop_duplex(ff: *mut snd_ff) {
    if (*ff).substreams_counter == 0 {
        amdtp_domain_stop(&mut (*ff).domain);
        finish_session(ff);

        fw_iso_resources_free(&mut (*ff).tx_resources);
        fw_iso_resources_free(&mut (*ff).rx_resources);
    }
}

pub unsafe fn snd_ff_stream_update_duplex(ff: *mut snd_ff) {
    amdtp_domain_stop(&mut (*ff).domain);

    // The device discontinue to transfer packets.
    amdtp_stream_pcm_abort(&mut (*ff).tx_stream);
    amdtp_stream_pcm_abort(&mut (*ff).rx_stream);
}

pub unsafe fn snd_ff_stream_lock_changed(ff: *mut snd_ff) {
    (*ff).dev_lock_changed = true;
    wake_up(&mut (*ff).hwdep_wait);
}

pub unsafe fn snd_ff_stream_lock_try(ff: *mut snd_ff) -> c_int {
    let _guard = spinlock_irq_guard(&mut (*ff).lock);

    /* user land lock this */
    if (*ff).dev_lock_count < 0 {
        return -EBUSY;
    }

    /* this is the first time */
    let old = (*ff).dev_lock_count;
    (*ff).dev_lock_count += 1;
    if old == 0 {
        snd_ff_stream_lock_changed(ff);
    }
    0
}

pub unsafe fn snd_ff_stream_lock_release(ff: *mut snd_ff) {
    let _guard = spinlock_irq_guard(&mut (*ff).lock);

    if WARN_ON((*ff).dev_lock_count <= 0) {
        return;
    }
    (*ff).dev_lock_count -= 1;
    if (*ff).dev_lock_count == 0 {
        snd_ff_stream_lock_changed(ff);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
