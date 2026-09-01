// SPDX-License-Identifier: GPL-2.0-only
/*
 * oxfw_stream.c - a part of driver for OXFW970/971 based devices
 *
 * Copyright (c) 2014 Takashi Sakamoto
 */

// Translated from C implementation source. Dependencies from "oxfw.h" and
// <linux/delay.h> are expected to be supplied by surrounding bindings.

const AVC_GENERIC_FRAME_MAXIMUM_BYTES: u32 = 512;
const READY_TIMEOUT_MS: u32 = 600;

/*
 * According to datasheet of Oxford Semiconductor:
 *  OXFW970: 32.0/44.1/48.0/96.0 Khz, 8 audio channels I/O
 *  OXFW971: 32.0/44.1/48.0/88.2/96.0/192.0 kHz, 16 audio channels I/O, MIDI I/O
 */
static oxfw_rate_table: [u32; 6] = [32000, 44100, 48000, 88200, 96000, 192000];

/*
 * See Table 5.7 – Sampling frequency for Multi-bit Audio
 * in AV/C Stream Format Information Specification 1.1 (Apr 2005, 1394TA)
 */
static avc_stream_rate_table: [u32; 6] = [0x02, 0x03, 0x04, 0x0a, 0x05, 0x07];

unsafe fn set_rate(oxfw: *mut snd_oxfw, rate: u32) -> i32 {
    let mut err: i32;

    err = avc_general_set_sig_fmt(
        (*oxfw).unit,
        rate,
        AVC_GENERAL_PLUG_DIR_IN,
        0,
    );
    if err < 0 {
        return err;
    }

    if (*oxfw).has_output {
        err = avc_general_set_sig_fmt(
            (*oxfw).unit,
            rate,
            AVC_GENERAL_PLUG_DIR_OUT,
            0,
        );
    }

    err
}

unsafe fn set_stream_format(
    oxfw: *mut snd_oxfw,
    s: *mut amdtp_stream,
    rate: u32,
    pcm_channels: u32,
) -> i32 {
    let formats: *mut *mut u8;
    let mut formation: snd_oxfw_stream_formation = core::mem::zeroed();
    let dir: avc_general_plug_dir;
    let len: u32;
    let mut i: i32;
    let mut err: i32;

    if s == core::ptr::addr_of_mut!((*oxfw).tx_stream) {
        formats = (*oxfw).tx_stream_formats.as_mut_ptr();
        dir = AVC_GENERAL_PLUG_DIR_OUT;
    } else {
        formats = (*oxfw).rx_stream_formats.as_mut_ptr();
        dir = AVC_GENERAL_PLUG_DIR_IN;
    }

    /* Seek stream format for requirements. */
    i = 0;
    while i < SND_OXFW_STREAM_FORMAT_ENTRIES as i32 {
        err = snd_oxfw_stream_parse_format(*formats.offset(i as isize), &mut formation);
        if err < 0 {
            return err;
        }

        if formation.rate == rate && formation.pcm == pcm_channels {
            break;
        }
        i += 1;
    }
    if i == SND_OXFW_STREAM_FORMAT_ENTRIES as i32 {
        return -EINVAL;
    }

    /* If assumed, just change rate. */
    if (*oxfw).assumed {
        return set_rate(oxfw, rate);
    }

    /* Calculate format length. */
    len = 5 + *(*formats.offset(i as isize)).add(4) as u32 * 2;

    err = avc_stream_set_format((*oxfw).unit, dir, 0, *formats.offset(i as isize), len);
    if err < 0 {
        return err;
    }

    /* Some requests just after changing format causes freezing. */
    msleep(100);

    0
}

unsafe fn start_stream(oxfw: *mut snd_oxfw, stream: *mut amdtp_stream) -> i32 {
    let conn: *mut cmp_connection;
    let mut err: i32;

    if stream == core::ptr::addr_of_mut!((*oxfw).rx_stream) {
        conn = core::ptr::addr_of_mut!((*oxfw).in_conn);
    } else {
        conn = core::ptr::addr_of_mut!((*oxfw).out_conn);
    }

    err = cmp_connection_establish(conn);
    if err < 0 {
        return err;
    }

    err = amdtp_domain_add_stream(
        core::ptr::addr_of_mut!((*oxfw).domain),
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

unsafe fn check_connection_used_by_others(
    oxfw: *mut snd_oxfw,
    stream: *mut amdtp_stream,
) -> i32 {
    let conn: *mut cmp_connection;
    let mut used: bool = false;
    let mut err: i32;

    if stream == core::ptr::addr_of_mut!((*oxfw).tx_stream) {
        conn = core::ptr::addr_of_mut!((*oxfw).out_conn);
    } else {
        conn = core::ptr::addr_of_mut!((*oxfw).in_conn);
    }

    err = cmp_connection_check_used(conn, &mut used);
    if err >= 0 && used && !amdtp_stream_running(stream) {
        dev_err!(
            core::ptr::addr_of_mut!((*(*oxfw).unit).device),
            "Connection established by others: %cPCR[%d]\n",
            if (*conn).direction == CMP_OUTPUT { 'o' } else { 'i' },
            (*conn).pcr_index
        );
        err = -EBUSY;
    }

    err
}

unsafe fn init_stream(oxfw: *mut snd_oxfw, stream: *mut amdtp_stream) -> i32 {
    let conn: *mut cmp_connection;
    let c_dir: cmp_direction;
    let s_dir: amdtp_stream_direction;
    let mut flags: u32 = 0;
    let mut err: i32;

    if (*oxfw).quirks & SND_OXFW_QUIRK_BLOCKING_TRANSMISSION == 0 {
        flags |= CIP_NONBLOCKING;
    } else {
        flags |= CIP_BLOCKING;
    }

    // OXFW 970/971 has no function to generate playback timing according to the sequence
    // of value in syt field, thus the packet should include NO_INFO value in the field.
    // However, some models just ignore data blocks in packet with NO_INFO for audio data
    // processing.
    if (*oxfw).quirks & SND_OXFW_QUIRK_IGNORE_NO_INFO_PACKET == 0 {
        flags |= CIP_UNAWARE_SYT;
    }

    if stream == core::ptr::addr_of_mut!((*oxfw).tx_stream) {
        conn = core::ptr::addr_of_mut!((*oxfw).out_conn);
        c_dir = CMP_OUTPUT;
        s_dir = AMDTP_IN_STREAM;

        if (*oxfw).quirks & SND_OXFW_QUIRK_JUMBO_PAYLOAD != 0 {
            flags |= CIP_JUMBO_PAYLOAD;
        }
        if (*oxfw).quirks & SND_OXFW_QUIRK_WRONG_DBS != 0 {
            flags |= CIP_WRONG_DBS;
        }
        if (*oxfw).quirks & SND_OXFW_QUIRK_DBC_IS_TOTAL_PAYLOAD_QUADLETS != 0 {
            flags |= CIP_DBC_IS_END_EVENT | CIP_DBC_IS_PAYLOAD_QUADLETS;
        }
    } else {
        conn = core::ptr::addr_of_mut!((*oxfw).in_conn);
        c_dir = CMP_INPUT;
        s_dir = AMDTP_OUT_STREAM;
    }

    err = cmp_connection_init(conn, (*oxfw).unit, c_dir, 0);
    if err < 0 {
        return err;
    }

    err = amdtp_am824_init(stream, (*oxfw).unit, s_dir, flags);
    if err < 0 {
        cmp_connection_destroy(conn);
        return err;
    }

    0
}

unsafe fn keep_resources(oxfw: *mut snd_oxfw, stream: *mut amdtp_stream) -> i32 {
    let dir: avc_general_plug_dir;
    let formats: *mut *mut u8;
    let mut formation: snd_oxfw_stream_formation = core::mem::zeroed();
    let conn: *mut cmp_connection;
    let mut i: i32;
    let mut err: i32;

    if stream == core::ptr::addr_of_mut!((*oxfw).rx_stream) {
        dir = AVC_GENERAL_PLUG_DIR_IN;
        formats = (*oxfw).rx_stream_formats.as_mut_ptr();
        conn = core::ptr::addr_of_mut!((*oxfw).in_conn);
    } else {
        dir = AVC_GENERAL_PLUG_DIR_OUT;
        formats = (*oxfw).tx_stream_formats.as_mut_ptr();
        conn = core::ptr::addr_of_mut!((*oxfw).out_conn);
    }

    err = snd_oxfw_stream_get_current_formation(oxfw, dir, &mut formation);
    if err < 0 {
        return err;
    }

    i = 0;
    while i < SND_OXFW_STREAM_FORMAT_ENTRIES as i32 {
        let mut fmt: snd_oxfw_stream_formation = core::mem::zeroed();

        if (*formats.offset(i as isize)).is_null() {
            break;
        }

        err = snd_oxfw_stream_parse_format(*formats.offset(i as isize), &mut fmt);
        if err < 0 {
            return err;
        }

        if fmt.rate == formation.rate && fmt.pcm == formation.pcm && fmt.midi == formation.midi {
            break;
        }
        i += 1;
    }
    if i == SND_OXFW_STREAM_FORMAT_ENTRIES as i32 {
        return -EINVAL;
    }

    // The stream should have one pcm channels at least.
    if formation.pcm == 0 {
        return -EINVAL;
    }

    err = amdtp_am824_set_parameters(
        stream,
        formation.rate,
        formation.pcm,
        formation.midi * 8,
        false,
    );
    if err < 0 {
        return err;
    }

    cmp_connection_reserve(conn, amdtp_stream_get_max_payload(stream))
}

pub unsafe fn snd_oxfw_stream_reserve_duplex(
    oxfw: *mut snd_oxfw,
    stream: *mut amdtp_stream,
    mut rate: u32,
    mut pcm_channels: u32,
    frames_per_period: u32,
    frames_per_buffer: u32,
) -> i32 {
    let mut formation: snd_oxfw_stream_formation = core::mem::zeroed();
    let dir: avc_general_plug_dir;
    let mut err: i32;

    // Considering JACK/FFADO streaming:
    // TODO: This can be removed hwdep functionality becomes popular.
    err = check_connection_used_by_others(oxfw, core::ptr::addr_of_mut!((*oxfw).rx_stream));
    if err < 0 {
        return err;
    }
    if (*oxfw).has_output {
        err = check_connection_used_by_others(oxfw, core::ptr::addr_of_mut!((*oxfw).tx_stream));
        if err < 0 {
            return err;
        }
    }

    if stream == core::ptr::addr_of_mut!((*oxfw).tx_stream) {
        dir = AVC_GENERAL_PLUG_DIR_OUT;
    } else {
        dir = AVC_GENERAL_PLUG_DIR_IN;
    }

    err = snd_oxfw_stream_get_current_formation(oxfw, dir, &mut formation);
    if err < 0 {
        return err;
    }
    if rate == 0 {
        rate = formation.rate;
        pcm_channels = formation.pcm;
    }
    if formation.rate != rate || formation.pcm != pcm_channels {
        amdtp_domain_stop(core::ptr::addr_of_mut!((*oxfw).domain));

        cmp_connection_break(core::ptr::addr_of_mut!((*oxfw).in_conn));
        cmp_connection_release(core::ptr::addr_of_mut!((*oxfw).in_conn));

        if (*oxfw).has_output {
            cmp_connection_break(core::ptr::addr_of_mut!((*oxfw).out_conn));
            cmp_connection_release(core::ptr::addr_of_mut!((*oxfw).out_conn));
        }
    }

    if (*oxfw).substreams_count == 0 || formation.rate != rate || formation.pcm != pcm_channels {
        err = set_stream_format(oxfw, stream, rate, pcm_channels);
        if err < 0 {
            dev_err!(
                core::ptr::addr_of_mut!((*(*oxfw).unit).device),
                "fail to set stream format: %d\n",
                err
            );
            return err;
        }

        err = keep_resources(oxfw, core::ptr::addr_of_mut!((*oxfw).rx_stream));
        if err < 0 {
            return err;
        }

        if (*oxfw).has_output {
            err = keep_resources(oxfw, core::ptr::addr_of_mut!((*oxfw).tx_stream));
            if err < 0 {
                cmp_connection_release(core::ptr::addr_of_mut!((*oxfw).in_conn));
                return err;
            }
        }

        err = amdtp_domain_set_events_per_period(
            core::ptr::addr_of_mut!((*oxfw).domain),
            frames_per_period,
            frames_per_buffer,
        );
        if err < 0 {
            cmp_connection_release(core::ptr::addr_of_mut!((*oxfw).in_conn));
            if (*oxfw).has_output {
                cmp_connection_release(core::ptr::addr_of_mut!((*oxfw).out_conn));
            }
            return err;
        }
    }

    0
}

pub unsafe fn snd_oxfw_stream_start_duplex(oxfw: *mut snd_oxfw) -> i32 {
    let mut err: i32;

    if (*oxfw).substreams_count == 0 {
        return -EIO;
    }

    if amdtp_streaming_error(core::ptr::addr_of_mut!((*oxfw).rx_stream))
        || amdtp_streaming_error(core::ptr::addr_of_mut!((*oxfw).tx_stream))
    {
        amdtp_domain_stop(core::ptr::addr_of_mut!((*oxfw).domain));

        cmp_connection_break(core::ptr::addr_of_mut!((*oxfw).in_conn));
        if (*oxfw).has_output {
            cmp_connection_break(core::ptr::addr_of_mut!((*oxfw).out_conn));
        }
    }

    if !amdtp_stream_running(core::ptr::addr_of_mut!((*oxfw).rx_stream)) {
        let mut tx_init_skip_cycles: u32 = 0;
        let mut replay_seq: bool = false;

        err = start_stream(oxfw, core::ptr::addr_of_mut!((*oxfw).rx_stream));
        if err < 0 {
            dev_err!(
                core::ptr::addr_of_mut!((*(*oxfw).unit).device),
                "fail to prepare rx stream: %d\n",
                err
            );
            goto_error_start_duplex(oxfw, err);
            return err;
        }

        if (*oxfw).has_output
            && !amdtp_stream_running(core::ptr::addr_of_mut!((*oxfw).tx_stream))
        {
            err = start_stream(oxfw, core::ptr::addr_of_mut!((*oxfw).tx_stream));
            if err < 0 {
                dev_err!(
                    core::ptr::addr_of_mut!((*(*oxfw).unit).device),
                    "fail to prepare tx stream: %d\n",
                    err
                );
                goto_error_start_duplex(oxfw, err);
                return err;
            }

            if (*oxfw).quirks & SND_OXFW_QUIRK_JUMBO_PAYLOAD != 0 {
                // Just after changing sampling transfer frequency, many cycles are
                // skipped for packet transmission.
                tx_init_skip_cycles = 400;
            } else if (*oxfw).quirks & SND_OXFW_QUIRK_VOLUNTARY_RECOVERY != 0 {
                // It takes a bit time for target device to adjust event frequency
                // according to nominal event frequency in isochronous packets from
                // ALSA oxfw driver.
                tx_init_skip_cycles = 4000;
            } else {
                replay_seq = true;
            }
        }

        // NOTE: The device ignores presentation time expressed by the value of syt field
        // of CIP header in received packets. The sequence of the number of data blocks per
        // packet is important for media clock recovery.
        err = amdtp_domain_start(
            core::ptr::addr_of_mut!((*oxfw).domain),
            tx_init_skip_cycles,
            replay_seq,
            false,
        );
        if err < 0 {
            goto_error_start_duplex(oxfw, err);
            return err;
        }

        if !amdtp_domain_wait_ready(core::ptr::addr_of_mut!((*oxfw).domain), READY_TIMEOUT_MS) {
            err = -ETIMEDOUT;
            goto_error_start_duplex(oxfw, err);
            return err;
        }
    }

    0
}

unsafe fn goto_error_start_duplex(oxfw: *mut snd_oxfw, err: i32) -> i32 {
    amdtp_domain_stop(core::ptr::addr_of_mut!((*oxfw).domain));

    cmp_connection_break(core::ptr::addr_of_mut!((*oxfw).in_conn));
    if (*oxfw).has_output {
        cmp_connection_break(core::ptr::addr_of_mut!((*oxfw).out_conn));
    }

    err
}

pub unsafe fn snd_oxfw_stream_stop_duplex(oxfw: *mut snd_oxfw) {
    if (*oxfw).substreams_count == 0 {
        amdtp_domain_stop(core::ptr::addr_of_mut!((*oxfw).domain));

        cmp_connection_break(core::ptr::addr_of_mut!((*oxfw).in_conn));
        cmp_connection_release(core::ptr::addr_of_mut!((*oxfw).in_conn));

        if (*oxfw).has_output {
            cmp_connection_break(core::ptr::addr_of_mut!((*oxfw).out_conn));
            cmp_connection_release(core::ptr::addr_of_mut!((*oxfw).out_conn));
        }
    }
}

unsafe fn destroy_stream(oxfw: *mut snd_oxfw, stream: *mut amdtp_stream) {
    let conn: *mut cmp_connection;

    if stream == core::ptr::addr_of_mut!((*oxfw).tx_stream) {
        conn = core::ptr::addr_of_mut!((*oxfw).out_conn);
    } else {
        conn = core::ptr::addr_of_mut!((*oxfw).in_conn);
    }

    amdtp_stream_destroy(stream);
    cmp_connection_destroy(conn);
}

pub unsafe fn snd_oxfw_stream_init_duplex(oxfw: *mut snd_oxfw) -> i32 {
    let mut err: i32;

    err = init_stream(oxfw, core::ptr::addr_of_mut!((*oxfw).rx_stream));
    if err < 0 {
        return err;
    }

    if (*oxfw).has_output {
        err = init_stream(oxfw, core::ptr::addr_of_mut!((*oxfw).tx_stream));
        if err < 0 {
            destroy_stream(oxfw, core::ptr::addr_of_mut!((*oxfw).rx_stream));
            return err;
        }
    }

    err = amdtp_domain_init(core::ptr::addr_of_mut!((*oxfw).domain));
    if err < 0 {
        destroy_stream(oxfw, core::ptr::addr_of_mut!((*oxfw).rx_stream));
        if (*oxfw).has_output {
            destroy_stream(oxfw, core::ptr::addr_of_mut!((*oxfw).tx_stream));
        }
    }

    err
}

// This function should be called before starting the stream or after stopping
// the streams.
pub unsafe fn snd_oxfw_stream_destroy_duplex(oxfw: *mut snd_oxfw) {
    amdtp_domain_destroy(core::ptr::addr_of_mut!((*oxfw).domain));

    destroy_stream(oxfw, core::ptr::addr_of_mut!((*oxfw).rx_stream));

    if (*oxfw).has_output {
        destroy_stream(oxfw, core::ptr::addr_of_mut!((*oxfw).tx_stream));
    }
}

pub unsafe fn snd_oxfw_stream_update_duplex(oxfw: *mut snd_oxfw) {
    amdtp_domain_stop(core::ptr::addr_of_mut!((*oxfw).domain));

    cmp_connection_break(core::ptr::addr_of_mut!((*oxfw).in_conn));

    amdtp_stream_pcm_abort(core::ptr::addr_of_mut!((*oxfw).rx_stream));

    if (*oxfw).has_output {
        cmp_connection_break(core::ptr::addr_of_mut!((*oxfw).out_conn));

        amdtp_stream_pcm_abort(core::ptr::addr_of_mut!((*oxfw).tx_stream));
    }
}

pub unsafe fn snd_oxfw_stream_get_current_formation(
    oxfw: *mut snd_oxfw,
    dir: avc_general_plug_dir,
    formation: *mut snd_oxfw_stream_formation,
) -> i32 {
    let mut err: i32;

    if (*oxfw).quirks & SND_OXFW_QUIRK_STREAM_FORMAT_INFO_UNSUPPORTED == 0 {
        let format: *mut u8;
        let mut len: u32;

        len = AVC_GENERIC_FRAME_MAXIMUM_BYTES;
        format = kmalloc(len as usize, GFP_KERNEL) as *mut u8;
        if format.is_null() {
            return -ENOMEM;
        }

        err = avc_stream_get_format_single((*oxfw).unit, dir, 0, format, &mut len);
        if err >= 0 {
            if len < 3 {
                err = -EIO;
            } else {
                err = snd_oxfw_stream_parse_format(format, formation);
            }
        }

        kfree(format as *const core::ffi::c_void);
    } else {
        // Miglia Harmony Audio does not support Extended Stream Format Information
        // command. Use the duplicated hard-coded format, instead.
        let mut rate: u32 = 0;
        let formats: *mut *mut u8;
        let mut i: i32;

        err = avc_general_get_sig_fmt((*oxfw).unit, &mut rate, dir, 0);
        if err < 0 {
            return err;
        }

        if dir == AVC_GENERAL_PLUG_DIR_IN {
            formats = (*oxfw).rx_stream_formats.as_mut_ptr();
        } else {
            formats = (*oxfw).tx_stream_formats.as_mut_ptr();
        }

        i = 0;
        while i < SND_OXFW_STREAM_FORMAT_ENTRIES as i32 {
            if (*formats.offset(i as isize)).is_null() {
                i += 1;
                continue;
            }

            err = snd_oxfw_stream_parse_format(*formats.offset(i as isize), formation);
            if err < 0 {
                i += 1;
                continue;
            }

            if (*formation).rate == rate {
                break;
            }
            i += 1;
        }
        if i == SND_OXFW_STREAM_FORMAT_ENTRIES as i32 {
            return -EIO;
        }
    }

    err
}

/*
 * See Table 6.16 - AM824 Stream Format
 *     Figure 6.19 - format_information field for AM824 Compound
 * in AV/C Stream Format Information Specification 1.1 (Apr 2005, 1394TA)
 * Also 'Clause 12 AM824 sequence adaption layers' in IEC 61883-6:2005
 */
pub unsafe fn snd_oxfw_stream_parse_format(
    format: *const u8,
    formation: *mut snd_oxfw_stream_formation,
) -> i32 {
    let mut i: u32;
    let mut e: u32;
    let mut channels: u32;
    let mut type_: u32;

    memset(
        formation as *mut core::ffi::c_void,
        0,
        core::mem::size_of::<snd_oxfw_stream_formation>(),
    );

    /*
     * this module can support a hierarchy combination that:
     *  Root:	Audio and Music (0x90)
     *  Level 1:	AM824 Compound  (0x40)
     */
    if *format.add(0) != 0x90 || *format.add(1) != 0x40 {
        return -ENXIO;
    }

    /* check the sampling rate */
    i = 0;
    while (i as usize) < avc_stream_rate_table.len() {
        if *format.add(2) as u32 == avc_stream_rate_table[i as usize] {
            break;
        }
        i += 1;
    }
    if i as usize == avc_stream_rate_table.len() {
        return -ENXIO;
    }

    (*formation).rate = oxfw_rate_table[i as usize];

    e = 0;
    while e < *format.add(4) as u32 {
        channels = *format.add((5 + e * 2) as usize) as u32;
        type_ = *format.add((6 + e * 2) as usize) as u32;

        match type_ {
            /* IEC 60958 Conformant, currently handled as MBLA */
            0x00 |
            /* Multi Bit Linear Audio (Raw) */
            0x06 => {
                (*formation).pcm += channels;
            }
            /* MIDI Conformant */
            0x0d => {
                (*formation).midi = channels;
            }
            /* IEC 61937-3 to 7 */
            0x01 | 0x02 | 0x03 | 0x04 | 0x05 |
            /* Multi Bit Linear Audio */
            0x07 | /* DVD-Audio */
            0x0c | /* High Precision */
            /* One Bit Audio */
            0x08 | /* (Plain) Raw */
            0x09 | /* (Plain) SACD */
            0x0a | /* (Encoded) Raw */
            0x0b | /* (Encoded) SACD */
            /* SMPTE Time-Code conformant */
            0x0e |
            /* Sample Count */
            0x0f |
            /* Anciliary Data */
            0x10 |
            /* Synchronization Stream (Stereo Raw audio) */
            0x40 |
            /* Don't care */
            0xff |
            _ => {
                return -ENXIO; /* not supported */
            }
        }
        e += 1;
    }

    if (*formation).pcm > AM824_MAX_CHANNELS_FOR_PCM
        || (*formation).midi > AM824_MAX_CHANNELS_FOR_MIDI
    {
        return -ENXIO;
    }

    0
}

unsafe fn assume_stream_formats(
    oxfw: *mut snd_oxfw,
    dir: avc_general_plug_dir,
    pid: u32,
    buf: *mut u8,
    len: *mut u32,
    formats: *mut *mut u8,
) -> i32 {
    let mut formation: snd_oxfw_stream_formation = core::mem::zeroed();
    let mut i: u32;
    let mut eid: u32;
    let mut err: i32;

    // get format at current sampling rate.
    if (*oxfw).quirks & SND_OXFW_QUIRK_STREAM_FORMAT_INFO_UNSUPPORTED == 0 {
        err = avc_stream_get_format_single((*oxfw).unit, dir, pid, buf, len);
        if err < 0 {
            dev_err!(
                core::ptr::addr_of_mut!((*(*oxfw).unit).device),
                "fail to get current stream format for isoc %s plug %d:%d\n",
                if dir == AVC_GENERAL_PLUG_DIR_IN { "in" } else { "out" },
                pid,
                err
            );
            return err;
        }
    } else {
        // Miglia Harmony Audio does not support Extended Stream Format Information
        // command. Use the hard-coded format, instead.
        *buf.add(0) = 0x90;
        *buf.add(1) = 0x40;
        *buf.add(2) = avc_stream_rate_table[0] as u8;
        *buf.add(3) = 0x00;
        *buf.add(4) = 0x01;

        if dir == AVC_GENERAL_PLUG_DIR_IN {
            *buf.add(5) = 0x08;
        } else {
            *buf.add(5) = 0x02;
        }

        *buf.add(6) = 0x06;

        *len = 7;
    }

    /* parse and set stream format */
    eid = 0;
    err = snd_oxfw_stream_parse_format(buf, &mut formation);
    if err < 0 {
        return err;
    }

    *formats.offset(eid as isize) = devm_kmemdup(
        core::ptr::addr_of_mut!((*(*oxfw).card).card_dev),
        buf as *const core::ffi::c_void,
        *len as usize,
        GFP_KERNEL,
    ) as *mut u8;
    if (*formats.offset(eid as isize)).is_null() {
        return -ENOMEM;
    }

    /* apply the format for each available sampling rate */
    i = 0;
    while (i as usize) < oxfw_rate_table.len() {
        if formation.rate == oxfw_rate_table[i as usize] {
            i += 1;
            continue;
        }

        err = avc_general_inquiry_sig_fmt((*oxfw).unit, oxfw_rate_table[i as usize], dir, pid);
        if err < 0 {
            i += 1;
            continue;
        }

        eid += 1;
        *formats.offset(eid as isize) = devm_kmemdup(
            core::ptr::addr_of_mut!((*(*oxfw).card).card_dev),
            buf as *const core::ffi::c_void,
            *len as usize,
            GFP_KERNEL,
        ) as *mut u8;
        if (*formats.offset(eid as isize)).is_null() {
            return -ENOMEM;
        }
        *(*formats.offset(eid as isize)).add(2) = avc_stream_rate_table[i as usize] as u8;
        i += 1;
    }

    err = 0;
    (*oxfw).assumed = true;
    err
}

unsafe fn fill_stream_formats(
    oxfw: *mut snd_oxfw,
    dir: avc_general_plug_dir,
    pid: u16,
) -> i32 {
    let buf: *mut u8;
    let formats: *mut *mut u8;
    let mut len: u32;
    let mut eid: u32 = 0;
    let mut dummy: snd_oxfw_stream_formation = core::mem::zeroed();
    let mut err: i32;

    buf = kmalloc(AVC_GENERIC_FRAME_MAXIMUM_BYTES as usize, GFP_KERNEL) as *mut u8;
    if buf.is_null() {
        return -ENOMEM;
    }

    if dir == AVC_GENERAL_PLUG_DIR_OUT {
        formats = (*oxfw).tx_stream_formats.as_mut_ptr();
    } else {
        formats = (*oxfw).rx_stream_formats.as_mut_ptr();
    }

    /* get first entry */
    len = AVC_GENERIC_FRAME_MAXIMUM_BYTES;
    err = avc_stream_get_format_list((*oxfw).unit, dir, 0, buf, &mut len, 0);
    if err == -ENXIO {
        /* LIST subfunction is not implemented */
        len = AVC_GENERIC_FRAME_MAXIMUM_BYTES;
        err = assume_stream_formats(oxfw, dir, pid as u32, buf, &mut len, formats);
        kfree(buf as *const core::ffi::c_void);
        return err;
    } else if err < 0 {
        dev_err!(
            core::ptr::addr_of_mut!((*(*oxfw).unit).device),
            "fail to get stream format %d for isoc %s plug %d:%d\n",
            eid,
            if dir == AVC_GENERAL_PLUG_DIR_IN { "in" } else { "out" },
            pid,
            err
        );
        kfree(buf as *const core::ffi::c_void);
        return err;
    }

    /* LIST subfunction is implemented */
    while eid < SND_OXFW_STREAM_FORMAT_ENTRIES {
        /* The format is too short. */
        if len < 3 {
            err = -EIO;
            break;
        }

        /* parse and set stream format */
        err = snd_oxfw_stream_parse_format(buf, &mut dummy);
        if err < 0 {
            break;
        }

        *formats.offset(eid as isize) = devm_kmemdup(
            core::ptr::addr_of_mut!((*(*oxfw).card).card_dev),
            buf as *const core::ffi::c_void,
            len as usize,
            GFP_KERNEL,
        ) as *mut u8;
        if (*formats.offset(eid as isize)).is_null() {
            err = -ENOMEM;
            break;
        }

        /* get next entry */
        len = AVC_GENERIC_FRAME_MAXIMUM_BYTES;
        eid += 1;
        err = avc_stream_get_format_list((*oxfw).unit, dir, 0, buf, &mut len, eid);
        /* No entries remained. */
        if err == -EINVAL {
            err = 0;
            break;
        } else if err < 0 {
            dev_err!(
                core::ptr::addr_of_mut!((*(*oxfw).unit).device),
                "fail to get stream format %d for isoc %s plug %d:%d\n",
                eid,
                if dir == AVC_GENERAL_PLUG_DIR_IN { "in" } else { "out" },
                pid,
                err
            );
            break;
        }
    }

    kfree(buf as *const core::ffi::c_void);
    err
}

pub unsafe fn snd_oxfw_stream_discover(oxfw: *mut snd_oxfw) -> i32 {
    let mut plugs: [u8; AVC_PLUG_INFO_BUF_BYTES as usize] = [0; AVC_PLUG_INFO_BUF_BYTES as usize];
    let mut formation: snd_oxfw_stream_formation = core::mem::zeroed();
    let mut format: *mut u8;
    let mut i: u32;
    let mut err: i32;

    /* the number of plugs for isoc in/out, ext in/out  */
    err = avc_general_get_plug_info((*oxfw).unit, 0x1f, 0x07, 0x00, plugs.as_mut_ptr());
    if err < 0 {
        dev_err!(
            core::ptr::addr_of_mut!((*(*oxfw).unit).device),
            "fail to get info for isoc/external in/out plugs: %d\n",
            err
        );
        return err;
    } else if plugs[0] == 0 && plugs[1] == 0 {
        return -ENXIO;
    }

    /* use oPCR[0] if exists */
    if plugs[1] > 0 {
        err = fill_stream_formats(oxfw, AVC_GENERAL_PLUG_DIR_OUT, 0);
        if err < 0 {
            if err != -ENXIO {
                return err;
            }

            // The oPCR is not available for isoc communication.
            err = 0;
        } else {
            i = 0;
            while i < SND_OXFW_STREAM_FORMAT_ENTRIES {
                format = (*oxfw).tx_stream_formats[i as usize];
                if format.is_null() {
                    i += 1;
                    continue;
                }
                err = snd_oxfw_stream_parse_format(format, &mut formation);
                if err < 0 {
                    i += 1;
                    continue;
                }

                /* Add one MIDI port. */
                if formation.midi > 0 {
                    (*oxfw).midi_input_ports = 1;
                }
                i += 1;
            }

            (*oxfw).has_output = true;
        }
    }

    /* use iPCR[0] if exists */
    if plugs[0] > 0 {
        err = fill_stream_formats(oxfw, AVC_GENERAL_PLUG_DIR_IN, 0);
        if err < 0 {
            if err != -ENXIO {
                return err;
            }

            // The iPCR is not available for isoc communication.
            err = 0;
        } else {
            i = 0;
            while i < SND_OXFW_STREAM_FORMAT_ENTRIES {
                format = (*oxfw).rx_stream_formats[i as usize];
                if format.is_null() {
                    i += 1;
                    continue;
                }
                err = snd_oxfw_stream_parse_format(format, &mut formation);
                if err < 0 {
                    i += 1;
                    continue;
                }

                /* Add one MIDI port. */
                if formation.midi > 0 {
                    (*oxfw).midi_output_ports = 1;
                }
                i += 1;
            }

            (*oxfw).has_input = true;
        }
    }

    err
}

pub unsafe fn snd_oxfw_stream_lock_changed(oxfw: *mut snd_oxfw) {
    (*oxfw).dev_lock_changed = true;
    wake_up(core::ptr::addr_of_mut!((*oxfw).hwdep_wait));
}

pub unsafe fn snd_oxfw_stream_lock_try(oxfw: *mut snd_oxfw) -> i32 {
    let _guard = guard_spinlock_irq(core::ptr::addr_of_mut!((*oxfw).lock));

    /* user land lock this */
    if (*oxfw).dev_lock_count < 0 {
        return -EBUSY;
    }

    /* this is the first time */
    let old = (*oxfw).dev_lock_count;
    (*oxfw).dev_lock_count += 1;
    if old == 0 {
        snd_oxfw_stream_lock_changed(oxfw);
    }
    0
}

pub unsafe fn snd_oxfw_stream_lock_release(oxfw: *mut snd_oxfw) {
    let _guard = guard_spinlock_irq(core::ptr::addr_of_mut!((*oxfw).lock));

    if WARN_ON((*oxfw).dev_lock_count <= 0) {
        return;
    }
    (*oxfw).dev_lock_count -= 1;
    if (*oxfw).dev_lock_count == 0 {
        snd_oxfw_stream_lock_changed(oxfw);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
