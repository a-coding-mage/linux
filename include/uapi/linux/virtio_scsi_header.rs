/*
 * This header is BSD licensed so anyone can use the definitions to implement
 * compatible drivers/servers.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

// Dependency: linux/virtio_types.h

pub const VIRTIO_SCSI_CDB_DEFAULT_SIZE: usize = 32;
pub const VIRTIO_SCSI_SENSE_DEFAULT_SIZE: usize = 96;

// C preprocessor override points; use the default values when no build-time
// override supplies VIRTIO_SCSI_CDB_SIZE or VIRTIO_SCSI_SENSE_SIZE.
pub const VIRTIO_SCSI_CDB_SIZE: usize = VIRTIO_SCSI_CDB_DEFAULT_SIZE;
pub const VIRTIO_SCSI_SENSE_SIZE: usize = VIRTIO_SCSI_SENSE_DEFAULT_SIZE;

#[repr(C, packed)]
pub struct virtio_scsi_cmd_req {
    pub lun: [__u8; 8],
    pub tag: __virtio64,
    pub task_attr: __u8,
    pub prio: __u8,
    pub crn: __u8,
    pub cdb: [__u8; VIRTIO_SCSI_CDB_SIZE],
}

#[repr(C, packed)]
pub struct virtio_scsi_cmd_req_pi {
    pub lun: [__u8; 8],
    pub tag: __virtio64,
    pub task_attr: __u8,
    pub prio: __u8,
    pub crn: __u8,
    pub pi_bytesout: __virtio32,
    pub pi_bytesin: __virtio32,
    pub cdb: [__u8; VIRTIO_SCSI_CDB_SIZE],
}

#[repr(C, packed)]
pub struct virtio_scsi_cmd_resp {
    pub sense_len: __virtio32,
    pub resid: __virtio32,
    pub status_qualifier: __virtio16,
    pub status: __u8,
    pub response: __u8,
    pub sense: [__u8; VIRTIO_SCSI_SENSE_SIZE],
}

#[repr(C, packed)]
pub struct virtio_scsi_ctrl_tmf_req {
    pub r#type: __virtio32,
    pub subtype: __virtio32,
    pub lun: [__u8; 8],
    pub tag: __virtio64,
}

#[repr(C, packed)]
pub struct virtio_scsi_ctrl_tmf_resp {
    pub response: __u8,
}

#[repr(C, packed)]
pub struct virtio_scsi_ctrl_an_req {
    pub r#type: __virtio32,
    pub lun: [__u8; 8],
    pub event_requested: __virtio32,
}

#[repr(C, packed)]
pub struct virtio_scsi_ctrl_an_resp {
    pub event_actual: __virtio32,
    pub response: __u8,
}

#[repr(C, packed)]
pub struct virtio_scsi_event {
    pub event: __virtio32,
    pub lun: [__u8; 8],
    pub reason: __virtio32,
}

#[repr(C, packed)]
pub struct virtio_scsi_config {
    pub num_queues: __virtio32,
    pub seg_max: __virtio32,
    pub max_sectors: __virtio32,
    pub cmd_per_lun: __virtio32,
    pub event_info_size: __virtio32,
    pub sense_size: __virtio32,
    pub cdb_size: __virtio32,
    pub max_channel: __virtio16,
    pub max_target: __virtio16,
    pub max_lun: __virtio32,
}

pub const VIRTIO_SCSI_F_INOUT: u32 = 0;
pub const VIRTIO_SCSI_F_HOTPLUG: u32 = 1;
pub const VIRTIO_SCSI_F_CHANGE: u32 = 2;
pub const VIRTIO_SCSI_F_T10_PI: u32 = 3;

pub const VIRTIO_SCSI_S_OK: u32 = 0;
pub const VIRTIO_SCSI_S_OVERRUN: u32 = 1;
pub const VIRTIO_SCSI_S_ABORTED: u32 = 2;
pub const VIRTIO_SCSI_S_BAD_TARGET: u32 = 3;
pub const VIRTIO_SCSI_S_RESET: u32 = 4;
pub const VIRTIO_SCSI_S_BUSY: u32 = 5;
pub const VIRTIO_SCSI_S_TRANSPORT_FAILURE: u32 = 6;
pub const VIRTIO_SCSI_S_TARGET_FAILURE: u32 = 7;
pub const VIRTIO_SCSI_S_NEXUS_FAILURE: u32 = 8;
pub const VIRTIO_SCSI_S_FAILURE: u32 = 9;
pub const VIRTIO_SCSI_S_FUNCTION_SUCCEEDED: u32 = 10;
pub const VIRTIO_SCSI_S_FUNCTION_REJECTED: u32 = 11;
pub const VIRTIO_SCSI_S_INCORRECT_LUN: u32 = 12;

pub const VIRTIO_SCSI_T_TMF: u32 = 0;
pub const VIRTIO_SCSI_T_AN_QUERY: u32 = 1;
pub const VIRTIO_SCSI_T_AN_SUBSCRIBE: u32 = 2;

pub const VIRTIO_SCSI_T_TMF_ABORT_TASK: u32 = 0;
pub const VIRTIO_SCSI_T_TMF_ABORT_TASK_SET: u32 = 1;
pub const VIRTIO_SCSI_T_TMF_CLEAR_ACA: u32 = 2;
pub const VIRTIO_SCSI_T_TMF_CLEAR_TASK_SET: u32 = 3;
pub const VIRTIO_SCSI_T_TMF_I_T_NEXUS_RESET: u32 = 4;
pub const VIRTIO_SCSI_T_TMF_LOGICAL_UNIT_RESET: u32 = 5;
pub const VIRTIO_SCSI_T_TMF_QUERY_TASK: u32 = 6;
pub const VIRTIO_SCSI_T_TMF_QUERY_TASK_SET: u32 = 7;

pub const VIRTIO_SCSI_T_EVENTS_MISSED: u32 = 0x8000_0000;
pub const VIRTIO_SCSI_T_NO_EVENT: u32 = 0;
pub const VIRTIO_SCSI_T_TRANSPORT_RESET: u32 = 1;
pub const VIRTIO_SCSI_T_ASYNC_NOTIFY: u32 = 2;
pub const VIRTIO_SCSI_T_PARAM_CHANGE: u32 = 3;

pub const VIRTIO_SCSI_EVT_RESET_HARD: u32 = 0;
pub const VIRTIO_SCSI_EVT_RESET_RESCAN: u32 = 1;
pub const VIRTIO_SCSI_EVT_RESET_REMOVED: u32 = 2;

pub const VIRTIO_SCSI_S_SIMPLE: u32 = 0;
pub const VIRTIO_SCSI_S_ORDERED: u32 = 1;
pub const VIRTIO_SCSI_S_HEAD: u32 = 2;
pub const VIRTIO_SCSI_S_ACA: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
