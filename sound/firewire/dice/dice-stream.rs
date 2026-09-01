// SPDX-License-Identifier: GPL-2.0-only
/*
 * dice_stream.c - a part of driver for DICE based devices
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 * Copyright (c) 2014 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// Dependency intent: translated from `#include "dice.h"`.
use crate::dice::*;

pub const READY_TIMEOUT_MS: c_uint = 200;
pub const NOTIFICATION_TIMEOUT_MS: c_uint = 100;

#[repr(C)]
pub struct reg_params {
    pub count: c_uint,
    pub size: c_uint,
}

pub static snd_dice_rates: [c_uint; SND_DICE_RATES_COUNT as usize] = [
    /* mode 0 */
    32000,
    44100,
    48000,
    /* mode 1 */
    88200,
    96000,
    /* mode 2 */
    176400,
    192000,
];

pub unsafe extern "C" fn snd_dice_stream_get_rate_mode(
    dice: *mut snd_dice,
    rate: c_uint,
    mode: *mut snd_dice_rate_mode,
) -> c_int {
    /* Corresponding to each entry in snd_dice_rates. */
    static modes: [snd_dice_rate_mode; SND_DICE_RATES_COUNT as usize] = [
        SND_DICE_RATE_MODE_LOW,
        SND_DICE_RATE_MODE_LOW,
        SND_DICE_RATE_MODE_LOW,
        SND_DICE_RATE_MODE_MIDDLE,
        SND_DICE_RATE_MODE_MIDDLE,
        SND_DICE_RATE_MODE_HIGH,
        SND_DICE_RATE_MODE_HIGH,
    ];
    let mut i: c_int;

    i = 0;
    while (i as usize) < snd_dice_rates.len() {
        if ((*dice).clock_caps & BIT(i as c_uint)) == 0 {
            i += 1;
            continue;
        }
        if snd_dice_rates[i as usize] != rate {
            i += 1;
            continue;
        }

        *mode = modes[i as usize];
        return 0;
    }

    -EINVAL
}

unsafe fn select_clock(dice: *mut snd_dice, rate: c_uint) -> c_int {
    let mut reg: __be32 = 0;
    let mut new: __be32;
    let mut data: u32;
    let mut i: c_int;
    let mut err: c_int;

    err = snd_dice_transaction_read_global(
        dice,
        GLOBAL_CLOCK_SELECT,
        &mut reg as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }

    data = be32_to_cpu(reg);

    data &= !CLOCK_RATE_MASK;
    i = 0;
    while (i as usize) < snd_dice_rates.len() {
        if snd_dice_rates[i as usize] == rate {
            break;
        }
        i += 1;
    }
    if (i as usize) == snd_dice_rates.len() {
        return -EINVAL;
    }
    data |= (i as u32) << CLOCK_RATE_SHIFT;

    if completion_done(&mut (*dice).clock_accepted) {
        reinit_completion(&mut (*dice).clock_accepted);
    }

    new = cpu_to_be32(data);
    err = snd_dice_transaction_write_global(
        dice,
        GLOBAL_CLOCK_SELECT,
        &mut new as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&new),
    );
    if err < 0 {
        return err;
    }

    if wait_for_completion_timeout(
        &mut (*dice).clock_accepted,
        msecs_to_jiffies(NOTIFICATION_TIMEOUT_MS),
    ) == 0
    {
        if reg != new {
            return -ETIMEDOUT;
        }
    }

    0
}

unsafe fn get_register_params(
    dice: *mut snd_dice,
    tx_params: *mut reg_params,
    rx_params: *mut reg_params,
) -> c_int {
    let mut reg: [__be32; 2] = [0; 2];
    let mut err: c_int;

    err = snd_dice_transaction_read_tx(
        dice,
        TX_NUMBER,
        reg.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    (*tx_params).count = core::cmp::min(be32_to_cpu(reg[0]) as c_uint, MAX_STREAMS as c_uint);
    (*tx_params).size = be32_to_cpu(reg[1]) as c_uint * 4;

    err = snd_dice_transaction_read_rx(
        dice,
        RX_NUMBER,
        reg.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    (*rx_params).count = core::cmp::min(be32_to_cpu(reg[0]) as c_uint, MAX_STREAMS as c_uint);
    (*rx_params).size = be32_to_cpu(reg[1]) as c_uint * 4;

    0
}

unsafe fn release_resources(dice: *mut snd_dice) {
    let mut i: c_int = 0;

    while i < MAX_STREAMS as c_int {
        fw_iso_resources_free(&mut (*dice).tx_resources[i as usize]);
        fw_iso_resources_free(&mut (*dice).rx_resources[i as usize]);
        i += 1;
    }
}

unsafe fn stop_streams(
    dice: *mut snd_dice,
    dir: amdtp_stream_direction,
    params: *mut reg_params,
) {
    let mut reg: __be32;
    let mut i: c_uint;

    i = 0;
    while i < (*params).count {
        reg = cpu_to_be32((-1i32) as u32);
        if dir == AMDTP_IN_STREAM {
            snd_dice_transaction_write_tx(
                dice,
                (*params).size * i + TX_ISOCHRONOUS,
                &mut reg as *mut __be32 as *mut c_void,
                core::mem::size_of_val(&reg),
            );
        } else {
            snd_dice_transaction_write_rx(
                dice,
                (*params).size * i + RX_ISOCHRONOUS,
                &mut reg as *mut __be32 as *mut c_void,
                core::mem::size_of_val(&reg),
            );
        }
        i += 1;
    }
}

unsafe fn keep_resources(
    dice: *mut snd_dice,
    stream: *mut amdtp_stream,
    resources: *mut fw_iso_resources,
    mut rate: c_uint,
    mut pcm_chs: c_uint,
    midi_ports: c_uint,
) -> c_int {
    let double_pcm_frames: bool;
    let mut i: c_uint;
    let mut err: c_int;

    // At 176.4/192.0 kHz, Dice has a quirk to transfer two PCM frames in
    // one data block of AMDTP packet. Thus sampling transfer frequency is
    // a half of PCM sampling frequency, i.e. PCM frames at 192.0 kHz are
    // transferred on AMDTP packets at 96 kHz. Two successive samples of a
    // channel are stored consecutively in the packet. This quirk is called
    // as 'Dual Wire'.
    // For this quirk, blocking mode is required and PCM buffer size should
    // be aligned to SYT_INTERVAL.
    double_pcm_frames = rate > 96000 && !(*dice).disable_double_pcm_frames;
    if double_pcm_frames {
        rate /= 2;
        pcm_chs *= 2;
    }

    err = amdtp_am824_set_parameters(stream, rate, pcm_chs, midi_ports, double_pcm_frames);
    if err < 0 {
        return err;
    }

    if double_pcm_frames {
        pcm_chs /= 2;

        i = 0;
        while i < pcm_chs {
            amdtp_am824_set_pcm_position(stream, i, i * 2);
            amdtp_am824_set_pcm_position(stream, i + pcm_chs, i * 2 + 1);
            i += 1;
        }
    }

    fw_iso_resources_allocate(
        resources,
        amdtp_stream_get_max_payload(stream),
        (*fw_parent_device((*dice).unit)).max_speed,
    )
}

unsafe fn keep_dual_resources(
    dice: *mut snd_dice,
    rate: c_uint,
    dir: amdtp_stream_direction,
    params: *mut reg_params,
) -> c_int {
    let mut mode: snd_dice_rate_mode = core::mem::zeroed();
    let mut i: c_int;
    let mut err: c_int;

    err = snd_dice_stream_get_rate_mode(dice, rate, &mut mode);
    if err < 0 {
        return err;
    }

    i = 0;
    while i < (*params).count as c_int {
        let mut reg: [__be32; 2] = [0; 2];
        let stream: *mut amdtp_stream;
        let resources: *mut fw_iso_resources;
        let pcm_cache: c_uint;
        let pcm_chs: c_uint;
        let midi_ports: c_uint;

        if dir == AMDTP_IN_STREAM {
            stream = &mut (*dice).tx_stream[i as usize];
            resources = &mut (*dice).tx_resources[i as usize];

            pcm_cache = (*dice).tx_pcm_chs[i as usize][mode as usize];
            err = snd_dice_transaction_read_tx(
                dice,
                (*params).size * i as c_uint + TX_NUMBER_AUDIO,
                reg.as_mut_ptr() as *mut c_void,
                core::mem::size_of_val(&reg),
            );
        } else {
            stream = &mut (*dice).rx_stream[i as usize];
            resources = &mut (*dice).rx_resources[i as usize];

            pcm_cache = (*dice).rx_pcm_chs[i as usize][mode as usize];
            err = snd_dice_transaction_read_rx(
                dice,
                (*params).size * i as c_uint + RX_NUMBER_AUDIO,
                reg.as_mut_ptr() as *mut c_void,
                core::mem::size_of_val(&reg),
            );
        }
        if err < 0 {
            return err;
        }
        pcm_chs = be32_to_cpu(reg[0]) as c_uint;
        midi_ports = be32_to_cpu(reg[1]) as c_uint;

        // These are important for developer of this driver.
        if pcm_chs != pcm_cache {
            dev_info(
                &mut (*(*dice).unit).device,
                c_str!("cache mismatch: pcm: %u:%u, midi: %u\n"),
                pcm_chs,
                pcm_cache,
                midi_ports,
            );
            return -EPROTO;
        }

        err = keep_resources(dice, stream, resources, rate, pcm_chs, midi_ports);
        if err < 0 {
            return err;
        }
        i += 1;
    }

    0
}

unsafe fn finish_session(
    dice: *mut snd_dice,
    tx_params: *mut reg_params,
    rx_params: *mut reg_params,
) {
    stop_streams(dice, AMDTP_IN_STREAM, tx_params);
    stop_streams(dice, AMDTP_OUT_STREAM, rx_params);

    snd_dice_transaction_clear_enable(dice);
}

pub unsafe extern "C" fn snd_dice_stream_reserve_duplex(
    dice: *mut snd_dice,
    mut rate: c_uint,
    events_per_period: c_uint,
    events_per_buffer: c_uint,
) -> c_int {
    let mut curr_rate: c_uint = 0;
    let mut err: c_int;

    // Check sampling transmission frequency.
    err = snd_dice_transaction_get_rate(dice, &mut curr_rate);
    if err < 0 {
        return err;
    }
    if rate == 0 {
        rate = curr_rate;
    }

    if (*dice).substreams_counter == 0 || curr_rate != rate {
        let mut tx_params = reg_params { count: 0, size: 0 };
        let mut rx_params = reg_params { count: 0, size: 0 };

        amdtp_domain_stop(&mut (*dice).domain);

        err = get_register_params(dice, &mut tx_params, &mut rx_params);
        if err < 0 {
            return err;
        }
        finish_session(dice, &mut tx_params, &mut rx_params);

        release_resources(dice);

        // Just after owning the unit (GLOBAL_OWNER), the unit can
        // return invalid stream formats. Selecting clock parameters
        // have an effect for the unit to refine it.
        err = select_clock(dice, rate);
        if err < 0 {
            return err;
        }

        // After changing sampling transfer frequency, the value of
        // register can be changed.
        err = get_register_params(dice, &mut tx_params, &mut rx_params);
        if err < 0 {
            return err;
        }

        err = keep_dual_resources(dice, rate, AMDTP_IN_STREAM, &mut tx_params);
        if err < 0 {
            release_resources(dice);
            return err;
        }

        err = keep_dual_resources(dice, rate, AMDTP_OUT_STREAM, &mut rx_params);
        if err < 0 {
            release_resources(dice);
            return err;
        }

        err = amdtp_domain_set_events_per_period(
            &mut (*dice).domain,
            events_per_period,
            events_per_buffer,
        );
        if err < 0 {
            release_resources(dice);
            return err;
        }
    }

    0
}

unsafe fn start_streams(
    dice: *mut snd_dice,
    dir: amdtp_stream_direction,
    rate: c_uint,
    params: *mut reg_params,
) -> c_int {
    let max_speed: c_uint = (*fw_parent_device((*dice).unit)).max_speed;
    let mut i: c_int;
    let mut err: c_int;

    i = 0;
    while i < (*params).count as c_int {
        let stream: *mut amdtp_stream;
        let resources: *mut fw_iso_resources;
        let mut reg: __be32;

        if dir == AMDTP_IN_STREAM {
            stream = (*dice).tx_stream.as_mut_ptr().add(i as usize);
            resources = (*dice).tx_resources.as_mut_ptr().add(i as usize);
        } else {
            stream = (*dice).rx_stream.as_mut_ptr().add(i as usize);
            resources = (*dice).rx_resources.as_mut_ptr().add(i as usize);
        }

        reg = cpu_to_be32((*resources).channel);
        if dir == AMDTP_IN_STREAM {
            err = snd_dice_transaction_write_tx(
                dice,
                (*params).size * i as c_uint + TX_ISOCHRONOUS,
                &mut reg as *mut __be32 as *mut c_void,
                core::mem::size_of_val(&reg),
            );
        } else {
            err = snd_dice_transaction_write_rx(
                dice,
                (*params).size * i as c_uint + RX_ISOCHRONOUS,
                &mut reg as *mut __be32 as *mut c_void,
                core::mem::size_of_val(&reg),
            );
        }
        if err < 0 {
            return err;
        }

        if dir == AMDTP_IN_STREAM {
            reg = cpu_to_be32(max_speed);
            err = snd_dice_transaction_write_tx(
                dice,
                (*params).size * i as c_uint + TX_SPEED,
                &mut reg as *mut __be32 as *mut c_void,
                core::mem::size_of_val(&reg),
            );
            if err < 0 {
                return err;
            }
        }

        err = amdtp_domain_add_stream(&mut (*dice).domain, stream, (*resources).channel, max_speed);
        if err < 0 {
            return err;
        }
        i += 1;
    }

    0
}

/*
 * MEMO: After this function, there're two states of streams:
 *  - None streams are running.
 *  - All streams are running.
 */
pub unsafe extern "C" fn snd_dice_stream_start_duplex(dice: *mut snd_dice) -> c_int {
    let generation: c_uint = (*dice).rx_resources[0].generation;
    let mut tx_params = reg_params { count: 0, size: 0 };
    let mut rx_params = reg_params { count: 0, size: 0 };
    let mut i: c_uint;
    let mut rate: c_uint = 0;
    let mut mode: snd_dice_rate_mode = core::mem::zeroed();
    let mut err: c_int;

    if (*dice).substreams_counter == 0 {
        return -EIO;
    }

    err = get_register_params(dice, &mut tx_params, &mut rx_params);
    if err < 0 {
        return err;
    }

    // Check error of packet streaming.
    i = 0;
    while i < MAX_STREAMS as c_uint {
        if amdtp_streaming_error(&mut (*dice).tx_stream[i as usize])
            || amdtp_streaming_error(&mut (*dice).rx_stream[i as usize])
        {
            amdtp_domain_stop(&mut (*dice).domain);
            finish_session(dice, &mut tx_params, &mut rx_params);
            break;
        }
        i += 1;
    }

    if generation != (*(*fw_parent_device((*dice).unit)).card).generation {
        i = 0;
        while i < MAX_STREAMS as c_uint {
            if i < tx_params.count {
                fw_iso_resources_update((*dice).tx_resources.as_mut_ptr().add(i as usize));
            }
            if i < rx_params.count {
                fw_iso_resources_update((*dice).rx_resources.as_mut_ptr().add(i as usize));
            }
            i += 1;
        }
    }

    // Check required streams are running or not.
    err = snd_dice_transaction_get_rate(dice, &mut rate);
    if err < 0 {
        return err;
    }
    err = snd_dice_stream_get_rate_mode(dice, rate, &mut mode);
    if err < 0 {
        return err;
    }
    i = 0;
    while i < MAX_STREAMS as c_uint {
        if (*dice).tx_pcm_chs[i as usize][mode as usize] > 0
            && !amdtp_stream_running(&mut (*dice).tx_stream[i as usize])
        {
            break;
        }
        if (*dice).rx_pcm_chs[i as usize][mode as usize] > 0
            && !amdtp_stream_running(&mut (*dice).rx_stream[i as usize])
        {
            break;
        }
        i += 1;
    }
    if i < MAX_STREAMS as c_uint {
        // Start both streams.
        err = start_streams(dice, AMDTP_IN_STREAM, rate, &mut tx_params);
        if err < 0 {
            amdtp_domain_stop(&mut (*dice).domain);
            finish_session(dice, &mut tx_params, &mut rx_params);
            return err;
        }

        err = start_streams(dice, AMDTP_OUT_STREAM, rate, &mut rx_params);
        if err < 0 {
            amdtp_domain_stop(&mut (*dice).domain);
            finish_session(dice, &mut tx_params, &mut rx_params);
            return err;
        }

        err = snd_dice_transaction_set_enable(dice);
        if err < 0 {
            dev_err(&mut (*(*dice).unit).device, c_str!("fail to enable interface\n"));
            amdtp_domain_stop(&mut (*dice).domain);
            finish_session(dice, &mut tx_params, &mut rx_params);
            return err;
        }

        // MEMO: The device immediately starts packet transmission when enabled. Some
        // devices are strictly to generate any discontinuity in the sequence of tx packet
        // when they receives invalid sequence of presentation time in CIP header. The
        // sequence replay for media clock recovery can suppress the behaviour.
        err = amdtp_domain_start(&mut (*dice).domain, 0, true, false);
        if err < 0 {
            amdtp_domain_stop(&mut (*dice).domain);
            finish_session(dice, &mut tx_params, &mut rx_params);
            return err;
        }

        if !amdtp_domain_wait_ready(&mut (*dice).domain, READY_TIMEOUT_MS) {
            err = -ETIMEDOUT;
            amdtp_domain_stop(&mut (*dice).domain);
            finish_session(dice, &mut tx_params, &mut rx_params);
            return err;
        }
    }

    0
}

/*
 * MEMO: After this function, there're two states of streams:
 *  - None streams are running.
 *  - All streams are running.
 */
pub unsafe extern "C" fn snd_dice_stream_stop_duplex(dice: *mut snd_dice) {
    let mut tx_params = reg_params { count: 0, size: 0 };
    let mut rx_params = reg_params { count: 0, size: 0 };

    if (*dice).substreams_counter == 0 {
        if get_register_params(dice, &mut tx_params, &mut rx_params) >= 0 {
            finish_session(dice, &mut tx_params, &mut rx_params);
        }

        amdtp_domain_stop(&mut (*dice).domain);
        release_resources(dice);
    }
}

unsafe fn init_stream(
    dice: *mut snd_dice,
    dir: amdtp_stream_direction,
    index: c_uint,
) -> c_int {
    let stream: *mut amdtp_stream;
    let resources: *mut fw_iso_resources;
    let mut err: c_int;

    if dir == AMDTP_IN_STREAM {
        stream = &mut (*dice).tx_stream[index as usize];
        resources = &mut (*dice).tx_resources[index as usize];
    } else {
        stream = &mut (*dice).rx_stream[index as usize];
        resources = &mut (*dice).rx_resources[index as usize];
    }

    err = fw_iso_resources_init(resources, (*dice).unit);
    if err < 0 {
        return err;
    }
    (*resources).channels_mask = 0x00000000ffffffffu64;

    err = amdtp_am824_init(stream, (*dice).unit, dir, CIP_BLOCKING);
    if err < 0 {
        amdtp_stream_destroy(stream);
        fw_iso_resources_destroy(resources);
    }
    err
}

/*
 * This function should be called before starting streams or after stopping
 * streams.
 */
unsafe fn destroy_stream(
    dice: *mut snd_dice,
    dir: amdtp_stream_direction,
    index: c_uint,
) {
    let stream: *mut amdtp_stream;
    let resources: *mut fw_iso_resources;

    if dir == AMDTP_IN_STREAM {
        stream = &mut (*dice).tx_stream[index as usize];
        resources = &mut (*dice).tx_resources[index as usize];
    } else {
        stream = &mut (*dice).rx_stream[index as usize];
        resources = &mut (*dice).rx_resources[index as usize];
    }

    amdtp_stream_destroy(stream);
    fw_iso_resources_destroy(resources);
}

pub unsafe extern "C" fn snd_dice_stream_init_duplex(dice: *mut snd_dice) -> c_int {
    let mut i: c_int;
    let mut err: c_int = 0;

    i = 0;
    while i < MAX_STREAMS as c_int {
        err = init_stream(dice, AMDTP_IN_STREAM, i as c_uint);
        if err < 0 {
            while i >= 0 {
                destroy_stream(dice, AMDTP_IN_STREAM, i as c_uint);
                i -= 1;
            }
            return err;
        }
        i += 1;
    }

    i = 0;
    while i < MAX_STREAMS as c_int {
        err = init_stream(dice, AMDTP_OUT_STREAM, i as c_uint);
        if err < 0 {
            while i >= 0 {
                destroy_stream(dice, AMDTP_OUT_STREAM, i as c_uint);
                i -= 1;
            }
            i = 0;
            while i < MAX_STREAMS as c_int {
                destroy_stream(dice, AMDTP_IN_STREAM, i as c_uint);
                i += 1;
            }
            return err;
        }
        i += 1;
    }

    err = amdtp_domain_init(&mut (*dice).domain);
    if err < 0 {
        i = 0;
        while i < MAX_STREAMS as c_int {
            destroy_stream(dice, AMDTP_OUT_STREAM, i as c_uint);
            destroy_stream(dice, AMDTP_IN_STREAM, i as c_uint);
            i += 1;
        }
    }
    err
}

pub unsafe extern "C" fn snd_dice_stream_destroy_duplex(dice: *mut snd_dice) {
    let mut i: c_uint;

    i = 0;
    while i < MAX_STREAMS as c_uint {
        destroy_stream(dice, AMDTP_IN_STREAM, i);
        destroy_stream(dice, AMDTP_OUT_STREAM, i);
        i += 1;
    }

    amdtp_domain_destroy(&mut (*dice).domain);
}

pub unsafe extern "C" fn snd_dice_stream_update_duplex(dice: *mut snd_dice) {
    let mut tx_params = reg_params { count: 0, size: 0 };
    let mut rx_params = reg_params { count: 0, size: 0 };

    /*
     * On a bus reset, the DICE firmware disables streaming and then goes
     * off contemplating its own navel for hundreds of milliseconds before
     * it can react to any of our attempts to reenable streaming.  This
     * means that we lose synchronization anyway, so we force our streams
     * to stop so that the application can restart them in an orderly
     * manner.
     */
    (*dice).global_enabled = false;

    if get_register_params(dice, &mut tx_params, &mut rx_params) == 0 {
        amdtp_domain_stop(&mut (*dice).domain);

        stop_streams(dice, AMDTP_IN_STREAM, &mut tx_params);
        stop_streams(dice, AMDTP_OUT_STREAM, &mut rx_params);
    }
}

pub unsafe extern "C" fn snd_dice_stream_detect_current_formats(
    dice: *mut snd_dice,
) -> c_int {
    let mut rate: c_uint = 0;
    let mut mode: snd_dice_rate_mode = core::mem::zeroed();
    let mut reg: [__be32; 2] = [0; 2];
    let mut tx_params = reg_params { count: 0, size: 0 };
    let mut rx_params = reg_params { count: 0, size: 0 };
    let mut i: c_int;
    let mut err: c_int;

    /* If extended protocol is available, detect detail spec. */
    err = snd_dice_detect_extension_formats(dice);
    if err >= 0 {
        return err;
    }

    /*
     * Available stream format is restricted at current mode of sampling
     * clock.
     */
    err = snd_dice_transaction_get_rate(dice, &mut rate);
    if err < 0 {
        return err;
    }

    err = snd_dice_stream_get_rate_mode(dice, rate, &mut mode);
    if err < 0 {
        return err;
    }

    /*
     * Just after owning the unit (GLOBAL_OWNER), the unit can return
     * invalid stream formats. Selecting clock parameters have an effect
     * for the unit to refine it.
     */
    err = select_clock(dice, rate);
    if err < 0 {
        return err;
    }

    err = get_register_params(dice, &mut tx_params, &mut rx_params);
    if err < 0 {
        return err;
    }

    i = 0;
    while i < tx_params.count as c_int {
        err = snd_dice_transaction_read_tx(
            dice,
            tx_params.size * i as c_uint + TX_NUMBER_AUDIO,
            reg.as_mut_ptr() as *mut c_void,
            core::mem::size_of_val(&reg),
        );
        if err < 0 {
            return err;
        }
        (*dice).tx_pcm_chs[i as usize][mode as usize] = be32_to_cpu(reg[0]) as c_uint;
        (*dice).tx_midi_ports[i as usize] = core::cmp::max(
            be32_to_cpu(reg[1]) as c_uint,
            (*dice).tx_midi_ports[i as usize],
        );
        i += 1;
    }
    i = 0;
    while i < rx_params.count as c_int {
        err = snd_dice_transaction_read_rx(
            dice,
            rx_params.size * i as c_uint + RX_NUMBER_AUDIO,
            reg.as_mut_ptr() as *mut c_void,
            core::mem::size_of_val(&reg),
        );
        if err < 0 {
            return err;
        }
        (*dice).rx_pcm_chs[i as usize][mode as usize] = be32_to_cpu(reg[0]) as c_uint;
        (*dice).rx_midi_ports[i as usize] = core::cmp::max(
            be32_to_cpu(reg[1]) as c_uint,
            (*dice).rx_midi_ports[i as usize],
        );
        i += 1;
    }

    0
}

unsafe fn dice_lock_changed(dice: *mut snd_dice) {
    (*dice).dev_lock_changed = true;
    wake_up(&mut (*dice).hwdep_wait);
}

pub unsafe extern "C" fn snd_dice_stream_lock_try(dice: *mut snd_dice) -> c_int {
    let _guard = guard_spinlock_irq(&mut (*dice).lock);

    if (*dice).dev_lock_count < 0 {
        return -EBUSY;
    }

    let old = (*dice).dev_lock_count;
    (*dice).dev_lock_count += 1;
    if old == 0 {
        dice_lock_changed(dice);
    }
    0
}

pub unsafe extern "C" fn snd_dice_stream_lock_release(dice: *mut snd_dice) {
    let _guard = guard_spinlock_irq(&mut (*dice).lock);

    if WARN_ON((*dice).dev_lock_count <= 0) {
        return;
    }

    (*dice).dev_lock_count -= 1;
    if (*dice).dev_lock_count == 0 {
        dice_lock_changed(dice);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
