/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * Copyright(c) 2003-2015 Intel Corporation. All rights reserved.
 * Intel Management Engine Interface (Intel MEI) Linux driver
 * Intel MEI Interface Header
 */

// Dependency supplied by linux/mei_uuid.h.
use crate::uuid_le;

/*
 * This IOCTL is used to associate the current file descriptor with a
 * FW Client (given by UUID). This opens a communication channel between
 * a host client and a FW client. From this point every read and write
 * will communicate with the associated FW client.
 * Only in close() (file_operation release()) is the communication between
 * the clients disconnected.
 *
 * The IOCTL argument is a struct with a union that contains
 * the input parameter and the output parameter for this IOCTL.
 *
 * The input parameter is UUID of the FW Client.
 * The output parameter is the properties of the FW client
 * (FW protocol version and max message size).
 */

#[repr(C)]
pub struct mei_client {
    pub max_msg_length: u32,
    pub protocol_version: u8,
    pub reserved: [u8; 3],
}

#[repr(C)]
pub union mei_connect_client_data_union {
    pub in_client_uuid: uuid_le,
    pub out_client_properties: mei_client,
}

#[repr(C)]
pub struct mei_connect_client_data {
    pub data: mei_connect_client_data_union,
}

/* Linux _IOC encoding used by the ioctl constants below. */
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn mei_ioc(dir: u32, nr: u32, size: usize) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (('H' as u32) << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | ((size as u32) << IOC_SIZESHIFT)
}

pub const IOCTL_MEI_CONNECT_CLIENT: u32 =
    mei_ioc(IOC_READ | IOC_WRITE, 0x01, core::mem::size_of::<mei_connect_client_data>());

/**
 * DOC: set and unset event notification for a connected client
 *
 * The IOCTL argument is 1 for enabling event notification and 0 for
 * disabling the service.
 * Return:  -EOPNOTSUPP if the devices doesn't support the feature
 */
pub const IOCTL_MEI_NOTIFY_SET: u32 = mei_ioc(IOC_WRITE, 0x02, core::mem::size_of::<u32>());

/**
 * DOC: retrieve notification
 *
 * The IOCTL output argument is 1 if an event was pending and 0 otherwise.
 * The ioctl has to be called in order to acknowledge pending event.
 *
 * Return:  -EOPNOTSUPP if the devices doesn't support the feature
 */
pub const IOCTL_MEI_NOTIFY_GET: u32 = mei_ioc(IOC_READ, 0x03, core::mem::size_of::<u32>());

/**
 * struct mei_connect_client_vtag - mei client information struct with vtag
 *
 * @in_client_uuid: UUID of client to connect
 * @vtag: virtual tag
 * @reserved: reserved for future use
 */
#[repr(C)]
pub struct mei_connect_client_vtag {
    pub in_client_uuid: uuid_le,
    pub vtag: u8,
    pub reserved: [u8; 3],
}

/**
 * struct mei_connect_client_data_vtag - IOCTL connect data union
 *
 * @connect: input connect data
 * @out_client_properties: output client data
 */
#[repr(C)]
pub union mei_connect_client_data_vtag_union {
    pub connect: mei_connect_client_vtag,
    pub out_client_properties: mei_client,
}

#[repr(C)]
pub struct mei_connect_client_data_vtag {
    pub data: mei_connect_client_data_vtag_union,
}

/**
 * DOC:
 * This IOCTL is used to associate the current file descriptor with a
 * FW Client (given by UUID), and virtual tag (vtag).
 * The IOCTL opens a communication channel between a host client and
 * a FW client on a tagged channel. From this point on, every read
 * and write will communicate with the associated FW client
 * on the tagged channel.
 * Upon close() the communication is terminated.
 *
 * The IOCTL argument is a struct with a union that contains
 * the input parameter and the output parameter for this IOCTL.
 *
 * The input parameter is UUID of the FW Client, a vtag [0,255].
 * The output parameter is the properties of the FW client
 * (FW protocol version and max message size).
 *
 * Clients that do not support tagged connection
 * will respond with -EOPNOTSUPP.
 */
pub const IOCTL_MEI_CONNECT_CLIENT_VTAG: u32 = mei_ioc(
    IOC_READ | IOC_WRITE,
    0x04,
    core::mem::size_of::<mei_connect_client_data_vtag>(),
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
