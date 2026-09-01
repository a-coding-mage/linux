// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017-2021 NXP

// Translated from soc/fsl/imx-pcm-rpmsg.c. Kernel, ALSA, rpmsg, and local
// header symbols are external dependencies supplied by the surrounding tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type snd_pcm_uframes_t = c_ulong;
type snd_pcm_sframes_t = c_long;
type c_long = i64;
type dma_addr_t = u64;

const EINVAL: c_int = 22;
const ETIMEDOUT: c_int = 110;
const EPIPE: c_int = 32;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const DMA_BIT_MASK_32: u64 = (1u64 << 32) - 1;

const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 0;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BATCH: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 3;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 4;
const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: c_uint = 1 << 5;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 6;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 7;

extern "C" {
    static mut jiffies: c_ulong;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;

    fn rpmsg_send(ept: *mut c_void, data: *mut c_void, len: usize) -> c_int;
    fn reinit_completion(completion: *mut completion);
    fn wait_for_completion_timeout(completion: *mut completion, timeout: c_ulong) -> c_ulong;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool_;
    fn flush_workqueue(wq: *mut workqueue_struct);
    fn alloc_ordered_workqueue(name: *const c_char, flags: c_uint) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn INIT_WORK(work: *mut work_struct, func: Option<unsafe extern "C" fn(*mut work_struct)>);

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> c_uint;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *mut snd_pcm_hardware,
    );
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_playback_hw_avail(runtime: *mut snd_pcm_runtime) -> snd_pcm_sframes_t;
    fn snd_pcm_capture_hw_avail(runtime: *mut snd_pcm_runtime) -> snd_pcm_sframes_t;
    fn snd_pcm_set_fixed_buffer(
        substream: *mut snd_pcm_substream,
        ty: c_int,
        dev: *mut device,
        size: c_uint,
    ) -> c_int;

    fn timer_setup(timer: *mut timer_list, callback: Option<unsafe extern "C" fn(*mut timer_list)>, flags: c_uint);
    fn timer_delete(timer: *mut timer_list);
    fn timer_pending(timer: *mut timer_list) -> c_int;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;

    fn init_completion(completion: *mut completion);
    fn mutex_init(mutex: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn cpu_latency_qos_add_request(req: *mut dev_pm_qos_request, value: c_int);
    fn cpu_latency_qos_remove_request(req: *mut dev_pm_qos_request);
}

extern "C" {
    static IMX_PCM_DRV_NAME: *const c_char;
}

#[repr(C)]
struct device {
    parent: *mut device,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct rpmsg_device_id {
    name: [c_char; 32],
}

#[repr(C)]
struct rpmsg_device {
    dev: device,
    id: rpmsg_device_id,
    ept: *mut c_void,
}

#[repr(C)]
struct snd_pcm_hardware {
    info: c_uint,
    buffer_bytes_max: c_uint,
    period_bytes_min: c_uint,
    period_bytes_max: c_uint,
    periods_min: c_uint,
    periods_max: c_uint,
    fifo_size: c_uint,
}

#[repr(C)]
struct snd_pcm_control {
    appl_ptr: snd_pcm_uframes_t,
}

#[repr(C)]
struct snd_pcm_runtime {
    access: c_int,
    info: c_uint,
    period_size: snd_pcm_uframes_t,
    periods: c_int,
    rate: c_uint,
    dma_addr: dma_addr_t,
    control: *mut snd_pcm_control,
}

#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_pcm_stream {
    substream: *mut snd_pcm_substream,
}

#[repr(C)]
struct snd_pcm {
    streams: [snd_pcm_stream; 2],
    card: *mut snd_card,
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
    snd_card: *mut snd_card,
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai_link {
    ignore_suspend: c_int,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_card,
    pcm: *mut snd_pcm,
    dev: *mut device,
    dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    ack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    debugfs_prefix: *const c_char,
}

#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct platform_device_id {
    name: [c_char; 32],
}

#[repr(C)]
struct platform_driver_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    id_table: *const platform_device_id,
    driver: platform_driver_driver,
}

#[repr(C)]
struct snd_pcm_hw_params;
#[repr(C)]
struct workqueue_struct;
#[repr(C)]
struct work_struct;
#[repr(C)]
struct timer_list;
#[repr(C)]
struct completion;
#[repr(C)]
struct mutex;
#[repr(C)]
struct spinlock_t;
#[repr(C)]
struct dev_pm_qos_request;

#[repr(C)]
struct rpmsg_header {
    cate: c_uint,
    major: c_uint,
    minor: c_uint,
    type_: c_uint,
    cmd: c_int,
}

#[repr(C)]
struct rpmsg_param {
    rate: c_uint,
    format: c_uint,
    channels: c_uint,
    buffer_addr: dma_addr_t,
    buffer_size: c_uint,
    period_size: c_uint,
    buffer_tail: c_int,
    buffer_offset: c_int,
    resp: c_int,
    audioindex: c_uint,
}

#[repr(C)]
struct rpmsg_s_msg {
    header: rpmsg_header,
    param: rpmsg_param,
}

#[repr(C)]
struct rpmsg_r_msg {
    header: rpmsg_header,
    param: rpmsg_param,
}

#[repr(C)]
struct rpmsg_msg {
    s_msg: rpmsg_s_msg,
    r_msg: rpmsg_r_msg,
}

#[repr(C)]
struct stream_timer {
    timer: timer_list,
    substream: *mut snd_pcm_substream,
    info: *mut rpmsg_info,
}

#[repr(C)]
struct work_of_rpmsg {
    work: work_struct,
    info: *mut rpmsg_info,
    msg: rpmsg_msg,
}

#[repr(C)]
struct fsl_rpmsg {
    buffer_size: [c_uint; 2],
    enable_lpa: c_int,
    force_lpa: c_int,
}

#[repr(C)]
struct rpmsg_info {
    rpdev: *mut rpmsg_device,
    dev: *mut device,
    rpmsg_wq: *mut workqueue_struct,
    work_write_index: c_int,
    work_read_index: c_int,
    send_message: Option<unsafe extern "C" fn(*mut rpmsg_msg, *mut rpmsg_info) -> c_int>,
    work_list: [work_of_rpmsg; WORK_MAX_NUM as usize],
    msg: [rpmsg_msg; MSG_MAX_NUM as usize],
    notify: [rpmsg_msg; 2],
    notify_updated: [bool_; 2],
    cmd_complete: completion,
    msg_lock: mutex,
    lock: [spinlock_t; 2],
    wq_lock: spinlock_t,
    msg_drop_count: [c_int; 2],
    stream_timer: [stream_timer; 2],
    num_period: [c_uint; 2],
    callback: [Option<unsafe extern "C" fn(*mut c_void)>; 2],
    callback_param: [*mut c_void; 2],
    r_msg: rpmsg_r_msg,
    pm_qos_req: dev_pm_qos_request,
}

extern "C" {
    static IMX_DEFAULT_DMABUF_SIZE: c_uint;
    static RPMSG_TIMEOUT: c_uint;
    static WORK_MAX_NUM: c_int;
    static MSG_MAX_NUM: c_int;
    static MSG_TYPE_A_NUM: c_int;
    static IMX_RPMSG_AUDIO: c_uint;
    static IMX_RMPSG_MAJOR: c_uint;
    static IMX_RMPSG_MINOR: c_uint;
    static MSG_TYPE_A: c_uint;
    static MSG_TYPE_C: c_uint;
    static TX: c_int;
    static RX: c_int;
    static TX_OPEN: c_int;
    static RX_OPEN: c_int;
    static TX_CLOSE: c_int;
    static RX_CLOSE: c_int;
    static TX_HW_PARAM: c_int;
    static RX_HW_PARAM: c_int;
    static TX_BUFFER: c_int;
    static RX_BUFFER: c_int;
    static TX_START: c_int;
    static RX_START: c_int;
    static TX_RESTART: c_int;
    static RX_RESTART: c_int;
    static TX_PAUSE: c_int;
    static RX_PAUSE: c_int;
    static TX_TERMINATE: c_int;
    static RX_TERMINATE: c_int;
    static TX_POINTER: c_int;
    static RX_POINTER: c_int;
    static TX_PERIOD_DONE: c_int;
    static RX_PERIOD_DONE: c_int;
    static TX_SUSPEND: c_int;
    static RX_SUSPEND: c_int;
    static TX_RESUME: c_int;
    static RX_RESUME: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_DSD_U16_LE: c_int;
    static SNDRV_PCM_FORMAT_DSD_U32_LE: c_int;
    static RPMSG_S16_LE: c_uint;
    static RPMSG_S24_LE: c_uint;
    static RPMSG_DSD_U16_LE: c_uint;
    static RPMSG_DSD_U32_LE: c_uint;
    static RPMSG_S32_LE: c_uint;
    static RPMSG_CH_LEFT: c_uint;
    static RPMSG_CH_STEREO: c_uint;
    static SNDRV_PCM_ACCESS_RW_INTERLEAVED: c_int;
    static SNDRV_PCM_ACCESS_RW_NONINTERLEAVED: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_HW_PARAM_PERIODS: c_int;
    static SNDRV_DMA_TYPE_DEV_WC: c_int;
    static WQ_HIGHPRI: c_uint;
    static WQ_UNBOUND: c_uint;
    static WQ_FREEZABLE: c_uint;
}

static imx_rpmsg_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_BATCH
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    buffer_bytes_max: unsafe { IMX_DEFAULT_DMABUF_SIZE },
    period_bytes_min: 512,
    period_bytes_max: 65536,
    periods_min: 2,
    periods_max: 6000,
    fifo_size: 0,
};

unsafe extern "C" fn imx_rpmsg_pcm_send_message(
    msg: *mut rpmsg_msg,
    info: *mut rpmsg_info,
) -> c_int {
    let rpdev = (*info).rpdev;
    let mut ret: c_int = 0;

    guard_mutex(&mut (*info).msg_lock);
    if rpdev.is_null() {
        dev_err((*info).dev, c"rpmsg channel not ready\n".as_ptr());
        return -EINVAL;
    }

    dev_dbg(&mut (*rpdev).dev, c"send cmd %d\n".as_ptr(), (*msg).s_msg.header.cmd);

    if !((*msg).s_msg.header.type_ == MSG_TYPE_C) {
        reinit_completion(&mut (*info).cmd_complete);
    }

    ret = rpmsg_send(
        (*rpdev).ept,
        &mut (*msg).s_msg as *mut rpmsg_s_msg as *mut c_void,
        size_of::<rpmsg_s_msg>(),
    );
    if ret != 0 {
        dev_err(&mut (*rpdev).dev, c"rpmsg_send failed: %d\n".as_ptr(), ret);
        return ret;
    }

    /* No receive msg for TYPE_C command */
    if (*msg).s_msg.header.type_ == MSG_TYPE_C {
        return 0;
    }

    /* wait response from rpmsg */
    ret = wait_for_completion_timeout(
        &mut (*info).cmd_complete,
        msecs_to_jiffies(RPMSG_TIMEOUT),
    ) as c_int;
    if ret == 0 {
        dev_err(
            &mut (*rpdev).dev,
            c"rpmsg_send cmd %d timeout!\n".as_ptr(),
            (*msg).s_msg.header.cmd,
        );
        return -ETIMEDOUT;
    }

    memcpy(
        &mut (*msg).r_msg as *mut rpmsg_r_msg as *mut c_void,
        &(*info).r_msg as *const rpmsg_r_msg as *const c_void,
        size_of::<rpmsg_r_msg>(),
    );
    memcpy(
        &mut (*info).msg[(*msg).r_msg.header.cmd as usize].r_msg as *mut rpmsg_r_msg as *mut c_void,
        &(*msg).r_msg as *const rpmsg_r_msg as *const c_void,
        size_of::<rpmsg_r_msg>(),
    );

    /*
     * Reset the buffer pointer to be zero, actully we have
     * set the buffer pointer to be zero in imx_rpmsg_terminate_all
     * But if there is timer task queued in queue, after it is
     * executed the buffer pointer will be changed, so need to
     * reset it again with TERMINATE command.
     */
    if (*msg).s_msg.header.cmd == TX_TERMINATE {
        (*info).msg[TX_POINTER as usize].r_msg.param.buffer_offset = 0;
    } else if (*msg).s_msg.header.cmd == RX_TERMINATE {
        (*info).msg[RX_POINTER as usize].r_msg.param.buffer_offset = 0;
    }

    dev_dbg(
        &mut (*rpdev).dev,
        c"cmd:%d, resp %d\n".as_ptr(),
        (*msg).s_msg.header.cmd,
        (*info).r_msg.param.resp,
    );

    0
}

unsafe extern "C" fn imx_rpmsg_insert_workqueue(
    substream: *mut snd_pcm_substream,
    msg: *mut rpmsg_msg,
    info: *mut rpmsg_info,
) -> c_int {
    let mut ret: c_int = 0;

    /*
     * Queue the work to workqueue.
     * If the queue is full, drop the message.
     */
    guard_spinlock_irqsave(&mut (*info).wq_lock);
    if (*info).work_write_index != (*info).work_read_index {
        let index = (*info).work_write_index as usize;

        memcpy(
            &mut (*info).work_list[index].msg as *mut rpmsg_msg as *mut c_void,
            msg as *const c_void,
            size_of::<rpmsg_s_msg>(),
        );

        queue_work((*info).rpmsg_wq, &mut (*info).work_list[index].work);
        (*info).work_write_index += 1;
        (*info).work_write_index %= WORK_MAX_NUM;
    } else {
        (*info).msg_drop_count[(*substream).stream as usize] += 1;
        ret = -EPIPE;
    }

    ret
}

unsafe extern "C" fn imx_rpmsg_pcm_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let info = dev_get_drvdata((*component).dev) as *mut rpmsg_info;
    let msg: *mut rpmsg_msg;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        msg = &mut (*info).msg[TX_HW_PARAM as usize];
        (*msg).s_msg.header.cmd = TX_HW_PARAM;
    } else {
        msg = &mut (*info).msg[RX_HW_PARAM as usize];
        (*msg).s_msg.header.cmd = RX_HW_PARAM;
    }

    (*msg).s_msg.param.rate = params_rate(params);

    if params_format(params) == SNDRV_PCM_FORMAT_S16_LE {
        (*msg).s_msg.param.format = RPMSG_S16_LE;
    } else if params_format(params) == SNDRV_PCM_FORMAT_S24_LE {
        (*msg).s_msg.param.format = RPMSG_S24_LE;
    } else if params_format(params) == SNDRV_PCM_FORMAT_DSD_U16_LE {
        (*msg).s_msg.param.format = RPMSG_DSD_U16_LE;
    } else if params_format(params) == SNDRV_PCM_FORMAT_DSD_U32_LE {
        (*msg).s_msg.param.format = RPMSG_DSD_U32_LE;
    } else {
        (*msg).s_msg.param.format = RPMSG_S32_LE;
    }

    match params_channels(params) {
        1 => (*msg).s_msg.param.channels = RPMSG_CH_LEFT,
        2 => (*msg).s_msg.param.channels = RPMSG_CH_STEREO,
        _ => (*msg).s_msg.param.channels = params_channels(params),
    }

    ((*info).send_message.unwrap())(msg, info);

    0
}

unsafe extern "C" fn imx_rpmsg_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let info = dev_get_drvdata((*component).dev) as *mut rpmsg_info;
    let msg: *mut rpmsg_msg;
    let mut pos: c_uint = 0;
    let mut buffer_tail: c_int = 0;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        msg = &mut (*info).msg[(TX_PERIOD_DONE + MSG_TYPE_A_NUM) as usize];
    } else {
        msg = &mut (*info).msg[(RX_PERIOD_DONE + MSG_TYPE_A_NUM) as usize];
    }

    buffer_tail = (*msg).r_msg.param.buffer_tail;
    pos = (buffer_tail as c_uint).wrapping_mul(snd_pcm_lib_period_bytes(substream));

    bytes_to_frames((*substream).runtime, pos)
}

unsafe extern "C" fn imx_rpmsg_timer_callback(t: *mut timer_list) {
    let stream_timer = timer_container_of_stream_timer(t);
    let substream = (*stream_timer).substream;
    let info = (*stream_timer).info;
    let msg: *mut rpmsg_msg;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        msg = &mut (*info).msg[(TX_PERIOD_DONE + MSG_TYPE_A_NUM) as usize];
        (*msg).s_msg.header.cmd = TX_PERIOD_DONE;
    } else {
        msg = &mut (*info).msg[(RX_PERIOD_DONE + MSG_TYPE_A_NUM) as usize];
        (*msg).s_msg.header.cmd = RX_PERIOD_DONE;
    }

    imx_rpmsg_insert_workqueue(substream, msg, info);
}

unsafe extern "C" fn imx_rpmsg_pcm_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let info = dev_get_drvdata((*component).dev) as *mut rpmsg_info;
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let rpmsg = dev_get_drvdata((*cpu_dai).dev) as *mut fsl_rpmsg;
    let mut pcm_hardware: snd_pcm_hardware;
    let msg: *mut rpmsg_msg;
    let mut ret: c_int = 0;
    let cmd: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        msg = &mut (*info).msg[TX_OPEN as usize];
        (*msg).s_msg.header.cmd = TX_OPEN;

        /* reinitialize buffer counter*/
        cmd = TX_PERIOD_DONE + MSG_TYPE_A_NUM;
        (*info).msg[cmd as usize].s_msg.param.buffer_tail = 0;
        (*info).msg[cmd as usize].r_msg.param.buffer_tail = 0;
        (*info).msg[TX_POINTER as usize].r_msg.param.buffer_offset = 0;
    } else {
        msg = &mut (*info).msg[RX_OPEN as usize];
        (*msg).s_msg.header.cmd = RX_OPEN;

        /* reinitialize buffer counter*/
        cmd = RX_PERIOD_DONE + MSG_TYPE_A_NUM;
        (*info).msg[cmd as usize].s_msg.param.buffer_tail = 0;
        (*info).msg[cmd as usize].r_msg.param.buffer_tail = 0;
        (*info).msg[RX_POINTER as usize].r_msg.param.buffer_offset = 0;
    }

    ((*info).send_message.unwrap())(msg, info);

    pcm_hardware = imx_rpmsg_pcm_hardware;
    pcm_hardware.buffer_bytes_max = (*rpmsg).buffer_size[(*substream).stream as usize];
    pcm_hardware.period_bytes_max = pcm_hardware.buffer_bytes_max / 2;

    snd_soc_set_runtime_hwparams(substream, &mut pcm_hardware);

    ret = snd_pcm_hw_constraint_integer((*substream).runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        return ret;
    }

    (*info).msg_drop_count[(*substream).stream as usize] = 0;

    /* Create timer*/
    (*info).stream_timer[(*substream).stream as usize].info = info;
    (*info).stream_timer[(*substream).stream as usize].substream = substream;
    timer_setup(
        &mut (*info).stream_timer[(*substream).stream as usize].timer,
        Some(imx_rpmsg_timer_callback),
        0,
    );
    ret
}

unsafe extern "C" fn imx_rpmsg_pcm_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let info = dev_get_drvdata((*component).dev) as *mut rpmsg_info;
    let msg: *mut rpmsg_msg;

    /* Flush work in workqueue to make TX_CLOSE is the last message */
    flush_workqueue((*info).rpmsg_wq);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        msg = &mut (*info).msg[TX_CLOSE as usize];
        (*msg).s_msg.header.cmd = TX_CLOSE;
    } else {
        msg = &mut (*info).msg[RX_CLOSE as usize];
        (*msg).s_msg.header.cmd = RX_CLOSE;
    }

    ((*info).send_message.unwrap())(msg, info);

    timer_delete(&mut (*info).stream_timer[(*substream).stream as usize].timer);

    (*(*rtd).dai_link).ignore_suspend = 0;

    if (*info).msg_drop_count[(*substream).stream as usize] != 0 {
        dev_warn(
            (*rtd).dev,
            c"Msg is dropped!, number is %d\n".as_ptr(),
            (*info).msg_drop_count[(*substream).stream as usize],
        );
    }

    0
}

unsafe extern "C" fn imx_rpmsg_pcm_prepare(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let rpmsg = dev_get_drvdata((*cpu_dai).dev) as *mut fsl_rpmsg;

    /*
     * NON-MMAP mode, NONBLOCK, Version 2, enable lpa in dts
     * four conditions to determine the lpa is enabled.
     */
    if ((*runtime).access == SNDRV_PCM_ACCESS_RW_INTERLEAVED
        || (*runtime).access == SNDRV_PCM_ACCESS_RW_NONINTERLEAVED)
        && (*rpmsg).enable_lpa != 0
    {
        /*
         * Ignore suspend operation in low power mode
         * M core will continue playback music on A core suspend.
         */
        (*(*rtd).dai_link).ignore_suspend = 1;
        (*rpmsg).force_lpa = 1;
    } else {
        (*rpmsg).force_lpa = 0;
    }

    0
}

unsafe extern "C" fn imx_rpmsg_pcm_dma_complete(arg: *mut c_void) {
    let substream = arg as *mut snd_pcm_substream;

    snd_pcm_period_elapsed(substream);
}

unsafe extern "C" fn imx_rpmsg_prepare_and_submit(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let info = dev_get_drvdata((*component).dev) as *mut rpmsg_info;
    let msg: *mut rpmsg_msg;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        msg = &mut (*info).msg[TX_BUFFER as usize];
        (*msg).s_msg.header.cmd = TX_BUFFER;
    } else {
        msg = &mut (*info).msg[RX_BUFFER as usize];
        (*msg).s_msg.header.cmd = RX_BUFFER;
    }

    /* Send buffer address and buffer size */
    (*msg).s_msg.param.buffer_addr = (*(*substream).runtime).dma_addr;
    (*msg).s_msg.param.buffer_size = snd_pcm_lib_buffer_bytes(substream);
    (*msg).s_msg.param.period_size = snd_pcm_lib_period_bytes(substream);
    (*msg).s_msg.param.buffer_tail = 0;

    (*info).num_period[(*substream).stream as usize] =
        (*msg).s_msg.param.buffer_size / (*msg).s_msg.param.period_size;

    (*info).callback[(*substream).stream as usize] = Some(imx_rpmsg_pcm_dma_complete);
    (*info).callback_param[(*substream).stream as usize] = substream as *mut c_void;

    imx_rpmsg_insert_workqueue(substream, msg, info)
}

unsafe extern "C" fn imx_rpmsg_async_issue_pending(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let info = dev_get_drvdata((*component).dev) as *mut rpmsg_info;
    let msg: *mut rpmsg_msg;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        msg = &mut (*info).msg[TX_START as usize];
        (*msg).s_msg.header.cmd = TX_START;
    } else {
        msg = &mut (*info).msg[RX_START as usize];
        (*msg).s_msg.header.cmd = RX_START;
    }

    imx_rpmsg_insert_workqueue(substream, msg, info)
}

unsafe extern "C" fn imx_rpmsg_restart(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let info = dev_get_drvdata((*component).dev) as *mut rpmsg_info;
    let msg: *mut rpmsg_msg;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        msg = &mut (*info).msg[TX_RESTART as usize];
        (*msg).s_msg.header.cmd = TX_RESTART;
    } else {
        msg = &mut (*info).msg[RX_RESTART as usize];
        (*msg).s_msg.header.cmd = RX_RESTART;
    }

    imx_rpmsg_insert_workqueue(substream, msg, info)
}

unsafe extern "C" fn imx_rpmsg_pause(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let info = dev_get_drvdata((*component).dev) as *mut rpmsg_info;
    let msg: *mut rpmsg_msg;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        msg = &mut (*info).msg[TX_PAUSE as usize];
        (*msg).s_msg.header.cmd = TX_PAUSE;
    } else {
        msg = &mut (*info).msg[RX_PAUSE as usize];
        (*msg).s_msg.header.cmd = RX_PAUSE;
    }

    imx_rpmsg_insert_workqueue(substream, msg, info)
}

unsafe extern "C" fn imx_rpmsg_terminate_all(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let info = dev_get_drvdata((*component).dev) as *mut rpmsg_info;
    let msg: *mut rpmsg_msg;
    let cmd: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        msg = &mut (*info).msg[TX_TERMINATE as usize];
        (*msg).s_msg.header.cmd = TX_TERMINATE;
        /* Clear buffer count*/
        cmd = TX_PERIOD_DONE + MSG_TYPE_A_NUM;
        (*info).msg[cmd as usize].s_msg.param.buffer_tail = 0;
        (*info).msg[cmd as usize].r_msg.param.buffer_tail = 0;
        (*info).msg[TX_POINTER as usize].r_msg.param.buffer_offset = 0;
    } else {
        msg = &mut (*info).msg[RX_TERMINATE as usize];
        (*msg).s_msg.header.cmd = RX_TERMINATE;
        /* Clear buffer count*/
        cmd = RX_PERIOD_DONE + MSG_TYPE_A_NUM;
        (*info).msg[cmd as usize].s_msg.param.buffer_tail = 0;
        (*info).msg[cmd as usize].r_msg.param.buffer_tail = 0;
        (*info).msg[RX_POINTER as usize].r_msg.param.buffer_offset = 0;
    }

    timer_delete(&mut (*info).stream_timer[(*substream).stream as usize].timer);

    imx_rpmsg_insert_workqueue(substream, msg, info)
}

unsafe extern "C" fn imx_rpmsg_pcm_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let rpmsg = dev_get_drvdata((*cpu_dai).dev) as *mut fsl_rpmsg;
    let mut ret: c_int = 0;

    if cmd == SNDRV_PCM_TRIGGER_START {
        ret = imx_rpmsg_prepare_and_submit(component, substream);
        if ret != 0 {
            return ret;
        }
        ret = imx_rpmsg_async_issue_pending(component, substream);
    } else if cmd == SNDRV_PCM_TRIGGER_RESUME {
        if (*rpmsg).force_lpa == 0 {
            ret = imx_rpmsg_restart(component, substream);
        }
    } else if cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE {
        ret = imx_rpmsg_restart(component, substream);
    } else if cmd == SNDRV_PCM_TRIGGER_SUSPEND {
        if (*rpmsg).force_lpa == 0 {
            if ((*runtime).info & SNDRV_PCM_INFO_PAUSE) != 0 {
                ret = imx_rpmsg_pause(component, substream);
            } else {
                ret = imx_rpmsg_terminate_all(component, substream);
            }
        }
    } else if cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH {
        ret = imx_rpmsg_pause(component, substream);
    } else if cmd == SNDRV_PCM_TRIGGER_STOP {
        ret = imx_rpmsg_terminate_all(component, substream);
    } else {
        return -EINVAL;
    }

    if ret != 0 {
        return ret;
    }

    0
}

/*
 * imx_rpmsg_pcm_ack
 *
 * Send the period index to M core through rpmsg, but not send
 * all the period index to M core, reduce some unnessesary msg
 * to reduce the pressure of rpmsg bandwidth.
 */
unsafe extern "C" fn imx_rpmsg_pcm_ack(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let rpmsg = dev_get_drvdata((*cpu_dai).dev) as *mut fsl_rpmsg;
    let info = dev_get_drvdata((*component).dev) as *mut rpmsg_info;
    let period_size: snd_pcm_uframes_t = (*runtime).period_size;
    let mut avail: snd_pcm_sframes_t;
    let timer: *mut timer_list;
    let msg: *mut rpmsg_msg;
    let mut buffer_tail: c_int = 0;
    let mut written_num: c_int;

    if (*rpmsg).force_lpa == 0 {
        return 0;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        msg = &mut (*info).msg[(TX_PERIOD_DONE + MSG_TYPE_A_NUM) as usize];
        (*msg).s_msg.header.cmd = TX_PERIOD_DONE;
    } else {
        msg = &mut (*info).msg[(RX_PERIOD_DONE + MSG_TYPE_A_NUM) as usize];
        (*msg).s_msg.header.cmd = RX_PERIOD_DONE;
    }

    (*msg).s_msg.header.type_ = MSG_TYPE_C;

    buffer_tail = (frames_to_bytes(runtime, (*(*runtime).control).appl_ptr)
        % snd_pcm_lib_buffer_bytes(substream)) as c_int;
    buffer_tail = buffer_tail / snd_pcm_lib_period_bytes(substream) as c_int;

    /* There is update for period index */
    if buffer_tail != (*msg).s_msg.param.buffer_tail {
        written_num = buffer_tail - (*msg).s_msg.param.buffer_tail;
        if written_num < 0 {
            written_num += (*runtime).periods;
        }

        (*msg).s_msg.param.buffer_tail = buffer_tail;

        /* The notification message is updated to latest */
        guard_spinlock_irqsave(&mut (*info).lock[(*substream).stream as usize]);
        memcpy(
            &mut (*info).notify[(*substream).stream as usize] as *mut rpmsg_msg as *mut c_void,
            msg as *const c_void,
            size_of::<rpmsg_s_msg>(),
        );
        (*info).notify_updated[(*substream).stream as usize] = true;

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            avail = snd_pcm_playback_hw_avail(runtime);
        } else {
            avail = snd_pcm_capture_hw_avail(runtime);
        }

        timer = &mut (*info).stream_timer[(*substream).stream as usize].timer;
        /*
         * If the data in the buffer is less than one period before
         * this fill, which means the data may not enough on M
         * core side, we need to send message immediately to let
         * M core know the pointer is updated.
         * if there is more than one period data in the buffer before
         * this fill, which means the data is enough on M core side,
         * we can delay one period (using timer) to send the message
         * for reduce the message number in workqueue, because the
         * pointer may be updated by ack function later, we can
         * send latest pointer to M core side.
         */
        if avail - written_num as snd_pcm_sframes_t * period_size as snd_pcm_sframes_t
            <= period_size as snd_pcm_sframes_t
        {
            imx_rpmsg_insert_workqueue(substream, msg, info);
        } else if (*rpmsg).force_lpa != 0 && timer_pending(timer) == 0 {
            let time_msec: c_int;

            time_msec = ((*runtime).period_size * 1000 / (*runtime).rate as c_ulong) as c_int;
            mod_timer(timer, jiffies + msecs_to_jiffies(time_msec as c_uint));
        }
    }

    0
}

unsafe extern "C" fn imx_rpmsg_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let card = (*(*rtd).card).snd_card;
    let pcm = (*rtd).pcm;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let rpmsg = dev_get_drvdata((*cpu_dai).dev) as *mut fsl_rpmsg;
    let mut substream: *mut snd_pcm_substream;
    let mut ret: c_int;

    ret = dma_coerce_mask_and_coherent((*card).dev, DMA_BIT_MASK_32);
    if ret != 0 {
        return ret;
    }

    substream = (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream;
    if !substream.is_null() {
        ret = snd_pcm_set_fixed_buffer(
            substream,
            SNDRV_DMA_TYPE_DEV_WC,
            (*(*pcm).card).dev,
            (*rpmsg).buffer_size[SNDRV_PCM_STREAM_PLAYBACK as usize],
        );
        if ret < 0 {
            return ret;
        }
    }
    substream = (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream;
    if !substream.is_null() {
        ret = snd_pcm_set_fixed_buffer(
            substream,
            SNDRV_DMA_TYPE_DEV_WC,
            (*(*pcm).card).dev,
            (*rpmsg).buffer_size[SNDRV_PCM_STREAM_CAPTURE as usize],
        );
        if ret < 0 {
            return ret;
        }
    }

    ret
}

extern "C" {
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
}

static imx_rpmsg_soc_component: snd_soc_component_driver = snd_soc_component_driver {
    name: unsafe { IMX_PCM_DRV_NAME },
    pcm_new: Some(imx_rpmsg_pcm_new),
    open: Some(imx_rpmsg_pcm_open),
    close: Some(imx_rpmsg_pcm_close),
    hw_params: Some(imx_rpmsg_pcm_hw_params),
    trigger: Some(imx_rpmsg_pcm_trigger),
    pointer: Some(imx_rpmsg_pcm_pointer),
    ack: Some(imx_rpmsg_pcm_ack),
    prepare: Some(imx_rpmsg_pcm_prepare),
    debugfs_prefix: c"rpmsg".as_ptr(),
};

unsafe extern "C" fn imx_rpmsg_pcm_work(work: *mut work_struct) {
    let work_of_rpmsg: *mut work_of_rpmsg;
    let mut is_notification: bool_ = false;
    let info: *mut rpmsg_info;
    let mut msg: rpmsg_msg = core::mem::zeroed();
    let mut updated: bool_;

    work_of_rpmsg = container_of_work_of_rpmsg(work);
    info = (*work_of_rpmsg).info;

    /*
     * Every work in the work queue, first we check if there
     * is update for period is filled, because there may be not
     * enough data in M core side, need to let M core know
     * data is updated immediately.
     */
    guard_spinlock_irqsave(&mut (*info).lock[TX as usize]);
    updated = (*info).notify_updated[TX as usize];
    if updated {
        memcpy(
            &mut msg as *mut rpmsg_msg as *mut c_void,
            &(*info).notify[TX as usize] as *const rpmsg_msg as *const c_void,
            size_of::<rpmsg_s_msg>(),
        );
        (*info).notify_updated[TX as usize] = false;
    }
    if updated {
        ((*info).send_message.unwrap())(&mut msg, info);
    }

    guard_spinlock_irqsave(&mut (*info).lock[RX as usize]);
    updated = (*info).notify_updated[RX as usize];
    if updated {
        memcpy(
            &mut msg as *mut rpmsg_msg as *mut c_void,
            &(*info).notify[RX as usize] as *const rpmsg_msg as *const c_void,
            size_of::<rpmsg_s_msg>(),
        );
        (*info).notify_updated[RX as usize] = false;
    }
    if updated {
        ((*info).send_message.unwrap())(&mut msg, info);
    }

    /* Skip the notification message for it has been processed above */
    if (*work_of_rpmsg).msg.s_msg.header.type_ == MSG_TYPE_C
        && ((*work_of_rpmsg).msg.s_msg.header.cmd == TX_PERIOD_DONE
            || (*work_of_rpmsg).msg.s_msg.header.cmd == RX_PERIOD_DONE)
    {
        is_notification = true;
    }

    if !is_notification {
        ((*info).send_message.unwrap())(&mut (*work_of_rpmsg).msg, info);
    }

    /* update read index */
    guard_spinlock_irqsave(&mut (*info).wq_lock);
    (*info).work_read_index += 1;
    (*info).work_read_index %= WORK_MAX_NUM;
}

unsafe extern "C" fn imx_rpmsg_pcm_probe(pdev: *mut platform_device) -> c_int {
    let info: *mut rpmsg_info;
    let mut ret: c_int;
    let mut i: c_int;

    info = devm_kzalloc(&mut (*pdev).dev, size_of::<rpmsg_info>(), GFP_KERNEL) as *mut rpmsg_info;
    if info.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, info as *mut c_void);

    (*info).rpdev = container_of_rpmsg_device((*pdev).dev.parent);
    (*info).dev = &mut (*pdev).dev;
    /* Setup work queue */
    (*info).rpmsg_wq = alloc_ordered_workqueue(
        (*(*info).rpdev).id.name.as_ptr(),
        WQ_HIGHPRI | WQ_UNBOUND | WQ_FREEZABLE,
    );
    if (*info).rpmsg_wq.is_null() {
        dev_err(&mut (*pdev).dev, c"workqueue create failed\n".as_ptr());
        return -ENOMEM;
    }

    /* Write index initialize 1, make it differ with the read index */
    (*info).work_write_index = 1;
    (*info).send_message = Some(imx_rpmsg_pcm_send_message);

    i = 0;
    while i < WORK_MAX_NUM {
        INIT_WORK(&mut (*info).work_list[i as usize].work, Some(imx_rpmsg_pcm_work));
        (*info).work_list[i as usize].info = info;
        i += 1;
    }

    /* Initialize msg */
    i = 0;
    while i < MSG_MAX_NUM {
        (*info).msg[i as usize].s_msg.header.cate = IMX_RPMSG_AUDIO;
        (*info).msg[i as usize].s_msg.header.major = IMX_RMPSG_MAJOR;
        (*info).msg[i as usize].s_msg.header.minor = IMX_RMPSG_MINOR;
        (*info).msg[i as usize].s_msg.header.type_ = MSG_TYPE_A;
        (*info).msg[i as usize].s_msg.param.audioindex = 0;
        i += 1;
    }

    init_completion(&mut (*info).cmd_complete);
    mutex_init(&mut (*info).msg_lock);
    spin_lock_init(&mut (*info).lock[TX as usize]);
    spin_lock_init(&mut (*info).lock[RX as usize]);
    spin_lock_init(&mut (*info).wq_lock);

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &imx_rpmsg_soc_component,
        ptr::null_mut(),
        0,
    );
    if ret != 0 {
        goto_fail(info);
        return ret;
    }

    0
}

unsafe fn goto_fail(info: *mut rpmsg_info) {
    if !(*info).rpmsg_wq.is_null() {
        destroy_workqueue((*info).rpmsg_wq);
    }
}

unsafe extern "C" fn imx_rpmsg_pcm_remove(pdev: *mut platform_device) {
    let info = platform_get_drvdata(pdev) as *mut rpmsg_info;

    if !(*info).rpmsg_wq.is_null() {
        destroy_workqueue((*info).rpmsg_wq);
    }
}

unsafe extern "C" fn imx_rpmsg_pcm_runtime_resume(dev: *mut device) -> c_int {
    let info = dev_get_drvdata(dev) as *mut rpmsg_info;

    cpu_latency_qos_add_request(&mut (*info).pm_qos_req, 0);

    0
}

unsafe extern "C" fn imx_rpmsg_pcm_runtime_suspend(dev: *mut device) -> c_int {
    let info = dev_get_drvdata(dev) as *mut rpmsg_info;

    cpu_latency_qos_remove_request(&mut (*info).pm_qos_req);

    0
}

unsafe extern "C" fn imx_rpmsg_pcm_suspend(dev: *mut device) -> c_int {
    let info = dev_get_drvdata(dev) as *mut rpmsg_info;
    let rpmsg_tx: *mut rpmsg_msg;
    let rpmsg_rx: *mut rpmsg_msg;

    rpmsg_tx = &mut (*info).msg[TX_SUSPEND as usize];
    rpmsg_rx = &mut (*info).msg[RX_SUSPEND as usize];

    (*rpmsg_tx).s_msg.header.cmd = TX_SUSPEND;
    ((*info).send_message.unwrap())(rpmsg_tx, info);

    (*rpmsg_rx).s_msg.header.cmd = RX_SUSPEND;
    ((*info).send_message.unwrap())(rpmsg_rx, info);

    0
}

unsafe extern "C" fn imx_rpmsg_pcm_resume(dev: *mut device) -> c_int {
    let info = dev_get_drvdata(dev) as *mut rpmsg_info;
    let rpmsg_tx: *mut rpmsg_msg;
    let rpmsg_rx: *mut rpmsg_msg;

    rpmsg_tx = &mut (*info).msg[TX_RESUME as usize];
    rpmsg_rx = &mut (*info).msg[RX_RESUME as usize];

    (*rpmsg_tx).s_msg.header.cmd = TX_RESUME;
    ((*info).send_message.unwrap())(rpmsg_tx, info);

    (*rpmsg_rx).s_msg.header.cmd = RX_RESUME;
    ((*info).send_message.unwrap())(rpmsg_rx, info);

    0
}

static imx_rpmsg_pcm_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(imx_rpmsg_pcm_runtime_suspend),
    runtime_resume: Some(imx_rpmsg_pcm_runtime_resume),
    suspend: Some(imx_rpmsg_pcm_suspend),
    resume: Some(imx_rpmsg_pcm_resume),
};

static imx_rpmsg_pcm_id_table: [platform_device_id; 3] = [
    platform_device_id {
        name: str32(c"rpmsg-audio-channel".as_ptr()),
    },
    platform_device_id {
        name: str32(c"rpmsg-micfil-channel".as_ptr()),
    },
    platform_device_id { name: [0; 32] },
];
// MODULE_DEVICE_TABLE(platform, imx_rpmsg_pcm_id_table);

static mut imx_pcm_rpmsg_driver: platform_driver = platform_driver {
    probe: Some(imx_rpmsg_pcm_probe),
    remove: Some(imx_rpmsg_pcm_remove),
    id_table: imx_rpmsg_pcm_id_table.as_ptr(),
    driver: platform_driver_driver {
        name: unsafe { IMX_PCM_DRV_NAME },
        pm: &imx_rpmsg_pcm_pm_ops,
    },
};
// module_platform_driver(imx_pcm_rpmsg_driver);

// MODULE_DESCRIPTION("Freescale SoC Audio RPMSG PCM interface");
// MODULE_AUTHOR("Shengjiu Wang <shengjiu.wang@nxp.com>");
// MODULE_ALIAS("platform:" IMX_PCM_DRV_NAME);
// MODULE_LICENSE("GPL v2");

extern "C" {
    fn guard_mutex(mutex: *mut mutex);
    fn guard_spinlock_irqsave(lock: *mut spinlock_t);
    fn timer_container_of_stream_timer(timer: *mut timer_list) -> *mut stream_timer;
    fn container_of_work_of_rpmsg(work: *mut work_struct) -> *mut work_of_rpmsg;
    fn container_of_rpmsg_device(dev: *mut device) -> *mut rpmsg_device;
}

const fn str32(_s: *const c_char) -> [c_char; 32] {
    [0; 32]
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
