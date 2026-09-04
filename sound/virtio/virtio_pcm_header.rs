//! SPDX-License-Identifier: GPL-2.0+
//!
//! virtio-snd: Virtio sound device
//! Copyright (C) 2021 OpenSynergy GmbH

// Translation of virtio_pcm.h header
//
// External type dependencies from:
// - <linux/atomic.h>: Linux kernel atomic types
// - <linux/virtio_config.h>: VirtIO device configuration types
// - <sound/pcm.h>: ALSA PCM types
// - <sound/pcm-indirect.h>: ALSA indirect PCM types

use std::ffi::{c_int, c_uint, c_ulong};

// Forward declarations of opaque external types

/// VirtIO sound device (from external module)
pub struct virtio_snd;

/// VirtIO PCM message (from external module)
pub struct virtio_pcm_msg;

/// ALSA PCM substream (from <sound/pcm.h>)
pub struct snd_pcm_substream;

/// ALSA indirect PCM structure (from <sound/pcm-indirect.h>)
pub struct snd_pcm_indirect;

/// ALSA PCM hardware descriptor (from <sound/pcm.h>)
pub struct snd_pcm_hardware;

/// Linux kernel work queue structure (from <linux/workqueue.h>)
pub struct work_struct;

/// Linux kernel spinlock (from <linux/spinlock.h>)
pub struct spinlock_t;

/// Linux kernel list head (from <linux/list.h>)
pub struct list_head;

/// ALSA PCM device (from <sound/pcm.h>)
pub struct snd_pcm;

/// ALSA PCM channel map element (from <sound/pcm.h>)
pub struct snd_pcm_chmap_elem;

/// VirtIO device (from <linux/virtio.h>)
pub struct virtio_device;

/// VirtIO sound event (from external module)
pub struct virtio_snd_event;

/// VirtIO virtual queue (from <linux/virtio.h>)
pub struct virtqueue;

/// ALSA PCM operations (from <sound/pcm.h>)
#[repr(C)]
pub struct snd_pcm_ops;

/// Linux kernel wait queue head type (from <linux/wait.h>)
pub type wait_queue_head_t = c_uint;

/// Linux kernel GFP allocation flags type (from <linux/gfp.h>)
pub type gfp_t = c_uint;

/// VirtIO PCM substream.
///
/// # Fields
///
/// * `snd` - VirtIO sound device
/// * `nid` - Function group node identifier
/// * `sid` - Stream identifier
/// * `direction` - Stream data flow direction (SNDRV_PCM_STREAM_XXX)
/// * `features` - Stream VirtIO feature bit map (1 << VIRTIO_SND_PCM_F_XXX)
/// * `substream` - Kernel ALSA substream
/// * `pcm_indirect` - Kernel indirect pcm structure
/// * `hw` - Kernel ALSA substream hardware descriptor
/// * `elapsed_period` - Kernel work to handle the elapsed period state
/// * `lock` - Spinlock that protects fields shared by interrupt handlers and
///   substream operators
/// * `buffer_bytes` - Current buffer size in bytes
/// * `hw_ptr` - Substream hardware pointer value in bytes [0 ... buffer_bytes)
/// * `xfer_enabled` - Data transfer state (0 - off, 1 - on)
/// * `xfer_xrun` - Data underflow/overflow state (0 - no xrun, 1 - xrun)
/// * `stopped` - True if the substream is stopped and must be released on the
///   device side
/// * `suspended` - True if the substream is suspended and must be reconfigured
///   on the device side at resume
/// * `msgs` - Allocated I/O messages
/// * `nmsgs` - Number of allocated I/O messages
/// * `msg_count` - Number of pending I/O messages in the virtqueue
/// * `msg_empty` - Notify when msg_count is zero
#[repr(C)]
pub struct virtio_pcm_substream {
    pub snd: *mut virtio_snd,
    pub nid: u32,
    pub sid: u32,
    pub direction: u32,
    pub features: u32,
    pub substream: *mut snd_pcm_substream,
    pub pcm_indirect: snd_pcm_indirect,
    pub hw: snd_pcm_hardware,
    pub elapsed_period: work_struct,
    pub lock: spinlock_t,
    pub buffer_bytes: usize,
    pub hw_ptr: usize,
    pub xfer_enabled: bool,
    pub xfer_xrun: bool,
    pub stopped: bool,
    pub suspended: bool,
    pub msgs: *mut *mut virtio_pcm_msg,
    pub nmsgs: c_uint,
    pub msg_count: c_uint,
    pub msg_empty: wait_queue_head_t,
}

/// VirtIO PCM stream.
///
/// # Fields
///
/// * `substreams` - VirtIO substreams belonging to the stream
/// * `nsubstreams` - Number of substreams
/// * `chmaps` - Kernel channel maps belonging to the stream
/// * `nchmaps` - Number of channel maps
#[repr(C)]
pub struct virtio_pcm_stream {
    pub substreams: *mut *mut virtio_pcm_substream,
    pub nsubstreams: u32,
    pub chmaps: *mut snd_pcm_chmap_elem,
    pub nchmaps: u32,
}

/// VirtIO PCM device.
///
/// # Fields
///
/// * `list` - VirtIO PCM list entry
/// * `nid` - Function group node identifier
/// * `pcm` - Kernel PCM device
/// * `streams` - VirtIO PCM streams (playback and capture)
///
/// Note: SNDRV_PCM_STREAM_LAST is defined in kernel headers and represents the
/// maximum PCM stream type index. The streams array has SNDRV_PCM_STREAM_LAST + 1 elements.
#[repr(C)]
pub struct virtio_pcm {
    pub list: list_head,
    pub nid: u32,
    pub pcm: *mut snd_pcm,
    // TODO: SNDRV_PCM_STREAM_LAST + 1 is a kernel compile-time constant
    // Temporarily using 2 as placeholder; actual constant should be imported from bindings
    pub streams: [virtio_pcm_stream; 2],
}

extern "C" {
    /// VirtIO PCM operations structure
    pub static virtsnd_pcm_ops: snd_pcm_ops;

    /// Validate VirtIO PCM device configuration
    pub fn virtsnd_pcm_validate(vdev: *mut virtio_device) -> c_int;

    /// Parse VirtIO PCM device configuration
    pub fn virtsnd_pcm_parse_cfg(snd: *mut virtio_snd) -> c_int;

    /// Build and initialize VirtIO PCM devices
    pub fn virtsnd_pcm_build_devs(snd: *mut virtio_snd) -> c_int;

    /// Handle VirtIO PCM device event
    pub fn virtsnd_pcm_event(snd: *mut virtio_snd, event: *mut virtio_snd_event);

    /// TX virtqueue notification callback
    pub fn virtsnd_pcm_tx_notify_cb(vqueue: *mut virtqueue);

    /// RX virtqueue notification callback
    pub fn virtsnd_pcm_rx_notify_cb(vqueue: *mut virtqueue);

    /// Find VirtIO PCM device by node identifier
    pub fn virtsnd_pcm_find(snd: *mut virtio_snd, nid: u32) -> *mut virtio_pcm;

    /// Find or create VirtIO PCM device by node identifier
    pub fn virtsnd_pcm_find_or_create(snd: *mut virtio_snd, nid: u32) -> *mut virtio_pcm;

    /// Allocate VirtIO PCM control message
    pub fn virtsnd_pcm_ctl_msg_alloc(
        vss: *mut virtio_pcm_substream,
        command: c_uint,
        gfp: gfp_t,
    ) -> *mut virtio_snd_msg;

    /// Allocate VirtIO PCM I/O messages
    pub fn virtsnd_pcm_msg_alloc(
        vss: *mut virtio_pcm_substream,
        periods: c_uint,
        period_bytes: c_uint,
    ) -> c_int;

    /// Free VirtIO PCM I/O messages
    pub fn virtsnd_pcm_msg_free(vss: *mut virtio_pcm_substream);

    /// Send VirtIO PCM I/O message
    pub fn virtsnd_pcm_msg_send(
        vss: *mut virtio_pcm_substream,
        offset: c_ulong,
        bytes: c_ulong,
    ) -> c_int;

    /// Get number of pending VirtIO PCM I/O messages
    pub fn virtsnd_pcm_msg_pending_num(vss: *mut virtio_pcm_substream) -> c_uint;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
