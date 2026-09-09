/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2022 Intel Corporation
 *
 * Author: Noah Klayman <noah.klayman@intel.com>
 */

// Translated from trace/events/sof.h.  The tracepoint machinery and the
// referenced kernel/SOF types are supplied by the surrounding environment.

#[allow(non_camel_case_types)]
pub enum snd_sof_widget {}
#[allow(non_camel_case_types)]
pub enum snd_sof_dev {}
#[allow(non_camel_case_types)]
pub enum sof_ipc_stream_posn {}
#[allow(non_camel_case_types)]
pub enum snd_sof_pcm {}
#[allow(non_camel_case_types)]
pub enum snd_pcm_substream {}
#[allow(non_camel_case_types)]
pub enum device {}

// C: DECLARE_EVENT_CLASS(sof_widget_template, ...)
// TP_PROTO: struct snd_sof_widget *swidget
// TP_STRUCT__entry: string name = swidget->widget->name; int use_count
// TP_fast_assign: name = swidget->widget->name; use_count = swidget->use_count
// TP_printk: "name=%s use_count=%d"
pub const SOF_WIDGET_TEMPLATE: &str = "name=%s use_count=%d";

// C: DEFINE_EVENT(sof_widget_template, sof_widget_setup,
//                 TP_PROTO(struct snd_sof_widget *swidget), TP_ARGS(swidget))
pub const SOF_WIDGET_SETUP: &str = "sof_widget_setup";

// C: DEFINE_EVENT(sof_widget_template, sof_widget_free,
//                 TP_PROTO(struct snd_sof_widget *swidget), TP_ARGS(swidget))
pub const SOF_WIDGET_FREE: &str = "sof_widget_free";

// C: TRACE_EVENT(sof_ipc3_period_elapsed_position, ...)
// TP_PROTO: struct snd_sof_dev *sdev, struct sof_ipc_stream_posn *posn
// TP_STRUCT__entry: string device_name = dev_name(sdev->dev);
//                  u64 host_posn, u64 dai_posn, u64 wallclock
// TP_fast_assign: device_name = dev_name(sdev->dev); host_posn = posn->host_posn;
//                 dai_posn = posn->dai_posn; wallclock = posn->wallclock
// TP_printk: "device_name=%s host_posn=%#llx dai_posn=%#llx wallclock=%#llx"
pub const SOF_IPC3_PERIOD_ELAPSED_POSITION: &str =
    "device_name=%s host_posn=%#llx dai_posn=%#llx wallclock=%#llx";

// C: TRACE_EVENT(sof_pcm_pointer_position, ...)
// TP_PROTO: struct snd_sof_dev *sdev, struct snd_sof_pcm *spcm,
//           struct snd_pcm_substream *substream, snd_pcm_uframes_t dma_posn,
//           snd_pcm_uframes_t dai_posn
// TP_STRUCT__entry: string device_name = dev_name(sdev->dev); u32 pcm_id;
//                   int stream; unsigned long dma_posn, dai_posn
// TP_fast_assign: device_name = dev_name(sdev->dev);
//                 pcm_id = le32_to_cpu(spcm->pcm.pcm_id);
//                 stream = substream->stream; dma_posn = dma_posn;
//                 dai_posn = dai_posn
// TP_printk: "device_name=%s pcm_id=%d stream=%d dma_posn=%lu dai_posn=%lu"
pub const SOF_PCM_POINTER_POSITION: &str =
    "device_name=%s pcm_id=%d stream=%d dma_posn=%lu dai_posn=%lu";

// C: TRACE_EVENT(sof_stream_position_ipc_rx, TP_PROTO(struct device *dev), ...)
// TP_STRUCT__entry: string device_name = dev_name(dev)
// TP_fast_assign: device_name = dev_name(dev)
// TP_printk: "device_name=%s"
pub const SOF_STREAM_POSITION_IPC_RX: &str = "device_name=%s";

// C: TRACE_EVENT(sof_ipc4_fw_config, ...)
// TP_PROTO: struct snd_sof_dev *sdev, char *key, u32 value
// TP_STRUCT__entry: string device_name = dev_name(sdev->dev); string key; u32 value
// TP_fast_assign: device_name = dev_name(sdev->dev); key = key; value = value
// TP_printk: "device_name=%s key=%s value=%d"
pub const SOF_IPC4_FW_CONFIG: &str = "device_name=%s key=%s value=%d";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
