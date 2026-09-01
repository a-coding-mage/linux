// SPDX-License-Identifier: GPL-2.0-only
/*
 * digi00x-stream.c - a part of driver for Digidesign Digi 002/003 family
 *
 * Copyright (c) 2014-2015 Takashi Sakamoto
 */

// Rust translation of implementation originally depending on "digi00x.h".

const READY_TIMEOUT_MS: c_uint = 200;

pub static snd_dg00x_stream_rates: [c_uint; SND_DG00X_RATE_COUNT as usize] = {
    let mut rates = [0 as c_uint; SND_DG00X_RATE_COUNT as usize];
    rates[SND_DG00X_RATE_44100 as usize] = 44100;
    rates[SND_DG00X_RATE_48000 as usize] = 48000;
    rates[SND_DG00X_RATE_88200 as usize] = 88200;
    rates[SND_DG00X_RATE_96000 as usize] = 96000;
    rates
};

/* Multi Bit Linear Audio data channels for each sampling transfer frequency. */
pub static snd_dg00x_stream_pcm_channels: [c_uint; SND_DG00X_RATE_COUNT as usize] = {
    let mut channels = [0 as c_uint; SND_DG00X_RATE_COUNT as usize];
    /* Analog/ADAT/SPDIF */
    channels[SND_DG00X_RATE_44100 as usize] = 8 + 8 + 2;
    channels[SND_DG00X_RATE_48000 as usize] = 8 + 8 + 2;
    /* Analog/SPDIF */
    channels[SND_DG00X_RATE_88200 as usize] = 8 + 2;
    channels[SND_DG00X_RATE_96000 as usize] = 8 + 2;
    channels
};

pub unsafe extern "C" fn snd_dg00x_stream_get_local_rate(
    dg00x: *mut snd_dg00x,
    rate: *mut c_uint,
) -> c_int {
    let mut data: u32;
    let mut reg: __be32 = unsafe { core::mem::zeroed() };
    let mut err: c_int;

    err = unsafe {
        snd_fw_transaction(
            (*dg00x).unit,
            TCODE_READ_QUADLET_REQUEST,
            DG00X_ADDR_BASE + DG00X_OFFSET_LOCAL_RATE,
            &mut reg as *mut __be32 as *mut c_void,
            core::mem::size_of_val(&reg),
            0,
        )
    };
    if err < 0 {
        return err;
    }

    data = unsafe { be32_to_cpu(reg) } & 0x0f;
    if (data as usize) < snd_dg00x_stream_rates.len() {
        unsafe {
            *rate = snd_dg00x_stream_rates[data as usize];
        }
    } else {
        err = -EIO;
    }

    err
}

pub unsafe extern "C" fn snd_dg00x_stream_set_local_rate(
    dg00x: *mut snd_dg00x,
    rate: c_uint,
) -> c_int {
    let mut reg: __be32;
    let mut i: c_uint;

    i = 0;
    while (i as usize) < snd_dg00x_stream_rates.len() {
        if rate == snd_dg00x_stream_rates[i as usize] {
            break;
        }
        i += 1;
    }
    if (i as usize) == snd_dg00x_stream_rates.len() {
        return -EINVAL;
    }

    reg = unsafe { cpu_to_be32(i) };
    unsafe {
        snd_fw_transaction(
            (*dg00x).unit,
            TCODE_WRITE_QUADLET_REQUEST,
            DG00X_ADDR_BASE + DG00X_OFFSET_LOCAL_RATE,
            &mut reg as *mut __be32 as *mut c_void,
            core::mem::size_of_val(&reg),
            0,
        )
    }
}

pub unsafe extern "C" fn snd_dg00x_stream_get_clock(
    dg00x: *mut snd_dg00x,
    clock: *mut snd_dg00x_clock,
) -> c_int {
    let mut reg: __be32 = unsafe { core::mem::zeroed() };
    let mut err: c_int;

    err = unsafe {
        snd_fw_transaction(
            (*dg00x).unit,
            TCODE_READ_QUADLET_REQUEST,
            DG00X_ADDR_BASE + DG00X_OFFSET_CLOCK_SOURCE,
            &mut reg as *mut __be32 as *mut c_void,
            core::mem::size_of_val(&reg),
            0,
        )
    };
    if err < 0 {
        return err;
    }

    unsafe {
        *clock = (be32_to_cpu(reg) & 0x0f) as snd_dg00x_clock;
        if *clock >= SND_DG00X_CLOCK_COUNT as snd_dg00x_clock {
            err = -EIO;
        }
    }

    err
}

pub unsafe extern "C" fn snd_dg00x_stream_check_external_clock(
    dg00x: *mut snd_dg00x,
    detect: *mut bool,
) -> c_int {
    let mut reg: __be32 = unsafe { core::mem::zeroed() };
    let err: c_int;

    err = unsafe {
        snd_fw_transaction(
            (*dg00x).unit,
            TCODE_READ_QUADLET_REQUEST,
            DG00X_ADDR_BASE + DG00X_OFFSET_DETECT_EXTERNAL,
            &mut reg as *mut __be32 as *mut c_void,
            core::mem::size_of_val(&reg),
            0,
        )
    };
    if err >= 0 {
        unsafe {
            *detect = be32_to_cpu(reg) > 0;
        }
    }

    err
}

pub unsafe extern "C" fn snd_dg00x_stream_get_external_rate(
    dg00x: *mut snd_dg00x,
    rate: *mut c_uint,
) -> c_int {
    let mut data: u32;
    let mut reg: __be32 = unsafe { core::mem::zeroed() };
    let mut err: c_int;

    err = unsafe {
        snd_fw_transaction(
            (*dg00x).unit,
            TCODE_READ_QUADLET_REQUEST,
            DG00X_ADDR_BASE + DG00X_OFFSET_EXTERNAL_RATE,
            &mut reg as *mut __be32 as *mut c_void,
            core::mem::size_of_val(&reg),
            0,
        )
    };
    if err < 0 {
        return err;
    }

    data = unsafe { be32_to_cpu(reg) } & 0x0f;
    if (data as usize) < snd_dg00x_stream_rates.len() {
        unsafe {
            *rate = snd_dg00x_stream_rates[data as usize];
        }
    } else {
        /* This means desync. */
        err = -EBUSY;
    }

    err
}

unsafe fn finish_session(dg00x: *mut snd_dg00x) {
    let mut data: __be32;

    data = unsafe { cpu_to_be32(0x00000003) };
    unsafe {
        snd_fw_transaction(
            (*dg00x).unit,
            TCODE_WRITE_QUADLET_REQUEST,
            DG00X_ADDR_BASE + DG00X_OFFSET_STREAMING_SET,
            &mut data as *mut __be32 as *mut c_void,
            core::mem::size_of_val(&data),
            0,
        );
    }

    // Unregister isochronous channels for both direction.
    data = 0;
    unsafe {
        snd_fw_transaction(
            (*dg00x).unit,
            TCODE_WRITE_QUADLET_REQUEST,
            DG00X_ADDR_BASE + DG00X_OFFSET_ISOC_CHANNELS,
            &mut data as *mut __be32 as *mut c_void,
            core::mem::size_of_val(&data),
            0,
        );
    }

    // Just after finishing the session, the device may lost transmitting
    // functionality for a short time.
    unsafe {
        msleep(50);
    }
}

unsafe fn begin_session(dg00x: *mut snd_dg00x) -> c_int {
    let mut data: __be32;
    let mut curr: u32;
    let mut err: c_int;

    // Register isochronous channels for both direction.
    data = unsafe { cpu_to_be32(((*dg00x).tx_resources.channel << 16) | (*dg00x).rx_resources.channel) };
    err = unsafe {
        snd_fw_transaction(
            (*dg00x).unit,
            TCODE_WRITE_QUADLET_REQUEST,
            DG00X_ADDR_BASE + DG00X_OFFSET_ISOC_CHANNELS,
            &mut data as *mut __be32 as *mut c_void,
            core::mem::size_of_val(&data),
            0,
        )
    };
    if err < 0 {
        return err;
    }

    err = unsafe {
        snd_fw_transaction(
            (*dg00x).unit,
            TCODE_READ_QUADLET_REQUEST,
            DG00X_ADDR_BASE + DG00X_OFFSET_STREAMING_STATE,
            &mut data as *mut __be32 as *mut c_void,
            core::mem::size_of_val(&data),
            0,
        )
    };
    if err < 0 {
        return err;
    }
    curr = unsafe { be32_to_cpu(data) };

    if curr == 0 {
        curr = 2;
    }

    curr -= 1;
    while curr > 0 {
        data = unsafe { cpu_to_be32(curr) };
        err = unsafe {
            snd_fw_transaction(
                (*dg00x).unit,
                TCODE_WRITE_QUADLET_REQUEST,
                DG00X_ADDR_BASE + DG00X_OFFSET_STREAMING_SET,
                &mut data as *mut __be32 as *mut c_void,
                core::mem::size_of_val(&data),
                0,
            )
        };
        if err < 0 {
            break;
        }

        unsafe {
            msleep(20);
        }
        curr -= 1;
    }

    err
}

unsafe fn keep_resources(
    dg00x: *mut snd_dg00x,
    stream: *mut amdtp_stream,
    rate: c_uint,
) -> c_int {
    let resources: *mut fw_iso_resources;
    let mut i: c_int;
    let mut err: c_int;

    // Check sampling rate.
    i = 0;
    while i < SND_DG00X_RATE_COUNT as c_int {
        if snd_dg00x_stream_rates[i as usize] == rate {
            break;
        }
        i += 1;
    }
    if i == SND_DG00X_RATE_COUNT as c_int {
        return -EINVAL;
    }

    if stream == unsafe { &mut (*dg00x).tx_stream as *mut amdtp_stream } {
        resources = unsafe { &mut (*dg00x).tx_resources as *mut fw_iso_resources };
    } else {
        resources = unsafe { &mut (*dg00x).rx_resources as *mut fw_iso_resources };
    }

    err = unsafe {
        amdtp_dot_set_parameters(
            stream,
            rate,
            snd_dg00x_stream_pcm_channels[i as usize],
        )
    };
    if err < 0 {
        return err;
    }

    unsafe {
        fw_iso_resources_allocate(
            resources,
            amdtp_stream_get_max_payload(stream),
            (*fw_parent_device((*dg00x).unit)).max_speed,
        )
    }
}

unsafe fn init_stream(dg00x: *mut snd_dg00x, s: *mut amdtp_stream) -> c_int {
    let resources: *mut fw_iso_resources;
    let dir: amdtp_stream_direction;
    let mut err: c_int;

    if s == unsafe { &mut (*dg00x).tx_stream as *mut amdtp_stream } {
        resources = unsafe { &mut (*dg00x).tx_resources as *mut fw_iso_resources };
        dir = AMDTP_IN_STREAM;
    } else {
        resources = unsafe { &mut (*dg00x).rx_resources as *mut fw_iso_resources };
        dir = AMDTP_OUT_STREAM;
    }

    err = unsafe { fw_iso_resources_init(resources, (*dg00x).unit) };
    if err < 0 {
        return err;
    }

    err = unsafe { amdtp_dot_init(s, (*dg00x).unit, dir) };
    if err < 0 {
        unsafe {
            fw_iso_resources_destroy(resources);
        }
    }

    err
}

unsafe fn destroy_stream(dg00x: *mut snd_dg00x, s: *mut amdtp_stream) {
    unsafe {
        amdtp_stream_destroy(s);
    }

    if s == unsafe { &mut (*dg00x).tx_stream as *mut amdtp_stream } {
        unsafe {
            fw_iso_resources_destroy(&mut (*dg00x).tx_resources);
        }
    } else {
        unsafe {
            fw_iso_resources_destroy(&mut (*dg00x).rx_resources);
        }
    }
}

pub unsafe extern "C" fn snd_dg00x_stream_init_duplex(dg00x: *mut snd_dg00x) -> c_int {
    let mut err: c_int;

    err = unsafe { init_stream(dg00x, &mut (*dg00x).rx_stream) };
    if err < 0 {
        return err;
    }

    err = unsafe { init_stream(dg00x, &mut (*dg00x).tx_stream) };
    if err < 0 {
        unsafe {
            destroy_stream(dg00x, &mut (*dg00x).rx_stream);
        }
        return err;
    }

    err = unsafe { amdtp_domain_init(&mut (*dg00x).domain) };
    if err < 0 {
        unsafe {
            destroy_stream(dg00x, &mut (*dg00x).rx_stream);
            destroy_stream(dg00x, &mut (*dg00x).tx_stream);
        }
    }

    err
}

/*
 * This function should be called before starting streams or after stopping
 * streams.
 */
pub unsafe extern "C" fn snd_dg00x_stream_destroy_duplex(dg00x: *mut snd_dg00x) {
    unsafe {
        amdtp_domain_destroy(&mut (*dg00x).domain);

        destroy_stream(dg00x, &mut (*dg00x).rx_stream);
        destroy_stream(dg00x, &mut (*dg00x).tx_stream);
    }
}

pub unsafe extern "C" fn snd_dg00x_stream_reserve_duplex(
    dg00x: *mut snd_dg00x,
    mut rate: c_uint,
    frames_per_period: c_uint,
    frames_per_buffer: c_uint,
) -> c_int {
    let mut curr_rate: c_uint = 0;
    let mut err: c_int;

    err = unsafe { snd_dg00x_stream_get_local_rate(dg00x, &mut curr_rate) };
    if err < 0 {
        return err;
    }
    if rate == 0 {
        rate = curr_rate;
    }

    if unsafe { (*dg00x).substreams_counter } == 0 || curr_rate != rate {
        unsafe {
            amdtp_domain_stop(&mut (*dg00x).domain);

            finish_session(dg00x);

            fw_iso_resources_free(&mut (*dg00x).tx_resources);
            fw_iso_resources_free(&mut (*dg00x).rx_resources);
        }

        err = unsafe { snd_dg00x_stream_set_local_rate(dg00x, rate) };
        if err < 0 {
            return err;
        }

        err = unsafe { keep_resources(dg00x, &mut (*dg00x).rx_stream, rate) };
        if err < 0 {
            return err;
        }

        err = unsafe { keep_resources(dg00x, &mut (*dg00x).tx_stream, rate) };
        if err < 0 {
            unsafe {
                fw_iso_resources_free(&mut (*dg00x).rx_resources);
            }
            return err;
        }

        err = unsafe {
            amdtp_domain_set_events_per_period(
                &mut (*dg00x).domain,
                frames_per_period,
                frames_per_buffer,
            )
        };
        if err < 0 {
            unsafe {
                fw_iso_resources_free(&mut (*dg00x).rx_resources);
                fw_iso_resources_free(&mut (*dg00x).tx_resources);
            }
            return err;
        }
    }

    0
}

pub unsafe extern "C" fn snd_dg00x_stream_start_duplex(dg00x: *mut snd_dg00x) -> c_int {
    let generation: c_uint = unsafe { (*dg00x).rx_resources.generation };
    let mut err: c_int = 0;

    if unsafe { (*dg00x).substreams_counter } == 0 {
        return 0;
    }

    if unsafe { amdtp_streaming_error(&mut (*dg00x).tx_stream) }
        || unsafe { amdtp_streaming_error(&mut (*dg00x).rx_stream) }
    {
        unsafe {
            amdtp_domain_stop(&mut (*dg00x).domain);
            finish_session(dg00x);
        }
    }

    if generation != unsafe { (*(*fw_parent_device((*dg00x).unit)).card).generation } {
        err = unsafe { fw_iso_resources_update(&mut (*dg00x).tx_resources) };
        if err < 0 {
            unsafe {
                amdtp_domain_stop(&mut (*dg00x).domain);
                finish_session(dg00x);
            }
            return err;
        }

        err = unsafe { fw_iso_resources_update(&mut (*dg00x).rx_resources) };
        if err < 0 {
            unsafe {
                amdtp_domain_stop(&mut (*dg00x).domain);
                finish_session(dg00x);
            }
            return err;
        }
    }

    /*
     * No packets are transmitted without receiving packets, reagardless of
     * which source of clock is used.
     */
    if !unsafe { amdtp_stream_running(&mut (*dg00x).rx_stream) } {
        let spd: c_int = unsafe { (*fw_parent_device((*dg00x).unit)).max_speed };

        err = unsafe { begin_session(dg00x) };
        if err < 0 {
            unsafe {
                amdtp_domain_stop(&mut (*dg00x).domain);
                finish_session(dg00x);
            }
            return err;
        }

        err = unsafe {
            amdtp_domain_add_stream(
                &mut (*dg00x).domain,
                &mut (*dg00x).rx_stream,
                (*dg00x).rx_resources.channel,
                spd,
            )
        };
        if err < 0 {
            unsafe {
                amdtp_domain_stop(&mut (*dg00x).domain);
                finish_session(dg00x);
            }
            return err;
        }

        err = unsafe {
            amdtp_domain_add_stream(
                &mut (*dg00x).domain,
                &mut (*dg00x).tx_stream,
                (*dg00x).tx_resources.channel,
                spd,
            )
        };
        if err < 0 {
            unsafe {
                amdtp_domain_stop(&mut (*dg00x).domain);
                finish_session(dg00x);
            }
            return err;
        }

        // NOTE: The device doesn't start packet transmission till receiving any packet.
        // It ignores presentation time expressed by the value of syt field of CIP header
        // in received packets. The sequence of the number of data blocks per packet is
        // important for media clock recovery.
        err = unsafe { amdtp_domain_start(&mut (*dg00x).domain, 0, true, true) };
        if err < 0 {
            unsafe {
                amdtp_domain_stop(&mut (*dg00x).domain);
                finish_session(dg00x);
            }
            return err;
        }

        if !unsafe { amdtp_domain_wait_ready(&mut (*dg00x).domain, READY_TIMEOUT_MS) } {
            err = -ETIMEDOUT;
            unsafe {
                amdtp_domain_stop(&mut (*dg00x).domain);
                finish_session(dg00x);
            }
            return err;
        }
    }

    0
}

pub unsafe extern "C" fn snd_dg00x_stream_stop_duplex(dg00x: *mut snd_dg00x) {
    if unsafe { (*dg00x).substreams_counter } == 0 {
        unsafe {
            amdtp_domain_stop(&mut (*dg00x).domain);
            finish_session(dg00x);

            fw_iso_resources_free(&mut (*dg00x).tx_resources);
            fw_iso_resources_free(&mut (*dg00x).rx_resources);
        }
    }
}

pub unsafe extern "C" fn snd_dg00x_stream_update_duplex(dg00x: *mut snd_dg00x) {
    unsafe {
        fw_iso_resources_update(&mut (*dg00x).tx_resources);
        fw_iso_resources_update(&mut (*dg00x).rx_resources);

        amdtp_stream_update(&mut (*dg00x).tx_stream);
        amdtp_stream_update(&mut (*dg00x).rx_stream);
    }
}

pub unsafe extern "C" fn snd_dg00x_stream_lock_changed(dg00x: *mut snd_dg00x) {
    unsafe {
        (*dg00x).dev_lock_changed = true;
        wake_up(&mut (*dg00x).hwdep_wait);
    }
}

pub unsafe extern "C" fn snd_dg00x_stream_lock_try(dg00x: *mut snd_dg00x) -> c_int {
    // C source uses: guard(spinlock_irq)(&dg00x->lock);
    let _guard = unsafe { guard_spinlock_irq(&mut (*dg00x).lock) };

    /* user land lock this */
    if unsafe { (*dg00x).dev_lock_count } < 0 {
        return -EBUSY;
    }

    /* this is the first time */
    unsafe {
        if (*dg00x).dev_lock_count == 0 {
            (*dg00x).dev_lock_count += 1;
            snd_dg00x_stream_lock_changed(dg00x);
        } else {
            (*dg00x).dev_lock_count += 1;
        }
    }
    0
}

pub unsafe extern "C" fn snd_dg00x_stream_lock_release(dg00x: *mut snd_dg00x) {
    // C source uses: guard(spinlock_irq)(&dg00x->lock);
    let _guard = unsafe { guard_spinlock_irq(&mut (*dg00x).lock) };

    if unsafe { WARN_ON((*dg00x).dev_lock_count <= 0) } {
        return;
    }
    unsafe {
        (*dg00x).dev_lock_count -= 1;
        if (*dg00x).dev_lock_count == 0 {
            snd_dg00x_stream_lock_changed(dg00x);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
