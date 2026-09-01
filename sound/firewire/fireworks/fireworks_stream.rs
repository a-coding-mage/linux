// SPDX-License-Identifier: GPL-2.0-only
/*
 * fireworks_stream.c - a part of driver for Fireworks based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Rust translation of dependencies from "./fireworks.h" is expected to be
// supplied by the surrounding driver translation.

const READY_TIMEOUT_MS: c_uint = 1000;

type c_int = i32;
type c_uint = u32;
type c_char = i8;

const CMP_OUTPUT: cmp_direction = 0;
const CMP_INPUT: cmp_direction = 1;
const AMDTP_IN_STREAM: amdtp_stream_direction = 0;
const AMDTP_OUT_STREAM: amdtp_stream_direction = 1;

const CIP_BLOCKING: c_uint = 0;
const CIP_UNAWARE_SYT: c_uint = 0;
const CIP_EMPTY_WITH_TAG0: c_uint = 0;
const CIP_DBC_IS_END_EVENT: c_uint = 0;
const CIP_SKIP_DBC_ZERO_CHECK: c_uint = 0;
const CIP_UNALIGHED_DBC: c_uint = 0;
const CIP_WRONG_DBS: c_uint = 0;

const SND_EFW_TRANSPORT_MODE_IEC61883: c_uint = 0;

const EBUSY: c_int = 16;
const EIO: c_int = 5;
const ETIMEDOUT: c_int = 110;

type cmp_direction = c_uint;
type amdtp_stream_direction = c_uint;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_unit {
    pub device: device,
}

#[repr(C)]
pub struct cmp_resources {
    pub channel: c_int,
}

#[repr(C)]
pub struct cmp_connection {
    pub resources: cmp_resources,
    pub speed: c_uint,
    pub direction: cmp_direction,
    pub pcr_index: c_int,
}

#[repr(C)]
pub struct amdtp_stream_tx {
    pub dbc_interval: c_uint,
}

#[repr(C)]
pub struct amdtp_stream_ctx_data {
    pub tx: amdtp_stream_tx,
}

#[repr(C)]
pub struct amdtp_stream {
    pub flags: c_uint,
    pub ctx_data: amdtp_stream_ctx_data,
}

#[repr(C)]
pub struct amdtp_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_efw {
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub out_conn: cmp_connection,
    pub in_conn: cmp_connection,
    pub unit: *mut fw_unit,
    pub domain: amdtp_domain,
    pub is_fireworks3: bool,
    pub firmware_version: c_uint,
    pub is_af9: bool,
    pub pcm_capture_channels: [c_uint; 0],
    pub midi_out_ports: c_uint,
    pub pcm_playback_channels: [c_uint; 0],
    pub midi_in_ports: c_uint,
    pub substreams_counter: c_uint,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,
    pub lock: spinlock_t,
    pub dev_lock_count: c_int,
}

unsafe extern "C" {
    fn cmp_connection_init(
        conn: *mut cmp_connection,
        unit: *mut fw_unit,
        direction: cmp_direction,
        pcr_index: c_int,
    ) -> c_int;
    fn cmp_connection_destroy(conn: *mut cmp_connection);
    fn cmp_connection_establish(conn: *mut cmp_connection) -> c_int;
    fn cmp_connection_break(conn: *mut cmp_connection);
    fn cmp_connection_check_used(conn: *mut cmp_connection, used: *mut bool) -> c_int;
    fn cmp_connection_reserve(conn: *mut cmp_connection, max_payload: c_uint) -> c_int;
    fn cmp_connection_release(conn: *mut cmp_connection);

    fn amdtp_am824_init(
        stream: *mut amdtp_stream,
        unit: *mut fw_unit,
        direction: amdtp_stream_direction,
        flags: c_uint,
    ) -> c_int;
    fn amdtp_stream_destroy(stream: *mut amdtp_stream);
    fn amdtp_domain_add_stream(
        domain: *mut amdtp_domain,
        stream: *mut amdtp_stream,
        channel: c_int,
        speed: c_uint,
    ) -> c_int;
    fn amdtp_stream_running(stream: *mut amdtp_stream) -> bool;
    fn amdtp_domain_init(domain: *mut amdtp_domain) -> c_int;
    fn amdtp_am824_set_parameters(
        stream: *mut amdtp_stream,
        rate: c_uint,
        pcm_channels: c_uint,
        midi_ports: c_uint,
        double_pcm_frames: bool,
    ) -> c_int;
    fn amdtp_stream_get_max_payload(stream: *mut amdtp_stream) -> c_uint;
    fn amdtp_domain_set_events_per_period(
        domain: *mut amdtp_domain,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    fn amdtp_domain_stop(domain: *mut amdtp_domain);
    fn amdtp_streaming_error(stream: *mut amdtp_stream) -> bool;
    fn amdtp_domain_start(
        domain: *mut amdtp_domain,
        tx_init_skip_cycles: c_uint,
        sync: bool,
        callbacked: bool,
    ) -> c_int;
    fn amdtp_domain_wait_ready(domain: *mut amdtp_domain, timeout_ms: c_uint) -> bool;
    fn amdtp_stream_pcm_abort(stream: *mut amdtp_stream);
    fn amdtp_domain_destroy(domain: *mut amdtp_domain);

    fn snd_efw_command_set_tx_mode(efw: *mut snd_efw, mode: c_uint) -> c_int;
    fn snd_efw_command_get_sampling_rate(efw: *mut snd_efw, rate: *mut c_uint) -> c_int;
    fn snd_efw_command_set_sampling_rate(efw: *mut snd_efw, rate: c_uint) -> c_int;
    fn snd_efw_get_multiplier_mode(rate: c_uint, mode: *mut c_uint) -> c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn wake_up(wait: *mut wait_queue_head_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn WARN_ON(condition: bool) -> bool;
}

unsafe fn init_stream(efw: *mut snd_efw, stream: *mut amdtp_stream) -> c_int {
    let conn: *mut cmp_connection;
    let c_dir: cmp_direction;
    let s_dir: amdtp_stream_direction;
    let mut err: c_int;

    if stream == &mut (*efw).tx_stream {
        conn = &mut (*efw).out_conn;
        c_dir = CMP_OUTPUT;
        s_dir = AMDTP_IN_STREAM;
    } else {
        conn = &mut (*efw).in_conn;
        c_dir = CMP_INPUT;
        s_dir = AMDTP_OUT_STREAM;
    }

    err = cmp_connection_init(conn, (*efw).unit, c_dir, 0);
    if err < 0 {
        return err;
    }

    err = amdtp_am824_init(stream, (*efw).unit, s_dir, CIP_BLOCKING | CIP_UNAWARE_SYT);
    if err < 0 {
        amdtp_stream_destroy(stream);
        cmp_connection_destroy(conn);
        return err;
    }

    if stream == &mut (*efw).tx_stream {
        // Fireworks transmits NODATA packets with TAG0.
        (*efw).tx_stream.flags |= CIP_EMPTY_WITH_TAG0;
        // Fireworks has its own meaning for dbc.
        (*efw).tx_stream.flags |= CIP_DBC_IS_END_EVENT;
        // Fireworks reset dbc at bus reset.
        (*efw).tx_stream.flags |= CIP_SKIP_DBC_ZERO_CHECK;
        // But Recent firmwares starts packets with non-zero dbc.
        // Driver version 5.7.6 installs firmware version 5.7.3.
        if (*efw).is_fireworks3
            && ((*efw).firmware_version == 0x5070000
                || (*efw).firmware_version == 0x5070300
                || (*efw).firmware_version == 0x5080000)
        {
            (*efw).tx_stream.flags |= CIP_UNALIGHED_DBC;
        }
        // AudioFire9 always reports wrong dbs. Onyx 1200F with the latest firmware (v4.6.0)
        // also report wrong dbs at 88.2 kHz or greater.
        if (*efw).is_af9 || (*efw).firmware_version == 0x4060000 {
            (*efw).tx_stream.flags |= CIP_WRONG_DBS;
        }
        // Firmware version 5.5 reports fixed interval for dbc.
        if (*efw).firmware_version == 0x5050000 {
            (*efw).tx_stream.ctx_data.tx.dbc_interval = 8;
        }
    }

    err
}

unsafe fn start_stream(
    efw: *mut snd_efw,
    stream: *mut amdtp_stream,
    _rate: c_uint,
) -> c_int {
    let conn: *mut cmp_connection;
    let mut err: c_int;

    if stream == &mut (*efw).tx_stream {
        conn = &mut (*efw).out_conn;
    } else {
        conn = &mut (*efw).in_conn;
    }

    // Establish connection via CMP.
    err = cmp_connection_establish(conn);
    if err < 0 {
        return err;
    }

    // Start amdtp stream.
    err = amdtp_domain_add_stream(
        &mut (*efw).domain,
        stream,
        (*conn).resources.channel,
        (*conn).speed,
    );
    if err < 0 {
        cmp_connection_break(conn);
        return err;
    }

    0
}

// This function should be called before starting the stream or after stopping
// the streams.
unsafe fn destroy_stream(efw: *mut snd_efw, stream: *mut amdtp_stream) {
    amdtp_stream_destroy(stream);

    if stream == &mut (*efw).tx_stream {
        cmp_connection_destroy(&mut (*efw).out_conn);
    } else {
        cmp_connection_destroy(&mut (*efw).in_conn);
    }
}

unsafe fn check_connection_used_by_others(
    efw: *mut snd_efw,
    s: *mut amdtp_stream,
) -> c_int {
    let conn: *mut cmp_connection;
    let mut used: bool = false;
    let mut err: c_int;

    if s == &mut (*efw).tx_stream {
        conn = &mut (*efw).out_conn;
    } else {
        conn = &mut (*efw).in_conn;
    }

    err = cmp_connection_check_used(conn, &mut used);
    if (err >= 0) && used && !amdtp_stream_running(s) {
        dev_err(
            &mut (*(*efw).unit).device,
            b"Connection established by others: %cPCR[%d]\n\0".as_ptr() as *const c_char,
            if (*conn).direction == CMP_OUTPUT {
                b'o' as c_int
            } else {
                b'i' as c_int
            },
            (*conn).pcr_index,
        );
        err = -EBUSY;
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_stream_init_duplex(efw: *mut snd_efw) -> c_int {
    let mut err: c_int;

    err = init_stream(efw, &mut (*efw).tx_stream);
    if err < 0 {
        return err;
    }

    err = init_stream(efw, &mut (*efw).rx_stream);
    if err < 0 {
        destroy_stream(efw, &mut (*efw).tx_stream);
        return err;
    }

    err = amdtp_domain_init(&mut (*efw).domain);
    if err < 0 {
        destroy_stream(efw, &mut (*efw).tx_stream);
        destroy_stream(efw, &mut (*efw).rx_stream);
        return err;
    }

    // set IEC61883 compliant mode (actually not fully compliant...).
    err = snd_efw_command_set_tx_mode(efw, SND_EFW_TRANSPORT_MODE_IEC61883);
    if err < 0 {
        destroy_stream(efw, &mut (*efw).tx_stream);
        destroy_stream(efw, &mut (*efw).rx_stream);
    }

    err
}

unsafe fn keep_resources(
    efw: *mut snd_efw,
    stream: *mut amdtp_stream,
    rate: c_uint,
    mode: c_uint,
) -> c_int {
    let pcm_channels: c_uint;
    let midi_ports: c_uint;
    let conn: *mut cmp_connection;
    let mut err: c_int;

    if stream == &mut (*efw).tx_stream {
        pcm_channels = (*efw).pcm_capture_channels[mode as usize];
        midi_ports = (*efw).midi_out_ports;
        conn = &mut (*efw).out_conn;
    } else {
        pcm_channels = (*efw).pcm_playback_channels[mode as usize];
        midi_ports = (*efw).midi_in_ports;
        conn = &mut (*efw).in_conn;
    }

    err = amdtp_am824_set_parameters(stream, rate, pcm_channels, midi_ports, false);
    if err < 0 {
        return err;
    }

    cmp_connection_reserve(conn, amdtp_stream_get_max_payload(stream))
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_stream_reserve_duplex(
    efw: *mut snd_efw,
    mut rate: c_uint,
    frames_per_period: c_uint,
    frames_per_buffer: c_uint,
) -> c_int {
    let mut curr_rate: c_uint = 0;
    let mut err: c_int;

    // Considering JACK/FFADO streaming:
    // TODO: This can be removed hwdep functionality becomes popular.
    err = check_connection_used_by_others(efw, &mut (*efw).rx_stream);
    if err < 0 {
        return err;
    }

    // stop streams if rate is different.
    err = snd_efw_command_get_sampling_rate(efw, &mut curr_rate);
    if err < 0 {
        return err;
    }
    if rate == 0 {
        rate = curr_rate;
    }
    if rate != curr_rate {
        amdtp_domain_stop(&mut (*efw).domain);

        cmp_connection_break(&mut (*efw).out_conn);
        cmp_connection_break(&mut (*efw).in_conn);

        cmp_connection_release(&mut (*efw).out_conn);
        cmp_connection_release(&mut (*efw).in_conn);
    }

    if (*efw).substreams_counter == 0 || rate != curr_rate {
        let mut mode: c_uint = 0;

        err = snd_efw_command_set_sampling_rate(efw, rate);
        if err < 0 {
            return err;
        }

        err = snd_efw_get_multiplier_mode(rate, &mut mode);
        if err < 0 {
            return err;
        }

        err = keep_resources(efw, &mut (*efw).tx_stream, rate, mode);
        if err < 0 {
            return err;
        }

        err = keep_resources(efw, &mut (*efw).rx_stream, rate, mode);
        if err < 0 {
            cmp_connection_release(&mut (*efw).in_conn);
            return err;
        }

        err = amdtp_domain_set_events_per_period(
            &mut (*efw).domain,
            frames_per_period,
            frames_per_buffer,
        );
        if err < 0 {
            cmp_connection_release(&mut (*efw).in_conn);
            cmp_connection_release(&mut (*efw).out_conn);
            return err;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_stream_start_duplex(efw: *mut snd_efw) -> c_int {
    let mut rate: c_uint = 0;
    let mut err: c_int = 0;

    // Need no substreams.
    if (*efw).substreams_counter == 0 {
        return -EIO;
    }

    if amdtp_streaming_error(&mut (*efw).rx_stream)
        || amdtp_streaming_error(&mut (*efw).tx_stream)
    {
        amdtp_domain_stop(&mut (*efw).domain);
        cmp_connection_break(&mut (*efw).out_conn);
        cmp_connection_break(&mut (*efw).in_conn);
    }

    err = snd_efw_command_get_sampling_rate(efw, &mut rate);
    if err < 0 {
        return err;
    }

    if !amdtp_stream_running(&mut (*efw).rx_stream) {
        let tx_init_skip_cycles: c_uint;

        // Audiofire 2/4 skip an isochronous cycle several thousands after starting
        // packet transmission.
        if (*efw).is_fireworks3 && !(*efw).is_af9 {
            tx_init_skip_cycles = 6000;
        } else {
            tx_init_skip_cycles = 0;
        }

        err = start_stream(efw, &mut (*efw).rx_stream, rate);
        if err < 0 {
            amdtp_domain_stop(&mut (*efw).domain);

            cmp_connection_break(&mut (*efw).out_conn);
            cmp_connection_break(&mut (*efw).in_conn);

            return err;
        }

        err = start_stream(efw, &mut (*efw).tx_stream, rate);
        if err < 0 {
            amdtp_domain_stop(&mut (*efw).domain);

            cmp_connection_break(&mut (*efw).out_conn);
            cmp_connection_break(&mut (*efw).in_conn);

            return err;
        }

        // NOTE: The device ignores presentation time expressed by the value of syt field
        // of CIP header in received packets. The sequence of the number of data blocks per
        // packet is important for media clock recovery.
        err = amdtp_domain_start(&mut (*efw).domain, tx_init_skip_cycles, true, false);
        if err < 0 {
            amdtp_domain_stop(&mut (*efw).domain);

            cmp_connection_break(&mut (*efw).out_conn);
            cmp_connection_break(&mut (*efw).in_conn);

            return err;
        }

        if !amdtp_domain_wait_ready(&mut (*efw).domain, READY_TIMEOUT_MS) {
            err = -ETIMEDOUT;

            amdtp_domain_stop(&mut (*efw).domain);

            cmp_connection_break(&mut (*efw).out_conn);
            cmp_connection_break(&mut (*efw).in_conn);

            return err;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_stream_stop_duplex(efw: *mut snd_efw) {
    if (*efw).substreams_counter == 0 {
        amdtp_domain_stop(&mut (*efw).domain);

        cmp_connection_break(&mut (*efw).out_conn);
        cmp_connection_break(&mut (*efw).in_conn);

        cmp_connection_release(&mut (*efw).out_conn);
        cmp_connection_release(&mut (*efw).in_conn);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_stream_update_duplex(efw: *mut snd_efw) {
    amdtp_domain_stop(&mut (*efw).domain);

    cmp_connection_break(&mut (*efw).out_conn);
    cmp_connection_break(&mut (*efw).in_conn);

    amdtp_stream_pcm_abort(&mut (*efw).rx_stream);
    amdtp_stream_pcm_abort(&mut (*efw).tx_stream);
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_stream_destroy_duplex(efw: *mut snd_efw) {
    amdtp_domain_destroy(&mut (*efw).domain);

    destroy_stream(efw, &mut (*efw).rx_stream);
    destroy_stream(efw, &mut (*efw).tx_stream);
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_stream_lock_changed(efw: *mut snd_efw) {
    (*efw).dev_lock_changed = true;
    wake_up(&mut (*efw).hwdep_wait);
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_stream_lock_try(efw: *mut snd_efw) -> c_int {
    spin_lock_irq(&mut (*efw).lock);

    /* user land lock this */
    if (*efw).dev_lock_count < 0 {
        spin_unlock_irq(&mut (*efw).lock);
        return -EBUSY;
    }

    /* this is the first time */
    let old_dev_lock_count = (*efw).dev_lock_count;
    (*efw).dev_lock_count += 1;
    if old_dev_lock_count == 0 {
        snd_efw_stream_lock_changed(efw);
    }

    spin_unlock_irq(&mut (*efw).lock);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_stream_lock_release(efw: *mut snd_efw) {
    spin_lock_irq(&mut (*efw).lock);

    if WARN_ON((*efw).dev_lock_count <= 0) {
        spin_unlock_irq(&mut (*efw).lock);
        return;
    }
    (*efw).dev_lock_count -= 1;
    if (*efw).dev_lock_count == 0 {
        snd_efw_stream_lock_changed(efw);
    }

    spin_unlock_irq(&mut (*efw).lock);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
