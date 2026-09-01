// SPDX-License-Identifier: GPL-2.0-only
/*
 * motu-stream.c - a part of driver for MOTU FireWire series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// Dependency intent from C source: #include "motu.h"

const READY_TIMEOUT_MS: u32 = 200;

const ISOC_COMM_CONTROL_OFFSET: u32 = 0x0b00;
const ISOC_COMM_CONTROL_MASK: u32 = 0xffff0000;
const CHANGE_RX_ISOC_COMM_STATE: u32 = 0x80000000;
const RX_ISOC_COMM_IS_ACTIVATED: u32 = 0x40000000;
const RX_ISOC_COMM_CHANNEL_MASK: u32 = 0x3f000000;
const RX_ISOC_COMM_CHANNEL_SHIFT: u32 = 24;
const CHANGE_TX_ISOC_COMM_STATE: u32 = 0x00800000;
const TX_ISOC_COMM_IS_ACTIVATED: u32 = 0x00400000;
const TX_ISOC_COMM_CHANNEL_MASK: u32 = 0x003f0000;
const TX_ISOC_COMM_CHANNEL_SHIFT: u32 = 16;

const PACKET_FORMAT_OFFSET: u32 = 0x0b10;
const TX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS: u32 = 0x00000080;
const RX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS: u32 = 0x00000040;
const TX_PACKET_TRANSMISSION_SPEED_MASK: u32 = 0x0000000f;

unsafe fn keep_resources(
    motu: *mut snd_motu,
    mut rate: core::ffi::c_uint,
    stream: *mut amdtp_stream,
) -> core::ffi::c_int {
    let resources: *mut fw_iso_resources;
    let packet_format: *mut snd_motu_packet_format;
    let mut midi_ports: core::ffi::c_uint = 0;
    let mut err: core::ffi::c_int;

    if stream == &mut (*motu).rx_stream {
        resources = &mut (*motu).rx_resources;
        packet_format = &mut (*motu).rx_packet_formats;

        if ((*(*motu).spec).flags & SND_MOTU_SPEC_RX_MIDI_2ND_Q) != 0
            || ((*(*motu).spec).flags & SND_MOTU_SPEC_RX_MIDI_3RD_Q) != 0
        {
            midi_ports = 1;
        }
    } else {
        resources = &mut (*motu).tx_resources;
        packet_format = &mut (*motu).tx_packet_formats;

        if ((*(*motu).spec).flags & SND_MOTU_SPEC_TX_MIDI_2ND_Q) != 0
            || ((*(*motu).spec).flags & SND_MOTU_SPEC_TX_MIDI_3RD_Q) != 0
        {
            midi_ports = 1;
        }
    }

    err = amdtp_motu_set_parameters(stream, rate, midi_ports, packet_format);
    if err < 0 {
        return err;
    }

    fw_iso_resources_allocate(
        resources,
        amdtp_stream_get_max_payload(stream),
        (*fw_parent_device((*motu).unit)).max_speed,
    )
}

unsafe fn begin_session(motu: *mut snd_motu) -> core::ffi::c_int {
    let mut reg: __be32 = core::mem::zeroed();
    let mut data: u32;
    let mut err: core::ffi::c_int;

    // Configure the unit to start isochronous communication.
    err = snd_motu_transaction_read(
        motu,
        ISOC_COMM_CONTROL_OFFSET,
        &mut reg as *mut __be32 as *mut core::ffi::c_void,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg) & !ISOC_COMM_CONTROL_MASK;

    data |= CHANGE_RX_ISOC_COMM_STATE
        | RX_ISOC_COMM_IS_ACTIVATED
        | ((*motu).rx_resources.channel << RX_ISOC_COMM_CHANNEL_SHIFT)
        | CHANGE_TX_ISOC_COMM_STATE
        | TX_ISOC_COMM_IS_ACTIVATED
        | ((*motu).tx_resources.channel << TX_ISOC_COMM_CHANNEL_SHIFT);

    reg = cpu_to_be32(data);
    snd_motu_transaction_write(
        motu,
        ISOC_COMM_CONTROL_OFFSET,
        &mut reg as *mut __be32 as *mut core::ffi::c_void,
        core::mem::size_of_val(&reg),
    )
}

unsafe fn finish_session(motu: *mut snd_motu) {
    let mut reg: __be32 = core::mem::zeroed();
    let mut data: u32;
    let mut err: core::ffi::c_int;

    err = snd_motu_protocol_switch_fetching_mode(motu, false);
    if err < 0 {
        return;
    }

    err = snd_motu_transaction_read(
        motu,
        ISOC_COMM_CONTROL_OFFSET,
        &mut reg as *mut __be32 as *mut core::ffi::c_void,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return;
    }
    data = be32_to_cpu(reg);

    data &= !(RX_ISOC_COMM_IS_ACTIVATED | TX_ISOC_COMM_IS_ACTIVATED);
    data |= CHANGE_RX_ISOC_COMM_STATE | CHANGE_TX_ISOC_COMM_STATE;

    reg = cpu_to_be32(data);
    snd_motu_transaction_write(
        motu,
        ISOC_COMM_CONTROL_OFFSET,
        &mut reg as *mut __be32 as *mut core::ffi::c_void,
        core::mem::size_of_val(&reg),
    );
}

pub unsafe extern "C" fn snd_motu_stream_cache_packet_formats(
    motu: *mut snd_motu,
) -> core::ffi::c_int {
    let mut err: core::ffi::c_int;

    err = snd_motu_protocol_cache_packet_formats(motu);
    if err < 0 {
        return err;
    }

    if ((*(*motu).spec).flags & SND_MOTU_SPEC_TX_MIDI_2ND_Q) != 0 {
        (*motu).tx_packet_formats.midi_flag_offset = 4;
        (*motu).tx_packet_formats.midi_byte_offset = 6;
    } else if ((*(*motu).spec).flags & SND_MOTU_SPEC_TX_MIDI_3RD_Q) != 0 {
        (*motu).tx_packet_formats.midi_flag_offset = 8;
        (*motu).tx_packet_formats.midi_byte_offset = 7;
    }

    if ((*(*motu).spec).flags & SND_MOTU_SPEC_RX_MIDI_2ND_Q) != 0 {
        (*motu).rx_packet_formats.midi_flag_offset = 4;
        (*motu).rx_packet_formats.midi_byte_offset = 6;
    } else if ((*(*motu).spec).flags & SND_MOTU_SPEC_RX_MIDI_3RD_Q) != 0 {
        (*motu).rx_packet_formats.midi_flag_offset = 8;
        (*motu).rx_packet_formats.midi_byte_offset = 7;
    }

    0
}

pub unsafe extern "C" fn snd_motu_stream_reserve_duplex(
    motu: *mut snd_motu,
    mut rate: core::ffi::c_uint,
    frames_per_period: core::ffi::c_uint,
    frames_per_buffer: core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut curr_rate: core::ffi::c_uint = 0;
    let mut err: core::ffi::c_int;

    err = snd_motu_protocol_get_clock_rate(motu, &mut curr_rate);
    if err < 0 {
        return err;
    }
    if rate == 0 {
        rate = curr_rate;
    }

    if (*motu).substreams_counter == 0 || curr_rate != rate {
        amdtp_domain_stop(&mut (*motu).domain);
        finish_session(motu);

        fw_iso_resources_free(&mut (*motu).tx_resources);
        fw_iso_resources_free(&mut (*motu).rx_resources);

        kfree((*motu).cache.event_offsets as *const core::ffi::c_void);
        (*motu).cache.event_offsets = core::ptr::null_mut();

        err = snd_motu_protocol_set_clock_rate(motu, rate);
        if err < 0 {
            dev_err(
                &mut (*(*motu).unit).device,
                c"fail to set sampling rate: %d\n".as_ptr(),
                err,
            );
            return err;
        }

        err = snd_motu_stream_cache_packet_formats(motu);
        if err < 0 {
            return err;
        }

        err = keep_resources(motu, rate, &mut (*motu).tx_stream);
        if err < 0 {
            return err;
        }

        err = keep_resources(motu, rate, &mut (*motu).rx_stream);
        if err < 0 {
            fw_iso_resources_free(&mut (*motu).tx_resources);
            return err;
        }

        err = amdtp_domain_set_events_per_period(
            &mut (*motu).domain,
            frames_per_period,
            frames_per_buffer,
        );
        if err < 0 {
            fw_iso_resources_free(&mut (*motu).tx_resources);
            fw_iso_resources_free(&mut (*motu).rx_resources);
            return err;
        }

        (*motu).cache.size = (*motu).tx_stream.syt_interval * frames_per_buffer;
        (*motu).cache.event_offsets = kcalloc(
            (*motu).cache.size as usize,
            core::mem::size_of_val(&*(*motu).cache.event_offsets),
            GFP_KERNEL,
        ) as *mut _;
        if (*motu).cache.event_offsets.is_null() {
            fw_iso_resources_free(&mut (*motu).tx_resources);
            fw_iso_resources_free(&mut (*motu).rx_resources);
            return -ENOMEM;
        }
    }

    0
}

unsafe fn ensure_packet_formats(motu: *mut snd_motu) -> core::ffi::c_int {
    let mut reg: __be32 = core::mem::zeroed();
    let mut data: u32;
    let mut err: core::ffi::c_int;

    err = snd_motu_transaction_read(
        motu,
        PACKET_FORMAT_OFFSET,
        &mut reg as *mut __be32 as *mut core::ffi::c_void,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg);

    data &= !(TX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS
        | RX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS
        | TX_PACKET_TRANSMISSION_SPEED_MASK);
    if (*(*motu).spec).tx_fixed_pcm_chunks[0] == (*motu).tx_packet_formats.pcm_chunks[0] {
        data |= TX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS;
    }
    if (*(*motu).spec).rx_fixed_pcm_chunks[0] == (*motu).rx_packet_formats.pcm_chunks[0] {
        data |= RX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS;
    }
    data |= (*fw_parent_device((*motu).unit)).max_speed;

    reg = cpu_to_be32(data);
    snd_motu_transaction_write(
        motu,
        PACKET_FORMAT_OFFSET,
        &mut reg as *mut __be32 as *mut core::ffi::c_void,
        core::mem::size_of_val(&reg),
    )
}

pub unsafe extern "C" fn snd_motu_stream_start_duplex(
    motu: *mut snd_motu,
) -> core::ffi::c_int {
    let generation: core::ffi::c_uint = (*motu).rx_resources.generation;
    let mut err: core::ffi::c_int = 0;

    if (*motu).substreams_counter == 0 {
        return 0;
    }

    if amdtp_streaming_error(&mut (*motu).rx_stream)
        || amdtp_streaming_error(&mut (*motu).tx_stream)
    {
        amdtp_domain_stop(&mut (*motu).domain);
        finish_session(motu);
    }

    if generation != (*(*fw_parent_device((*motu).unit)).card).generation {
        err = fw_iso_resources_update(&mut (*motu).rx_resources);
        if err < 0 {
            return err;
        }

        err = fw_iso_resources_update(&mut (*motu).tx_resources);
        if err < 0 {
            return err;
        }
    }

    if !amdtp_stream_running(&mut (*motu).rx_stream) {
        let spd: core::ffi::c_int = (*fw_parent_device((*motu).unit)).max_speed as core::ffi::c_int;

        err = ensure_packet_formats(motu);
        if err < 0 {
            return err;
        }

        if ((*(*motu).spec).flags & SND_MOTU_SPEC_REGISTER_DSP) != 0 {
            err = snd_motu_register_dsp_message_parser_init(motu);
            if err < 0 {
                return err;
            }
        } else if ((*(*motu).spec).flags & SND_MOTU_SPEC_COMMAND_DSP) != 0 {
            err = snd_motu_command_dsp_message_parser_init(motu, (*motu).tx_stream.sfc);
            if err < 0 {
                return err;
            }
        }

        err = begin_session(motu);
        if err < 0 {
            dev_err(
                &mut (*(*motu).unit).device,
                c"fail to start isochronous comm: %d\n".as_ptr(),
                err,
            );
            amdtp_domain_stop(&mut (*motu).domain);
            finish_session(motu);
            return err;
        }

        err = amdtp_domain_add_stream(
            &mut (*motu).domain,
            &mut (*motu).tx_stream,
            (*motu).tx_resources.channel,
            spd,
        );
        if err < 0 {
            amdtp_domain_stop(&mut (*motu).domain);
            finish_session(motu);
            return err;
        }

        err = amdtp_domain_add_stream(
            &mut (*motu).domain,
            &mut (*motu).rx_stream,
            (*motu).rx_resources.channel,
            spd,
        );
        if err < 0 {
            amdtp_domain_stop(&mut (*motu).domain);
            finish_session(motu);
            return err;
        }

        (*motu).cache.tail = 0;
        (*motu).cache.tx_cycle_count = UINT_MAX;
        (*motu).cache.head = 0;
        (*motu).cache.rx_cycle_count = UINT_MAX;

        // NOTE: The device requires both of replay; the sequence of the number of data
        // blocks per packet, and the sequence of source packet header per data block as
        // presentation time.
        err = amdtp_domain_start(&mut (*motu).domain, 0, true, false);
        if err < 0 {
            amdtp_domain_stop(&mut (*motu).domain);
            finish_session(motu);
            return err;
        }

        if !amdtp_domain_wait_ready(&mut (*motu).domain, READY_TIMEOUT_MS) {
            err = -ETIMEDOUT;
            amdtp_domain_stop(&mut (*motu).domain);
            finish_session(motu);
            return err;
        }

        err = snd_motu_protocol_switch_fetching_mode(motu, true);
        if err < 0 {
            dev_err(
                &mut (*(*motu).unit).device,
                c"fail to enable frame fetching: %d\n".as_ptr(),
                err,
            );
            amdtp_domain_stop(&mut (*motu).domain);
            finish_session(motu);
            return err;
        }
    }

    0
}

pub unsafe extern "C" fn snd_motu_stream_stop_duplex(motu: *mut snd_motu) {
    if (*motu).substreams_counter == 0 {
        amdtp_domain_stop(&mut (*motu).domain);
        finish_session(motu);

        fw_iso_resources_free(&mut (*motu).tx_resources);
        fw_iso_resources_free(&mut (*motu).rx_resources);

        kfree((*motu).cache.event_offsets as *const core::ffi::c_void);
        (*motu).cache.event_offsets = core::ptr::null_mut();
    }
}

unsafe fn init_stream(
    motu: *mut snd_motu,
    s: *mut amdtp_stream,
) -> core::ffi::c_int {
    let resources: *mut fw_iso_resources;
    let dir: amdtp_stream_direction;
    let mut err: core::ffi::c_int;

    if s == &mut (*motu).tx_stream {
        resources = &mut (*motu).tx_resources;
        dir = AMDTP_IN_STREAM;
    } else {
        resources = &mut (*motu).rx_resources;
        dir = AMDTP_OUT_STREAM;
    }

    err = fw_iso_resources_init(resources, (*motu).unit);
    if err < 0 {
        return err;
    }

    err = amdtp_motu_init(s, (*motu).unit, dir, (*motu).spec, &mut (*motu).cache);
    if err < 0 {
        fw_iso_resources_destroy(resources);
    }

    err
}

unsafe fn destroy_stream(motu: *mut snd_motu, s: *mut amdtp_stream) {
    amdtp_stream_destroy(s);

    if s == &mut (*motu).tx_stream {
        fw_iso_resources_destroy(&mut (*motu).tx_resources);
    } else {
        fw_iso_resources_destroy(&mut (*motu).rx_resources);
    }
}

pub unsafe extern "C" fn snd_motu_stream_init_duplex(
    motu: *mut snd_motu,
) -> core::ffi::c_int {
    let mut err: core::ffi::c_int;

    err = init_stream(motu, &mut (*motu).tx_stream);
    if err < 0 {
        return err;
    }

    err = init_stream(motu, &mut (*motu).rx_stream);
    if err < 0 {
        destroy_stream(motu, &mut (*motu).tx_stream);
        return err;
    }

    err = amdtp_domain_init(&mut (*motu).domain);
    if err < 0 {
        destroy_stream(motu, &mut (*motu).tx_stream);
        destroy_stream(motu, &mut (*motu).rx_stream);
    }

    err
}

// This function should be called before starting streams or after stopping
// streams.
pub unsafe extern "C" fn snd_motu_stream_destroy_duplex(motu: *mut snd_motu) {
    amdtp_domain_destroy(&mut (*motu).domain);

    destroy_stream(motu, &mut (*motu).rx_stream);
    destroy_stream(motu, &mut (*motu).tx_stream);

    (*motu).substreams_counter = 0;
}

unsafe fn motu_lock_changed(motu: *mut snd_motu) {
    (*motu).dev_lock_changed = true;
    wake_up(&mut (*motu).hwdep_wait);
}

pub unsafe extern "C" fn snd_motu_stream_lock_try(
    motu: *mut snd_motu,
) -> core::ffi::c_int {
    // C source uses guard(spinlock_irq)(&motu->lock), holding the lock for this scope.
    let _guard = guard_spinlock_irq(&mut (*motu).lock);

    if (*motu).dev_lock_count < 0 {
        return -EBUSY;
    }

    let old_count = (*motu).dev_lock_count;
    (*motu).dev_lock_count += 1;
    if old_count == 0 {
        motu_lock_changed(motu);
    }
    0
}

pub unsafe extern "C" fn snd_motu_stream_lock_release(motu: *mut snd_motu) {
    // C source uses guard(spinlock_irq)(&motu->lock), holding the lock for this scope.
    let _guard = guard_spinlock_irq(&mut (*motu).lock);

    if WARN_ON((*motu).dev_lock_count <= 0) {
        return;
    }

    (*motu).dev_lock_count -= 1;
    if (*motu).dev_lock_count == 0 {
        motu_lock_changed(motu);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
