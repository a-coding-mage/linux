/* SPDX-License-Identifier: GPL-2.0 */
/* Source-level Rust translation of soc/qcom/qdsp6/q6apm.h. */
/* C include dependencies: linux/types.h, linux/atomic.h, linux/slab.h,
 * linux/wait.h, linux/kernel.h, linux/module.h, linux/sched.h, linux/of.h,
 * linux/delay.h, sound/soc.h, linux/of_platform.h, linux/jiffies.h,
 * linux/soc/qcom/apr.h, ../common.h, audioreach.h.
 */

pub const APM_PORT_MAX: u32 = LPASS_MAX_PORT;
pub const APM_PORT_MAX_AUDIO_CHAN_CNT: u32 = 8;
pub const PCM_CHANNEL_NULL: u32 = 0;
pub const PCM_CHANNEL_FL: u32 = 1; /* Front left channel. */
pub const PCM_CHANNEL_FR: u32 = 2; /* Front right channel. */
pub const PCM_CHANNEL_FC: u32 = 3; /* Front center channel. */
pub const PCM_CHANNEL_LS: u32 = 4; /* Left surround channel. */
pub const PCM_CHANNEL_RS: u32 = 5; /* Right surround channel. */
pub const PCM_CHANNEL_LFE: u32 = 6; /* Low frequency effect channel. */
pub const PCM_CHANNEL_CS: u32 = 7; /* Center surround channel; Rear center ch */
pub const PCM_CHANNEL_LB: u32 = 8; /* Left back channel; Rear left channel. */
pub const PCM_CHANNEL_RB: u32 = 9; /* Right back channel; Rear right channel. */
pub const PCM_CHANNELS: u32 = 10; /* Top surround channel. */

pub const APM_TIMESTAMP_FLAG: u32 = 0x80000000;
pub const FORMAT_LINEAR_PCM: u32 = 0x0000;

/* APM client callback events */
pub const APM_CMD_EOS: u32 = 0x0003;
pub const APM_CLIENT_EVENT_CMD_EOS_DONE: u32 = 0x1003;
pub const APM_CMD_CLOSE: u32 = 0x0004;
pub const APM_CLIENT_EVENT_CMD_CLOSE_DONE: u32 = 0x1004;
pub const APM_CLIENT_EVENT_CMD_RUN_DONE: u32 = 0x1008;
pub const APM_CLIENT_EVENT_DATA_WRITE_DONE: u32 = 0x1009;
pub const APM_CLIENT_EVENT_DATA_READ_DONE: u32 = 0x100a;
pub const APM_CLIENT_EVENT_WATERMARK_EVENT: u32 = 0x100b;

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    if h == 31 {
        u32::MAX << l
    } else {
        ((1u32 << (h + 1)) - 1) & !((1u32 << l) - 1)
    }
}

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const APM_WRITE_TOKEN_MASK: u32 = GENMASK(15, 0);
pub const APM_WRITE_TOKEN_LEN_MASK: u32 = GENMASK(31, 16);
pub const APM_WRITE_TOKEN_LEN_SHIFT: u32 = 16;

pub const APM_MAX_SESSIONS: u32 = 8;
pub const APM_LAST_BUFFER_FLAG: u32 = BIT(30);
pub const NO_TIMESTAMP: u32 = 0xFF00;

#[repr(C)]
pub struct q6apm {
    pub dev: *mut device,
    pub port: *mut gpr_port_t,
    pub gdev: *mut gpr_device_t,
    /* For Graph OPEN/START/STOP/CLOSE operations */
    pub wait: wait_queue_head_t,
    pub result: gpr_ibasic_rsp_result_t,

    pub cmd_lock: mutex,
    pub lock: mutex,
    pub state: u32,

    pub widget_list: list_head,
    pub graph_idr: idr,
    pub graph_info_idr: idr,
    pub sub_graphs_idr: idr,
    pub containers_idr: idr,
    pub modules_idr: idr,
}

#[repr(C)]
pub struct audio_buffer {
    pub phys: phys_addr_t,
    pub size: u32, /* size of buffer */
}

#[repr(C)]
pub struct audioreach_graph_data {
    pub buf: *mut audio_buffer,
    pub num_periods: u32,
    pub dsp_buf: u32,
    pub hw_ptr: atomic_t,
}

#[repr(C)]
pub struct audioreach_graph {
    pub info: *mut audioreach_graph_info,
    pub id: u32,
    pub state: ::core::ffi::c_int,
    pub start_count: ::core::ffi::c_int,
    /* Cached Graph data */
    pub graph: *mut ::core::ffi::c_void,
    pub refcount: kref,
    pub apm: *mut q6apm,
}

pub type q6apm_cb = Option<
    unsafe extern "C" fn(
        opcode: u32,
        token: u32,
        payload: *mut ::core::ffi::c_void,
        priv_: *mut ::core::ffi::c_void,
    ),
>;

#[repr(C)]
pub struct q6apm_graph {
    pub priv_: *mut ::core::ffi::c_void,
    pub cb: q6apm_cb,
    pub id: u32,
    pub shm_iid: u32,
    pub dev: *mut device,
    pub apm: *mut q6apm,
    pub port: *mut gpr_port_t,
    pub rx_data: audioreach_graph_data,
    pub tx_data: audioreach_graph_data,
    pub result: gpr_ibasic_rsp_result_t,
    pub cmd_wait: wait_queue_head_t,
    pub lock: mutex,
    pub ar_graph: *mut audioreach_graph,
    pub info: *mut audioreach_graph_info,
}

unsafe extern "C" {
    /* Graph Operations */
    pub fn q6apm_graph_open(
        dev: *mut device,
        cb: q6apm_cb,
        priv_: *mut ::core::ffi::c_void,
        graph_id: ::core::ffi::c_int,
        dir: ::core::ffi::c_int,
    ) -> *mut q6apm_graph;
    pub fn q6apm_graph_close(graph: *mut q6apm_graph) -> ::core::ffi::c_int;
    pub fn q6apm_graph_prepare(graph: *mut q6apm_graph) -> ::core::ffi::c_int;
    pub fn q6apm_graph_start(graph: *mut q6apm_graph) -> ::core::ffi::c_int;
    pub fn q6apm_graph_stop(graph: *mut q6apm_graph) -> ::core::ffi::c_int;
    pub fn q6apm_graph_flush(graph: *mut q6apm_graph) -> ::core::ffi::c_int;

    /* Media Format */
    pub fn q6apm_graph_media_format_pcm(
        graph: *mut q6apm_graph,
        cfg: *mut audioreach_module_config,
    ) -> ::core::ffi::c_int;

    pub fn q6apm_graph_media_format_shmem(
        graph: *mut q6apm_graph,
        cfg: *mut audioreach_module_config,
    ) -> ::core::ffi::c_int;

    /* read/write related */
    pub fn q6apm_read(graph: *mut q6apm_graph) -> ::core::ffi::c_int;
    pub fn q6apm_write_async(
        graph: *mut q6apm_graph,
        len: u32,
        msw_ts: u32,
        lsw_ts: u32,
        wflags: u32,
    ) -> ::core::ffi::c_int;

    /* Memory Map related */
    pub fn q6apm_map_memory_fixed_region(
        dev: *mut device,
        graph_id: ::core::ffi::c_uint,
        phys: phys_addr_t,
        sz: size_t,
    ) -> ::core::ffi::c_int;
    pub fn q6apm_map_pos_buffer(
        dev: *mut device,
        graph_id: ::core::ffi::c_uint,
        phys: phys_addr_t,
        sz: size_t,
    ) -> ::core::ffi::c_int;
    pub fn q6apm_unmap_pos_buffer(
        dev: *mut device,
        graph_id: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn q6apm_alloc_fragments(
        graph: *mut q6apm_graph,
        dir: ::core::ffi::c_uint,
        phys: phys_addr_t,
        period_sz: size_t,
        periods: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn q6apm_free_fragments(
        graph: *mut q6apm_graph,
        dir: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn q6apm_unmap_memory_fixed_region(
        dev: *mut device,
        graph_id: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    /* Helpers */
    pub fn q6apm_send_cmd_sync(
        apm: *mut q6apm,
        pkt: *const gpr_pkt,
        rsp_opcode: u32,
    ) -> ::core::ffi::c_int;

    /* Callback for graph specific */
    pub fn q6apm_find_module_by_mid(
        graph: *mut q6apm_graph,
        mid: u32,
    ) -> *mut audioreach_module;
    pub fn q6apm_is_adsp_ready() -> bool;

    pub fn q6apm_enable_compress_module(
        dev: *mut device,
        graph: *mut q6apm_graph,
        en: bool,
    ) -> ::core::ffi::c_int;
    pub fn q6apm_remove_initial_silence(
        dev: *mut device,
        graph: *mut q6apm_graph,
        samples: u32,
    ) -> ::core::ffi::c_int;
    pub fn q6apm_remove_trailing_silence(
        dev: *mut device,
        graph: *mut q6apm_graph,
        samples: u32,
    ) -> ::core::ffi::c_int;
    pub fn q6apm_set_real_module_id(
        dev: *mut device,
        graph: *mut q6apm_graph,
        codec_id: u32,
    ) -> ::core::ffi::c_int;
    pub fn q6apm_get_hw_pointer(
        graph: *mut q6apm_graph,
        dir: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn q6apm_is_graph_in_push_pull_mode(graph: *mut q6apm_graph) -> bool;
    pub fn q6apm_is_graph_in_push_pull_mode_from_id(
        dev: *mut device,
        graph_id: ::core::ffi::c_uint,
        dir: ::core::ffi::c_int,
    ) -> bool;
    pub fn q6apm_push_pull_config(
        graph: *mut q6apm_graph,
        bphys: phys_addr_t,
        pphys: phys_addr_t,
        size: u32,
    ) -> ::core::ffi::c_int;

    pub fn q6apm_register_watermark_event(
        graph: *mut q6apm_graph,
        watermark_bytes: ::core::ffi::c_int,
        num_levels: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
