/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2017-2021  NXP
 *
 ******************************************************************************
 * Communication stack of audio with rpmsg
 ******************************************************************************
 * Packet structure:
 *   A SRTM message consists of a 10 bytes header followed by 0~N bytes of data
 *
 * Audio control:
 *   SRTM Audio Control Category Request Command Table, Response Command Table,
 *   Notification Command Table, sample formats, and channel definitions are
 *   documented in the original C header.
 */

/* C header dependencies:
 * #include <linux/pm_qos.h>
 * #include <linux/interrupt.h>
 * #include <sound/dmaengine_pcm.h>
 */

pub const RPMSG_TIMEOUT: i32 = 1000;

/* RPMSG Command (TYPE A) */
pub const TX_OPEN: u32 = 0x0;
pub const TX_START: u32 = 0x1;
pub const TX_PAUSE: u32 = 0x2;
pub const TX_RESTART: u32 = 0x3;
pub const TX_TERMINATE: u32 = 0x4;
pub const TX_CLOSE: u32 = 0x5;
pub const TX_HW_PARAM: u32 = 0x6;
pub const TX_BUFFER: u32 = 0x7;
pub const TX_SUSPEND: u32 = 0x8;
pub const TX_RESUME: u32 = 0x9;

pub const RX_OPEN: u32 = 0xA;
pub const RX_START: u32 = 0xB;
pub const RX_PAUSE: u32 = 0xC;
pub const RX_RESTART: u32 = 0xD;
pub const RX_TERMINATE: u32 = 0xE;
pub const RX_CLOSE: u32 = 0xF;
pub const RX_HW_PARAM: u32 = 0x10;
pub const RX_BUFFER: u32 = 0x11;
pub const RX_SUSPEND: u32 = 0x12;
pub const RX_RESUME: u32 = 0x13;
pub const SET_CODEC_VALUE: u32 = 0x14;
pub const GET_CODEC_VALUE: u32 = 0x15;
pub const TX_POINTER: u32 = 0x16;
pub const RX_POINTER: u32 = 0x17;
/* Total msg numver for type A */
pub const MSG_TYPE_A_NUM: usize = 0x18;

/* RPMSG Command (TYPE C) */
pub const TX_PERIOD_DONE: u32 = 0x0;
pub const RX_PERIOD_DONE: u32 = 0x1;
/* Total msg numver for type C */
pub const MSG_TYPE_C_NUM: usize = 0x2;

pub const MSG_MAX_NUM: usize = MSG_TYPE_A_NUM + MSG_TYPE_C_NUM;

pub const MSG_TYPE_A: u32 = 0x0;
pub const MSG_TYPE_B: u32 = 0x1;
pub const MSG_TYPE_C: u32 = 0x2;

pub const RESP_NONE: u32 = 0x0;
pub const RESP_NOT_ALLOWED: u32 = 0x1;
pub const RESP_SUCCESS: u32 = 0x2;
pub const RESP_FAILED: u32 = 0x3;

pub const RPMSG_S16_LE: u32 = 0x0;
pub const RPMSG_S24_LE: u32 = 0x1;
pub const RPMSG_S32_LE: u32 = 0x2;
pub const RPMSG_DSD_U16_LE: u32 = 49; /* SNDRV_PCM_FORMAT_DSD_U16_LE */
pub const RPMSG_DSD_U24_LE: u32 = 0x4;
pub const RPMSG_DSD_U32_LE: u32 = 50; /* SNDRV_PCM_FORMAT_DSD_U32_LE */

pub const RPMSG_CH_LEFT: u32 = 0x0;
pub const RPMSG_CH_RIGHT: u32 = 0x1;
pub const RPMSG_CH_STEREO: u32 = 0x2;

pub const WORK_MAX_NUM: usize = 0x30;

/* Category define */
pub const IMX_RMPSG_LIFECYCLE: u32 = 1;
pub const IMX_RPMSG_PMIC: u32 = 2;
pub const IMX_RPMSG_AUDIO: u32 = 3;
pub const IMX_RPMSG_KEY: u32 = 4;
pub const IMX_RPMSG_GPIO: u32 = 5;
pub const IMX_RPMSG_RTC: u32 = 6;
pub const IMX_RPMSG_SENSOR: u32 = 7;

/* rpmsg version */
pub const IMX_RMPSG_MAJOR: u32 = 1;
pub const IMX_RMPSG_MINOR: u32 = 0;

pub const TX: u32 = SNDRV_PCM_STREAM_PLAYBACK;
pub const RX: u32 = SNDRV_PCM_STREAM_CAPTURE;

unsafe extern "C" {
    pub type rpmsg_device;
    pub type device;
    pub type completion;
    pub type pm_qos_request;
    pub type work_struct;
    pub type timer_list;
    pub type snd_pcm_substream;
    pub type workqueue_struct;
    pub type spinlock_t;
    pub type mutex;

    pub static SNDRV_PCM_STREAM_PLAYBACK: u32;
    pub static SNDRV_PCM_STREAM_CAPTURE: u32;
}

/**
 * struct rpmsg_head: rpmsg header structure
 *
 * @cate: category
 * @major: major version
 * @minor: minor version
 * @type: message type (A/B/C)
 * @cmd: message command
 * @reserved: reserved space
 */
#[repr(C, packed)]
pub struct rpmsg_head {
    pub cate: u8,
    pub major: u8,
    pub minor: u8,
    pub type_: u8,
    pub cmd: u8,
    pub reserved: [u8; 5],
}

/**
 * struct param_s: sent rpmsg parameter
 *
 * @audioindex: audio instance index
 * @format: audio format
 * @channels: audio channel number
 * @rate: sample rate
 * @buffer_addr: dma buffer physical address or register for SET_CODEC_VALUE
 * @buffer_size: dma buffer size or register value for SET_CODEC_VALUE
 * @period_size: period size
 * @buffer_tail: current period index
 */
#[repr(C, packed)]
pub struct param_s {
    pub audioindex: core::ffi::c_uchar,
    pub format: core::ffi::c_uchar,
    pub channels: core::ffi::c_uchar,
    pub rate: core::ffi::c_uint,
    pub buffer_addr: core::ffi::c_uint,
    pub buffer_size: core::ffi::c_uint,
    pub period_size: core::ffi::c_uint,
    pub buffer_tail: core::ffi::c_uint,
}

/**
 * struct param_s: send rpmsg parameter
 *
 * @audioindex: audio instance index
 * @resp: response value
 * @reserved1: reserved space
 * @buffer_offset: the consumed offset of buffer
 * @reg_addr: register addr of codec
 * @reg_data: register value of codec
 * @reserved2: reserved space
 * @buffer_tail: current period index
 */
#[repr(C, packed)]
pub struct param_r {
    pub audioindex: core::ffi::c_uchar,
    pub resp: core::ffi::c_uchar,
    pub reserved1: [core::ffi::c_uchar; 1],
    pub buffer_offset: core::ffi::c_uint,
    pub reg_addr: core::ffi::c_uint,
    pub reg_data: core::ffi::c_uint,
    pub reserved2: [core::ffi::c_uchar; 4],
    pub buffer_tail: core::ffi::c_uint,
}

/* Struct of sent message */
#[repr(C)]
pub struct rpmsg_s_msg {
    pub header: rpmsg_head,
    pub param: param_s,
}

/* Struct of received message */
#[repr(C)]
pub struct rpmsg_r_msg {
    pub header: rpmsg_head,
    pub param: param_r,
}

/* Struct of rpmsg */
#[repr(C)]
pub struct rpmsg_msg {
    pub s_msg: rpmsg_s_msg,
    pub r_msg: rpmsg_r_msg,
}

/* Struct of rpmsg for workqueue */
#[repr(C)]
pub struct work_of_rpmsg {
    pub info: *mut rpmsg_info,
    /* Sent msg for each work */
    pub msg: rpmsg_msg,
    pub work: work_struct,
}

/* Struct of timer */
#[repr(C)]
pub struct stream_timer {
    pub timer: timer_list,
    pub info: *mut rpmsg_info,
    pub substream: *mut snd_pcm_substream,
}

pub type dma_callback = Option<unsafe extern "C" fn(arg: *mut core::ffi::c_void)>;

/**
 * struct rpmsg_info: rpmsg audio information
 *
 * @rpdev: pointer of rpmsg_device
 * @dev: pointer for imx_pcm_rpmsg device
 * @cmd_complete: command is finished
 * @pm_qos_req: request of pm qos
 * @r_msg: received rpmsg
 * @msg: array of rpmsg
 * @notify: notification msg (type C) for TX & RX
 * @notify_updated: notification flag for TX & RX
 * @rpmsg_wq: rpmsg workqueue
 * @work_list: array of work list for workqueue
 * @work_write_index: write index of work list
 * @work_read_index: read index of work list
 * @msg_drop_count: counter of dropped msg for TX & RX
 * @num_period: period number for TX & RX
 * @callback_param: parameter for period elapse callback for TX & RX
 * @callback: period elapse callback for TX & RX
 * @send_message: function pointer for send message
 * @lock: spin lock for TX & RX
 * @wq_lock: lock for work queue
 * @msg_lock: lock for send message
 * @stream_timer: timer for tigger workqueue
 */
#[repr(C)]
pub struct rpmsg_info {
    pub rpdev: *mut rpmsg_device,
    pub dev: *mut device,
    pub cmd_complete: completion,
    pub pm_qos_req: pm_qos_request,

    /* Received msg (global) */
    pub r_msg: rpmsg_r_msg,
    pub msg: [rpmsg_msg; MSG_MAX_NUM],
    /* period done */
    pub notify: [rpmsg_msg; 2],
    pub notify_updated: [bool; 2],

    pub rpmsg_wq: *mut workqueue_struct,
    pub work_list: [work_of_rpmsg; WORK_MAX_NUM],
    pub work_write_index: core::ffi::c_int,
    pub work_read_index: core::ffi::c_int,
    pub msg_drop_count: [core::ffi::c_int; 2],
    pub num_period: [core::ffi::c_int; 2],
    pub callback_param: [*mut core::ffi::c_void; 2],
    pub callback: [dma_callback; 2],
    pub send_message:
        Option<unsafe extern "C" fn(msg: *mut rpmsg_msg, info: *mut rpmsg_info) -> core::ffi::c_int>,
    pub lock: [spinlock_t; 2], /* spin lock for resource protection */
    pub wq_lock: spinlock_t,   /* spin lock for resource protection */
    pub msg_lock: mutex,       /* mutex for resource protection */
    pub stream_timer: [stream_timer; 2],
}

pub const IMX_PCM_DRV_NAME: &[u8; 14] = b"imx_pcm_rpmsg\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
