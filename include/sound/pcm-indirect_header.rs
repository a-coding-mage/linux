/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Helper functions for indirect PCM data transfer
 *
 *  Copyright (c) by Takashi Iwai <tiwai@suse.de>
 *                   Jaroslav Kysela <perex@perex.cz>
 */

// Dependency: snd_pcm_uframes_t, snd_pcm_sframes_t, snd_pcm_substream,
// snd_pcm_runtime, frames_to_bytes, bytes_to_frames, EPIPE, and
// SNDRV_PCM_POS_XRUN are supplied by sound/pcm.h and related dependencies.

#[repr(C)]
pub struct snd_pcm_indirect {
    pub hw_buffer_size: u32, /* Byte size of hardware buffer */
    pub hw_queue_size: u32,  /* Max queue size of hw buffer (0 = buffer size) */
    pub hw_data: u32,        /* Offset to next dst (or src) in hw ring buffer */
    pub hw_io: u32,          /* Ring buffer hw pointer */
    pub hw_ready: i32,       /* Bytes ready for play (or captured) in hw ring buffer */
    pub sw_buffer_size: u32, /* Byte size of software buffer */
    pub sw_data: u32,        /* Offset to next dst (or src) in sw ring buffer */
    pub sw_io: u32,          /* Current software pointer in bytes */
    pub sw_ready: i32,       /* Bytes ready to be transferred to/from hw */
    pub appl_ptr: snd_pcm_uframes_t, /* Last seen appl_ptr */
}

pub type snd_pcm_indirect_copy_t = unsafe extern "C" fn(
    substream: *mut snd_pcm_substream,
    rec: *mut snd_pcm_indirect,
    bytes: usize,
);

/* helper function for playback ack callback */
#[inline]
pub unsafe fn snd_pcm_indirect_playback_transfer(
    substream: *mut snd_pcm_substream,
    rec: *mut snd_pcm_indirect,
    copy: snd_pcm_indirect_copy_t,
) -> i32 {
    let runtime = (*substream).runtime;
    let appl_ptr = (*(*runtime).control).appl_ptr;
    let mut diff: snd_pcm_sframes_t = appl_ptr.wrapping_sub((*rec).appl_ptr) as snd_pcm_sframes_t;
    let qsize: i32;

    if diff != 0 {
        if diff < -((*runtime).boundary / 2) as snd_pcm_sframes_t {
            diff = diff.wrapping_add((*runtime).boundary as snd_pcm_sframes_t);
        }
        if diff < 0 {
            return -EPIPE;
        }
        (*rec).sw_ready += frames_to_bytes(runtime, diff) as i32;
        (*rec).appl_ptr = appl_ptr;
    }
    qsize = if (*rec).hw_queue_size != 0 {
        (*rec).hw_queue_size as i32
    } else {
        (*rec).hw_buffer_size as i32
    };
    while (*rec).hw_ready < qsize && (*rec).sw_ready > 0 {
        let hw_to_end = (*rec).hw_buffer_size - (*rec).hw_data;
        let sw_to_end = (*rec).sw_buffer_size - (*rec).sw_data;
        let mut bytes = (qsize - (*rec).hw_ready) as u32;
        if (*rec).sw_ready < bytes as i32 { bytes = (*rec).sw_ready as u32; }
        if hw_to_end < bytes { bytes = hw_to_end; }
        if sw_to_end < bytes { bytes = sw_to_end; }
        if bytes == 0 { break; }
        copy(substream, rec, bytes as usize);
        (*rec).hw_data += bytes;
        if (*rec).hw_data == (*rec).hw_buffer_size { (*rec).hw_data = 0; }
        (*rec).sw_data += bytes;
        if (*rec).sw_data == (*rec).sw_buffer_size { (*rec).sw_data = 0; }
        (*rec).hw_ready += bytes as i32;
        (*rec).sw_ready -= bytes as i32;
    }
    0
}

/* helper function for playback pointer callback
 * ptr = current byte pointer
 */
#[inline]
pub unsafe fn snd_pcm_indirect_playback_pointer(
    substream: *mut snd_pcm_substream,
    rec: *mut snd_pcm_indirect,
    ptr: u32,
) -> snd_pcm_uframes_t {
    let mut bytes = ptr as i32 - (*rec).hw_io as i32;
    if bytes < 0 { bytes += (*rec).hw_buffer_size as i32; }
    (*rec).hw_io = ptr;
    (*rec).hw_ready -= bytes;
    (*rec).sw_io += bytes as u32;
    if (*rec).sw_io >= (*rec).sw_buffer_size { (*rec).sw_io -= (*rec).sw_buffer_size; }
    if let Some(ack) = (*(*substream).ops).ack {
        let err = ack(substream);
        if err == -EPIPE { return SNDRV_PCM_POS_XRUN; }
    }
    bytes_to_frames((*substream).runtime, (*rec).sw_io)
}

/* helper function for capture ack callback */
#[inline]
pub unsafe fn snd_pcm_indirect_capture_transfer(
    substream: *mut snd_pcm_substream,
    rec: *mut snd_pcm_indirect,
    copy: snd_pcm_indirect_copy_t,
) -> i32 {
    let runtime = (*substream).runtime;
    let appl_ptr = (*(*runtime).control).appl_ptr;
    let mut diff: snd_pcm_sframes_t = appl_ptr.wrapping_sub((*rec).appl_ptr) as snd_pcm_sframes_t;
    if diff != 0 {
        if diff < -((*runtime).boundary / 2) as snd_pcm_sframes_t { diff = diff.wrapping_add((*runtime).boundary as snd_pcm_sframes_t); }
        if diff < 0 { return -EPIPE; }
        (*rec).sw_ready -= frames_to_bytes(runtime, diff) as i32;
        (*rec).appl_ptr = appl_ptr;
    }
    while (*rec).hw_ready > 0 && (*rec).sw_ready < (*rec).sw_buffer_size as i32 {
        let hw_to_end = (*rec).hw_buffer_size - (*rec).hw_data;
        let sw_to_end = (*rec).sw_buffer_size - (*rec).sw_data;
        let mut bytes = (*rec).sw_buffer_size - (*rec).sw_ready as u32;
        if (*rec).hw_ready < bytes as i32 { bytes = (*rec).hw_ready as u32; }
        if hw_to_end < bytes { bytes = hw_to_end; }
        if sw_to_end < bytes { bytes = sw_to_end; }
        if bytes == 0 { break; }
        copy(substream, rec, bytes as usize);
        (*rec).hw_data += bytes;
        if (*rec).hw_data as i32 == (*rec).hw_buffer_size as i32 { (*rec).hw_data = 0; }
        (*rec).sw_data += bytes;
        if (*rec).sw_data == (*rec).sw_buffer_size { (*rec).sw_data = 0; }
        (*rec).hw_ready -= bytes as i32;
        (*rec).sw_ready += bytes as i32;
    }
    0
}

/* helper function for capture pointer callback,
 * ptr = current byte pointer
 */
#[inline]
pub unsafe fn snd_pcm_indirect_capture_pointer(
    substream: *mut snd_pcm_substream,
    rec: *mut snd_pcm_indirect,
    ptr: u32,
) -> snd_pcm_uframes_t {
    let mut bytes = ptr as i32 - (*rec).hw_io as i32;
    if bytes < 0 { bytes += (*rec).hw_buffer_size as i32; }
    (*rec).hw_io = ptr;
    (*rec).hw_ready += bytes;
    let qsize = if (*rec).hw_queue_size != 0 { (*rec).hw_queue_size as i32 } else { (*rec).hw_buffer_size as i32 };
    if (*rec).hw_ready > qsize { return SNDRV_PCM_POS_XRUN; }
    (*rec).sw_io += bytes as u32;
    if (*rec).sw_io >= (*rec).sw_buffer_size { (*rec).sw_io -= (*rec).sw_buffer_size; }
    if let Some(ack) = (*(*substream).ops).ack {
        let err = ack(substream);
        if err == -EPIPE { return SNDRV_PCM_POS_XRUN; }
    }
    bytes_to_frames((*substream).runtime, (*rec).sw_io)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
