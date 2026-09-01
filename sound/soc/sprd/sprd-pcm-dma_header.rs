// SPDX-License-Identifier: GPL-2.0

// C header guard removed: __SPRD_PCM_DMA_H

pub const DRV_NAME: &[u8; 13] = b"sprd_pcm_dma\0";
pub const SPRD_PCM_CHANNEL_MAX: usize = 2;

unsafe extern "C" {
    pub static sprd_platform_compress_ops: snd_compress_ops;
}

#[repr(C)]
pub struct sprd_pcm_dma_params {
    pub dev_phys: [dma_addr_t; SPRD_PCM_CHANNEL_MAX],
    pub datawidth: [u32; SPRD_PCM_CHANNEL_MAX],
    pub fragment_len: [u32; SPRD_PCM_CHANNEL_MAX],
    pub chan_name: [*const ::core::ffi::c_char; SPRD_PCM_CHANNEL_MAX],
}

#[repr(C)]
pub struct sprd_compr_playinfo {
    pub total_time: ::core::ffi::c_int,
    pub current_time: ::core::ffi::c_int,
    pub total_data_length: ::core::ffi::c_int,
    pub current_data_offset: u64,
}

#[repr(C)]
pub struct sprd_compr_params {
    pub direction: u32,
    pub rate: u32,
    pub sample_rate: u32,
    pub channels: u32,
    pub format: u32,
    pub period: u32,
    pub periods: u32,
    pub info_phys: u32,
    pub info_size: u32,
}

#[repr(C)]
pub struct sprd_compr_callback {
    pub drain_notify: Option<unsafe extern "C" fn(data: *mut ::core::ffi::c_void)>,
    pub drain_data: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct sprd_compr_ops {
    pub open: Option<
        unsafe extern "C" fn(
            str_id: ::core::ffi::c_int,
            cb: *mut sprd_compr_callback,
        ) -> ::core::ffi::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(str_id: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub start: Option<unsafe extern "C" fn(str_id: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub stop: Option<unsafe extern "C" fn(str_id: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub pause: Option<unsafe extern "C" fn(str_id: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub pause_release:
        Option<unsafe extern "C" fn(str_id: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub drain: Option<unsafe extern "C" fn(received_total: u64) -> ::core::ffi::c_int>,
    pub set_params: Option<
        unsafe extern "C" fn(
            str_id: ::core::ffi::c_int,
            params: *mut sprd_compr_params,
        ) -> ::core::ffi::c_int,
    >,
}

#[repr(C)]
pub struct sprd_compr_data {
    pub ops: *mut sprd_compr_ops,
    pub dma_params: *mut sprd_pcm_dma_params,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
