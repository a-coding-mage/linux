// SPDX-License-Identifier: GPL-2.0
// Rust translation of q6asm.h.
// Original C dependency: q6dsp-common.h.

use core::ffi::{c_int, c_uint, c_void};

/* ASM client callback events */
pub const CMD_PAUSE: u32 = 0x0001;
pub const ASM_CLIENT_EVENT_CMD_PAUSE_DONE: u32 = 0x1001;
pub const CMD_FLUSH: u32 = 0x0002;
pub const ASM_CLIENT_EVENT_CMD_FLUSH_DONE: u32 = 0x1002;
pub const CMD_EOS: u32 = 0x0003;
pub const ASM_CLIENT_EVENT_CMD_EOS_DONE: u32 = 0x1003;
pub const CMD_CLOSE: u32 = 0x0004;
pub const ASM_CLIENT_EVENT_CMD_CLOSE_DONE: u32 = 0x1004;
pub const CMD_OUT_FLUSH: u32 = 0x0005;
pub const ASM_CLIENT_EVENT_CMD_OUT_FLUSH_DONE: u32 = 0x1005;
pub const CMD_SUSPEND: u32 = 0x0006;
pub const ASM_CLIENT_EVENT_CMD_SUSPEND_DONE: u32 = 0x1006;
pub const ASM_CLIENT_EVENT_CMD_RUN_DONE: u32 = 0x1008;
pub const ASM_CLIENT_EVENT_DATA_WRITE_DONE: u32 = 0x1009;
pub const ASM_CLIENT_EVENT_DATA_READ_DONE: u32 = 0x100a;
pub const ASM_WRITE_TOKEN_MASK: u32 = 0x0000ffff;
pub const ASM_WRITE_TOKEN_LEN_MASK: u32 = 0xffff0000;
pub const ASM_WRITE_TOKEN_LEN_SHIFT: u32 = 16;

pub const LEGACY_PCM_MODE: u32 = 0;
pub const LOW_LATENCY_PCM_MODE: u32 = 1;
pub const ULTRA_LOW_LATENCY_PCM_MODE: u32 = 2;
pub const ULL_POST_PROCESSING_PCM_MODE: u32 = 3;

pub const MAX_SESSIONS: u32 = 8;
pub const FORMAT_LINEAR_PCM: u32 = 0x0000;
pub const ASM_LAST_BUFFER_FLAG: u32 = 1u32 << 30;

#[repr(C)]
pub struct q6asm_flac_cfg {
    pub sample_rate: u32,
    pub ext_sample_rate: u32,
    pub min_frame_size: u32,
    pub max_frame_size: u32,
    pub stream_info_present: u16,
    pub min_blk_size: u16,
    pub max_blk_size: u16,
    pub ch_cfg: u16,
    pub sample_size: u16,
    pub md5_sum: u16,
}

#[repr(C)]
pub struct q6asm_wma_cfg {
    pub fmtag: u32,
    pub num_channels: u32,
    pub sample_rate: u32,
    pub bytes_per_sec: u32,
    pub block_align: u32,
    pub bits_per_sample: u32,
    pub channel_mask: u32,
    pub enc_options: u32,
    pub adv_enc_options: u32,
    pub adv_enc_options2: u32,
}

#[repr(C)]
pub struct q6asm_alac_cfg {
    pub frame_length: u32,
    pub compatible_version: u8,
    pub bit_depth: u8,
    pub pb: u8,
    pub mb: u8,
    pub kb: u8,
    pub num_channels: u8,
    pub max_run: u16,
    pub max_frame_bytes: u32,
    pub avg_bit_rate: u32,
    pub sample_rate: u32,
    pub channel_layout_tag: u32,
}

#[repr(C)]
pub struct q6asm_ape_cfg {
    pub compatible_version: u16,
    pub compression_level: u16,
    pub format_flags: u32,
    pub blocks_per_frame: u32,
    pub final_frame_blocks: u32,
    pub total_frames: u32,
    pub bits_per_sample: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub seek_table_present: u32,
}

pub type phys_addr_t = usize;
pub type size_t = usize;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct audio_client {
    _private: [u8; 0],
}

pub type q6asm_cb = Option<
    unsafe extern "C" fn(opcode: u32, token: u32, payload: *mut c_void, priv_: *mut c_void),
>;

unsafe extern "C" {
    pub fn q6asm_audio_client_alloc(
        dev: *mut device,
        cb: q6asm_cb,
        priv_: *mut c_void,
        session_id: c_int,
        perf_mode: c_int,
    ) -> *mut audio_client;
    pub fn q6asm_audio_client_free(ac: *mut audio_client);
    pub fn q6asm_write_async(
        ac: *mut audio_client,
        stream_id: u32,
        len: u32,
        msw_ts: u32,
        lsw_ts: u32,
        wflags: u32,
    ) -> c_int;
    pub fn q6asm_open_write(
        ac: *mut audio_client,
        stream_id: u32,
        format: u32,
        codec_profile: u32,
        bits_per_sample: u16,
        is_gapless: bool,
    ) -> c_int;

    pub fn q6asm_open_read(
        ac: *mut audio_client,
        stream_id: u32,
        format: u32,
        bits_per_sample: u16,
    ) -> c_int;
    pub fn q6asm_enc_cfg_blk_pcm_format_support(
        ac: *mut audio_client,
        stream_id: u32,
        rate: u32,
        channels: u32,
        bits_per_sample: u16,
    ) -> c_int;

    pub fn q6asm_read(ac: *mut audio_client, stream_id: u32) -> c_int;

    pub fn q6asm_media_format_block_multi_ch_pcm(
        ac: *mut audio_client,
        stream_id: u32,
        rate: u32,
        channels: u32,
        channel_map: *mut u8,
        bits_per_sample: u16,
    ) -> c_int;
    pub fn q6asm_stream_media_format_block_flac(
        ac: *mut audio_client,
        stream_id: u32,
        cfg: *mut q6asm_flac_cfg,
    ) -> c_int;
    pub fn q6asm_stream_media_format_block_wma_v9(
        ac: *mut audio_client,
        stream_id: u32,
        cfg: *mut q6asm_wma_cfg,
    ) -> c_int;
    pub fn q6asm_stream_media_format_block_wma_v10(
        ac: *mut audio_client,
        stream_id: u32,
        cfg: *mut q6asm_wma_cfg,
    ) -> c_int;
    pub fn q6asm_stream_media_format_block_alac(
        ac: *mut audio_client,
        stream_id: u32,
        cfg: *mut q6asm_alac_cfg,
    ) -> c_int;
    pub fn q6asm_stream_media_format_block_ape(
        ac: *mut audio_client,
        stream_id: u32,
        cfg: *mut q6asm_ape_cfg,
    ) -> c_int;
    pub fn q6asm_run(
        ac: *mut audio_client,
        stream_id: u32,
        flags: u32,
        msw_ts: u32,
        lsw_ts: u32,
    ) -> c_int;
    pub fn q6asm_run_nowait(
        ac: *mut audio_client,
        stream_id: u32,
        flags: u32,
        msw_ts: u32,
        lsw_ts: u32,
    ) -> c_int;
    pub fn q6asm_stream_remove_initial_silence(
        ac: *mut audio_client,
        stream_id: u32,
        initial_samples: u32,
    ) -> c_int;
    pub fn q6asm_stream_remove_trailing_silence(
        ac: *mut audio_client,
        stream_id: u32,
        trailing_samples: u32,
    ) -> c_int;
    pub fn q6asm_cmd(ac: *mut audio_client, stream_id: u32, cmd: c_int) -> c_int;
    pub fn q6asm_cmd_nowait(ac: *mut audio_client, stream_id: u32, cmd: c_int) -> c_int;
    pub fn q6asm_get_session_id(c: *mut audio_client) -> c_int;
    pub fn q6asm_map_memory_regions(
        dir: c_uint,
        ac: *mut audio_client,
        phys: phys_addr_t,
        period_sz: size_t,
        periods: c_uint,
    ) -> c_int;
    pub fn q6asm_unmap_memory_regions(dir: c_uint, ac: *mut audio_client) -> c_int;
    pub fn q6asm_get_hw_pointer(ac: *mut audio_client, dir: c_uint) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
