/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: scsi_lun and SCSI_SENSE_BUFFERSIZE are supplied by the
// corresponding SCSI headers.

/* Common header for all IUs */
#[repr(C, packed)]
pub struct iu {
	pub iu_id: __u8,
	pub rsvd1: __u8,
	pub tag: __be16,
}

pub const IU_ID_COMMAND: i32 = 0x01;
pub const IU_ID_STATUS: i32 = 0x03;
pub const IU_ID_RESPONSE: i32 = 0x04;
pub const IU_ID_TASK_MGMT: i32 = 0x05;
pub const IU_ID_READ_READY: i32 = 0x06;
pub const IU_ID_WRITE_READY: i32 = 0x07;

pub const TMF_ABORT_TASK: i32 = 0x01;
pub const TMF_ABORT_TASK_SET: i32 = 0x02;
pub const TMF_CLEAR_TASK_SET: i32 = 0x04;
pub const TMF_LOGICAL_UNIT_RESET: i32 = 0x08;
pub const TMF_I_T_NEXUS_RESET: i32 = 0x10;
pub const TMF_CLEAR_ACA: i32 = 0x40;
pub const TMF_QUERY_TASK: i32 = 0x80;
pub const TMF_QUERY_TASK_SET: i32 = 0x81;
pub const TMF_QUERY_ASYNC_EVENT: i32 = 0x82;

pub const RC_TMF_COMPLETE: i32 = 0x00;
pub const RC_INVALID_INFO_UNIT: i32 = 0x02;
pub const RC_TMF_NOT_SUPPORTED: i32 = 0x04;
pub const RC_TMF_FAILED: i32 = 0x05;
pub const RC_TMF_SUCCEEDED: i32 = 0x08;
pub const RC_INCORRECT_LUN: i32 = 0x09;
pub const RC_OVERLAPPED_TAG: i32 = 0x0a;

#[repr(C, packed)]
pub struct command_iu {
	pub iu_id: __u8,
	pub rsvd1: __u8,
	pub tag: __be16,
	pub prio_attr: __u8,
	pub rsvd5: __u8,
	pub len: __u8,
	pub rsvd7: __u8,
	pub lun: scsi_lun,
	// XXX: Overflow-checking tools may misunderstand
	pub cdb: [__u8; 16],
}

#[repr(C, packed)]
pub struct task_mgmt_iu {
	pub iu_id: __u8,
	pub rsvd1: __u8,
	pub tag: __be16,
	pub function: __u8,
	pub rsvd2: __u8,
	pub task_tag: __be16,
	pub lun: scsi_lun,
}

/*
 * Also used for the Read Ready and Write Ready IUs since they have the
 * same first four bytes
 */
#[repr(C, packed)]
pub struct sense_iu {
	pub iu_id: __u8,
	pub rsvd1: __u8,
	pub tag: __be16,
	pub status_qual: __be16,
	pub status: __u8,
	pub rsvd7: [__u8; 7],
	pub len: __be16,
	pub sense: [__u8; SCSI_SENSE_BUFFERSIZE],
}

#[repr(C, packed)]
pub struct response_iu {
	pub iu_id: __u8,
	pub rsvd1: __u8,
	pub tag: __be16,
	pub add_response_info: [__u8; 3],
	pub response_code: __u8,
}

#[repr(C, packed)]
pub struct usb_pipe_usage_descriptor {
	pub bLength: __u8,
	pub bDescriptorType: __u8,
	pub bPipeID: __u8,
	pub Reserved: __u8,
}

pub const CMD_PIPE_ID: i32 = 1;
pub const STATUS_PIPE_ID: i32 = 2;
pub const DATA_IN_PIPE_ID: i32 = 3;
pub const DATA_OUT_PIPE_ID: i32 = 4;

pub const UAS_SIMPLE_TAG: i32 = 0;
pub const UAS_HEAD_TAG: i32 = 1;
pub const UAS_ORDERED_TAG: i32 = 2;
pub const UAS_ACA: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
