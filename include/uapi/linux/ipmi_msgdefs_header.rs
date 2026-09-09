/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * ipmi_smi.h
 *
 * MontaVista IPMI system management interface
 *
 * Author: MontaVista Software, Inc.
 *         Corey Minyard <minyard@mvista.com>
 *         source@mvista.com
 *
 * Copyright 2002 MontaVista Software Inc.
 *
 */

/* Various definitions for IPMI messages used by almost everything in
   the IPMI stack. */

/* NetFNs and commands used inside the IPMI stack. */

pub const IPMI_NETFN_SENSOR_EVENT_REQUEST: u32 = 0x04;
pub const IPMI_NETFN_SENSOR_EVENT_RESPONSE: u32 = 0x05;
pub const IPMI_GET_EVENT_RECEIVER_CMD: u32 = 0x01;

pub const IPMI_NETFN_APP_REQUEST: u32 = 0x06;
pub const IPMI_NETFN_APP_RESPONSE: u32 = 0x07;
pub const IPMI_GET_DEVICE_ID_CMD: u32 = 0x01;
pub const IPMI_COLD_RESET_CMD: u32 = 0x02;
pub const IPMI_WARM_RESET_CMD: u32 = 0x03;
pub const IPMI_CLEAR_MSG_FLAGS_CMD: u32 = 0x30;
pub const IPMI_GET_DEVICE_GUID_CMD: u32 = 0x08;
pub const IPMI_GET_MSG_FLAGS_CMD: u32 = 0x31;
pub const IPMI_SEND_MSG_CMD: u32 = 0x34;
pub const IPMI_GET_MSG_CMD: u32 = 0x33;
pub const IPMI_SET_BMC_GLOBAL_ENABLES_CMD: u32 = 0x2e;
pub const IPMI_GET_BMC_GLOBAL_ENABLES_CMD: u32 = 0x2f;
pub const IPMI_READ_EVENT_MSG_BUFFER_CMD: u32 = 0x35;
pub const IPMI_GET_CHANNEL_INFO_CMD: u32 = 0x42;

/* Bit for BMC global enables. */
pub const IPMI_BMC_RCV_MSG_INTR: u32 = 0x01;
pub const IPMI_BMC_EVT_MSG_INTR: u32 = 0x02;
pub const IPMI_BMC_EVT_MSG_BUFF: u32 = 0x04;
pub const IPMI_BMC_SYS_LOG: u32 = 0x08;

pub const IPMI_NETFN_STORAGE_REQUEST: u32 = 0x0a;
pub const IPMI_NETFN_STORAGE_RESPONSE: u32 = 0x0b;
pub const IPMI_ADD_SEL_ENTRY_CMD: u32 = 0x44;

pub const IPMI_NETFN_FIRMWARE_REQUEST: u32 = 0x08;
pub const IPMI_NETFN_FIRMWARE_RESPONSE: u32 = 0x09;

/* The default slave address */
pub const IPMI_BMC_SLAVE_ADDR: u32 = 0x20;

/* The BT interface on high-end HP systems supports up to 255 bytes in
 * one transfer.  Its "virtual" BMC supports some commands that are longer
 * than 128 bytes.  Use the full 256, plus NetFn/LUN, Cmd, cCode, plus
 * some overhead; it's not worth the effort to dynamically size this based
 * on the results of the "Get BT Capabilities" command. */
pub const IPMI_MAX_MSG_LENGTH: u32 = 272; /* multiple of 16 */

pub const IPMI_CC_NO_ERROR: u32 = 0x00;
pub const IPMI_NODE_BUSY_ERR: u32 = 0xc0;
pub const IPMI_INVALID_COMMAND_ERR: u32 = 0xc1;
pub const IPMI_TIMEOUT_ERR: u32 = 0xc3;
pub const IPMI_ERR_MSG_TRUNCATED: u32 = 0xc6;
pub const IPMI_REQ_LEN_INVALID_ERR: u32 = 0xc7;
pub const IPMI_REQ_LEN_EXCEEDED_ERR: u32 = 0xc8;
pub const IPMI_DEVICE_IN_FW_UPDATE_ERR: u32 = 0xd1;
pub const IPMI_DEVICE_IN_INIT_ERR: u32 = 0xd2;
pub const IPMI_NOT_IN_MY_STATE_ERR: u32 = 0xd5; /* IPMI 2.0 */
pub const IPMI_LOST_ARBITRATION_ERR: u32 = 0x81;
pub const IPMI_BUS_ERR: u32 = 0x82;
pub const IPMI_NAK_ON_WRITE_ERR: u32 = 0x83;
pub const IPMI_ERR_UNSPECIFIED: u32 = 0xff;

pub const IPMI_CHANNEL_PROTOCOL_IPMB: u32 = 1;
pub const IPMI_CHANNEL_PROTOCOL_ICMB: u32 = 2;
pub const IPMI_CHANNEL_PROTOCOL_SMBUS: u32 = 4;
pub const IPMI_CHANNEL_PROTOCOL_KCS: u32 = 5;
pub const IPMI_CHANNEL_PROTOCOL_SMIC: u32 = 6;
pub const IPMI_CHANNEL_PROTOCOL_BT10: u32 = 7;
pub const IPMI_CHANNEL_PROTOCOL_BT15: u32 = 8;
pub const IPMI_CHANNEL_PROTOCOL_TMODE: u32 = 9;

pub const IPMI_CHANNEL_MEDIUM_IPMB: u32 = 1;
pub const IPMI_CHANNEL_MEDIUM_ICMB10: u32 = 2;
pub const IPMI_CHANNEL_MEDIUM_ICMB09: u32 = 3;
pub const IPMI_CHANNEL_MEDIUM_8023LAN: u32 = 4;
pub const IPMI_CHANNEL_MEDIUM_ASYNC: u32 = 5;
pub const IPMI_CHANNEL_MEDIUM_OTHER_LAN: u32 = 6;
pub const IPMI_CHANNEL_MEDIUM_PCI_SMBUS: u32 = 7;
pub const IPMI_CHANNEL_MEDIUM_SMBUS1: u32 = 8;
pub const IPMI_CHANNEL_MEDIUM_SMBUS2: u32 = 9;
pub const IPMI_CHANNEL_MEDIUM_USB1: u32 = 10;
pub const IPMI_CHANNEL_MEDIUM_USB2: u32 = 11;
pub const IPMI_CHANNEL_MEDIUM_SYSINTF: u32 = 12;
pub const IPMI_CHANNEL_MEDIUM_OEM_MIN: u32 = 0x60;
pub const IPMI_CHANNEL_MEDIUM_OEM_MAX: u32 = 0x7f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
