/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2022 Intel Corporation
 *
 * Author: Noah Klayman <noah.klayman@intel.com>
 */

// TRACE_SYSTEM sof_intel
// The Linux tracepoint, HDAudio, SOF audio, and trace-definition headers are
// external dependencies of this translated header.

use core::ffi::c_char;

// Opaque types supplied by the corresponding external headers.
#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct hdac_stream {
    pub index: u32,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: u32,
}

pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;
pub type snd_pcm_uframes_t = usize;

// TRACE_EVENT(sof_intel_hda_irq,
//     TP_PROTO(struct snd_sof_dev *sdev, char *source),
//     TP_ARGS(sdev, source),
//     TP_STRUCT__entry(__string(device_name, dev_name(sdev->dev))
//                      __string(source, source)),
//     TP_fast_assign(__assign_str(device_name); __assign_str(source);),
//     TP_printk("device_name=%s source=%s", __get_str(device_name),
//               __get_str(source)))
pub unsafe fn sof_intel_hda_irq(sdev: *mut snd_sof_dev, source: *mut c_char) {
    let _ = (sdev, source);
}

// DECLARE_EVENT_CLASS(sof_intel_ipc_firmware_template,
//     TP_ARGS(struct snd_sof_dev *sdev, u32 msg, u32 msg_ext),
//     TP_PROTO(sdev, msg, msg_ext),
//     TP_STRUCT__entry(__string(device_name, dev_name(sdev->dev))
//                      __field(u32, msg) __field(u32, msg_ext)),
//     TP_fast_assign(__assign_str(device_name); __entry->msg = msg;
//                    __entry->msg_ext = msg_ext;),
//     TP_printk("device_name=%s msg=%#x msg_ext=%#x", __get_str(device_name),
//               __entry->msg, __entry->msg_ext))
#[repr(C)]
pub struct sof_intel_ipc_firmware_template_entry {
    pub msg: u32,
    pub msg_ext: u32,
}

// DEFINE_EVENT(sof_intel_ipc_firmware_template, sof_intel_ipc_firmware_response,
//              TP_PROTO(struct snd_sof_dev *sdev, u32 msg, u32 msg_ext),
//              TP_ARGS(sdev, msg, msg_ext))
pub unsafe fn sof_intel_ipc_firmware_response(
    sdev: *mut snd_sof_dev,
    msg: u32,
    msg_ext: u32,
) {
    let _ = (sdev, msg, msg_ext);
}

// DEFINE_EVENT(sof_intel_ipc_firmware_template, sof_intel_ipc_firmware_initiated,
//              TP_PROTO(struct snd_sof_dev *sdev, u32 msg, u32 msg_ext),
//              TP_ARGS(sdev, msg, msg_ext))
pub unsafe fn sof_intel_ipc_firmware_initiated(
    sdev: *mut snd_sof_dev,
    msg: u32,
    msg_ext: u32,
) {
    let _ = (sdev, msg, msg_ext);
}

// TRACE_EVENT(sof_intel_D0I3C_updated,
//     TP_PROTO(struct snd_sof_dev *sdev, u8 reg), TP_ARGS(sdev, reg),
//     TP_STRUCT__entry(__string(device_name, dev_name(sdev->dev))
//                      __field(u8, reg)),
//     TP_fast_assign(__assign_str(device_name); __entry->reg = reg;),
//     TP_printk("device_name=%s register=%#x", __get_str(device_name),
//               __entry->reg))
pub unsafe fn sof_intel_D0I3C_updated(sdev: *mut snd_sof_dev, reg: u8) {
    let _ = (sdev, reg);
}

// TRACE_EVENT(sof_intel_hda_irq_ipc_check,
//     TP_PROTO(struct snd_sof_dev *sdev, u32 irq_status),
//     TP_ARGS(sdev, irq_status),
//     TP_STRUCT__entry(__string(device_name, dev_name(sdev->dev))
//                      __field(u32, irq_status)),
//     TP_fast_assign(__assign_str(device_name); __entry->irq_status = irq_status;),
//     TP_printk("device_name=%s irq_status=%#x", __get_str(device_name),
//               __entry->irq_status))
pub unsafe fn sof_intel_hda_irq_ipc_check(sdev: *mut snd_sof_dev, irq_status: u32) {
    let _ = (sdev, irq_status);
}

// TRACE_EVENT(sof_intel_hda_dsp_pcm,
//     TP_PROTO(struct snd_sof_dev *sdev, struct hdac_stream *hstream,
//              struct snd_pcm_substream *substream, snd_pcm_uframes_t pos),
//     TP_ARGS(sdev, hstream, substream, pos),
//     TP_STRUCT__entry(__string(device_name, dev_name(sdev->dev))
//                      __field(u32, hstream_index) __field(u32, substream)
//                      __field(unsigned long, pos)),
//     TP_fast_assign(__assign_str(device_name); __entry->hstream_index = hstream->index;
//                    __entry->substream = substream->stream; __entry->pos = pos;),
//     TP_printk("device_name=%s hstream_index=%d substream=%d pos=%lu",
//               __get_str(device_name), __entry->hstream_index,
//               __entry->substream, __entry->pos))
#[repr(C)]
pub struct sof_intel_hda_dsp_pcm_entry {
    pub hstream_index: u32,
    pub substream: u32,
    pub pos: usize,
}

// TRACE_EVENT(sof_intel_hda_dsp_stream_status,
//     TP_PROTO(struct device *dev, struct hdac_stream *s, u32 status),
//     TP_ARGS(dev, s, status),
//     TP_STRUCT__entry(__string(device_name, dev_name(dev))
//                      __field(u32, stream) __field(u32, status)),
//     TP_fast_assign(__assign_str(device_name); __entry->stream = s->index;
//                    __entry->status = status;),
//     TP_printk("device_name=%s stream=%d status=%#x", __get_str(device_name),
//               __entry->stream, __entry->status))
#[repr(C)]
pub struct sof_intel_hda_dsp_stream_status_entry {
    pub stream: u32,
    pub status: u32,
}

// TRACE_EVENT(sof_intel_hda_dsp_check_stream_irq,
//     TP_PROTO(struct snd_sof_dev *sdev, u32 status), TP_ARGS(sdev, status),
//     TP_STRUCT__entry(__string(device_name, dev_name(sdev->dev))
//                      __field(u32, status)),
//     TP_fast_assign(__assign_str(device_name); __entry->status = status;),
//     TP_printk("device_name=%s status=%#x", __get_str(device_name),
//               __entry->status))
pub unsafe fn sof_intel_hda_dsp_check_stream_irq(sdev: *mut snd_sof_dev, status: u32) {
    let _ = (sdev, status);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
