/* SPDX-License-Identifier: BSD-3-Clause */
/*
 * Virtio Mem Device
 *
 * Copyright Red Hat, Inc. 2020
 *
 * Authors:
 *     David Hildenbrand <david@redhat.com>
 *
 * This header is BSD licensed so anyone can use the definitions
 * to implement compatible drivers/servers:
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of IBM nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
 * USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT
 * OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

// Dependencies supplied by the corresponding Linux type and virtio headers.

/* --- virtio-mem: feature bits --- */
pub const VIRTIO_MEM_F_ACPI_PXM: u32 = 0;
pub const VIRTIO_MEM_F_UNPLUGGED_INACCESSIBLE: u32 = 1;
pub const VIRTIO_MEM_F_PERSISTENT_SUSPEND: u32 = 2;

/* --- virtio-mem: guest -> host requests --- */
pub const VIRTIO_MEM_REQ_PLUG: u32 = 0;
pub const VIRTIO_MEM_REQ_UNPLUG: u32 = 1;
pub const VIRTIO_MEM_REQ_UNPLUG_ALL: u32 = 2;
pub const VIRTIO_MEM_REQ_STATE: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_mem_req_plug {
    pub addr: __virtio64,
    pub nb_blocks: __virtio16,
    pub padding: [__virtio16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_mem_req_unplug {
    pub addr: __virtio64,
    pub nb_blocks: __virtio16,
    pub padding: [__virtio16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_mem_req_state {
    pub addr: __virtio64,
    pub nb_blocks: __virtio16,
    pub padding: [__virtio16; 3],
}

#[repr(C)]
pub union virtio_mem_req_u {
    pub plug: virtio_mem_req_plug,
    pub unplug: virtio_mem_req_unplug,
    pub state: virtio_mem_req_state,
}

#[repr(C)]
pub struct virtio_mem_req {
    pub type_: __virtio16,
    pub padding: [__virtio16; 3],
    pub u: virtio_mem_req_u,
}

/* --- virtio-mem: host -> guest response --- */
pub const VIRTIO_MEM_RESP_ACK: u32 = 0;
pub const VIRTIO_MEM_RESP_NACK: u32 = 1;
pub const VIRTIO_MEM_RESP_BUSY: u32 = 2;
pub const VIRTIO_MEM_RESP_ERROR: u32 = 3;

pub const VIRTIO_MEM_STATE_PLUGGED: u32 = 0;
pub const VIRTIO_MEM_STATE_UNPLUGGED: u32 = 1;
pub const VIRTIO_MEM_STATE_MIXED: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_mem_resp_state {
    pub state: __virtio16,
}

#[repr(C)]
pub union virtio_mem_resp_u {
    pub state: virtio_mem_resp_state,
}

#[repr(C)]
pub struct virtio_mem_resp {
    pub type_: __virtio16,
    pub padding: [__virtio16; 3],
    pub u: virtio_mem_resp_u,
}

/* --- virtio-mem: configuration --- */
#[repr(C)]
pub struct virtio_mem_config {
    /* Block size and alignment. Cannot change. */
    pub block_size: __le64,
    /* Valid with VIRTIO_MEM_F_ACPI_PXM. Cannot change. */
    pub node_id: __le16,
    pub padding: [__u8; 6],
    /* Start address of the memory region. Cannot change. */
    pub addr: __le64,
    /* Region size (maximum). Cannot change. */
    pub region_size: __le64,
    /* Currently usable region size. Can grow up to region_size. Can
     * shrink due to VIRTIO_MEM_REQ_UNPLUG_ALL (in which case no config
     * update will be sent). */
    pub usable_region_size: __le64,
    /* Currently used size. Changes due to plug/unplug requests, but no
     * config updates will be sent. */
    pub plugged_size: __le64,
    /* Requested size. New plug requests cannot exceed it. Can change. */
    pub requested_size: __le64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
