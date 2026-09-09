/* This header, excluding the #ifdef __KERNEL__ part, is BSD licensed so
 * anyone can use the definitions to implement compatible drivers/servers.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of IBM nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL IBM OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE. */

/* Virtio devices use a standardized configuration space to define their
 * features and pass configuration information, but each implementation can
 * store and access that space differently. */

/* Status byte for guest to report progress, and synchronize features. */
/* We have seen device and processed generic fields (VIRTIO_CONFIG_F_VIRTIO) */
pub const VIRTIO_CONFIG_S_ACKNOWLEDGE: u32 = 1;
/* We have found a driver for the device. */
pub const VIRTIO_CONFIG_S_DRIVER: u32 = 2;
/* Driver has used its parts of the config, and is happy */
pub const VIRTIO_CONFIG_S_DRIVER_OK: u32 = 4;
/* Driver has finished configuring features */
pub const VIRTIO_CONFIG_S_FEATURES_OK: u32 = 8;
/* Device entered invalid state, driver must reset it */
pub const VIRTIO_CONFIG_S_NEEDS_RESET: u32 = 0x40;
/* We've given up on this device. */
pub const VIRTIO_CONFIG_S_FAILED: u32 = 0x80;

/*
 * Virtio feature bits VIRTIO_TRANSPORT_F_START through
 * VIRTIO_TRANSPORT_F_END are reserved for the transport
 * being used (e.g. virtio_ring, virtio_pci etc.), the
 * rest are per-device feature bits.
 */
pub const VIRTIO_TRANSPORT_F_START: u32 = 28;
pub const VIRTIO_TRANSPORT_F_END: u32 = 42;

/* C build condition: these legacy definitions are omitted when
 * VIRTIO_CONFIG_NO_LEGACY is defined. */
#[cfg(not(feature = "VIRTIO_CONFIG_NO_LEGACY"))]
pub const VIRTIO_F_NOTIFY_ON_EMPTY: u32 = 24;
#[cfg(not(feature = "VIRTIO_CONFIG_NO_LEGACY"))]
pub const VIRTIO_F_ANY_LAYOUT: u32 = 27;

/* v1.0 compliant. */
pub const VIRTIO_F_VERSION_1: u32 = 32;

/*
 * If clear - device has the platform DMA (e.g. IOMMU) bypass quirk feature.
 * If set - use platform DMA tools to access the memory.
 *
 * Note the reverse polarity (compared to most other features),
 * this is for compatibility with legacy systems.
 */
pub const VIRTIO_F_ACCESS_PLATFORM: u32 = 33;
/* C build condition: the legacy userspace alias is omitted for __KERNEL__. */
#[cfg(not(feature = "__KERNEL__"))]
pub const VIRTIO_F_IOMMU_PLATFORM: u32 = VIRTIO_F_ACCESS_PLATFORM;

/* This feature indicates support for the packed virtqueue layout. */
pub const VIRTIO_F_RING_PACKED: u32 = 34;

/*
 * Inorder feature indicates that all buffers are used by the device
 * in the same order in which they have been made available.
 */
pub const VIRTIO_F_IN_ORDER: u32 = 35;

/*
 * This feature indicates that memory accesses by the driver and the
 * device are ordered in a way described by the platform.
 */
pub const VIRTIO_F_ORDER_PLATFORM: u32 = 36;

/* Does the device support Single Root I/O Virtualization? */
pub const VIRTIO_F_SR_IOV: u32 = 37;

/* This feature indicates that the driver passes extra data (besides
 * identifying the virtqueue) in its device notifications.
 */
pub const VIRTIO_F_NOTIFICATION_DATA: u32 = 38;

/* This feature indicates that the driver uses the data provided by the device
 * as a virtqueue identifier in available buffer notifications.
 */
pub const VIRTIO_F_NOTIF_CONFIG_DATA: u32 = 39;

/* This feature indicates that the driver can reset a queue individually. */
pub const VIRTIO_F_RING_RESET: u32 = 40;

/* This feature indicates that the device support administration virtqueues. */
pub const VIRTIO_F_ADMIN_VQ: u32 = 41;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
