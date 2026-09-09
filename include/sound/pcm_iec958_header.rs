/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux ALSA type definitions.

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn snd_pcm_create_iec958_consumer_default(cs: *mut u8, len: usize) -> ::core::ffi::c_int;

    pub fn snd_pcm_fill_iec958_consumer(
        runtime: *mut snd_pcm_runtime,
        cs: *mut u8,
        len: usize,
    ) -> ::core::ffi::c_int;

    pub fn snd_pcm_fill_iec958_consumer_hw_params(
        params: *mut snd_pcm_hw_params,
        cs: *mut u8,
        len: usize,
    ) -> ::core::ffi::c_int;

    pub fn snd_pcm_create_iec958_consumer(
        runtime: *mut snd_pcm_runtime,
        cs: *mut u8,
        len: usize,
    ) -> ::core::ffi::c_int;

    pub fn snd_pcm_create_iec958_consumer_hw_params(
        params: *mut snd_pcm_hw_params,
        cs: *mut u8,
        len: usize,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
