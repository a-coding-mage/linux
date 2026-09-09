/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

// Dependencies supplied by the corresponding SOF header and stream modules:
// sof_ipc_cmd_hdr, sof_ipc_host_buffer, sof_ipc_reply, and sof_ipc_hdr.

/*
 * DMA for Trace
 */

pub const SOF_TRACE_FILENAME_SIZE: usize = 32;

/* DMA for Trace params info - SOF_IPC_DEBUG_DMA_PARAMS */
/* Deprecated - use sof_ipc_dma_trace_params_ext */
#[repr(C, packed)]
pub struct sof_ipc_dma_trace_params {
    pub hdr: sof_ipc_cmd_hdr,
    pub buffer: sof_ipc_host_buffer,
    pub stream_tag: u32,
}

/* DMA for Trace params info - SOF_IPC_DEBUG_DMA_PARAMS_EXT */
#[repr(C, packed)]
pub struct sof_ipc_dma_trace_params_ext {
    pub hdr: sof_ipc_cmd_hdr,
    pub buffer: sof_ipc_host_buffer,
    pub stream_tag: u32,
    pub timestamp_ns: u64, /* in nanosecond */
    pub reserved: [u32; 8],
}

/* DMA for Trace params info - SOF_IPC_DEBUG_DMA_PARAMS */
#[repr(C, packed)]
pub struct sof_ipc_dma_trace_posn {
    pub rhdr: sof_ipc_reply,
    pub host_offset: u32, /* Offset of DMA host buffer */
    pub overflow: u32,    /* overflow bytes if any */
    pub messages: u32,    /* total trace messages */
}

/* Values used in sof_ipc_trace_filter_elem: */

/* bits 6..0 */
pub const SOF_IPC_TRACE_FILTER_ELEM_SET_LEVEL: u32 = 0x01; /* trace level for selected components */
pub const SOF_IPC_TRACE_FILTER_ELEM_BY_UUID: u32 = 0x02; /* filter by uuid key */
pub const SOF_IPC_TRACE_FILTER_ELEM_BY_PIPE: u32 = 0x03; /* filter by pipeline */
pub const SOF_IPC_TRACE_FILTER_ELEM_BY_COMP: u32 = 0x04; /* filter by component id */

/* bit 7 */
pub const SOF_IPC_TRACE_FILTER_ELEM_FIN: u32 = 0x80; /* mark last filter in set */

/* bits 31..8: Unused */

/** part of sof_ipc_trace_filter, ABI3.17 */
#[repr(C, packed)]
pub struct sof_ipc_trace_filter_elem {
    pub key: u32,   /* SOF_IPC_TRACE_FILTER_ELEM_ {LEVEL, UUID, COMP, PIPE} */
    pub value: u32, /* element value */
}

/** Runtime tracing filtration data - SOF_IPC_TRACE_FILTER_UPDATE, ABI3.17 */
#[repr(C, packed)]
pub struct sof_ipc_trace_filter {
    pub hdr: sof_ipc_cmd_hdr, /* IPC command header */
    pub elem_cnt: u32,        /* number of entries in elems[] array */
    pub reserved: [u32; 8],   /* reserved for future usage */
    /* variable size array with new filtering settings */
    pub elems: [sof_ipc_trace_filter_elem; 0],
}

/*
 * Commom debug
 */

/*
 * SOF panic codes
 */
pub const SOF_IPC_PANIC_MAGIC: u32 = 0x0dead000;
pub const SOF_IPC_PANIC_MAGIC_MASK: u32 = 0x0ffff000;
pub const SOF_IPC_PANIC_CODE_MASK: u32 = 0x00000fff;
pub const SOF_IPC_PANIC_MEM: u32 = SOF_IPC_PANIC_MAGIC | 0x0;
pub const SOF_IPC_PANIC_WORK: u32 = SOF_IPC_PANIC_MAGIC | 0x1;
pub const SOF_IPC_PANIC_IPC: u32 = SOF_IPC_PANIC_MAGIC | 0x2;
pub const SOF_IPC_PANIC_ARCH: u32 = SOF_IPC_PANIC_MAGIC | 0x3;
pub const SOF_IPC_PANIC_PLATFORM: u32 = SOF_IPC_PANIC_MAGIC | 0x4;
pub const SOF_IPC_PANIC_TASK: u32 = SOF_IPC_PANIC_MAGIC | 0x5;
pub const SOF_IPC_PANIC_EXCEPTION: u32 = SOF_IPC_PANIC_MAGIC | 0x6;
pub const SOF_IPC_PANIC_DEADLOCK: u32 = SOF_IPC_PANIC_MAGIC | 0x7;
pub const SOF_IPC_PANIC_STACK: u32 = SOF_IPC_PANIC_MAGIC | 0x8;
pub const SOF_IPC_PANIC_IDLE: u32 = SOF_IPC_PANIC_MAGIC | 0x9;
pub const SOF_IPC_PANIC_WFI: u32 = SOF_IPC_PANIC_MAGIC | 0xa;
pub const SOF_IPC_PANIC_ASSERT: u32 = SOF_IPC_PANIC_MAGIC | 0xb;

/* panic info include filename and line number
 * filename array will not include null terminator if fully filled
 */
#[repr(C, packed)]
pub struct sof_ipc_panic_info {
    pub hdr: sof_ipc_hdr,
    pub code: u32, /* SOF_IPC_PANIC_ */
    pub filename: [u8; SOF_TRACE_FILENAME_SIZE],
    pub linenum: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
