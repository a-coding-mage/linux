// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Linux USB driver
 *
 * Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
 */

// Dependencies: <sound/pcm.h>, "driver.h"

/*
 * When the TonePort is used with jack in full duplex mode and the outputs are
 * not connected, the software monitor produces an ugly noise since everything
 * written to the output buffer (i.e., the input signal) will be repeated in
 * the next period (sounds like a delay effect). As a workaround, the output
 * buffer is cleared after the data have been read, but there must be a better
 * solution. Until one is found, this workaround can be used to fix the
 * problem.
 */
pub const USE_CLEAR_BUFFER_WORKAROUND: u32 = 1;

// Opaque types from external dependencies
pub struct snd_pcm_ops;
pub struct snd_line6_pcm;

extern "C" {
    pub static snd_line6_playback_ops: snd_pcm_ops;

    pub fn line6_create_audio_out_urbs(line6pcm: *mut snd_line6_pcm) -> i32;
    pub fn line6_submit_audio_out_all_urbs(line6pcm: *mut snd_line6_pcm) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
