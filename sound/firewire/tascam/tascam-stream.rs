// SPDX-License-Identifier: GPL-2.0-only
/*
 * tascam-stream.c - a part of driver for TASCAM FireWire series
 *
 * Copyright (c) 2015 Takashi Sakamoto
 */

// Rust translation of implementation source. External kernel, FireWire, ALSA,
// and local driver symbols are expected to be supplied by surrounding modules.

const CLOCK_STATUS_MASK: u32 = 0xffff0000;
const CLOCK_CONFIG_MASK: u32 = 0x0000ffff;

const READY_TIMEOUT_MS: u32 = 4000;

unsafe fn get_clock(tscm: *mut snd_tscm, data: *mut u32) -> c_int {
    let mut trial: c_int = 0;
    let mut reg: __be32 = 0;
    let mut err: c_int;

    while {
        trial += 1;
        trial < 5 + 1
    } {
        err = snd_fw_transaction(
            (*tscm).unit,
            TCODE_READ_QUADLET_REQUEST,
            TSCM_ADDR_BASE + TSCM_OFFSET_CLOCK_STATUS,
            &mut reg as *mut __be32 as *mut c_void,
            core::mem::size_of_val(&reg),
            0,
        );
        if err < 0 {
            return err;
        }

        *data = be32_to_cpu(reg);
        if (*data & CLOCK_STATUS_MASK) != 0 {
            break;
        }

        // In intermediate state after changing clock status.
        msleep(50);
    }

    // Still in the intermediate state.
    if trial >= 5 {
        return -EAGAIN;
    }

    0
}

unsafe fn set_clock(tscm: *mut snd_tscm, rate: c_uint, clock: snd_tscm_clock) -> c_int {
    let mut data: u32 = 0;
    let mut reg: __be32;
    let mut err: c_int;

    err = get_clock(tscm, &mut data);
    if err < 0 {
        return err;
    }
    data &= CLOCK_CONFIG_MASK;

    if rate > 0 {
        data &= 0x000000ff;
        /* Base rate. */
        if (rate % 44100) == 0 {
            data |= 0x00000100;
            /* Multiplier. */
            if rate / 44100 == 2 {
                data |= 0x00008000;
            }
        } else if (rate % 48000) == 0 {
            data |= 0x00000200;
            /* Multiplier. */
            if rate / 48000 == 2 {
                data |= 0x00008000;
            }
        } else {
            return -EAGAIN;
        }
    }

    if clock != INT_MAX {
        data &= 0x0000ff00;
        data |= (clock + 1) as u32;
    }

    reg = cpu_to_be32(data);

    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_CLOCK_STATUS,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    if (data & 0x00008000) != 0 {
        reg = cpu_to_be32(0x0000001a);
    } else {
        reg = cpu_to_be32(0x0000000d);
    }

    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_MULTIPLEX_MODE,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    )
}

pub unsafe fn snd_tscm_stream_get_rate(tscm: *mut snd_tscm, rate: *mut c_uint) -> c_int {
    let mut data: u32 = 0;
    let mut err: c_int;

    err = get_clock(tscm, &mut data);
    if err < 0 {
        return err;
    }

    data = (data & 0xff000000) >> 24;

    /* Check base rate. */
    if (data & 0x0f) == 0x01 {
        *rate = 44100;
    } else if (data & 0x0f) == 0x02 {
        *rate = 48000;
    } else {
        return -EAGAIN;
    }

    /* Check multiplier. */
    if (data & 0xf0) == 0x80 {
        *rate *= 2;
    } else if (data & 0xf0) != 0x00 {
        return -EAGAIN;
    }

    err
}

pub unsafe fn snd_tscm_stream_get_clock(
    tscm: *mut snd_tscm,
    clock: *mut snd_tscm_clock,
) -> c_int {
    let mut data: u32 = 0;
    let mut err: c_int;

    err = get_clock(tscm, &mut data);
    if err < 0 {
        return err;
    }

    *clock = (((data & 0x00ff0000) >> 16) as snd_tscm_clock) - 1;
    if *clock < 0 || *clock > SND_TSCM_CLOCK_ADAT {
        return -EIO;
    }

    0
}

unsafe fn enable_data_channels(tscm: *mut snd_tscm) -> c_int {
    let mut reg: __be32;
    let mut data: u32;
    let mut i: c_uint;
    let mut err: c_int;

    data = 0;
    i = 0;
    while i < (*(*tscm).spec).pcm_capture_analog_channels {
        data |= BIT(i);
        i += 1;
    }
    if (*(*tscm).spec).has_adat {
        data |= 0x0000ff00;
    }
    if (*(*tscm).spec).has_spdif {
        data |= 0x00030000;
    }

    reg = cpu_to_be32(data);
    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_TX_PCM_CHANNELS,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    data = 0;
    i = 0;
    while i < (*(*tscm).spec).pcm_playback_analog_channels {
        data |= BIT(i);
        i += 1;
    }
    if (*(*tscm).spec).has_adat {
        data |= 0x0000ff00;
    }
    if (*(*tscm).spec).has_spdif {
        data |= 0x00030000;
    }

    reg = cpu_to_be32(data);
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_RX_PCM_CHANNELS,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    )
}

unsafe fn set_stream_formats(tscm: *mut snd_tscm, _rate: c_uint) -> c_int {
    let mut reg: __be32;
    let mut err: c_int;

    // Set an option for unknown purpose.
    reg = cpu_to_be32(0x00200000);
    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_SET_OPTION,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    enable_data_channels(tscm)
}

unsafe fn finish_session(tscm: *mut snd_tscm) {
    let mut reg: __be32;

    reg = 0;
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_START_STREAMING,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );

    reg = 0;
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_RX_ON,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );

    // Unregister channels.
    reg = cpu_to_be32(0x00000000);
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_TX_CH,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
    reg = cpu_to_be32(0x00000000);
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_UNKNOWN,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
    reg = cpu_to_be32(0x00000000);
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_RX_CH,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
}

unsafe fn begin_session(tscm: *mut snd_tscm) -> c_int {
    let mut reg: __be32;
    let mut err: c_int;

    // Register the isochronous channel for transmitting stream.
    reg = cpu_to_be32((*tscm).tx_resources.channel);
    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_TX_CH,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    // Unknown.
    reg = cpu_to_be32(0x00000002);
    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_UNKNOWN,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    // Register the isochronous channel for receiving stream.
    reg = cpu_to_be32((*tscm).rx_resources.channel);
    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_RX_CH,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    reg = cpu_to_be32(0x00000001);
    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_START_STREAMING,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    reg = cpu_to_be32(0x00000001);
    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_RX_ON,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    // Set an option for unknown purpose.
    reg = cpu_to_be32(0x00002000);
    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_SET_OPTION,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    // Start multiplexing PCM samples on packets.
    reg = cpu_to_be32(0x00000001);
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_TX_ON,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    )
}

unsafe fn keep_resources(
    tscm: *mut snd_tscm,
    rate: c_uint,
    stream: *mut amdtp_stream,
) -> c_int {
    let resources: *mut fw_iso_resources;
    let speed: c_int;
    let mut err: c_int;

    if stream == &mut (*tscm).tx_stream {
        resources = &mut (*tscm).tx_resources;
        speed = (*fw_parent_device((*tscm).unit)).max_speed;
    } else {
        resources = &mut (*tscm).rx_resources;
        speed = SCODE_400;
    }

    err = amdtp_tscm_set_parameters(stream, rate);
    if err < 0 {
        return err;
    }

    fw_iso_resources_allocate(resources, amdtp_stream_get_max_payload(stream), speed)
}

unsafe fn init_stream(tscm: *mut snd_tscm, s: *mut amdtp_stream) -> c_int {
    let resources: *mut fw_iso_resources;
    let dir: amdtp_stream_direction;
    let mut pcm_channels: c_uint;
    let mut err: c_int;

    if s == &mut (*tscm).tx_stream {
        resources = &mut (*tscm).tx_resources;
        dir = AMDTP_IN_STREAM;
        pcm_channels = (*(*tscm).spec).pcm_capture_analog_channels;
    } else {
        resources = &mut (*tscm).rx_resources;
        dir = AMDTP_OUT_STREAM;
        pcm_channels = (*(*tscm).spec).pcm_playback_analog_channels;
    }

    if (*(*tscm).spec).has_adat {
        pcm_channels += 8;
    }
    if (*(*tscm).spec).has_spdif {
        pcm_channels += 2;
    }

    err = fw_iso_resources_init(resources, (*tscm).unit);
    if err < 0 {
        return err;
    }

    err = amdtp_tscm_init(s, (*tscm).unit, dir, pcm_channels);
    if err < 0 {
        fw_iso_resources_free(resources);
    }

    err
}

unsafe fn destroy_stream(tscm: *mut snd_tscm, s: *mut amdtp_stream) {
    amdtp_stream_destroy(s);

    if s == &mut (*tscm).tx_stream {
        fw_iso_resources_destroy(&mut (*tscm).tx_resources);
    } else {
        fw_iso_resources_destroy(&mut (*tscm).rx_resources);
    }
}

pub unsafe fn snd_tscm_stream_init_duplex(tscm: *mut snd_tscm) -> c_int {
    let mut err: c_int;

    err = init_stream(tscm, &mut (*tscm).tx_stream);
    if err < 0 {
        return err;
    }

    err = init_stream(tscm, &mut (*tscm).rx_stream);
    if err < 0 {
        destroy_stream(tscm, &mut (*tscm).tx_stream);
        return err;
    }

    err = amdtp_domain_init(&mut (*tscm).domain);
    if err < 0 {
        destroy_stream(tscm, &mut (*tscm).tx_stream);
        destroy_stream(tscm, &mut (*tscm).rx_stream);
    }

    err
}

// At bus reset, streaming is stopped and some registers are clear.
pub unsafe fn snd_tscm_stream_update_duplex(tscm: *mut snd_tscm) {
    amdtp_domain_stop(&mut (*tscm).domain);

    amdtp_stream_pcm_abort(&mut (*tscm).tx_stream);
    amdtp_stream_pcm_abort(&mut (*tscm).rx_stream);
}

// This function should be called before starting streams or after stopping
// streams.
pub unsafe fn snd_tscm_stream_destroy_duplex(tscm: *mut snd_tscm) {
    amdtp_domain_destroy(&mut (*tscm).domain);

    destroy_stream(tscm, &mut (*tscm).rx_stream);
    destroy_stream(tscm, &mut (*tscm).tx_stream);
}

pub unsafe fn snd_tscm_stream_reserve_duplex(
    tscm: *mut snd_tscm,
    rate: c_uint,
    frames_per_period: c_uint,
    frames_per_buffer: c_uint,
) -> c_int {
    let mut curr_rate: c_uint = 0;
    let mut err: c_int;

    err = snd_tscm_stream_get_rate(tscm, &mut curr_rate);
    if err < 0 {
        return err;
    }

    if (*tscm).substreams_counter == 0 || rate != curr_rate {
        amdtp_domain_stop(&mut (*tscm).domain);

        finish_session(tscm);

        fw_iso_resources_free(&mut (*tscm).tx_resources);
        fw_iso_resources_free(&mut (*tscm).rx_resources);

        err = set_clock(tscm, rate, INT_MAX);
        if err < 0 {
            return err;
        }

        err = keep_resources(tscm, rate, &mut (*tscm).tx_stream);
        if err < 0 {
            return err;
        }

        err = keep_resources(tscm, rate, &mut (*tscm).rx_stream);
        if err < 0 {
            fw_iso_resources_free(&mut (*tscm).tx_resources);
            return err;
        }

        err = amdtp_domain_set_events_per_period(
            &mut (*tscm).domain,
            frames_per_period,
            frames_per_buffer,
        );
        if err < 0 {
            fw_iso_resources_free(&mut (*tscm).tx_resources);
            fw_iso_resources_free(&mut (*tscm).rx_resources);
            return err;
        }

        (*tscm).need_long_tx_init_skip = rate != curr_rate;
    }

    0
}

pub unsafe fn snd_tscm_stream_start_duplex(tscm: *mut snd_tscm, rate: c_uint) -> c_int {
    let generation: c_uint = (*tscm).rx_resources.generation;
    let mut err: c_int;

    if (*tscm).substreams_counter == 0 {
        return 0;
    }

    if amdtp_streaming_error(&mut (*tscm).rx_stream) || amdtp_streaming_error(&mut (*tscm).tx_stream)
    {
        amdtp_domain_stop(&mut (*tscm).domain);
        finish_session(tscm);
    }

    if generation != (*(*fw_parent_device((*tscm).unit)).card).generation {
        err = fw_iso_resources_update(&mut (*tscm).tx_resources);
        if err < 0 {
            goto_error(tscm);
            return err;
        }

        err = fw_iso_resources_update(&mut (*tscm).rx_resources);
        if err < 0 {
            goto_error(tscm);
            return err;
        }
    }

    if !amdtp_stream_running(&mut (*tscm).rx_stream) {
        let tx_init_skip_cycles: c_uint;

        err = set_stream_formats(tscm, rate);
        if err < 0 {
            goto_error(tscm);
            return err;
        }

        err = begin_session(tscm);
        if err < 0 {
            goto_error(tscm);
            return err;
        }

        err = amdtp_domain_add_stream(
            &mut (*tscm).domain,
            &mut (*tscm).rx_stream,
            (*tscm).rx_resources.channel,
            (*fw_parent_device((*tscm).unit)).max_speed,
        );
        if err < 0 {
            goto_error(tscm);
            return err;
        }

        err = amdtp_domain_add_stream(
            &mut (*tscm).domain,
            &mut (*tscm).tx_stream,
            (*tscm).tx_resources.channel,
            SCODE_400,
        );
        if err < 0 {
            goto_error(tscm);
            return err;
        }

        if (*tscm).need_long_tx_init_skip {
            tx_init_skip_cycles = 16000;
        } else {
            tx_init_skip_cycles = 0;
        }

        // MEMO: Just after starting packet streaming, it transfers packets without any
        // event. Enough after receiving the sequence of packets, it multiplexes events into
        // the packet. However, just after changing sampling transfer frequency, it stops
        // multiplexing during packet transmission. Enough after, it restarts multiplexing
        // again. The device ignores presentation time expressed by the value of syt field
        // of CIP header in received packets. The sequence of the number of data blocks per
        // packet is important for media clock recovery.
        err = amdtp_domain_start(&mut (*tscm).domain, tx_init_skip_cycles, true, true);
        if err < 0 {
            goto_error(tscm);
            return err;
        }

        if !amdtp_domain_wait_ready(&mut (*tscm).domain, READY_TIMEOUT_MS) {
            err = -ETIMEDOUT;
            goto_error(tscm);
            return err;
        }
    }

    0
}

unsafe fn goto_error(tscm: *mut snd_tscm) {
    amdtp_domain_stop(&mut (*tscm).domain);
    finish_session(tscm);
}

pub unsafe fn snd_tscm_stream_stop_duplex(tscm: *mut snd_tscm) {
    if (*tscm).substreams_counter == 0 {
        amdtp_domain_stop(&mut (*tscm).domain);
        finish_session(tscm);

        fw_iso_resources_free(&mut (*tscm).tx_resources);
        fw_iso_resources_free(&mut (*tscm).rx_resources);

        (*tscm).need_long_tx_init_skip = false;
    }
}

pub unsafe fn snd_tscm_stream_lock_changed(tscm: *mut snd_tscm) {
    (*tscm).dev_lock_changed = true;
    wake_up(&mut (*tscm).hwdep_wait);
}

pub unsafe fn snd_tscm_stream_lock_try(tscm: *mut snd_tscm) -> c_int {
    guard_spinlock_irq(&mut (*tscm).lock);

    /* user land lock this */
    if (*tscm).dev_lock_count < 0 {
        return -EBUSY;
    }

    /* this is the first time */
    let old_dev_lock_count = (*tscm).dev_lock_count;
    (*tscm).dev_lock_count += 1;
    if old_dev_lock_count == 0 {
        snd_tscm_stream_lock_changed(tscm);
    }
    0
}

pub unsafe fn snd_tscm_stream_lock_release(tscm: *mut snd_tscm) {
    guard_spinlock_irq(&mut (*tscm).lock);

    if WARN_ON((*tscm).dev_lock_count <= 0) {
        return;
    }
    (*tscm).dev_lock_count -= 1;
    if (*tscm).dev_lock_count == 0 {
        snd_tscm_stream_lock_changed(tscm);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
