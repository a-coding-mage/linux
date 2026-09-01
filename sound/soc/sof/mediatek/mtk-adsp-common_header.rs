/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */

// C header guard removed. Original header depends on external declarations for:
// struct snd_sof_dev, struct snd_sof_ipc_msg, struct mtk_adsp_ipc,
// struct snd_pcm_substream, struct snd_pcm_hw_params,
// struct snd_sof_platform_stream_params, u32, and snd_pcm_uframes_t.

pub const EXCEPT_MAX_HDR_SIZE: u32 = 0x400;
pub const MTK_ADSP_STACK_DUMP_SIZE: u32 = 32;

unsafe extern "C" {
    pub fn mtk_adsp_dump(sdev: *mut snd_sof_dev, flags: u32);
    pub fn mtk_adsp_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> core::ffi::c_int;
    pub fn mtk_adsp_handle_reply(ipc: *mut mtk_adsp_ipc);
    pub fn mtk_adsp_handle_request(ipc: *mut mtk_adsp_ipc);
    pub fn mtk_adsp_get_bar_index(sdev: *mut snd_sof_dev, type_: u32) -> core::ffi::c_int;
    pub fn mtk_adsp_stream_pcm_hw_params(
        sdev: *mut snd_sof_dev,
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        platform_params: *mut snd_sof_platform_stream_params,
    ) -> core::ffi::c_int;
    pub fn mtk_adsp_stream_pcm_pointer(
        sdev: *mut snd_sof_dev,
        substream: *mut snd_pcm_substream,
    ) -> snd_pcm_uframes_t;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
