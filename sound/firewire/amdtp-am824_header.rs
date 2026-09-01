/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <sound/pcm.h>
// #include <sound/rawmidi.h>
// #include "amdtp-stream.h"

pub const AM824_IN_PCM_FORMAT_BITS: u64 = SNDRV_PCM_FMTBIT_S32;

pub const AM824_OUT_PCM_FORMAT_BITS: u64 = SNDRV_PCM_FMTBIT_S32;

/*
 * This module supports maximum 64 PCM channels for one PCM stream
 * This is for our convenience.
 */
pub const AM824_MAX_CHANNELS_FOR_PCM: u32 = 64;

/*
 * AMDTP packet can include channels for MIDI conformant data.
 * Each MIDI conformant data channel includes 8 MPX-MIDI data stream.
 * Each MPX-MIDI data stream includes one data stream from/to MIDI ports.
 *
 * This module supports maximum 1 MIDI conformant data channels.
 * Then this AMDTP packets can transfer maximum 8 MIDI data streams.
 */
pub const AM824_MAX_CHANNELS_FOR_MIDI: u32 = 1;

unsafe extern "C" {
    pub static SNDRV_PCM_FMTBIT_S32: u64;

    pub fn amdtp_am824_set_parameters(
        s: *mut amdtp_stream,
        rate: ::std::os::raw::c_uint,
        pcm_channels: ::std::os::raw::c_uint,
        midi_ports: ::std::os::raw::c_uint,
        double_pcm_frames: bool,
    ) -> ::std::os::raw::c_int;

    pub fn amdtp_am824_set_pcm_position(
        s: *mut amdtp_stream,
        index: ::std::os::raw::c_uint,
        position: ::std::os::raw::c_uint,
    );

    pub fn amdtp_am824_set_midi_position(
        s: *mut amdtp_stream,
        position: ::std::os::raw::c_uint,
    );

    pub fn amdtp_am824_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> ::std::os::raw::c_int;

    pub fn amdtp_am824_midi_trigger(
        s: *mut amdtp_stream,
        port: ::std::os::raw::c_uint,
        midi: *mut snd_rawmidi_substream,
    );

    pub fn amdtp_am824_init(
        s: *mut amdtp_stream,
        unit: *mut fw_unit,
        dir: amdtp_stream_direction,
        flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
