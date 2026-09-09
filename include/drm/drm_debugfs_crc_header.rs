/*
 * Copyright © 2016 Collabora Ltd.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external. The original header included spinlock, integer, and wait-queue
// type definitions here.

pub const DRM_MAX_CRC_NR: usize = 10;

/**
 * struct drm_crtc_crc_entry - entry describing a frame's content
 * @has_frame_counter: whether the source was able to provide a frame number
 * @frame: number of the frame this CRC is about, if @has_frame_counter is true
 * @crcs: array of values that characterize the frame
 */
#[repr(C)]
pub struct drm_crtc_crc_entry {
    pub has_frame_counter: bool,
    pub frame: u32,
    pub crcs: [u32; DRM_MAX_CRC_NR],
}

pub const DRM_CRC_ENTRIES_NR: usize = 128;

/**
 * struct drm_crtc_crc - data supporting CRC capture on a given CRTC
 * @lock: protects the fields in this struct
 * @source: name of the currently configured source of CRCs
 * @opened: whether userspace has opened the data file for reading
 * @overflow: whether an overflow occurred
 * @entries: array of entries, with size of %DRM_CRC_ENTRIES_NR
 * @head: head of circular queue
 * @tail: tail of circular queue
 * @values_cnt: number of CRC values per entry, up to %DRM_MAX_CRC_NR
 * @wq: workqueue used to synchronize reading and writing
 */
#[repr(C)]
pub struct drm_crtc_crc {
    pub lock: spinlock_t,
    pub source: *const core::ffi::c_char,
    pub opened: bool,
    pub overflow: bool,
    pub entries: *mut drm_crtc_crc_entry,
    pub head: core::ffi::c_int,
    pub tail: core::ffi::c_int,
    pub values_cnt: usize,
    pub wq: wait_queue_head_t,
}

// When CONFIG_DEBUG_FS is enabled, this function is provided externally.
#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub fn drm_crtc_add_crc_entry(
        crtc: *mut drm_crtc,
        has_frame: bool,
        frame: u32,
        crcs: *mut u32,
    ) -> core::ffi::c_int;
}

// When CONFIG_DEBUG_FS is disabled, the C header provides this inline stub.
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub unsafe fn drm_crtc_add_crc_entry(
    _crtc: *mut drm_crtc,
    _has_frame: bool,
    _frame: u32,
    _crcs: *mut u32,
) -> core::ffi::c_int {
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
