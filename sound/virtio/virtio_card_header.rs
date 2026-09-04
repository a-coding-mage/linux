// SPDX-License-Identifier: GPL-2.0+
//
// virtio-snd: Virtio sound device
// Copyright (C) 2021 OpenSynergy GmbH

// External C dependencies (from included headers):
// - linux/slab.h (kernel memory allocation)
// - linux/virtio.h (virtio core types and functions)
// - sound/core.h (ALSA core types)
// - uapi/linux/virtio_snd.h (virtio sound device constants and structures)
// - virtio_ctl_msg.h (control message types)
// - virtio_pcm.h (PCM stream types)

pub const VIRTIO_SND_CARD_DRIVER: &str = "virtio-snd";
pub const VIRTIO_SND_CARD_NAME: &str = "VirtIO SoundCard";
pub const VIRTIO_SND_PCM_NAME: &str = "VirtIO PCM";

// Forward declarations for types defined in other modules/files
pub struct virtio_jack;
pub struct virtio_pcm_substream;

// Virtqueue wrapper structure.
// Synchronizes access to an underlying virtqueue.
#[repr(C)]
pub struct virtio_snd_queue {
    pub lock: spinlock_t,
    pub vqueue: *mut virtqueue,
}

// VirtIO control element.
// Wraps an ALSA control element with enumeration items.
#[repr(C)]
pub struct virtio_kctl {
    pub kctl: *mut snd_kcontrol,
    pub items: *mut virtio_snd_ctl_enum_item,
}

// VirtIO sound card device.
// Main device structure containing all state for a virtio sound device.
#[repr(C)]
pub struct virtio_snd {
    pub vdev: *mut virtio_device,
    pub queues: [virtio_snd_queue; VIRTIO_SND_VQ_MAX as usize],
    pub card: *mut snd_card,
    pub ctl_msgs: list_head,
    pub event_msgs: *mut virtio_snd_event,
    pub pcm_list: list_head,
    pub jacks: *mut virtio_jack,
    pub njacks: u32,
    pub substreams: *mut virtio_pcm_substream,
    pub nsubstreams: u32,
    pub chmaps: *mut virtio_snd_chmap_info,
    pub nchmaps: u32,
    pub kctl_infos: *mut virtio_snd_ctl_info,
    pub kctls: *mut virtio_kctl,
    pub nkctls: u32,
}

// Message completion timeout in milliseconds (module parameter).
extern "C" {
    pub static mut virtsnd_msg_timeout_ms: u32;
}

// Returns a mutable pointer to the control queue.
#[inline]
pub unsafe fn virtsnd_control_queue(snd: *mut virtio_snd) -> *mut virtio_snd_queue {
    core::ptr::addr_of_mut!((*snd).queues[VIRTIO_SND_VQ_CONTROL as usize])
}

// Returns a mutable pointer to the event queue.
#[inline]
pub unsafe fn virtsnd_event_queue(snd: *mut virtio_snd) -> *mut virtio_snd_queue {
    core::ptr::addr_of_mut!((*snd).queues[VIRTIO_SND_VQ_EVENT as usize])
}

// Returns a mutable pointer to the transmit queue.
#[inline]
pub unsafe fn virtsnd_tx_queue(snd: *mut virtio_snd) -> *mut virtio_snd_queue {
    core::ptr::addr_of_mut!((*snd).queues[VIRTIO_SND_VQ_TX as usize])
}

// Returns a mutable pointer to the receive queue.
#[inline]
pub unsafe fn virtsnd_rx_queue(snd: *mut virtio_snd) -> *mut virtio_snd_queue {
    core::ptr::addr_of_mut!((*snd).queues[VIRTIO_SND_VQ_RX as usize])
}

// Returns the PCM queue for the given PCM substream.
// For playback streams, returns the transmit queue; for capture streams, returns the receive queue.
#[inline]
pub unsafe fn virtsnd_pcm_queue(vss: *mut virtio_pcm_substream) -> *mut virtio_snd_queue {
    if (*vss).direction == SNDRV_PCM_STREAM_PLAYBACK {
        virtsnd_tx_queue((*vss).snd)
    } else {
        virtsnd_rx_queue((*vss).snd)
    }
}

extern "C" {
    pub fn virtsnd_jack_parse_cfg(snd: *mut virtio_snd) -> i32;

    pub fn virtsnd_jack_build_devs(snd: *mut virtio_snd) -> i32;

    pub fn virtsnd_jack_event(snd: *mut virtio_snd, event: *mut virtio_snd_event);

    pub fn virtsnd_chmap_parse_cfg(snd: *mut virtio_snd) -> i32;

    pub fn virtsnd_chmap_build_devs(snd: *mut virtio_snd) -> i32;

    pub fn virtsnd_kctl_parse_cfg(snd: *mut virtio_snd) -> i32;

    pub fn virtsnd_kctl_build_devs(snd: *mut virtio_snd) -> i32;

    pub fn virtsnd_kctl_event(snd: *mut virtio_snd, event: *mut virtio_snd_event);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
