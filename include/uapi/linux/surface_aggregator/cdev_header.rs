/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Surface System Aggregator Module (SSAM) user-space EC interface.
 *
 * Definitions, structs, and IOCTLs for the /dev/surface/aggregator misc
 * device. This device provides direct user-space access to the SSAM EC.
 * Intended for debugging and development.
 *
 * Copyright (C) 2020-2021 Maximilian Luz <luzmaximilian@gmail.com>
 */

/**
 * enum ssam_cdev_request_flags - Request flags for SSAM cdev request IOCTL.
 *
 * @SSAM_CDEV_REQUEST_HAS_RESPONSE:
 *	Specifies that the request expects a response. If not set, the request
 *	will be directly completed after its underlying packet has been
 *	transmitted. If set, the request transport system waits for a response
 *	of the request.
 *
 * @SSAM_CDEV_REQUEST_UNSEQUENCED:
 *	Specifies that the request should be transmitted via an unsequenced
 *	packet. If set, the request must not have a response, meaning that this
 *	flag and the %SSAM_CDEV_REQUEST_HAS_RESPONSE flag are mutually
 *	exclusive.
 */
#[repr(i32)]
pub enum SsamCdevRequestFlags {
    SSAM_CDEV_REQUEST_HAS_RESPONSE = 0x01,
    SSAM_CDEV_REQUEST_UNSEQUENCED = 0x02,
}

/**
 * struct ssam_cdev_request - Controller request IOCTL argument.
 * @target_category: Target category of the SAM request.
 * @target_id:       Target ID of the SAM request.
 * @command_id:      Command ID of the SAM request.
 * @instance_id:     Instance ID of the SAM request.
 * @flags:           Request flags (see &enum ssam_cdev_request_flags).
 * @status:          Request status (output).
 * @payload:         Request payload (input data).
 * @payload.data:    Pointer to request payload data.
 * @payload.length:  Length of request payload data (in bytes).
 * @response:        Request response (output data).
 * @response.data:   Pointer to response buffer.
 * @response.length: On input: Capacity of response buffer (in bytes).
 *                   On output: Length of request response (number of bytes
 *                   in the buffer that are actually used).
 */
#[repr(C, packed)]
pub struct SsamCdevRequest {
    pub target_category: u8,
    pub target_id: u8,
    pub command_id: u8,
    pub instance_id: u8,
    pub flags: u16,
    pub status: i16,
    pub payload: SsamCdevBuffer,
    pub response: SsamCdevBuffer,
}

#[repr(C, packed)]
pub struct SsamCdevBuffer {
    pub data: u64,
    pub length: u16,
    pub __pad: [u8; 6],
}

/** struct ssam_cdev_notifier_desc - Notifier descriptor. */
#[repr(C, packed)]
pub struct SsamCdevNotifierDesc {
    pub priority: i32,
    pub target_category: u8,
}

/** struct ssam_cdev_event_desc - Event descriptor. */
#[repr(C, packed)]
pub struct SsamCdevEventDesc {
    pub reg: SsamCdevEventReg,
    pub id: SsamCdevEventId,
    pub flags: u8,
}

#[repr(C, packed)]
pub struct SsamCdevEventReg {
    pub target_category: u8,
    pub target_id: u8,
    pub cid_enable: u8,
    pub cid_disable: u8,
}

#[repr(C, packed)]
pub struct SsamCdevEventId {
    pub target_category: u8,
    pub instance: u8,
}

/** struct ssam_cdev_event - SSAM event sent by the EC. */
#[repr(C, packed)]
pub struct SsamCdevEvent {
    pub target_category: u8,
    pub target_id: u8,
    pub command_id: u8,
    pub instance_id: u8,
    pub length: u16,
    pub data: [u8; 0],
}

// Linux ioctl encoding: _IOWR/_IOW(type, number, packed structure size).
pub const SSAM_CDEV_REQUEST: u32 = 0xC026_A501;
pub const SSAM_CDEV_NOTIF_REGISTER: u32 = 0x4005_A502;
pub const SSAM_CDEV_NOTIF_UNREGISTER: u32 = 0x4005_A503;
pub const SSAM_CDEV_EVENT_ENABLE: u32 = 0x4007_A504;
pub const SSAM_CDEV_EVENT_DISABLE: u32 = 0x4007_A505;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
