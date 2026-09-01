// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob_stream.c - a part of driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Original C dependency: #include "./bebob.h"
use crate::*;

const READY_TIMEOUT_MS: u32 = 4000;

/*
 * NOTE;
 * For BeBoB streams, Both of input and output CMP connection are important.
 *
 * For most devices, each CMP connection starts to transmit/receive a
 * corresponding stream. But for a few devices, both of CMP connection needs
 * to start transmitting stream. An example is 'M-Audio Firewire 410'.
 */

/* 128 is an arbitrary length but it seems to be enough */
const FORMAT_MAXIMUM_LENGTH: usize = 128;

#[no_mangle]
pub static snd_bebob_rate_table: [u32; SND_BEBOB_STRM_FMT_ENTRIES as usize] = [
    32000,
    44100,
    48000,
    88200,
    96000,
    176400,
    192000,
];

/*
 * See: Table 51: Extended Stream Format Info 'Sampling Frequency'
 * in Additional AVC commands (Nov 2003, BridgeCo)
 */
static bridgeco_freq_table: [u32; 7] = [
    0x02,
    0x03,
    0x04,
    0x0a,
    0x05,
    0x06,
    0x07,
];

unsafe fn get_formation_index(rate: u32, index: *mut u32) -> c_int {
    let mut i: u32;

    i = 0;
    while (i as usize) < snd_bebob_rate_table.len() {
        if snd_bebob_rate_table[i as usize] == rate {
            *index = i;
            return 0;
        }
        i += 1;
    }
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_get_rate(
    bebob: *mut snd_bebob,
    curr_rate: *mut u32,
) -> c_int {
    let mut tx_rate: u32 = 0;
    let mut rx_rate: u32 = 0;
    let mut trials: u32;
    let mut err: c_int;

    trials = 0;
    loop {
        err = avc_general_get_sig_fmt(
            (*bebob).unit,
            &mut tx_rate,
            AVC_GENERAL_PLUG_DIR_OUT,
            0,
        );
        if !(err == -EAGAIN && {
            trials += 1;
            trials < 3
        }) {
            break;
        }
    }
    if err < 0 {
        return err;
    }

    trials = 0;
    loop {
        err = avc_general_get_sig_fmt(
            (*bebob).unit,
            &mut rx_rate,
            AVC_GENERAL_PLUG_DIR_IN,
            0,
        );
        if !(err == -EAGAIN && {
            trials += 1;
            trials < 3
        }) {
            break;
        }
    }
    if err < 0 {
        return err;
    }

    *curr_rate = rx_rate;
    if rx_rate == tx_rate {
        return err;
    }

    /* synchronize receive stream rate to transmit stream rate */
    err = avc_general_set_sig_fmt((*bebob).unit, rx_rate, AVC_GENERAL_PLUG_DIR_IN, 0);
    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_set_rate(
    bebob: *mut snd_bebob,
    rate: u32,
) -> c_int {
    let mut err: c_int;

    err = avc_general_set_sig_fmt((*bebob).unit, rate, AVC_GENERAL_PLUG_DIR_OUT, 0);
    if err < 0 {
        return err;
    }

    err = avc_general_set_sig_fmt((*bebob).unit, rate, AVC_GENERAL_PLUG_DIR_IN, 0);
    if err < 0 {
        return err;
    }

    /*
     * Some devices need a bit time for transition.
     * 300msec is got by some experiments.
     */
    msleep(300);
    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_get_clock_src(
    bebob: *mut snd_bebob,
    src: *mut snd_bebob_clock_type,
) -> c_int {
    let clk_spec: *const snd_bebob_clock_spec = (*(*bebob).spec).clock;
    let mut addr: [u8; AVC_BRIDGECO_ADDR_BYTES as usize] = [0; AVC_BRIDGECO_ADDR_BYTES as usize];
    let mut input: [u8; 7] = [0; 7];
    let mut id: u32 = 0;
    let mut type_: avc_bridgeco_plug_type = core::mem::zeroed();
    let mut err: c_int = 0;

    /* 1.The device has its own operation to switch source of clock */
    if !clk_spec.is_null() {
        err = ((*clk_spec).get.unwrap())(bebob, &mut id);
        if err < 0 {
            dev_err(
                &mut (*(*(*bebob).unit).device) as *mut _,
                c_str!("fail to get clock source: %d\n"),
                err,
            );
            return err;
        }

        if id >= (*clk_spec).num {
            dev_err(
                &mut (*(*(*bebob).unit).device) as *mut _,
                c_str!("clock source %d out of range 0..%d\n"),
                id,
                (*clk_spec).num - 1,
            );
            err = -EIO;
            return err;
        }

        *src = *(*clk_spec).types.add(id as usize);
        return err;
    }

    /*
     * 2.The device don't support to switch source of clock then assumed
     *   to use internal clock always
     */
    if (*bebob).sync_input_plug < 0 {
        *src = SND_BEBOB_CLOCK_TYPE_INTERNAL;
        return err;
    }

    /*
     * 3.The device supports to switch source of clock by an usual way.
     *   Let's check input for 'Music Sub Unit Sync Input' plug.
     */
    avc_bridgeco_fill_msu_addr(
        addr.as_mut_ptr(),
        AVC_BRIDGECO_PLUG_DIR_IN,
        (*bebob).sync_input_plug as u32,
    );
    err = avc_bridgeco_get_plug_input((*bebob).unit, addr.as_mut_ptr(), input.as_mut_ptr());
    if err < 0 {
        dev_err(
            &mut (*(*(*bebob).unit).device) as *mut _,
            c_str!("fail to get an input for MSU in plug %d: %d\n"),
            (*bebob).sync_input_plug,
            err,
        );
        return err;
    }

    /*
     * If there are no input plugs, all of fields are 0xff.
     * Here check the first field. This field is used for direction.
     */
    if input[0] == 0xff {
        *src = SND_BEBOB_CLOCK_TYPE_INTERNAL;
        return err;
    }

    /* The source from any output plugs is for one purpose only. */
    if input[0] == AVC_BRIDGECO_PLUG_DIR_OUT as u8 {
        /*
         * In BeBoB architecture, the source from music subunit may
         * bypass from oPCR[0]. This means that this source gives
         * synchronization to IEEE 1394 cycle start packet.
         */
        if input[1] == AVC_BRIDGECO_PLUG_MODE_SUBUNIT as u8 && input[2] == 0x0c {
            *src = SND_BEBOB_CLOCK_TYPE_INTERNAL;
            return err;
        }
    /* The source from any input units is for several purposes. */
    } else if input[1] == AVC_BRIDGECO_PLUG_MODE_UNIT as u8 {
        if input[2] == AVC_BRIDGECO_PLUG_UNIT_ISOC as u8 {
            if input[3] == 0x00 {
                /*
                 * This source comes from iPCR[0]. This means
                 * that presentation timestamp calculated by
                 * SYT series of the received packets. In
                 * short, this driver is the master of
                 * synchronization.
                 */
                *src = SND_BEBOB_CLOCK_TYPE_SYT;
                return err;
            } else {
                /*
                 * This source comes from iPCR[1-29]. This
                 * means that the synchronization stream is not
                 * the Audio/MIDI compound stream.
                 */
                *src = SND_BEBOB_CLOCK_TYPE_EXTERNAL;
                return err;
            }
        } else if input[2] == AVC_BRIDGECO_PLUG_UNIT_EXT as u8 {
            /* Check type of this plug.  */
            avc_bridgeco_fill_unit_addr(
                addr.as_mut_ptr(),
                AVC_BRIDGECO_PLUG_DIR_IN,
                AVC_BRIDGECO_PLUG_UNIT_EXT,
                input[3] as u32,
            );
            err = avc_bridgeco_get_plug_type((*bebob).unit, addr.as_mut_ptr(), &mut type_);
            if err < 0 {
                return err;
            }

            if type_ == AVC_BRIDGECO_PLUG_TYPE_DIG {
                /*
                 * SPDIF/ADAT or sometimes (not always) word
                 * clock.
                 */
                *src = SND_BEBOB_CLOCK_TYPE_EXTERNAL;
                return err;
            } else if type_ == AVC_BRIDGECO_PLUG_TYPE_SYNC {
                /* Often word clock. */
                *src = SND_BEBOB_CLOCK_TYPE_EXTERNAL;
                return err;
            } else if type_ == AVC_BRIDGECO_PLUG_TYPE_ADDITION {
                /*
                 * Not standard.
                 * Mostly, additional internal clock.
                 */
                *src = SND_BEBOB_CLOCK_TYPE_INTERNAL;
                return err;
            }
        }
    }

    /* Not supported. */
    -EIO
}

unsafe fn map_data_channels(bebob: *mut snd_bebob, s: *mut amdtp_stream) -> c_int {
    let mut sec: u32;
    let sections: u32;
    let mut ch: u32;
    let channels: u32;
    let mut pcm: u32;
    let mut midi: u32;
    let location: u32;
    let stm_pos: u32;
    let mut sec_loc: u32;
    let mut pos: u32;
    let buf: *mut u8;
    let mut addr: [u8; AVC_BRIDGECO_ADDR_BYTES as usize] = [0; AVC_BRIDGECO_ADDR_BYTES as usize];
    let mut type_: u8 = 0;
    let dir: avc_bridgeco_plug_dir;
    let mut err: c_int;

    /*
     * The length of return value of this command cannot be expected. Here
     * use the maximum length of FCP.
     */
    buf = kzalloc(256, GFP_KERNEL) as *mut u8;
    if buf.is_null() {
        return -ENOMEM;
    }

    if s == &mut (*bebob).tx_stream as *mut _ {
        dir = AVC_BRIDGECO_PLUG_DIR_OUT;
    } else {
        dir = AVC_BRIDGECO_PLUG_DIR_IN;
    }

    avc_bridgeco_fill_unit_addr(addr.as_mut_ptr(), dir, AVC_BRIDGECO_PLUG_UNIT_ISOC, 0);
    err = avc_bridgeco_get_plug_ch_pos((*bebob).unit, addr.as_mut_ptr(), buf, 256);
    if err < 0 {
        dev_err(
            &mut (*(*(*bebob).unit).device) as *mut _,
            c_str!("fail to get channel position for isoc %s plug 0: %d\n"),
            if dir == AVC_BRIDGECO_PLUG_DIR_IN { c_str!("in") } else { c_str!("out") },
            err,
        );
        kfree(buf as *const c_void);
        return err;
    }
    pos = 0;

    /* positions in I/O buffer */
    pcm = 0;
    midi = 0;

    /* the number of sections in AMDTP packet */
    sections = *buf.add({
        let old = pos;
        pos += 1;
        old as usize
    }) as u32;

    sec = 0;
    while sec < sections {
        /* type of this section */
        avc_bridgeco_fill_unit_addr(addr.as_mut_ptr(), dir, AVC_BRIDGECO_PLUG_UNIT_ISOC, 0);
        err = avc_bridgeco_get_plug_section_type(
            (*bebob).unit,
            addr.as_mut_ptr(),
            sec,
            &mut type_,
        );
        if err < 0 {
            dev_err(
                &mut (*(*(*bebob).unit).device) as *mut _,
                c_str!("fail to get section type for isoc %s plug 0: %d\n"),
                if dir == AVC_BRIDGECO_PLUG_DIR_IN { c_str!("in") } else { c_str!("out") },
                err,
            );
            kfree(buf as *const c_void);
            return err;
        }
        /* NoType */
        if type_ == 0xff {
            kfree(buf as *const c_void);
            return -ENOSYS;
        }

        /* the number of channels in this section */
        channels = *buf.add({
            let old = pos;
            pos += 1;
            old as usize
        }) as u32;

        ch = 0;
        while ch < channels {
            /* position of this channel in AMDTP packet */
            stm_pos = (*buf.add({
                let old = pos;
                pos += 1;
                old as usize
            }) as u32).wrapping_sub(1);
            /* location of this channel in this section */
            sec_loc = (*buf.add({
                let old = pos;
                pos += 1;
                old as usize
            }) as u32).wrapping_sub(1);

            /*
             * Basically the number of location is within the
             * number of channels in this section. But some models
             * of M-Audio don't follow this. Its location for MIDI
             * is the position of MIDI channels in AMDTP packet.
             */
            if sec_loc >= channels {
                sec_loc = ch;
            }

            match type_ {
                /* for MIDI conformant data channel */
                0x0a => {
                    /* AMDTP_MAX_CHANNELS_FOR_MIDI is 1. */
                    if midi > 0 && stm_pos != midi {
                        kfree(buf as *const c_void);
                        return -ENOSYS;
                    }
                    amdtp_am824_set_midi_position(s, stm_pos);
                    midi = stm_pos;
                }
                /* for PCM data channel */
                0x01 | /* Headphone */
                0x02 | /* Microphone */
                0x03 | /* Line */
                0x04 | /* SPDIF */
                0x05 | /* ADAT */
                0x06 | /* TDIF */
                0x07 | /* MADI */
                /* for undefined/changeable signal  */
                0x08 | /* Analog */
                0x09 | /* Digital */
                _ => {
                    location = pcm + sec_loc;
                    if location >= AM824_MAX_CHANNELS_FOR_PCM {
                        kfree(buf as *const c_void);
                        return -ENOSYS;
                    }
                    amdtp_am824_set_pcm_position(s, location, stm_pos);
                }
            }
            ch += 1;
        }

        if type_ != 0x0a {
            pcm += channels;
        } else {
            midi += channels;
        }
        sec += 1;
    }
    kfree(buf as *const c_void);
    err
}

unsafe fn check_connection_used_by_others(
    bebob: *mut snd_bebob,
    s: *mut amdtp_stream,
) -> c_int {
    let conn: *mut cmp_connection;
    let mut used: bool = false;
    let mut err: c_int;

    if s == &mut (*bebob).tx_stream as *mut _ {
        conn = &mut (*bebob).out_conn;
    } else {
        conn = &mut (*bebob).in_conn;
    }

    err = cmp_connection_check_used(conn, &mut used);
    if err >= 0 && used && !amdtp_stream_running(s) {
        dev_err(
            &mut (*(*(*bebob).unit).device) as *mut _,
            c_str!("Connection established by others: %cPCR[%d]\n"),
            if (*conn).direction == CMP_OUTPUT { 'o' as c_int } else { 'i' as c_int },
            (*conn).pcr_index,
        );
        err = -EBUSY;
    }

    err
}

unsafe fn break_both_connections(bebob: *mut snd_bebob) {
    cmp_connection_break(&mut (*bebob).in_conn);
    cmp_connection_break(&mut (*bebob).out_conn);
}

unsafe fn start_stream(bebob: *mut snd_bebob, stream: *mut amdtp_stream) -> c_int {
    let conn: *mut cmp_connection;
    let mut err: c_int = 0;

    if stream == &mut (*bebob).rx_stream as *mut _ {
        conn = &mut (*bebob).in_conn;
    } else {
        conn = &mut (*bebob).out_conn;
    }

    // channel mapping.
    if (*bebob).maudio_special_quirk.is_null() {
        err = map_data_channels(bebob, stream);
        if err < 0 {
            return err;
        }
    }

    err = cmp_connection_establish(conn);
    if err < 0 {
        return err;
    }

    amdtp_domain_add_stream(
        &mut (*bebob).domain,
        stream,
        (*conn).resources.channel,
        (*conn).speed,
    )
}

unsafe fn init_stream(bebob: *mut snd_bebob, stream: *mut amdtp_stream) -> c_int {
    let mut flags: u32 = CIP_BLOCKING;
    let dir_stream: amdtp_stream_direction;
    let conn: *mut cmp_connection;
    let dir_conn: cmp_direction;
    let mut err: c_int;

    if stream == &mut (*bebob).tx_stream as *mut _ {
        dir_stream = AMDTP_IN_STREAM;
        conn = &mut (*bebob).out_conn;
        dir_conn = CMP_OUTPUT;
    } else {
        dir_stream = AMDTP_OUT_STREAM;
        conn = &mut (*bebob).in_conn;
        dir_conn = CMP_INPUT;
    }

    if stream == &mut (*bebob).tx_stream as *mut _ {
        if (*bebob).quirks & SND_BEBOB_QUIRK_WRONG_DBC != 0 {
            flags |= CIP_EMPTY_HAS_WRONG_DBC;
        }
    }

    err = cmp_connection_init(conn, (*bebob).unit, dir_conn, 0);
    if err < 0 {
        return err;
    }

    err = amdtp_am824_init(stream, (*bebob).unit, dir_stream, flags);
    if err < 0 {
        cmp_connection_destroy(conn);
        return err;
    }

    0
}

unsafe fn destroy_stream(bebob: *mut snd_bebob, stream: *mut amdtp_stream) {
    amdtp_stream_destroy(stream);

    if stream == &mut (*bebob).tx_stream as *mut _ {
        cmp_connection_destroy(&mut (*bebob).out_conn);
    } else {
        cmp_connection_destroy(&mut (*bebob).in_conn);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_init_duplex(bebob: *mut snd_bebob) -> c_int {
    let mut err: c_int;

    err = init_stream(bebob, &mut (*bebob).tx_stream);
    if err < 0 {
        return err;
    }

    err = init_stream(bebob, &mut (*bebob).rx_stream);
    if err < 0 {
        destroy_stream(bebob, &mut (*bebob).tx_stream);
        return err;
    }

    err = amdtp_domain_init(&mut (*bebob).domain);
    if err < 0 {
        destroy_stream(bebob, &mut (*bebob).tx_stream);
        destroy_stream(bebob, &mut (*bebob).rx_stream);
    }

    err
}

unsafe fn keep_resources(
    bebob: *mut snd_bebob,
    stream: *mut amdtp_stream,
    rate: u32,
    index: u32,
) -> c_int {
    let pcm_channels: u32;
    let midi_ports: u32;
    let conn: *mut cmp_connection;
    let mut err: c_int;

    if stream == &mut (*bebob).tx_stream as *mut _ {
        pcm_channels = (*bebob).tx_stream_formations[index as usize].pcm;
        midi_ports = (*bebob).midi_input_ports;
        conn = &mut (*bebob).out_conn;
    } else {
        pcm_channels = (*bebob).rx_stream_formations[index as usize].pcm;
        midi_ports = (*bebob).midi_output_ports;
        conn = &mut (*bebob).in_conn;
    }

    err = amdtp_am824_set_parameters(stream, rate, pcm_channels, midi_ports, false);
    if err < 0 {
        return err;
    }

    cmp_connection_reserve(conn, amdtp_stream_get_max_payload(stream))
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_reserve_duplex(
    bebob: *mut snd_bebob,
    mut rate: u32,
    frames_per_period: u32,
    frames_per_buffer: u32,
) -> c_int {
    let mut curr_rate: u32 = 0;
    let mut err: c_int;

    // Considering JACK/FFADO streaming:
    // TODO: This can be removed hwdep functionality becomes popular.
    err = check_connection_used_by_others(bebob, &mut (*bebob).rx_stream);
    if err < 0 {
        return err;
    }

    err = ((*(*(*bebob).spec).rate).get.unwrap())(bebob, &mut curr_rate);
    if err < 0 {
        return err;
    }
    if rate == 0 {
        rate = curr_rate;
    }
    if curr_rate != rate {
        amdtp_domain_stop(&mut (*bebob).domain);
        break_both_connections(bebob);

        cmp_connection_release(&mut (*bebob).out_conn);
        cmp_connection_release(&mut (*bebob).in_conn);
    }

    if (*bebob).substreams_counter == 0 || curr_rate != rate {
        let mut index: u32 = 0;

        // NOTE:
        // If establishing connections at first, Yamaha GO46
        // (and maybe Terratec X24) don't generate sound.
        //
        // For firmware customized by M-Audio, refer to next NOTE.
        err = ((*(*(*bebob).spec).rate).set.unwrap())(bebob, rate);
        if err < 0 {
            dev_err(
                &mut (*(*(*bebob).unit).device) as *mut _,
                c_str!("fail to set sampling rate: %d\n"),
                err,
            );
            return err;
        }

        err = get_formation_index(rate, &mut index);
        if err < 0 {
            return err;
        }

        err = keep_resources(bebob, &mut (*bebob).tx_stream, rate, index);
        if err < 0 {
            return err;
        }

        err = keep_resources(bebob, &mut (*bebob).rx_stream, rate, index);
        if err < 0 {
            cmp_connection_release(&mut (*bebob).out_conn);
            return err;
        }

        err = amdtp_domain_set_events_per_period(
            &mut (*bebob).domain,
            frames_per_period,
            frames_per_buffer,
        );
        if err < 0 {
            cmp_connection_release(&mut (*bebob).out_conn);
            cmp_connection_release(&mut (*bebob).in_conn);
            return err;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_start_duplex(bebob: *mut snd_bebob) -> c_int {
    let mut err: c_int;

    // Need no substreams.
    if (*bebob).substreams_counter == 0 {
        return -EIO;
    }

    // packet queueing error or detecting discontinuity
    if amdtp_streaming_error(&mut (*bebob).rx_stream)
        || amdtp_streaming_error(&mut (*bebob).tx_stream)
    {
        amdtp_domain_stop(&mut (*bebob).domain);
        break_both_connections(bebob);
    }

    if !amdtp_stream_running(&mut (*bebob).rx_stream) {
        let mut src: snd_bebob_clock_type = core::mem::zeroed();
        let mut curr_rate: u32 = 0;
        let tx_init_skip_cycles: u32;

        if !(*bebob).maudio_special_quirk.is_null() {
            err = ((*(*(*bebob).spec).rate).get.unwrap())(bebob, &mut curr_rate);
            if err < 0 {
                return err;
            }
        }

        err = snd_bebob_stream_get_clock_src(bebob, &mut src);
        if err < 0 {
            return err;
        }

        err = start_stream(bebob, &mut (*bebob).rx_stream);
        if err < 0 {
            amdtp_domain_stop(&mut (*bebob).domain);
            break_both_connections(bebob);
            return err;
        }

        err = start_stream(bebob, &mut (*bebob).tx_stream);
        if err < 0 {
            amdtp_domain_stop(&mut (*bebob).domain);
            break_both_connections(bebob);
            return err;
        }

        if (*bebob).quirks & SND_BEBOB_QUIRK_INITIAL_DISCONTINUOUS_DBC == 0 {
            tx_init_skip_cycles = 0;
        } else {
            tx_init_skip_cycles = 16000;
        }

        // MEMO: Some devices start packet transmission long enough after establishment of
        // CMP connection. In the early stage of packet streaming, any device transfers
        // NODATA packets. After several hundred cycles, it begins to multiplex event into
        // the packet with adequate value of syt field in CIP header. Some devices are
        // strictly to generate any discontinuity in the sequence of tx packet when they
        // receives inadequate sequence of value in syt field of CIP header. In the case,
        // the request to break CMP connection is often corrupted, then any transaction
        // results in unrecoverable error, sometimes generate bus-reset.
        err = amdtp_domain_start(&mut (*bebob).domain, tx_init_skip_cycles, true, false);
        if err < 0 {
            amdtp_domain_stop(&mut (*bebob).domain);
            break_both_connections(bebob);
            return err;
        }

        // NOTE:
        // The firmware customized by M-Audio uses these commands to
        // start transmitting stream. This is not usual way.
        if !(*bebob).maudio_special_quirk.is_null() {
            err = ((*(*(*bebob).spec).rate).set.unwrap())(bebob, curr_rate);
            if err < 0 {
                dev_err(
                    &mut (*(*(*bebob).unit).device) as *mut _,
                    c_str!("fail to ensure sampling rate: %d\n"),
                    err,
                );
                amdtp_domain_stop(&mut (*bebob).domain);
                break_both_connections(bebob);
                return err;
            }
        }

        // Some devices postpone start of transmission mostly for 1 sec after receives
        // packets firstly.
        if !amdtp_domain_wait_ready(&mut (*bebob).domain, READY_TIMEOUT_MS) {
            err = -ETIMEDOUT;
            amdtp_domain_stop(&mut (*bebob).domain);
            break_both_connections(bebob);
            return err;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_stop_duplex(bebob: *mut snd_bebob) {
    if (*bebob).substreams_counter == 0 {
        amdtp_domain_stop(&mut (*bebob).domain);
        break_both_connections(bebob);

        cmp_connection_release(&mut (*bebob).out_conn);
        cmp_connection_release(&mut (*bebob).in_conn);
    }
}

/*
 * This function should be called before starting streams or after stopping
 * streams.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_destroy_duplex(bebob: *mut snd_bebob) {
    amdtp_domain_destroy(&mut (*bebob).domain);

    destroy_stream(bebob, &mut (*bebob).tx_stream);
    destroy_stream(bebob, &mut (*bebob).rx_stream);
}

/*
 * See: Table 50: Extended Stream Format Info Format Hierarchy Level 2'
 * in Additional AVC commands (Nov 2003, BridgeCo)
 * Also 'Clause 12 AM824 sequence adaption layers' in IEC 61883-6:2005
 */
unsafe fn parse_stream_formation(
    buf: *mut u8,
    _len: u32,
    formation: *mut snd_bebob_stream_formation,
) -> c_int {
    let mut i: u32;
    let mut e: u32;
    let channels: u32;
    let format: u32;

    /*
     * this module can support a hierarchy combination that:
     *  Root:	Audio and Music (0x90)
     *  Level 1:	AM824 Compound  (0x40)
     */
    if *buf.add(0) != 0x90 || *buf.add(1) != 0x40 {
        return -ENOSYS;
    }

    /* check sampling rate */
    i = 0;
    while (i as usize) < bridgeco_freq_table.len() {
        if *buf.add(2) as u32 == bridgeco_freq_table[i as usize] {
            break;
        }
        i += 1;
    }
    if (i as usize) == bridgeco_freq_table.len() {
        return -ENOSYS;
    }

    /* Avoid double count by different entries for the same rate. */
    memset(
        formation.add(i as usize) as *mut c_void,
        0,
        core::mem::size_of::<snd_bebob_stream_formation>(),
    );

    e = 0;
    while e < *buf.add(4) as u32 {
        channels = *buf.add((5 + e * 2) as usize) as u32;
        format = *buf.add((6 + e * 2) as usize) as u32;

        match format {
            /* IEC 60958 Conformant, currently handled as MBLA */
            0x00 |
            /* Multi bit linear audio */
            0x06 /* Raw */ => {
                (*formation.add(i as usize)).pcm += channels;
            }
            /* MIDI Conformant */
            0x0d => {
                (*formation.add(i as usize)).midi += channels;
            }
            /* IEC 61937-3 to 7 */
            0x01 | 0x02 | 0x03 | 0x04 | 0x05 |
            /* Multi bit linear audio */
            0x07 | /* DVD-Audio */
            0x0c | /* High Precision */
            /* One Bit Audio */
            0x08 | /* (Plain) Raw */
            0x09 | /* (Plain) SACD */
            0x0a | /* (Encoded) Raw */
            0x0b | /* (Encoded) SACD */
            /* Synchronization Stream (Stereo Raw audio) */
            0x40 |
            /* Don't care */
            0xff |
            _ => {
                return -ENOSYS; /* not supported */
            }
        }
        e += 1;
    }

    if (*formation.add(i as usize)).pcm > AM824_MAX_CHANNELS_FOR_PCM
        || (*formation.add(i as usize)).midi > AM824_MAX_CHANNELS_FOR_MIDI
    {
        return -ENOSYS;
    }

    0
}

unsafe fn fill_stream_formations(
    bebob: *mut snd_bebob,
    addr: *mut u8,
    plug_dir: avc_bridgeco_plug_dir,
    plug_id: u32,
    formations: *mut snd_bebob_stream_formation,
) -> c_int {
    let mut plug_type: avc_bridgeco_plug_type = core::mem::zeroed();
    let buf: *mut u8;
    let mut len: u32;
    let mut eid: u32;
    let mut err: c_int;

    avc_bridgeco_fill_unit_addr(addr, plug_dir, AVC_BRIDGECO_PLUG_UNIT_ISOC, plug_id);

    err = avc_bridgeco_get_plug_type((*bebob).unit, addr, &mut plug_type);
    if err < 0 {
        dev_err(
            &mut (*(*(*bebob).unit).device) as *mut _,
            c_str!("Fail to get type for isoc %d plug 0: %d\n"),
            plug_dir,
            err,
        );
        return err;
    } else if plug_type != AVC_BRIDGECO_PLUG_TYPE_ISOC {
        return -ENXIO;
    }

    buf = kmalloc(FORMAT_MAXIMUM_LENGTH, GFP_KERNEL) as *mut u8;
    if buf.is_null() {
        return -ENOMEM;
    }

    eid = 0;
    while eid < SND_BEBOB_STRM_FMT_ENTRIES {
        avc_bridgeco_fill_unit_addr(addr, plug_dir, AVC_BRIDGECO_PLUG_UNIT_ISOC, plug_id);

        len = FORMAT_MAXIMUM_LENGTH as u32;
        err = avc_bridgeco_get_plug_strm_fmt((*bebob).unit, addr, buf, &mut len, eid);
        // No entries remained.
        if err == -EINVAL && eid > 0 {
            err = 0;
            break;
        } else if err < 0 {
            dev_err(
                &mut (*(*(*bebob).unit).device) as *mut _,
                c_str!("fail to get stream format %d for isoc %d plug %d:%d\n"),
                eid,
                plug_dir,
                plug_id,
                err,
            );
            break;
        }

        err = parse_stream_formation(buf, len, formations);
        if err < 0 {
            break;
        }
        eid += 1;
    }

    kfree(buf as *const c_void);
    err
}

unsafe fn detect_midi_ports(
    bebob: *mut snd_bebob,
    formats: *const snd_bebob_stream_formation,
    addr: *mut u8,
    plug_dir: avc_bridgeco_plug_dir,
    plug_count: u32,
    midi_ports: *mut u32,
) -> c_int {
    let mut i: c_int;
    let mut err: c_int = 0;

    *midi_ports = 0;

    /// Detect the number of available MIDI ports when packet has MIDI conformant data channel.
    i = 0;
    while i < SND_BEBOB_STRM_FMT_ENTRIES as c_int {
        if (*formats.add(i as usize)).midi > 0 {
            break;
        }
        i += 1;
    }
    if i >= SND_BEBOB_STRM_FMT_ENTRIES as c_int {
        return 0;
    }

    i = 0;
    while i < plug_count as c_int {
        let mut plug_type: avc_bridgeco_plug_type = core::mem::zeroed();
        let mut ch_count: u32 = 0;

        avc_bridgeco_fill_unit_addr(
            addr,
            plug_dir,
            AVC_BRIDGECO_PLUG_UNIT_EXT,
            i as u32,
        );

        err = avc_bridgeco_get_plug_type((*bebob).unit, addr, &mut plug_type);
        if err < 0 {
            dev_err(
                &mut (*(*(*bebob).unit).device) as *mut _,
                c_str!("fail to get type for external %d plug %d: %d\n"),
                plug_dir,
                i,
                err,
            );
            break;
        } else if plug_type != AVC_BRIDGECO_PLUG_TYPE_MIDI {
            i += 1;
            continue;
        }

        err = avc_bridgeco_get_plug_ch_count((*bebob).unit, addr, &mut ch_count);
        if err < 0 {
            break;
        }
        // Yamaha GO44, GO46, Terratec Phase 24, Phase x24 reports 0 for the number of
        // channels in external output plug 3 (MIDI type) even if it has a pair of physical
        // MIDI jacks. As a workaround, assume it as one.
        if ch_count == 0 {
            ch_count = 1;
        }
        *midi_ports += ch_count;
        i += 1;
    }

    err
}

unsafe fn seek_msu_sync_input_plug(bebob: *mut snd_bebob) -> c_int {
    let mut plugs: [u8; AVC_PLUG_INFO_BUF_BYTES as usize] = [0; AVC_PLUG_INFO_BUF_BYTES as usize];
    let mut addr: [u8; AVC_BRIDGECO_ADDR_BYTES as usize] = [0; AVC_BRIDGECO_ADDR_BYTES as usize];
    let mut i: u32;
    let mut type_: avc_bridgeco_plug_type = core::mem::zeroed();
    let mut err: c_int;

    /* Get the number of Music Sub Unit for both direction. */
    err = avc_general_get_plug_info((*bebob).unit, 0x0c, 0x00, 0x00, plugs.as_mut_ptr());
    if err < 0 {
        dev_err(
            &mut (*(*(*bebob).unit).device) as *mut _,
            c_str!("fail to get info for MSU in/out plugs: %d\n"),
            err,
        );
        return err;
    }

    /* seek destination plugs for 'MSU sync input' */
    (*bebob).sync_input_plug = -1;
    i = 0;
    while i < plugs[0] as u32 {
        avc_bridgeco_fill_msu_addr(addr.as_mut_ptr(), AVC_BRIDGECO_PLUG_DIR_IN, i);
        err = avc_bridgeco_get_plug_type((*bebob).unit, addr.as_mut_ptr(), &mut type_);
        if err < 0 {
            dev_err(
                &mut (*(*(*bebob).unit).device) as *mut _,
                c_str!("fail to get type for MSU in plug %d: %d\n"),
                i,
                err,
            );
            return err;
        }

        if type_ == AVC_BRIDGECO_PLUG_TYPE_SYNC {
            (*bebob).sync_input_plug = i as c_int;
            break;
        }
        i += 1;
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_discover(bebob: *mut snd_bebob) -> c_int {
    let clk_spec: *const snd_bebob_clock_spec = (*(*bebob).spec).clock;
    let mut plugs: [u8; AVC_PLUG_INFO_BUF_BYTES as usize] = [0; AVC_PLUG_INFO_BUF_BYTES as usize];
    let mut addr: [u8; AVC_BRIDGECO_ADDR_BYTES as usize] = [0; AVC_BRIDGECO_ADDR_BYTES as usize];
    let mut err: c_int;

    /* the number of plugs for isoc in/out, ext in/out  */
    err = avc_general_get_plug_info((*bebob).unit, 0x1f, 0x07, 0x00, plugs.as_mut_ptr());
    if err < 0 {
        dev_err(
            &mut (*(*(*bebob).unit).device) as *mut _,
            c_str!("fail to get info for isoc/external in/out plugs: %d\n"),
            err,
        );
        return err;
    }

    /*
     * This module supports at least one isoc input plug and one isoc
     * output plug.
     */
    if plugs[0] == 0 || plugs[1] == 0 {
        return -ENOSYS;
    }

    err = fill_stream_formations(
        bebob,
        addr.as_mut_ptr(),
        AVC_BRIDGECO_PLUG_DIR_IN,
        0,
        (*bebob).rx_stream_formations.as_mut_ptr(),
    );
    if err < 0 {
        return err;
    }

    err = fill_stream_formations(
        bebob,
        addr.as_mut_ptr(),
        AVC_BRIDGECO_PLUG_DIR_OUT,
        0,
        (*bebob).tx_stream_formations.as_mut_ptr(),
    );
    if err < 0 {
        return err;
    }

    err = detect_midi_ports(
        bebob,
        (*bebob).tx_stream_formations.as_ptr(),
        addr.as_mut_ptr(),
        AVC_BRIDGECO_PLUG_DIR_IN,
        plugs[2] as u32,
        &mut (*bebob).midi_input_ports,
    );
    if err < 0 {
        return err;
    }

    err = detect_midi_ports(
        bebob,
        (*bebob).rx_stream_formations.as_ptr(),
        addr.as_mut_ptr(),
        AVC_BRIDGECO_PLUG_DIR_OUT,
        plugs[3] as u32,
        &mut (*bebob).midi_output_ports,
    );
    if err < 0 {
        return err;
    }

    /* for check source of clock later */
    if clk_spec.is_null() {
        err = seek_msu_sync_input_plug(bebob);
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_lock_changed(bebob: *mut snd_bebob) {
    (*bebob).dev_lock_changed = true;
    wake_up(&mut (*bebob).hwdep_wait);
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_lock_try(bebob: *mut snd_bebob) -> c_int {
    let _guard = spinlock_irq_guard(&mut (*bebob).lock);

    /* user land lock this */
    if (*bebob).dev_lock_count < 0 {
        return -EBUSY;
    }

    /* this is the first time */
    if {
        let old = (*bebob).dev_lock_count;
        (*bebob).dev_lock_count += 1;
        old
    } == 0
    {
        snd_bebob_stream_lock_changed(bebob);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_stream_lock_release(bebob: *mut snd_bebob) {
    let _guard = spinlock_irq_guard(&mut (*bebob).lock);

    if WARN_ON((*bebob).dev_lock_count <= 0) {
        return;
    }
    (*bebob).dev_lock_count -= 1;
    if (*bebob).dev_lock_count == 0 {
        snd_bebob_stream_lock_changed(bebob);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
