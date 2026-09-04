// SPDX-License-Identifier: GPL-2.0+
/*
 * virtio-snd: Virtio sound device
 * Copyright (C) 2021 OpenSynergy GmbH
 */
// Dependencies: sound/pcm_params.h, virtio_card.h

/*
 * I/O messages lifetime
 * ---------------------
 *
 * Allocation:
 *   Messages are initially allocated in the ops->hw_params() after the size and
 *   number of periods have been successfully negotiated.
 *
 * Freeing:
 *   Messages can be safely freed after the queue has been successfully flushed
 *   (RELEASE command in the ops->sync_stop()) and the ops->hw_free() has been
 *   called.
 *
 *   When the substream stops, the ops->sync_stop() waits until the device has
 *   completed all pending messages. This wait can be interrupted either by a
 *   signal or due to a timeout. In this case, the device can still access
 *   messages even after calling ops->hw_free(). It can also issue an interrupt,
 *   and the interrupt handler will also try to access message structures.
 *
 *   Therefore, freeing of already allocated messages occurs:
 *
 *   - in ops->hw_params(), if this operator was called several times in a row,
 *     or if ops->hw_free() failed to free messages previously;
 *
 *   - in ops->hw_free(), if the queue has been successfully flushed;
 *
 *   - in dev->release().
 */

/* Map for converting ALSA format to VirtIO format. */
#[repr(C)]
struct VirtsndA2vFormat {
    alsa_bit: u32, // snd_pcm_format_t
    vio_bit: u32,
}

static G_A2V_FORMAT_MAP: &[VirtsndA2vFormat] = &[
    VirtsndA2vFormat { alsa_bit: 0x00020000, vio_bit: 0x00000001 }, // SNDRV_PCM_FORMAT_IMA_ADPCM, VIRTIO_SND_PCM_FMT_IMA_ADPCM
    VirtsndA2vFormat { alsa_bit: 0x00000001, vio_bit: 0x00000002 }, // SNDRV_PCM_FORMAT_MU_LAW, VIRTIO_SND_PCM_FMT_MU_LAW
    VirtsndA2vFormat { alsa_bit: 0x00000002, vio_bit: 0x00000004 }, // SNDRV_PCM_FORMAT_A_LAW, VIRTIO_SND_PCM_FMT_A_LAW
    VirtsndA2vFormat { alsa_bit: 0x00000008, vio_bit: 0x00000008 }, // SNDRV_PCM_FORMAT_S8, VIRTIO_SND_PCM_FMT_S8
    VirtsndA2vFormat { alsa_bit: 0x00000004, vio_bit: 0x00000010 }, // SNDRV_PCM_FORMAT_U8, VIRTIO_SND_PCM_FMT_U8
    VirtsndA2vFormat { alsa_bit: 0x00000010, vio_bit: 0x00000020 }, // SNDRV_PCM_FORMAT_S16_LE, VIRTIO_SND_PCM_FMT_S16
    VirtsndA2vFormat { alsa_bit: 0x00000020, vio_bit: 0x00000040 }, // SNDRV_PCM_FORMAT_U16_LE, VIRTIO_SND_PCM_FMT_U16
    VirtsndA2vFormat { alsa_bit: 0x00000040, vio_bit: 0x00000080 }, // SNDRV_PCM_FORMAT_S18_3LE, VIRTIO_SND_PCM_FMT_S18_3
    VirtsndA2vFormat { alsa_bit: 0x00000080, vio_bit: 0x00000100 }, // SNDRV_PCM_FORMAT_U18_3LE, VIRTIO_SND_PCM_FMT_U18_3
    VirtsndA2vFormat { alsa_bit: 0x00000100, vio_bit: 0x00000200 }, // SNDRV_PCM_FORMAT_S20_3LE, VIRTIO_SND_PCM_FMT_S20_3
    VirtsndA2vFormat { alsa_bit: 0x00000200, vio_bit: 0x00000400 }, // SNDRV_PCM_FORMAT_U20_3LE, VIRTIO_SND_PCM_FMT_U20_3
    VirtsndA2vFormat { alsa_bit: 0x00000400, vio_bit: 0x00000800 }, // SNDRV_PCM_FORMAT_S24_3LE, VIRTIO_SND_PCM_FMT_S24_3
    VirtsndA2vFormat { alsa_bit: 0x00000800, vio_bit: 0x00001000 }, // SNDRV_PCM_FORMAT_U24_3LE, VIRTIO_SND_PCM_FMT_U24_3
    VirtsndA2vFormat { alsa_bit: 0x00001000, vio_bit: 0x00002000 }, // SNDRV_PCM_FORMAT_S20_LE, VIRTIO_SND_PCM_FMT_S20
    VirtsndA2vFormat { alsa_bit: 0x00002000, vio_bit: 0x00004000 }, // SNDRV_PCM_FORMAT_U20_LE, VIRTIO_SND_PCM_FMT_U20
    VirtsndA2vFormat { alsa_bit: 0x00004000, vio_bit: 0x00008000 }, // SNDRV_PCM_FORMAT_S24_LE, VIRTIO_SND_PCM_FMT_S24
    VirtsndA2vFormat { alsa_bit: 0x00008000, vio_bit: 0x00010000 }, // SNDRV_PCM_FORMAT_U24_LE, VIRTIO_SND_PCM_FMT_U24
    VirtsndA2vFormat { alsa_bit: 0x00010000, vio_bit: 0x00020000 }, // SNDRV_PCM_FORMAT_S32_LE, VIRTIO_SND_PCM_FMT_S32
    VirtsndA2vFormat { alsa_bit: 0x00020000, vio_bit: 0x00040000 }, // SNDRV_PCM_FORMAT_U32_LE, VIRTIO_SND_PCM_FMT_U32
    VirtsndA2vFormat { alsa_bit: 0x00040000, vio_bit: 0x00080000 }, // SNDRV_PCM_FORMAT_FLOAT_LE, VIRTIO_SND_PCM_FMT_FLOAT
    VirtsndA2vFormat { alsa_bit: 0x00080000, vio_bit: 0x00100000 }, // SNDRV_PCM_FORMAT_FLOAT64_LE, VIRTIO_SND_PCM_FMT_FLOAT64
    VirtsndA2vFormat { alsa_bit: 0x00100000, vio_bit: 0x00200000 }, // SNDRV_PCM_FORMAT_DSD_U8, VIRTIO_SND_PCM_FMT_DSD_U8
    VirtsndA2vFormat { alsa_bit: 0x00200000, vio_bit: 0x00400000 }, // SNDRV_PCM_FORMAT_DSD_U16_LE, VIRTIO_SND_PCM_FMT_DSD_U16
    VirtsndA2vFormat { alsa_bit: 0x00400000, vio_bit: 0x00800000 }, // SNDRV_PCM_FORMAT_DSD_U32_LE, VIRTIO_SND_PCM_FMT_DSD_U32
    VirtsndA2vFormat { alsa_bit: 0x00800000, vio_bit: 0x01000000 }, // SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE, VIRTIO_SND_PCM_FMT_IEC958_SUBFRAME
];

/* Map for converting ALSA frame rate to VirtIO frame rate. */
#[repr(C)]
struct VirtsndA2vRate {
    rate: u32,
    vio_bit: u32,
}

static G_A2V_RATE_MAP: &[VirtsndA2vRate] = &[
    VirtsndA2vRate { rate: 5512, vio_bit: 0x00000001 },   // VIRTIO_SND_PCM_RATE_5512
    VirtsndA2vRate { rate: 8000, vio_bit: 0x00000002 },   // VIRTIO_SND_PCM_RATE_8000
    VirtsndA2vRate { rate: 11025, vio_bit: 0x00000004 },  // VIRTIO_SND_PCM_RATE_11025
    VirtsndA2vRate { rate: 16000, vio_bit: 0x00000008 },  // VIRTIO_SND_PCM_RATE_16000
    VirtsndA2vRate { rate: 22050, vio_bit: 0x00000010 },  // VIRTIO_SND_PCM_RATE_22050
    VirtsndA2vRate { rate: 32000, vio_bit: 0x00000020 },  // VIRTIO_SND_PCM_RATE_32000
    VirtsndA2vRate { rate: 44100, vio_bit: 0x00000040 },  // VIRTIO_SND_PCM_RATE_44100
    VirtsndA2vRate { rate: 48000, vio_bit: 0x00000080 },  // VIRTIO_SND_PCM_RATE_48000
    VirtsndA2vRate { rate: 64000, vio_bit: 0x00000100 },  // VIRTIO_SND_PCM_RATE_64000
    VirtsndA2vRate { rate: 88200, vio_bit: 0x00000200 },  // VIRTIO_SND_PCM_RATE_88200
    VirtsndA2vRate { rate: 96000, vio_bit: 0x00000400 },  // VIRTIO_SND_PCM_RATE_96000
    VirtsndA2vRate { rate: 176400, vio_bit: 0x00000800 }, // VIRTIO_SND_PCM_RATE_176400
    VirtsndA2vRate { rate: 192000, vio_bit: 0x00001000 }, // VIRTIO_SND_PCM_RATE_192000
    VirtsndA2vRate { rate: 384000, vio_bit: 0x00002000 }, // VIRTIO_SND_PCM_RATE_384000
];

extern "C" {
    type SndPcmSubstream;
    type VirtioPcm;
    type VirtioPcmStream;
    type VirtioPcmSubstream;
    type VirtioDevice;
    type VirtioSnd;
    type VirtioSndQueue;
    type VirtioSndMsg;
    type VirtioSndPcmSetParams;
    type SndPcmHwParams;
    type SndPcmRuntime;

    fn snd_pcm_substream_chip(substream: *mut SndPcmSubstream) -> *mut VirtioPcm;
    fn snd_pcm_lib_buffer_bytes(substream: *mut SndPcmSubstream) -> u32;
    fn snd_pcm_lib_period_bytes(substream: *mut SndPcmSubstream) -> u32;
    fn snd_pcm_hw_constraint_integer(runtime: *mut SndPcmRuntime, param: u32);
    fn virtsnd_pcm_msg_pending_num(vss: *mut VirtioPcmSubstream) -> u32;
    fn virtsnd_pcm_ctl_msg_alloc(vss: *mut VirtioPcmSubstream, command: u32, flags: u32) -> *mut VirtioSndMsg;
    fn virtsnd_ctl_msg_request(msg: *mut VirtioSndMsg) -> *mut VirtioSndPcmSetParams;
    fn virtsnd_ctl_msg_send_sync(snd: *mut VirtioSnd, msg: *mut VirtioSndMsg) -> i32;
    fn virtsnd_pcm_queue(vss: *mut VirtioPcmSubstream) -> *mut VirtioSndQueue;
    fn virtsnd_pcm_msg_send(vss: *mut VirtioPcmSubstream, sw_data: usize, bytes: usize) -> i32;
    fn virtsnd_pcm_msg_free(vss: *mut VirtioPcmSubstream);
    fn virtsnd_pcm_msg_alloc(vss: *mut VirtioPcmSubstream, periods: u32, period_bytes: u32) -> i32;
    fn cancel_work_sync(work: *mut core::ffi::c_void);
    fn wait_event_interruptible_timeout(wait: *mut core::ffi::c_void, condition: bool, timeout: u32) -> i32;
    fn cpu_to_le32(value: u32) -> u32;
    fn dev_err(dev: *mut VirtioDevice, fmt: *const u8, ...);
    fn dev_warn(dev: *mut VirtioDevice, fmt: *const u8, ...);
    fn snd_pcm_indirect_playback_pointer(substream: *mut SndPcmSubstream, rec: *mut core::ffi::c_void, hw_ptr: u32) -> u32;
    fn snd_pcm_indirect_capture_pointer(substream: *mut SndPcmSubstream, rec: *mut core::ffi::c_void, hw_ptr: u32) -> u32;
    fn snd_pcm_indirect_playback_transfer(substream: *mut SndPcmSubstream, rec: *mut core::ffi::c_void, copy: extern "C" fn(*mut SndPcmSubstream, *mut core::ffi::c_void, usize)) -> i32;
    fn snd_pcm_indirect_capture_transfer(substream: *mut SndPcmSubstream, rec: *mut core::ffi::c_void, copy: extern "C" fn(*mut SndPcmSubstream, *mut core::ffi::c_void, usize)) -> i32;
    fn snd_pcm_lib_ioctl(substream: *mut SndPcmSubstream, cmd: u32, arg: *mut core::ffi::c_void) -> i32;

    // Kernel module interface
    pub static virtsnd_msg_timeout_ms: u32;

    // External functions for helper macros
    fn msecs_to_jiffies(msecs: u32) -> u32;
}

/// virtsnd_pcm_open() - Open the PCM substream.
/// @substream: Kernel ALSA substream.
///
/// Context: Process context.
/// Return: 0 on success, -errno on failure.
unsafe extern "C" fn virtsnd_pcm_open(substream: *mut SndPcmSubstream) -> i32 {
    let vpcm = snd_pcm_substream_chip(substream);
    // Access to streams array through vpcm
    // struct virtio_pcm_stream *vs = &vpcm->streams[substream->stream];
    // struct virtio_pcm_substream *vss = vs->substreams[substream->number];
    //
    // substream->runtime->hw = vss->hw;
    // substream->private_data = vss;
    //
    // snd_pcm_hw_constraint_integer(substream->runtime,
    //                               SNDRV_PCM_HW_PARAM_PERIODS);
    //
    // vss->stopped = !!virtsnd_pcm_msg_pending_num(vss);
    // vss->suspended = false;
    //
    // If the substream has already been used, then the I/O queue may be in
    // an invalid state. Just in case, we do a check and try to return the
    // queue to its original state, if necessary.
    // return virtsnd_pcm_sync_stop(substream);

    virtsnd_pcm_sync_stop(substream)
}

/// virtsnd_pcm_close() - Close the PCM substream.
/// @substream: Kernel ALSA substream.
///
/// Context: Process context.
/// Return: 0.
extern "C" fn virtsnd_pcm_close(_substream: *mut SndPcmSubstream) -> i32 {
    0
}

/// virtsnd_pcm_dev_set_params() - Set the parameters of the PCM substream on
///                                the device side.
/// @vss: VirtIO PCM substream.
/// @buffer_bytes: Size of the hardware buffer.
/// @period_bytes: Size of the hardware period.
/// @channels: Selected number of channels.
/// @format: Selected sample format (SNDRV_PCM_FORMAT_XXX).
/// @rate: Selected frame rate.
///
/// Context: Any context that permits to sleep.
/// Return: 0 on success, -errno on failure.
unsafe extern "C" fn virtsnd_pcm_dev_set_params(
    vss: *mut VirtioPcmSubstream,
    buffer_bytes: u32,
    period_bytes: u32,
    channels: u32,
    format: u32,
    rate: u32,
) -> i32 {
    let mut vformat = -1i32;
    let mut vrate = -1i32;

    // Find format mapping
    for i in 0..G_A2V_FORMAT_MAP.len() {
        if G_A2V_FORMAT_MAP[i].alsa_bit == format {
            vformat = G_A2V_FORMAT_MAP[i].vio_bit as i32;
            break;
        }
    }

    // Find rate mapping
    for i in 0..G_A2V_RATE_MAP.len() {
        if G_A2V_RATE_MAP[i].rate == rate {
            vrate = G_A2V_RATE_MAP[i].vio_bit as i32;
            break;
        }
    }

    if vformat == -1 || vrate == -1 {
        return -22; // -EINVAL
    }

    let msg = virtsnd_pcm_ctl_msg_alloc(vss, 10, 0); // VIRTIO_SND_R_PCM_SET_PARAMS = 10, GFP_KERNEL = 0
    if msg.is_null() {
        return -12; // -ENOMEM
    }

    let request = virtsnd_ctl_msg_request(msg);
    (*request).buffer_bytes = cpu_to_le32(buffer_bytes);
    (*request).period_bytes = cpu_to_le32(period_bytes);
    (*request).channels = channels as u8;
    (*request).format = vformat as u32;
    (*request).rate = vrate as u32;

    // Access vss->features and vss->snd
    // if (vss->features & (1U << VIRTIO_SND_PCM_F_MSG_POLLING))
    //     request->features |= cpu_to_le32(1U << VIRTIO_SND_PCM_F_MSG_POLLING);
    // if (vss->features & (1U << VIRTIO_SND_PCM_F_EVT_XRUNS))
    //     request->features |= cpu_to_le32(1U << VIRTIO_SND_PCM_F_EVT_XRUNS);

    // return virtsnd_ctl_msg_send_sync(vss->snd, msg);
    0 // Placeholder for external call
}

/// virtsnd_pcm_hw_params() - Set the parameters of the PCM substream.
/// @substream: Kernel ALSA substream.
/// @hw_params: Hardware parameters.
///
/// Context: Process context.
/// Return: 0 on success, -errno on failure.
unsafe extern "C" fn virtsnd_pcm_hw_params(
    substream: *mut SndPcmSubstream,
    hw_params: *mut SndPcmHwParams,
) -> i32 {
    // struct virtio_pcm_substream *vss = snd_pcm_substream_chip(substream);
    // struct virtio_device *vdev = vss->snd->vdev;

    // Placeholder access to structure members
    // if (virtsnd_pcm_msg_pending_num(vss)) {
    //     dev_err(&vdev->dev, "SID %u: invalid I/O queue state\n", vss->sid);
    //     return -EBADFD;
    // }

    // rc = virtsnd_pcm_dev_set_params(vss, params_buffer_bytes(hw_params), ...);
    // if (rc)
    //     return rc;
    //
    // virtsnd_pcm_msg_free(vss);
    // return virtsnd_pcm_msg_alloc(vss, params_periods(hw_params), ...);
    0
}

/// virtsnd_pcm_hw_free() - Reset the parameters of the PCM substream.
/// @substream: Kernel ALSA substream.
///
/// Context: Process context.
/// Return: 0
extern "C" fn virtsnd_pcm_hw_free(_substream: *mut SndPcmSubstream) -> i32 {
    // struct virtio_pcm_substream *vss = snd_pcm_substream_chip(substream);
    // if (!virtsnd_pcm_msg_pending_num(vss))
    //     virtsnd_pcm_msg_free(vss);
    0
}

/// virtsnd_pcm_prepare() - Prepare the PCM substream.
/// @substream: Kernel ALSA substream.
///
/// Context: Process context.
/// Return: 0 on success, -errno on failure.
unsafe extern "C" fn virtsnd_pcm_prepare(substream: *mut SndPcmSubstream) -> i32 {
    // struct virtio_pcm_substream *vss = snd_pcm_substream_chip(substream);
    // struct virtio_device *vdev = vss->snd->vdev;
    // struct virtio_snd_msg *msg;
    //
    // if (!vss->suspended) {
    //     if (virtsnd_pcm_msg_pending_num(vss)) {
    //         dev_err(&vdev->dev, "SID %u: invalid I/O queue state\n", vss->sid);
    //         return -EBADFD;
    //     }
    //     vss->buffer_bytes = snd_pcm_lib_buffer_bytes(substream);
    //     vss->hw_ptr = 0;
    // } else {
    //     struct snd_pcm_runtime *runtime = substream->runtime;
    //     unsigned int buffer_bytes = snd_pcm_lib_buffer_bytes(substream);
    //     unsigned int period_bytes = snd_pcm_lib_period_bytes(substream);
    //     int rc;
    //     rc = virtsnd_pcm_dev_set_params(vss, buffer_bytes, period_bytes,
    //                                     runtime->channels, runtime->format,
    //                                     runtime->rate);
    //     if (rc)
    //         return rc;
    // }
    //
    // vss->xfer_xrun = false;
    // vss->suspended = false;
    // vss->msg_count = 0;
    //
    // memset(&vss->pcm_indirect, 0, sizeof(vss->pcm_indirect));
    // vss->pcm_indirect.sw_buffer_size = vss->pcm_indirect.hw_buffer_size =
    //     snd_pcm_lib_buffer_bytes(substream);
    //
    // msg = virtsnd_pcm_ctl_msg_alloc(vss, VIRTIO_SND_R_PCM_PREPARE, GFP_KERNEL);
    // if (!msg)
    //     return -ENOMEM;
    //
    // return virtsnd_ctl_msg_send_sync(vss->snd, msg);
    0
}

/// virtsnd_pcm_trigger() - Process command for the PCM substream.
/// @substream: Kernel ALSA substream.
/// @command: Substream command (SNDRV_PCM_TRIGGER_XXX).
///
/// Context: Any context. Takes and releases the VirtIO substream spinlock.
///          May take and release the tx/rx queue spinlock.
/// Return: 0 on success, -errno on failure.
unsafe extern "C" fn virtsnd_pcm_trigger(substream: *mut SndPcmSubstream, command: i32) -> i32 {
    // struct virtio_pcm_substream *vss = snd_pcm_substream_chip(substream);
    // struct virtio_snd *snd = vss->snd;
    // struct virtio_snd_queue *queue;
    // struct virtio_snd_msg *msg;
    // int rc = 0;
    //
    // switch (command) {
    // case SNDRV_PCM_TRIGGER_START:
    // case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    //     queue = virtsnd_pcm_queue(vss);
    //     scoped_guard(spinlock_irqsave, &queue->lock) {
    //         guard(spinlock)(&vss->lock);
    //         if (vss->direction == SNDRV_PCM_STREAM_CAPTURE)
    //             rc = virtsnd_pcm_msg_send(vss, 0, vss->buffer_bytes);
    //         if (rc)
    //             return rc;
    //         vss->xfer_enabled = true;
    //     }
    //     msg = virtsnd_pcm_ctl_msg_alloc(vss, VIRTIO_SND_R_PCM_START, GFP_KERNEL);
    //     if (!msg) {
    //         guard(spinlock_irqsave)(&vss->lock);
    //         vss->xfer_enabled = false;
    //         return -ENOMEM;
    //     }
    //     return virtsnd_ctl_msg_send_sync(snd, msg);
    // case SNDRV_PCM_TRIGGER_SUSPEND:
    //     vss->suspended = true;
    //     fallthrough;
    // case SNDRV_PCM_TRIGGER_STOP:
    //     vss->stopped = true;
    //     fallthrough;
    // case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    //     scoped_guard(spinlock_irqsave, &vss->lock) {
    //         vss->xfer_enabled = false;
    //     }
    //     msg = virtsnd_pcm_ctl_msg_alloc(vss, VIRTIO_SND_R_PCM_STOP, GFP_KERNEL);
    //     if (!msg)
    //         return -ENOMEM;
    //     return virtsnd_ctl_msg_send_sync(snd, msg);
    // default:
    //     return -EINVAL;
    // }
    0
}

/// virtsnd_pcm_sync_stop() - Synchronous PCM substream stop.
/// @substream: Kernel ALSA substream.
///
/// The function can be called both from the upper level or from the driver
/// itself.
///
/// Context: Process context. Takes and releases the VirtIO substream spinlock.
/// Return: 0 on success, -errno on failure.
unsafe extern "C" fn virtsnd_pcm_sync_stop(substream: *mut SndPcmSubstream) -> i32 {
    // struct virtio_pcm_substream *vss = snd_pcm_substream_chip(substream);
    // struct virtio_snd *snd = vss->snd;
    // struct virtio_snd_msg *msg;
    // unsigned int js = msecs_to_jiffies(virtsnd_msg_timeout_ms);
    // int rc;
    //
    // cancel_work_sync(&vss->elapsed_period);
    //
    // if (!vss->stopped)
    //     return 0;
    //
    // msg = virtsnd_pcm_ctl_msg_alloc(vss, VIRTIO_SND_R_PCM_RELEASE, GFP_KERNEL);
    // if (!msg)
    //     return -ENOMEM;
    //
    // rc = virtsnd_ctl_msg_send_sync(snd, msg);
    // if (rc)
    //     return rc;
    //
    // rc = wait_event_interruptible_timeout(vss->msg_empty,
    //                                       !virtsnd_pcm_msg_pending_num(vss),
    //                                       js);
    // if (rc <= 0) {
    //     dev_warn(&snd->vdev->dev, "SID %u: failed to flush I/O queue\n", vss->sid);
    //     return !rc ? -ETIMEDOUT : rc;
    // }
    //
    // vss->stopped = false;
    // return 0;
    0
}

/// virtsnd_pcm_pb_pointer() - Get the current hardware position for the PCM
///                         substream for playback.
/// @substream: Kernel ALSA substream.
///
/// Context: Any context.
/// Return: Hardware position in frames inside [0 ... buffer_size) range.
unsafe extern "C" fn virtsnd_pcm_pb_pointer(substream: *mut SndPcmSubstream) -> u32 {
    // struct virtio_pcm_substream *vss = snd_pcm_substream_chip(substream);
    // return snd_pcm_indirect_playback_pointer(substream, &vss->pcm_indirect, vss->hw_ptr);
    0
}

/// virtsnd_pcm_cp_pointer() - Get the current hardware position for the PCM
///                         substream for capture.
/// @substream: Kernel ALSA substream.
///
/// Context: Any context.
/// Return: Hardware position in frames inside [0 ... buffer_size) range.
unsafe extern "C" fn virtsnd_pcm_cp_pointer(substream: *mut SndPcmSubstream) -> u32 {
    // struct virtio_pcm_substream *vss = snd_pcm_substream_chip(substream);
    // return snd_pcm_indirect_capture_pointer(substream, &vss->pcm_indirect, vss->hw_ptr);
    0
}

unsafe extern "C" fn virtsnd_pcm_trans_copy(
    substream: *mut SndPcmSubstream,
    rec: *mut core::ffi::c_void,
    bytes: usize,
) {
    // struct virtio_pcm_substream *vss = snd_pcm_substream_chip(substream);
    // virtsnd_pcm_msg_send(vss, rec->sw_data, bytes);
}

unsafe extern "C" fn virtsnd_pcm_pb_ack(substream: *mut SndPcmSubstream) -> i32 {
    // struct virtio_pcm_substream *vss = snd_pcm_substream_chip(substream);
    // struct virtio_snd_queue *queue = virtsnd_pcm_queue(vss);
    // guard(spinlock_irqsave)(&queue->lock);
    // guard(spinlock)(&vss->lock);
    // return snd_pcm_indirect_playback_transfer(substream, &vss->pcm_indirect, virtsnd_pcm_trans_copy);
    0
}

unsafe extern "C" fn virtsnd_pcm_cp_ack(substream: *mut SndPcmSubstream) -> i32 {
    // struct virtio_pcm_substream *vss = snd_pcm_substream_chip(substream);
    // struct virtio_snd_queue *queue = virtsnd_pcm_queue(vss);
    // guard(spinlock_irqsave)(&queue->lock);
    // guard(spinlock)(&vss->lock);
    // return snd_pcm_indirect_capture_transfer(substream, &vss->pcm_indirect, virtsnd_pcm_trans_copy);
    0
}

/* PCM substream operators map. */
#[no_mangle]
pub static virtsnd_pcm_ops: [SndPcmOps; 2] = [
    SndPcmOps {
        open: Some(virtsnd_pcm_open),
        close: Some(virtsnd_pcm_close),
        ioctl: Some(snd_pcm_lib_ioctl),
        hw_params: Some(virtsnd_pcm_hw_params),
        hw_free: Some(virtsnd_pcm_hw_free),
        prepare: Some(virtsnd_pcm_prepare),
        trigger: Some(virtsnd_pcm_trigger),
        sync_stop: Some(virtsnd_pcm_sync_stop),
        pointer: Some(virtsnd_pcm_pb_pointer),
        ack: Some(virtsnd_pcm_pb_ack),
    },
    SndPcmOps {
        open: Some(virtsnd_pcm_open),
        close: Some(virtsnd_pcm_close),
        ioctl: Some(snd_pcm_lib_ioctl),
        hw_params: Some(virtsnd_pcm_hw_params),
        hw_free: Some(virtsnd_pcm_hw_free),
        prepare: Some(virtsnd_pcm_prepare),
        trigger: Some(virtsnd_pcm_trigger),
        sync_stop: Some(virtsnd_pcm_sync_stop),
        pointer: Some(virtsnd_pcm_cp_pointer),
        ack: Some(virtsnd_pcm_cp_ack),
    },
];

#[repr(C)]
pub struct SndPcmOps {
    pub open: Option<extern "C" fn(*mut SndPcmSubstream) -> i32>,
    pub close: Option<extern "C" fn(*mut SndPcmSubstream) -> i32>,
    pub ioctl: Option<extern "C" fn(*mut SndPcmSubstream, u32, *mut core::ffi::c_void) -> i32>,
    pub hw_params: Option<extern "C" fn(*mut SndPcmSubstream, *mut SndPcmHwParams) -> i32>,
    pub hw_free: Option<extern "C" fn(*mut SndPcmSubstream) -> i32>,
    pub prepare: Option<extern "C" fn(*mut SndPcmSubstream) -> i32>,
    pub trigger: Option<extern "C" fn(*mut SndPcmSubstream, i32) -> i32>,
    pub sync_stop: Option<extern "C" fn(*mut SndPcmSubstream) -> i32>,
    pub pointer: Option<extern "C" fn(*mut SndPcmSubstream) -> u32>,
    pub ack: Option<extern "C" fn(*mut SndPcmSubstream) -> i32>,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
