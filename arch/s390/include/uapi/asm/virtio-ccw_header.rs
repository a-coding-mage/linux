/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * Definitions for virtio-ccw devices.
 *
 * Copyright IBM Corp. 2013
 *
 *  Author(s): Cornelia Huck <cornelia.huck@de.ibm.com>
 */

/* Alignment of vring buffers. */
pub const KVM_VIRTIO_CCW_RING_ALIGN: u32 = 4096;

/* Subcode for diagnose 500 (virtio hypercall). */
pub const KVM_S390_VIRTIO_CCW_NOTIFY: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
