/* SPDX-License-Identifier: GPL-2.0 */

// C header dependency: <linux/kernel.h>

pub const PCM_MAX_NUM_CHANNEL: usize = 8;
pub const PCM_CHANNEL_NULL: i32 = 0;

pub const PCM_CHANNEL_FL: i32 = 1; /* Front left channel. */
pub const PCM_CHANNEL_FR: i32 = 2; /* Front right channel. */
pub const PCM_CHANNEL_FC: i32 = 3; /* Front center channel. */
pub const PCM_CHANNEL_LS: i32 = 4; /* Left surround channel. */
pub const PCM_CHANNEL_RS: i32 = 5; /* Right surround channel. */
pub const PCM_CHANNEL_LFE: i32 = 6; /* Low frequency effect channel. */
pub const PCM_CHANNEL_CS: i32 = 7; /* Center surround channel; Rear center ch */
pub const PCM_CHANNEL_LB: i32 = 8; /* Left back channel; Rear left channel. */
pub const PCM_CHANNEL_RB: i32 = 9; /* Right back channel; Rear right channel. */
pub const PCM_CHANNELS: i32 = 10; /* Top surround channel. */

unsafe extern "C" {
    pub fn q6dsp_map_channels(ch_map: *mut u8, ch: core::ffi::c_int) -> core::ffi::c_int;
    pub fn q6dsp_get_channel_allocation(channels: core::ffi::c_int) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
