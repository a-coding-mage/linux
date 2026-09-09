/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * Copyright (C) 2020-2021 OpenSynergy GmbH
 * Copyright (C) 2021 ARM Ltd.
 */

// Dependency intent from the original header: <linux/virtio_types.h>

/* Device implements some SCMI notifications, or delayed responses. */
pub const VIRTIO_SCMI_F_P2A_CHANNELS: u32 = 0;

/* Device implements any SCMI statistics shared memory region */
pub const VIRTIO_SCMI_F_SHARED_MEMORY: u32 = 1;

/* Virtqueues */

pub const VIRTIO_SCMI_VQ_TX: u32 = 0; /* cmdq */
pub const VIRTIO_SCMI_VQ_RX: u32 = 1; /* eventq */
pub const VIRTIO_SCMI_VQ_MAX_CNT: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
