/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

// <linux/types.h> and <uapi/sound/sof/abi.h> provide related dependencies.

/** SOF uAPI specification. */

/*
 * IPC messages have a prefixed 32 bit identifier made up as follows :-
 *
 * 0xGCCCNNNN where
 * G is global cmd type (4 bits)
 * C is command type (12 bits)
 * I is the ID number (16 bits) - monotonic and overflows
 *
 * This is sent at the start of the IPM message in the mailbox. Messages should
 * not be sent in the doorbell (special exceptions for firmware .
 */

/* Global Message - Generic */
pub const SOF_GLB_TYPE_SHIFT: u32 = 28;
pub const SOF_GLB_TYPE_MASK: u32 = 0xf_u32 << SOF_GLB_TYPE_SHIFT;
#[inline]
pub const fn SOF_GLB_TYPE(x: u32) -> u32 { x << SOF_GLB_TYPE_SHIFT }

/* Command Message - Generic */
pub const SOF_CMD_TYPE_SHIFT: u32 = 16;
pub const SOF_CMD_TYPE_MASK: u32 = 0xfff_u32 << SOF_CMD_TYPE_SHIFT;
#[inline]
pub const fn SOF_CMD_TYPE(x: u32) -> u32 { x << SOF_CMD_TYPE_SHIFT }

/* Global Message Types */
pub const SOF_IPC_GLB_REPLY: u32 = SOF_GLB_TYPE(0x1);
pub const SOF_IPC_GLB_COMPOUND: u32 = SOF_GLB_TYPE(0x2);
pub const SOF_IPC_GLB_TPLG_MSG: u32 = SOF_GLB_TYPE(0x3);
pub const SOF_IPC_GLB_PM_MSG: u32 = SOF_GLB_TYPE(0x4);
pub const SOF_IPC_GLB_COMP_MSG: u32 = SOF_GLB_TYPE(0x5);
pub const SOF_IPC_GLB_STREAM_MSG: u32 = SOF_GLB_TYPE(0x6);
pub const SOF_IPC_FW_READY: u32 = SOF_GLB_TYPE(0x7);
pub const SOF_IPC_GLB_DAI_MSG: u32 = SOF_GLB_TYPE(0x8);
pub const SOF_IPC_GLB_TRACE_MSG: u32 = SOF_GLB_TYPE(0x9);
pub const SOF_IPC_GLB_GDB_DEBUG: u32 = SOF_GLB_TYPE(0xA);
pub const SOF_IPC_GLB_TEST_MSG: u32 = SOF_GLB_TYPE(0xB);
pub const SOF_IPC_GLB_PROBE: u32 = SOF_GLB_TYPE(0xC);
pub const SOF_IPC_GLB_DEBUG: u32 = SOF_GLB_TYPE(0xD);

/* DSP Command Message Types */

/* topology */
pub const SOF_IPC_TPLG_COMP_NEW: u32 = SOF_CMD_TYPE(0x001);
pub const SOF_IPC_TPLG_COMP_FREE: u32 = SOF_CMD_TYPE(0x002);
pub const SOF_IPC_TPLG_COMP_CONNECT: u32 = SOF_CMD_TYPE(0x003);
pub const SOF_IPC_TPLG_PIPE_NEW: u32 = SOF_CMD_TYPE(0x010);
pub const SOF_IPC_TPLG_PIPE_FREE: u32 = SOF_CMD_TYPE(0x011);
pub const SOF_IPC_TPLG_PIPE_CONNECT: u32 = SOF_CMD_TYPE(0x012);
pub const SOF_IPC_TPLG_PIPE_COMPLETE: u32 = SOF_CMD_TYPE(0x013);
pub const SOF_IPC_TPLG_BUFFER_NEW: u32 = SOF_CMD_TYPE(0x020);
pub const SOF_IPC_TPLG_BUFFER_FREE: u32 = SOF_CMD_TYPE(0x021);

/* PM */
pub const SOF_IPC_PM_CTX_SAVE: u32 = SOF_CMD_TYPE(0x001);
pub const SOF_IPC_PM_CTX_RESTORE: u32 = SOF_CMD_TYPE(0x002);
pub const SOF_IPC_PM_CTX_SIZE: u32 = SOF_CMD_TYPE(0x003);
pub const SOF_IPC_PM_CLK_SET: u32 = SOF_CMD_TYPE(0x004);
pub const SOF_IPC_PM_CLK_GET: u32 = SOF_CMD_TYPE(0x005);
pub const SOF_IPC_PM_CLK_REQ: u32 = SOF_CMD_TYPE(0x006);
pub const SOF_IPC_PM_CORE_ENABLE: u32 = SOF_CMD_TYPE(0x007);
pub const SOF_IPC_PM_GATE: u32 = SOF_CMD_TYPE(0x008);

/* component runtime config - multiple different types */
pub const SOF_IPC_COMP_SET_VALUE: u32 = SOF_CMD_TYPE(0x001);
pub const SOF_IPC_COMP_GET_VALUE: u32 = SOF_CMD_TYPE(0x002);
pub const SOF_IPC_COMP_SET_DATA: u32 = SOF_CMD_TYPE(0x003);
pub const SOF_IPC_COMP_GET_DATA: u32 = SOF_CMD_TYPE(0x004);
pub const SOF_IPC_COMP_NOTIFICATION: u32 = SOF_CMD_TYPE(0x005);

/* DAI messages */
pub const SOF_IPC_DAI_CONFIG: u32 = SOF_CMD_TYPE(0x001);
pub const SOF_IPC_DAI_LOOPBACK: u32 = SOF_CMD_TYPE(0x002);

/* stream */
pub const SOF_IPC_STREAM_PCM_PARAMS: u32 = SOF_CMD_TYPE(0x001);
pub const SOF_IPC_STREAM_PCM_PARAMS_REPLY: u32 = SOF_CMD_TYPE(0x002);
pub const SOF_IPC_STREAM_PCM_FREE: u32 = SOF_CMD_TYPE(0x003);
pub const SOF_IPC_STREAM_TRIG_START: u32 = SOF_CMD_TYPE(0x004);
pub const SOF_IPC_STREAM_TRIG_STOP: u32 = SOF_CMD_TYPE(0x005);
pub const SOF_IPC_STREAM_TRIG_PAUSE: u32 = SOF_CMD_TYPE(0x006);
pub const SOF_IPC_STREAM_TRIG_RELEASE: u32 = SOF_CMD_TYPE(0x007);
pub const SOF_IPC_STREAM_TRIG_DRAIN: u32 = SOF_CMD_TYPE(0x008);
pub const SOF_IPC_STREAM_TRIG_XRUN: u32 = SOF_CMD_TYPE(0x009);
pub const SOF_IPC_STREAM_POSITION: u32 = SOF_CMD_TYPE(0x00a);
pub const SOF_IPC_STREAM_VORBIS_PARAMS: u32 = SOF_CMD_TYPE(0x010);
pub const SOF_IPC_STREAM_VORBIS_FREE: u32 = SOF_CMD_TYPE(0x011);

/* probe */
pub const SOF_IPC_PROBE_INIT: u32 = SOF_CMD_TYPE(0x001);
pub const SOF_IPC_PROBE_DEINIT: u32 = SOF_CMD_TYPE(0x002);
pub const SOF_IPC_PROBE_DMA_ADD: u32 = SOF_CMD_TYPE(0x003);
pub const SOF_IPC_PROBE_DMA_INFO: u32 = SOF_CMD_TYPE(0x004);
pub const SOF_IPC_PROBE_DMA_REMOVE: u32 = SOF_CMD_TYPE(0x005);
pub const SOF_IPC_PROBE_POINT_ADD: u32 = SOF_CMD_TYPE(0x006);
pub const SOF_IPC_PROBE_POINT_INFO: u32 = SOF_CMD_TYPE(0x007);
pub const SOF_IPC_PROBE_POINT_REMOVE: u32 = SOF_CMD_TYPE(0x008);

/* trace */
pub const SOF_IPC_TRACE_DMA_PARAMS: u32 = SOF_CMD_TYPE(0x001);
pub const SOF_IPC_TRACE_DMA_POSITION: u32 = SOF_CMD_TYPE(0x002);
pub const SOF_IPC_TRACE_DMA_PARAMS_EXT: u32 = SOF_CMD_TYPE(0x003);
pub const SOF_IPC_TRACE_FILTER_UPDATE: u32 = SOF_CMD_TYPE(0x004); /**< ABI3.17 */
pub const SOF_IPC_TRACE_DMA_FREE: u32 = SOF_CMD_TYPE(0x005); /**< ABI3.20 */

/* debug */
pub const SOF_IPC_DEBUG_MEM_USAGE: u32 = SOF_CMD_TYPE(0x001);

/* test */
pub const SOF_IPC_TEST_IPC_FLOOD: u32 = SOF_CMD_TYPE(0x001);

/* Get message component id */
#[inline]
pub const fn SOF_IPC_MESSAGE_ID(x: u32) -> u32 { x & 0xffff }

/* maximum message size for mailbox Tx/Rx */
pub const SOF_IPC_MSG_MAX_SIZE: usize = 384;

/*
 * Structure Header - Header for all IPC structures except command structs.
 * The size can be greater than the structure size and that means there is
 * extended bespoke data beyond the end of the structure including variable
 * arrays.
 */
#[repr(C, packed)]
pub struct sof_ipc_hdr {
    pub size: u32, /**< size of structure */
}

/*
 * Command Header - Header for all IPC commands. Identifies IPC message.
 * The size can be greater than the structure size and that means there is
 * extended bespoke data beyond the end of the structure including variable
 * arrays.
 */
#[repr(C, packed)]
pub struct sof_ipc_cmd_hdr {
    pub size: u32, /**< size of structure */
    pub cmd: u32, /**< SOF_IPC_GLB_ + cmd */
}

/* Generic reply message. Some commands override this with their own reply
 * types that must include this at start.
 */
#[repr(C, packed)]
pub struct sof_ipc_reply {
    pub hdr: sof_ipc_cmd_hdr,
    pub error: i32, /**< negative error numbers */
}

/*
 * Compound commands - SOF_IPC_GLB_COMPOUND.
 *
 * Compound commands are sent to the DSP as a single IPC operation. The
 * commands are split into blocks and each block has a header. This header
 * identifies the command type and the number of commands before the next
 * header.
 */
#[repr(C, packed)]
pub struct sof_ipc_compound_hdr {
    pub hdr: sof_ipc_cmd_hdr,
    pub count: u32, /**< count of 0 means end of compound sequence */
}

/* OOPS header architecture specific data. */
#[repr(C, packed)]
pub struct sof_ipc_dsp_oops_arch_hdr {
    pub arch: u32, /* Identifier of architecture */
    pub totalsize: u32, /* Total size of oops message */
}

/* OOPS header platform specific data. */
#[repr(C, packed)]
pub struct sof_ipc_dsp_oops_plat_hdr {
    pub configidhi: u32, /* ConfigID hi 32bits */
    pub configidlo: u32, /* ConfigID lo 32bits */
    pub numaregs: u32, /* Special regs num */
    pub stackoffset: u32, /* Offset to stack pointer from beginning of oops message */
    pub stackptr: u32, /* Stack ptr */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
