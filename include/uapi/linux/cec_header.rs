/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * cec - HDMI Consumer Electronics Control public header
 *
 * Copyright 2016 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
 */

pub const CEC_MAX_MSG_SIZE: u32 = 16;

/**
 * pub struct cec_msg - CEC message structure.
 * @tx_ts:	Timestamp in nanoseconds using CLOCK_MONOTONIC. Set by the
 *		driver when the message transmission has finished.
 * @rx_ts:	Timestamp in nanoseconds using CLOCK_MONOTONIC. Set by the
 *		driver when the message was received.
 * @len:	Length in bytes of the message.
 * @timeout:	The timeout (in ms) that is used to timeout CEC_RECEIVE.
 *		Set to 0 if you want to wait forever. This timeout can also be
 *		used with CEC_TRANSMIT as the timeout for waiting for a reply.
 *		If 0, then it will use a 1 second timeout instead of waiting
 *		forever as is done with CEC_RECEIVE.
 * @sequence:	The framework assigns a sequence number to messages that are
 *		sent. This can be used to track replies to previously sent
 *		messages.
 * @flags:	Set to 0.
 * @msg:	The message payload.
 * @reply:	This field is ignored with CEC_RECEIVE and is only used by
 *		CEC_TRANSMIT. If non-zero, then wait for a reply with this
 *		opcode. Set to CEC_MSG_FEATURE_ABORT if you want to wait for
 *		a possible ABORT reply. If there was an error when sending the
 *		msg or FeatureAbort was returned, then reply is set to 0.
 *		If reply is non-zero upon return, then len/msg are set to
 *		the received message.
 *		If reply is zero upon return and status has the
 *		CEC_TX_STATUS_FEATURE_ABORT bit set, then len/msg are set to
 *		the received feature abort message.
 *		If reply is zero upon return and status has the
 *		CEC_TX_STATUS_MAX_RETRIES bit set, then no reply was seen at
 *		all. If reply is non-zero for CEC_TRANSMIT and the message is a
 *		broadcast, then -EINVAL is returned.
 *		if reply is non-zero, then timeout is set to 1000 (the required
 *		maximum response time).
 * @rx_status:	The message receive status bits. Set by the driver.
 * @tx_status:	The message transmit status bits. Set by the driver.
 * @tx_arb_lost_cnt: The number of 'Arbitration Lost' events. Set by the driver.
 * @tx_nack_cnt: The number of 'Not Acknowledged' events. Set by the driver.
 * @tx_low_drive_cnt: The number of 'Low Drive Detected' events. Set by the
 *		driver.
 * @tx_error_cnt: The number of 'Error' events. Set by the driver.
 */
pub struct cec_msg {
	u64 tx_ts;
	u64 rx_ts;
	u32 len;
	u32 timeout;
	u32 sequence;
	u32 flags;
	u8 msg[CEC_MAX_MSG_SIZE];
	u8 reply;
	u8 rx_status;
	u8 tx_status;
	u8 tx_arb_lost_cnt;
	u8 tx_nack_cnt;
	u8 tx_low_drive_cnt;
	u8 tx_error_cnt;
};

/**
 * cec_msg_initiator - return the initiator's logical address.
 * @msg:	the message structure
 */
pub unsafe fn u8 cec_msg_initiator(const pub struct cec_msg (*msg))
{
	return (*msg).msg[0] >> 4;
}

/**
 * cec_msg_destination - return the destination's logical address.
 * @msg:	the message structure
 */
pub unsafe fn u8 cec_msg_destination(const pub struct cec_msg (*msg))
{
	return (*msg).msg[0] & 0xf;
}

/**
 * cec_msg_opcode - return the opcode of the message, -1 for poll
 * @msg:	the message structure
 */
pub unsafe fn int cec_msg_opcode(const pub struct cec_msg (*msg))
{
	return (*msg).len > 1 ? (*msg).msg[1] : -1;
}

/**
 * cec_msg_is_broadcast - return true if this is a broadcast message.
 * @msg:	the message structure
 */
pub unsafe fn int cec_msg_is_broadcast(const pub struct cec_msg (*msg))
{
	return ((*msg).msg[0] & 0xf) == 0xf;
}

/**
 * cec_msg_init - initialize the message structure.
 * @msg:	the message structure
 * @initiator:	the logical address of the initiator
 * @destination:the logical address of the destination (0xf for broadcast)
 *
 * The whole structure is zeroed, the len field is set to 1 (i.e. a poll
 * message) and the initiator and destination are filled in.
 */
pub unsafe fn void cec_msg_init(pub struct cec_msg (*msg),
				u8 initiator, u8 destination)
{
	memset(msg, 0, sizeof((*msg)));
	(*msg).msg[0] = (initiator << 4) | destination;
	(*msg).len = 1;
}

/**
 * cec_msg_set_reply_to - fill in destination/initiator in a reply message.
 * @msg:	the message structure for the reply
 * @orig:	the original message structure
 *
 * Set the msg destination to the orig initiator and the msg initiator to the
 * orig destination. Note that msg and orig may be the same pointer, in which
 * case the change is done in place.
 *
 * It also zeroes the reply, timeout and flags fields.
 */
pub unsafe fn void cec_msg_set_reply_to(pub struct cec_msg (*msg),
					pub struct cec_msg (*orig))
{
	/* The destination becomes the initiator and vice versa */
	(*msg).msg[0] = (cec_msg_destination(orig) << 4) |
		      cec_msg_initiator(orig);
	(*msg).reply = 0;
	(*msg).timeout = 0;
	(*msg).flags = 0;
}

/**
 * cec_msg_recv_is_tx_result - return true if this message contains the
 *			       result of an earlier non-blocking transmit
 * @msg:	the message structure from CEC_RECEIVE
 */
pub unsafe fn int cec_msg_recv_is_tx_result(const pub struct cec_msg (*msg))
{
	return (*msg).sequence && (*msg).tx_status && !(*msg).rx_status;
}

/**
 * cec_msg_recv_is_rx_result - return true if this message contains the
 *			       reply of an earlier non-blocking transmit
 * @msg:	the message structure from CEC_RECEIVE
 */
pub unsafe fn int cec_msg_recv_is_rx_result(const pub struct cec_msg (*msg))
{
	return (*msg).sequence && !(*msg).tx_status && (*msg).rx_status;
}

/* cec_msg flags field */
pub const CEC_MSG_FL_REPLY_TO_FOLLOWERS: u32 = (1 << 0);
pub const CEC_MSG_FL_RAW: u32 = (1 << 1);
pub const CEC_MSG_FL_REPLY_VENDOR_ID: u32 = (1 << 2);

/* cec_msg tx/rx_status field */
pub const CEC_TX_STATUS_OK: u32 = (1 << 0);
pub const CEC_TX_STATUS_ARB_LOST: u32 = (1 << 1);
pub const CEC_TX_STATUS_NACK: u32 = (1 << 2);
pub const CEC_TX_STATUS_LOW_DRIVE: u32 = (1 << 3);
pub const CEC_TX_STATUS_ERROR: u32 = (1 << 4);
pub const CEC_TX_STATUS_MAX_RETRIES: u32 = (1 << 5);
pub const CEC_TX_STATUS_ABORTED: u32 = (1 << 6);
pub const CEC_TX_STATUS_TIMEOUT: u32 = (1 << 7);

pub const CEC_RX_STATUS_OK: u32 = (1 << 0);
pub const CEC_RX_STATUS_TIMEOUT: u32 = (1 << 1);
pub const CEC_RX_STATUS_FEATURE_ABORT: u32 = (1 << 2);
pub const CEC_RX_STATUS_ABORTED: u32 = (1 << 3);

pub unsafe fn int cec_msg_status_is_ok(const pub struct cec_msg (*msg))
{
	if ((*msg).tx_status && !((*msg).tx_status & CEC_TX_STATUS_OK))
		return 0;
	if ((*msg).rx_status && !((*msg).rx_status & CEC_RX_STATUS_OK))
		return 0;
	if (!(*msg).tx_status && !(*msg).rx_status)
		return 0;
	return !((*msg).rx_status & CEC_RX_STATUS_FEATURE_ABORT);
}

pub const CEC_LOG_ADDR_INVALID: u32 = 0xff;
pub const CEC_PHYS_ADDR_INVALID: u32 = 0xffff;

/*
 * The maximum number of logical addresses one device can be assigned to.
 * The CEC 2.0 spec allows for only 2 logical addresses at the moment. The
 * Analog Devices CEC hardware supports 3. So let's go wild and go for 4.
 */
pub const CEC_MAX_LOG_ADDRS: u32 = 4;

/* The logical addresses defined by CEC 2.0 */
pub const CEC_LOG_ADDR_TV: u32 = 0;
pub const CEC_LOG_ADDR_RECORD_1: u32 = 1;
pub const CEC_LOG_ADDR_RECORD_2: u32 = 2;
pub const CEC_LOG_ADDR_TUNER_1: u32 = 3;
pub const CEC_LOG_ADDR_PLAYBACK_1: u32 = 4;
pub const CEC_LOG_ADDR_AUDIOSYSTEM: u32 = 5;
pub const CEC_LOG_ADDR_TUNER_2: u32 = 6;
pub const CEC_LOG_ADDR_TUNER_3: u32 = 7;
pub const CEC_LOG_ADDR_PLAYBACK_2: u32 = 8;
pub const CEC_LOG_ADDR_RECORD_3: u32 = 9;
pub const CEC_LOG_ADDR_TUNER_4: u32 = 10;
pub const CEC_LOG_ADDR_PLAYBACK_3: u32 = 11;
pub const CEC_LOG_ADDR_BACKUP_1: u32 = 12;
pub const CEC_LOG_ADDR_BACKUP_2: u32 = 13;
pub const CEC_LOG_ADDR_SPECIFIC: u32 = 14;
pub const CEC_LOG_ADDR_UNREGISTERED: u32 = 15 /* as initiator address */;
pub const CEC_LOG_ADDR_BROADCAST: u32 = 15 /* as destination address */;

/* The logical address types that the CEC device wants to claim */
pub const CEC_LOG_ADDR_TYPE_TV: u32 = 0;
pub const CEC_LOG_ADDR_TYPE_RECORD: u32 = 1;
pub const CEC_LOG_ADDR_TYPE_TUNER: u32 = 2;
pub const CEC_LOG_ADDR_TYPE_PLAYBACK: u32 = 3;
pub const CEC_LOG_ADDR_TYPE_AUDIOSYSTEM: u32 = 4;
pub const CEC_LOG_ADDR_TYPE_SPECIFIC: u32 = 5;
pub const CEC_LOG_ADDR_TYPE_UNREGISTERED: u32 = 6;
/*
 * Switches should use UNREGISTERED.
 * Processors should use SPECIFIC.
 */

pub const CEC_LOG_ADDR_MASK_TV: u32 = (1 << CEC_LOG_ADDR_TV);
pub const CEC_LOG_ADDR_MASK_RECORD: u32 = ((1 << CEC_LOG_ADDR_RECORD_1) | \;
					 (1 << CEC_LOG_ADDR_RECORD_2) | \
					 (1 << CEC_LOG_ADDR_RECORD_3))
pub const CEC_LOG_ADDR_MASK_TUNER: u32 = ((1 << CEC_LOG_ADDR_TUNER_1) | \;
					 (1 << CEC_LOG_ADDR_TUNER_2) | \
					 (1 << CEC_LOG_ADDR_TUNER_3) | \
					 (1 << CEC_LOG_ADDR_TUNER_4))
pub const CEC_LOG_ADDR_MASK_PLAYBACK: u32 = ((1 << CEC_LOG_ADDR_PLAYBACK_1) | \;
					 (1 << CEC_LOG_ADDR_PLAYBACK_2) | \
					 (1 << CEC_LOG_ADDR_PLAYBACK_3))
pub const CEC_LOG_ADDR_MASK_AUDIOSYSTEM: u32 = (1 << CEC_LOG_ADDR_AUDIOSYSTEM);
pub const CEC_LOG_ADDR_MASK_BACKUP: u32 = ((1 << CEC_LOG_ADDR_BACKUP_1) | \;
					 (1 << CEC_LOG_ADDR_BACKUP_2))
pub const CEC_LOG_ADDR_MASK_SPECIFIC: u32 = (1 << CEC_LOG_ADDR_SPECIFIC);
pub const CEC_LOG_ADDR_MASK_UNREGISTERED: u32 = (1 << CEC_LOG_ADDR_UNREGISTERED);

pub unsafe fn int cec_has_tv(u16 log_addr_mask)
{
	return log_addr_mask & CEC_LOG_ADDR_MASK_TV;
}

pub unsafe fn int cec_has_record(u16 log_addr_mask)
{
	return log_addr_mask & CEC_LOG_ADDR_MASK_RECORD;
}

pub unsafe fn int cec_has_tuner(u16 log_addr_mask)
{
	return log_addr_mask & CEC_LOG_ADDR_MASK_TUNER;
}

pub unsafe fn int cec_has_playback(u16 log_addr_mask)
{
	return log_addr_mask & CEC_LOG_ADDR_MASK_PLAYBACK;
}

pub unsafe fn int cec_has_audiosystem(u16 log_addr_mask)
{
	return log_addr_mask & CEC_LOG_ADDR_MASK_AUDIOSYSTEM;
}

pub unsafe fn int cec_has_backup(u16 log_addr_mask)
{
	return log_addr_mask & CEC_LOG_ADDR_MASK_BACKUP;
}

pub unsafe fn int cec_has_specific(u16 log_addr_mask)
{
	return log_addr_mask & CEC_LOG_ADDR_MASK_SPECIFIC;
}

pub unsafe fn int cec_is_unregistered(u16 log_addr_mask)
{
	return log_addr_mask & CEC_LOG_ADDR_MASK_UNREGISTERED;
}

pub unsafe fn int cec_is_unconfigured(u16 log_addr_mask)
{
	return log_addr_mask == 0;
}

/*
 * Use this if there is no vendor ID (CEC_G_VENDOR_ID) or if the vendor ID
 * should be disabled (CEC_S_VENDOR_ID)
 */
pub const CEC_VENDOR_ID_NONE: u32 = 0xffffffff;

/* The message handling modes */
/* Modes for initiator */
pub const CEC_MODE_NO_INITIATOR: u32 = (0x0 << 0);
pub const CEC_MODE_INITIATOR: u32 = (0x1 << 0);
pub const CEC_MODE_EXCL_INITIATOR: u32 = (0x2 << 0);
pub const CEC_MODE_INITIATOR_MSK: u32 = 0x0f;

/* Modes for follower */
pub const CEC_MODE_NO_FOLLOWER: u32 = (0x0 << 4);
pub const CEC_MODE_FOLLOWER: u32 = (0x1 << 4);
pub const CEC_MODE_EXCL_FOLLOWER: u32 = (0x2 << 4);
pub const CEC_MODE_EXCL_FOLLOWER_PASSTHRU: u32 = (0x3 << 4);
pub const CEC_MODE_MONITOR_PIN: u32 = (0xd << 4);
pub const CEC_MODE_MONITOR: u32 = (0xe << 4);
pub const CEC_MODE_MONITOR_ALL: u32 = (0xf << 4);
pub const CEC_MODE_FOLLOWER_MSK: u32 = 0xf0;

/* Userspace has to configure the physical address */
pub const CEC_CAP_PHYS_ADDR: u32 = (1 << 0);
/* Userspace has to configure the logical addresses */
pub const CEC_CAP_LOG_ADDRS: u32 = (1 << 1);
/* Userspace can transmit messages (and thus become follower as well) */
pub const CEC_CAP_TRANSMIT: u32 = (1 << 2);
/*
 * Passthrough all messages instead of processing them.
 */
pub const CEC_CAP_PASSTHROUGH: u32 = (1 << 3);
/* Supports remote control */
pub const CEC_CAP_RC: u32 = (1 << 4);
/* Hardware can monitor all messages, not just directed and broadcast. */
pub const CEC_CAP_MONITOR_ALL: u32 = (1 << 5);
/* Hardware can use CEC only if the HDMI HPD pin is high. */
pub const CEC_CAP_NEEDS_HPD: u32 = (1 << 6);
/* Hardware can monitor CEC pin transitions */
pub const CEC_CAP_MONITOR_PIN: u32 = (1 << 7);
/* CEC_ADAP_G_CONNECTOR_INFO is available */
pub const CEC_CAP_CONNECTOR_INFO: u32 = (1 << 8);
/* CEC_MSG_FL_REPLY_VENDOR_ID is available */
pub const CEC_CAP_REPLY_VENDOR_ID: u32 = (1 << 9);

/**
 * pub struct cec_caps - CEC capabilities structure.
 * @driver: name of the CEC device driver.
 * @name: name of the CEC device. @driver + @name must be unique.
 * @available_log_addrs: number of available logical addresses.
 * @capabilities: capabilities of the CEC adapter.
 * @version: version of the CEC adapter framework.
 */
pub struct cec_caps {
	i8 driver[32];
	i8 name[32];
	u32 available_log_addrs;
	u32 capabilities;
	u32 version;
};

/**
 * pub struct cec_log_addrs - CEC logical addresses structure.
 * @log_addr: the claimed logical addresses. Set by the driver.
 * @log_addr_mask: current logical address mask. Set by the driver.
 * @cec_version: the CEC version that the adapter should implement. Set by the
 *	caller.
 * @num_log_addrs: how many logical addresses should be claimed. Set by the
 *	caller.
 * @vendor_id: the vendor ID of the device. Set by the caller.
 * @flags: flags.
 * @osd_name: the OSD name of the device. Set by the caller.
 * @primary_device_type: the primary device type for each logical address.
 *	Set by the caller.
 * @log_addr_type: the logical address types. Set by the caller.
 * @all_device_types: CEC 2.0: all device types represented by the logical
 *	address. Set by the caller.
 * @features:	CEC 2.0: The logical address features. Set by the caller.
 */
pub struct cec_log_addrs {
	u8 log_addr[CEC_MAX_LOG_ADDRS];
	u16 log_addr_mask;
	u8 cec_version;
	u8 num_log_addrs;
	u32 vendor_id;
	u32 flags;
	i8 osd_name[15];
	u8 primary_device_type[CEC_MAX_LOG_ADDRS];
	u8 log_addr_type[CEC_MAX_LOG_ADDRS];

	/* CEC 2.0 */
	u8 all_device_types[CEC_MAX_LOG_ADDRS];
	u8 features[CEC_MAX_LOG_ADDRS][12];
};

/* Allow a fallback to unregistered */
pub const CEC_LOG_ADDRS_FL_ALLOW_UNREG_FALLBACK: u32 = (1 << 0);
/* Passthrough RC messages to the input subsystem */
pub const CEC_LOG_ADDRS_FL_ALLOW_RC_PASSTHRU: u32 = (1 << 1);
/* CDC-Only device: supports only CDC messages */
pub const CEC_LOG_ADDRS_FL_CDC_ONLY: u32 = (1 << 2);
/* Configuration failed */
pub const CEC_LOG_ADDRS_FL_CONFIG_FAILED: u32 = (1 << 3);

/**
 * pub struct cec_drm_connector_info - tells which drm connector is
 * associated with the CEC adapter.
 * @card_no: drm card number
 * @connector_id: drm connector ID
 */
pub struct cec_drm_connector_info {
	u32 card_no;
	u32 connector_id;
};

pub const CEC_CONNECTOR_TYPE_NO_CONNECTOR: u32 = 0;
pub const CEC_CONNECTOR_TYPE_DRM: u32 = 1;

/**
 * pub struct cec_connector_info - tells if and which connector is
 * associated with the CEC adapter.
 * @type: connector type (if any)
 * @drm: drm connector info
 * @raw: array to pad the union
 */
pub struct cec_connector_info {
	u32 type;
	union {
		pub struct cec_drm_connector_info drm;
		u32 raw[16];
	};
};

/* Events */

/* Event that occurs when the adapter state changes */
pub const CEC_EVENT_STATE_CHANGE: u32 = 1;
/*
 * This event is sent when messages are lost because the application
 * didn't empty the message queue in time
 */
pub const CEC_EVENT_LOST_MSGS: u32 = 2;
pub const CEC_EVENT_PIN_CEC_LOW: u32 = 3;
pub const CEC_EVENT_PIN_CEC_HIGH: u32 = 4;
pub const CEC_EVENT_PIN_HPD_LOW: u32 = 5;
pub const CEC_EVENT_PIN_HPD_HIGH: u32 = 6;
pub const CEC_EVENT_PIN_5V_LOW: u32 = 7;
pub const CEC_EVENT_PIN_5V_HIGH: u32 = 8;

pub const CEC_EVENT_FL_INITIAL_STATE: u32 = (1 << 0);
pub const CEC_EVENT_FL_DROPPED_EVENTS: u32 = (1 << 1);

/**
 * pub pub struct cec_event_state_change - used when the CEC adapter changes state.
 * @phys_addr: the current physical address
 * @log_addr_mask: the current logical address mask
 * @have_conn_info: if non-zero, then HDMI connector information is available.
 *	This field is only valid if CEC_CAP_CONNECTOR_INFO is set. If that
 *	capability is set and @have_conn_info is zero, then that indicates
 *	that the HDMI connector device is not instantiated, either because
 *	the HDMI driver is still configuring the device or because the HDMI
 *	device was unbound.
 */
pub pub struct cec_event_state_change {
	u16 phys_addr;
	u16 log_addr_mask;
	u16 have_conn_info;
};

/**
 * pub pub struct cec_event_lost_msgs - tells you how many messages were lost.
 * @lost_msgs: how many messages were lost.
 */
pub pub struct cec_event_lost_msgs {
	u32 lost_msgs;
};

/**
 * pub struct cec_event - CEC event structure
 * @ts: the timestamp of when the event was sent.
 * @event: the event.
 * @flags: event flags.
 * @state_change: the event payload for CEC_EVENT_STATE_CHANGE.
 * @lost_msgs: the event payload for CEC_EVENT_LOST_MSGS.
 * @raw: array to pad the union.
 */
pub struct cec_event {
	u64 ts;
	u32 event;
	u32 flags;
	union {
		pub pub struct cec_event_state_change state_change;
		pub pub struct cec_event_lost_msgs lost_msgs;
		u32 raw[16];
	};
};

/* ioctls */

/* Adapter capabilities */
pub const CEC_ADAP_G_CAPS: u32 = IOWR('a',  0, pub struct cec_caps);

/*
 * phys_addr is either 0 (if this is the CEC root device)
 * or a valid physical address obtained from the sink's EDID
 * as read by this CEC device (if this is a source device)
 * or a physical address obtained and modified from a sink
 * EDID and used for a sink CEC device.
 * If nothing is connected, then phys_addr is 0xffff.
 * See HDMI 1.4b, section 8.7 (Physical Address).
 *
 * The CEC_ADAP_S_PHYS_ADDR ioctl may not be available if that is handled
 * internally.
 */
pub const CEC_ADAP_G_PHYS_ADDR: u32 = IOR('a',  1, u16);
pub const CEC_ADAP_S_PHYS_ADDR: u32 = IOW('a',  2, u16);

/*
 * Configure the CEC adapter. It sets the device type and which
 * logical types it will try to claim. It will return which
 * logical addresses it could actually claim.
 * An error is returned if the adapter is disabled or if there
 * is no physical address assigned.
 */

pub const CEC_ADAP_G_LOG_ADDRS: u32 = IOR('a',  3, pub struct cec_log_addrs);
pub const CEC_ADAP_S_LOG_ADDRS: u32 = IOWR('a',  4, pub struct cec_log_addrs);

/* Transmit/receive a CEC command */
pub const CEC_TRANSMIT: u32 = IOWR('a',  5, pub struct cec_msg);
pub const CEC_RECEIVE: u32 = IOWR('a',  6, pub struct cec_msg);

/* Dequeue CEC events */
pub const CEC_DQEVENT: u32 = IOWR('a',  7, pub struct cec_event);

/*
 * Get and set the message handling mode for this filehandle.
 */
pub const CEC_G_MODE: u32 = IOR('a',  8, u32);
pub const CEC_S_MODE: u32 = IOW('a',  9, u32);

/* Get the connector info */
pub const CEC_ADAP_G_CONNECTOR_INFO: u32 = IOR('a',  10, pub struct cec_connector_info);

/*
 * The remainder of this header defines all CEC messages and operands.
 * The format matters since it the cec-ctl utility parses it to generate
 * code for implementing all these messages.
 *
 * Comments ending with 'Feature' group messages for each feature.
 * If messages are part of multiple features, then the "Has also"
 * comment is used to list the previously defined messages that are
 * supported by the feature.
 *
 * Before operands are defined a comment is added that gives the
 * name of the operand and in brackets the variable name of the
 * corresponding argument in the cec-funcs.h function.
 */

/* Messages */

/* One Touch Play Feature */
pub const CEC_MSG_ACTIVE_SOURCE: u32 = 0x82;
pub const CEC_MSG_IMAGE_VIEW_ON: u32 = 0x04;
pub const CEC_MSG_TEXT_VIEW_ON: u32 = 0x0d;


/* Routing Control Feature */

/*
 * Has also:
 *	CEC_MSG_ACTIVE_SOURCE
 */

pub const CEC_MSG_INACTIVE_SOURCE: u32 = 0x9d;
pub const CEC_MSG_REQUEST_ACTIVE_SOURCE: u32 = 0x85;
pub const CEC_MSG_ROUTING_CHANGE: u32 = 0x80;
pub const CEC_MSG_ROUTING_INFORMATION: u32 = 0x81;
pub const CEC_MSG_SET_STREAM_PATH: u32 = 0x86;


/* Standby Feature */
pub const CEC_MSG_STANDBY: u32 = 0x36;


/* One Touch Record Feature */
pub const CEC_MSG_RECORD_OFF: u32 = 0x0b;
pub const CEC_MSG_RECORD_ON: u32 = 0x09;
/* Record Source Type Operand (rec_src_type) */
pub const CEC_OP_RECORD_SRC_OWN: u32 = 1;
pub const CEC_OP_RECORD_SRC_DIGITAL: u32 = 2;
pub const CEC_OP_RECORD_SRC_ANALOG: u32 = 3;
pub const CEC_OP_RECORD_SRC_EXT_PLUG: u32 = 4;
pub const CEC_OP_RECORD_SRC_EXT_PHYS_ADDR: u32 = 5;
/* Service Identification Method Operand (service_id_method) */
pub const CEC_OP_SERVICE_ID_METHOD_BY_DIG_ID: u32 = 0;
pub const CEC_OP_SERVICE_ID_METHOD_BY_CHANNEL: u32 = 1;
/* Digital Service Broadcast System Operand (dig_bcast_system) */
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ARIB_GEN: u32 = 0x00;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ATSC_GEN: u32 = 0x01;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_DVB_GEN: u32 = 0x02;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ARIB_BS: u32 = 0x08;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ARIB_CS: u32 = 0x09;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ARIB_T: u32 = 0x0a;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ATSC_CABLE: u32 = 0x10;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ATSC_SAT: u32 = 0x11;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ATSC_T: u32 = 0x12;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_DVB_C: u32 = 0x18;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_DVB_S: u32 = 0x19;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_DVB_S2: u32 = 0x1a;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_DVB_T: u32 = 0x1b;
/* Analogue Broadcast Type Operand (ana_bcast_type) */
pub const CEC_OP_ANA_BCAST_TYPE_CABLE: u32 = 0;
pub const CEC_OP_ANA_BCAST_TYPE_SATELLITE: u32 = 1;
pub const CEC_OP_ANA_BCAST_TYPE_TERRESTRIAL: u32 = 2;
/* Broadcast System Operand (bcast_system) */
pub const CEC_OP_BCAST_SYSTEM_PAL_BG: u32 = 0x00;
pub const CEC_OP_BCAST_SYSTEM_SECAM_LQ: u32 = 0x01 /* SECAM L' */;
pub const CEC_OP_BCAST_SYSTEM_PAL_M: u32 = 0x02;
pub const CEC_OP_BCAST_SYSTEM_NTSC_M: u32 = 0x03;
pub const CEC_OP_BCAST_SYSTEM_PAL_I: u32 = 0x04;
pub const CEC_OP_BCAST_SYSTEM_SECAM_DK: u32 = 0x05;
pub const CEC_OP_BCAST_SYSTEM_SECAM_BG: u32 = 0x06;
pub const CEC_OP_BCAST_SYSTEM_SECAM_L: u32 = 0x07;
pub const CEC_OP_BCAST_SYSTEM_PAL_DK: u32 = 0x08;
pub const CEC_OP_BCAST_SYSTEM_OTHER: u32 = 0x1f;
/* Channel Number Format Operand (channel_number_fmt) */
pub const CEC_OP_CHANNEL_NUMBER_FMT_1_PART: u32 = 0x01;
pub const CEC_OP_CHANNEL_NUMBER_FMT_2_PART: u32 = 0x02;

pub const CEC_MSG_RECORD_STATUS: u32 = 0x0a;
/* Record Status Operand (rec_status) */
pub const CEC_OP_RECORD_STATUS_CUR_SRC: u32 = 0x01;
pub const CEC_OP_RECORD_STATUS_DIG_SERVICE: u32 = 0x02;
pub const CEC_OP_RECORD_STATUS_ANA_SERVICE: u32 = 0x03;
pub const CEC_OP_RECORD_STATUS_EXT_INPUT: u32 = 0x04;
pub const CEC_OP_RECORD_STATUS_NO_DIG_SERVICE: u32 = 0x05;
pub const CEC_OP_RECORD_STATUS_NO_ANA_SERVICE: u32 = 0x06;
pub const CEC_OP_RECORD_STATUS_NO_SERVICE: u32 = 0x07;
pub const CEC_OP_RECORD_STATUS_INVALID_EXT_PLUG: u32 = 0x09;
pub const CEC_OP_RECORD_STATUS_INVALID_EXT_PHYS_ADDR: u32 = 0x0a;
pub const CEC_OP_RECORD_STATUS_UNSUP_CA: u32 = 0x0b;
pub const CEC_OP_RECORD_STATUS_NO_CA_ENTITLEMENTS: u32 = 0x0c;
pub const CEC_OP_RECORD_STATUS_CANT_COPY_SRC: u32 = 0x0d;
pub const CEC_OP_RECORD_STATUS_NO_MORE_COPIES: u32 = 0x0e;
pub const CEC_OP_RECORD_STATUS_NO_MEDIA: u32 = 0x10;
pub const CEC_OP_RECORD_STATUS_PLAYING: u32 = 0x11;
pub const CEC_OP_RECORD_STATUS_ALREADY_RECORDING: u32 = 0x12;
pub const CEC_OP_RECORD_STATUS_MEDIA_PROT: u32 = 0x13;
pub const CEC_OP_RECORD_STATUS_NO_SIGNAL: u32 = 0x14;
pub const CEC_OP_RECORD_STATUS_MEDIA_PROBLEM: u32 = 0x15;
pub const CEC_OP_RECORD_STATUS_NO_SPACE: u32 = 0x16;
pub const CEC_OP_RECORD_STATUS_PARENTAL_LOCK: u32 = 0x17;
pub const CEC_OP_RECORD_STATUS_TERMINATED_OK: u32 = 0x1a;
pub const CEC_OP_RECORD_STATUS_ALREADY_TERM: u32 = 0x1b;
pub const CEC_OP_RECORD_STATUS_OTHER: u32 = 0x1f;

pub const CEC_MSG_RECORD_TV_SCREEN: u32 = 0x0f;


/* Timer Programming Feature */
pub const CEC_MSG_CLEAR_ANALOGUE_TIMER: u32 = 0x33;
/* Recording Sequence Operand (recording_seq) */
pub const CEC_OP_REC_SEQ_SUNDAY: u32 = 0x01;
pub const CEC_OP_REC_SEQ_MONDAY: u32 = 0x02;
pub const CEC_OP_REC_SEQ_TUESDAY: u32 = 0x04;
pub const CEC_OP_REC_SEQ_WEDNESDAY: u32 = 0x08;
pub const CEC_OP_REC_SEQ_THURSDAY: u32 = 0x10;
pub const CEC_OP_REC_SEQ_FRIDAY: u32 = 0x20;
pub const CEC_OP_REC_SEQ_SATURDAY: u32 = 0x40;
pub const CEC_OP_REC_SEQ_ONCE_ONLY: u32 = 0x00;

pub const CEC_MSG_CLEAR_DIGITAL_TIMER: u32 = 0x99;

pub const CEC_MSG_CLEAR_EXT_TIMER: u32 = 0xa1;
/* External Source Specifier Operand (ext_src_spec) */
pub const CEC_OP_EXT_SRC_PLUG: u32 = 0x04;
pub const CEC_OP_EXT_SRC_PHYS_ADDR: u32 = 0x05;

pub const CEC_MSG_SET_ANALOGUE_TIMER: u32 = 0x34;
pub const CEC_MSG_SET_DIGITAL_TIMER: u32 = 0x97;
pub const CEC_MSG_SET_EXT_TIMER: u32 = 0xa2;

pub const CEC_MSG_SET_TIMER_PROGRAM_TITLE: u32 = 0x67;
pub const CEC_MSG_TIMER_CLEARED_STATUS: u32 = 0x43;
/* Timer Cleared Status Data Operand (timer_cleared_status) */
pub const CEC_OP_TIMER_CLR_STAT_RECORDING: u32 = 0x00;
pub const CEC_OP_TIMER_CLR_STAT_NO_MATCHING: u32 = 0x01;
pub const CEC_OP_TIMER_CLR_STAT_NO_INFO: u32 = 0x02;
pub const CEC_OP_TIMER_CLR_STAT_CLEARED: u32 = 0x80;

pub const CEC_MSG_TIMER_STATUS: u32 = 0x35;
/* Timer Overlap Warning Operand (timer_overlap_warning) */
pub const CEC_OP_TIMER_OVERLAP_WARNING_NO_OVERLAP: u32 = 0;
pub const CEC_OP_TIMER_OVERLAP_WARNING_OVERLAP: u32 = 1;
/* Media Info Operand (media_info) */
pub const CEC_OP_MEDIA_INFO_UNPROT_MEDIA: u32 = 0;
pub const CEC_OP_MEDIA_INFO_PROT_MEDIA: u32 = 1;
pub const CEC_OP_MEDIA_INFO_NO_MEDIA: u32 = 2;
/* Programmed Indicator Operand (prog_indicator) */
pub const CEC_OP_PROG_IND_NOT_PROGRAMMED: u32 = 0;
pub const CEC_OP_PROG_IND_PROGRAMMED: u32 = 1;
/* Programmed Info Operand (prog_info) */
pub const CEC_OP_PROG_INFO_ENOUGH_SPACE: u32 = 0x08;
pub const CEC_OP_PROG_INFO_NOT_ENOUGH_SPACE: u32 = 0x09;
pub const CEC_OP_PROG_INFO_MIGHT_NOT_BE_ENOUGH_SPACE: u32 = 0x0b;
pub const CEC_OP_PROG_INFO_NONE_AVAILABLE: u32 = 0x0a;
/* Not Programmed Error Info Operand (prog_error) */
pub const CEC_OP_PROG_ERROR_NO_FREE_TIMER: u32 = 0x01;
pub const CEC_OP_PROG_ERROR_DATE_OUT_OF_RANGE: u32 = 0x02;
pub const CEC_OP_PROG_ERROR_REC_SEQ_ERROR: u32 = 0x03;
pub const CEC_OP_PROG_ERROR_INV_EXT_PLUG: u32 = 0x04;
pub const CEC_OP_PROG_ERROR_INV_EXT_PHYS_ADDR: u32 = 0x05;
pub const CEC_OP_PROG_ERROR_CA_UNSUPP: u32 = 0x06;
pub const CEC_OP_PROG_ERROR_INSUF_CA_ENTITLEMENTS: u32 = 0x07;
pub const CEC_OP_PROG_ERROR_RESOLUTION_UNSUPP: u32 = 0x08;
pub const CEC_OP_PROG_ERROR_PARENTAL_LOCK: u32 = 0x09;
pub const CEC_OP_PROG_ERROR_CLOCK_FAILURE: u32 = 0x0a;
pub const CEC_OP_PROG_ERROR_DUPLICATE: u32 = 0x0e;


/* System Information Feature */
pub const CEC_MSG_CEC_VERSION: u32 = 0x9e;
/* CEC Version Operand (cec_version) */
pub const CEC_OP_CEC_VERSION_1_3A: u32 = 4;
pub const CEC_OP_CEC_VERSION_1_4: u32 = 5;
pub const CEC_OP_CEC_VERSION_2_0: u32 = 6;

pub const CEC_MSG_GET_CEC_VERSION: u32 = 0x9f;
pub const CEC_MSG_GIVE_PHYSICAL_ADDR: u32 = 0x83;
pub const CEC_MSG_GET_MENU_LANGUAGE: u32 = 0x91;
pub const CEC_MSG_REPORT_PHYSICAL_ADDR: u32 = 0x84;
/* Primary Device Type Operand (prim_devtype) */
pub const CEC_OP_PRIM_DEVTYPE_TV: u32 = 0;
pub const CEC_OP_PRIM_DEVTYPE_RECORD: u32 = 1;
pub const CEC_OP_PRIM_DEVTYPE_TUNER: u32 = 3;
pub const CEC_OP_PRIM_DEVTYPE_PLAYBACK: u32 = 4;
pub const CEC_OP_PRIM_DEVTYPE_AUDIOSYSTEM: u32 = 5;
pub const CEC_OP_PRIM_DEVTYPE_SWITCH: u32 = 6;
pub const CEC_OP_PRIM_DEVTYPE_PROCESSOR: u32 = 7;

pub const CEC_MSG_SET_MENU_LANGUAGE: u32 = 0x32;
pub const CEC_MSG_REPORT_FEATURES: u32 = 0xa6	/* CEC 2.0 */;
/* All Device Types Operand (all_device_types) */
pub const CEC_OP_ALL_DEVTYPE_TV: u32 = 0x80;
pub const CEC_OP_ALL_DEVTYPE_RECORD: u32 = 0x40;
pub const CEC_OP_ALL_DEVTYPE_TUNER: u32 = 0x20;
pub const CEC_OP_ALL_DEVTYPE_PLAYBACK: u32 = 0x10;
pub const CEC_OP_ALL_DEVTYPE_AUDIOSYSTEM: u32 = 0x08;
pub const CEC_OP_ALL_DEVTYPE_SWITCH: u32 = 0x04;
/*
 * And if you wondering what happened to PROCESSOR devices: those should
 * be mapped to a SWITCH.
 */

/* Valid for RC Profile and Device Feature operands */
pub const CEC_OP_FEAT_EXT: u32 = 0x80	/* Extension bit */;
/* RC Profile Operand (rc_profile) */
pub const CEC_OP_FEAT_RC_TV_PROFILE_NONE: u32 = 0x00;
pub const CEC_OP_FEAT_RC_TV_PROFILE_1: u32 = 0x02;
pub const CEC_OP_FEAT_RC_TV_PROFILE_2: u32 = 0x06;
pub const CEC_OP_FEAT_RC_TV_PROFILE_3: u32 = 0x0a;
pub const CEC_OP_FEAT_RC_TV_PROFILE_4: u32 = 0x0e;
pub const CEC_OP_FEAT_RC_SRC_HAS_DEV_ROOT_MENU: u32 = 0x50;
pub const CEC_OP_FEAT_RC_SRC_HAS_DEV_SETUP_MENU: u32 = 0x48;
pub const CEC_OP_FEAT_RC_SRC_HAS_CONTENTS_MENU: u32 = 0x44;
pub const CEC_OP_FEAT_RC_SRC_HAS_MEDIA_TOP_MENU: u32 = 0x42;
pub const CEC_OP_FEAT_RC_SRC_HAS_MEDIA_CONTEXT_MENU: u32 = 0x41;
/* Device Feature Operand (dev_features) */
pub const CEC_OP_FEAT_DEV_HAS_RECORD_TV_SCREEN: u32 = 0x40;
pub const CEC_OP_FEAT_DEV_HAS_SET_OSD_STRING: u32 = 0x20;
pub const CEC_OP_FEAT_DEV_HAS_DECK_CONTROL: u32 = 0x10;
pub const CEC_OP_FEAT_DEV_HAS_SET_AUDIO_RATE: u32 = 0x08;
pub const CEC_OP_FEAT_DEV_SINK_HAS_ARC_TX: u32 = 0x04;
pub const CEC_OP_FEAT_DEV_SOURCE_HAS_ARC_RX: u32 = 0x02;
pub const CEC_OP_FEAT_DEV_HAS_SET_AUDIO_VOLUME_LEVEL: u32 = 0x01;

pub const CEC_MSG_GIVE_FEATURES: u32 = 0xa5	/* CEC 2.0 */;


/* Deck Control Feature */
pub const CEC_MSG_DECK_CONTROL: u32 = 0x42;
/* Deck Control Mode Operand (deck_control_mode) */
pub const CEC_OP_DECK_CTL_MODE_SKIP_FWD: u32 = 1;
pub const CEC_OP_DECK_CTL_MODE_SKIP_REV: u32 = 2;
pub const CEC_OP_DECK_CTL_MODE_STOP: u32 = 3;
pub const CEC_OP_DECK_CTL_MODE_EJECT: u32 = 4;

pub const CEC_MSG_DECK_STATUS: u32 = 0x1b;
/* Deck Info Operand (deck_info) */
pub const CEC_OP_DECK_INFO_PLAY: u32 = 0x11;
pub const CEC_OP_DECK_INFO_RECORD: u32 = 0x12;
pub const CEC_OP_DECK_INFO_PLAY_REV: u32 = 0x13;
pub const CEC_OP_DECK_INFO_STILL: u32 = 0x14;
pub const CEC_OP_DECK_INFO_SLOW: u32 = 0x15;
pub const CEC_OP_DECK_INFO_SLOW_REV: u32 = 0x16;
pub const CEC_OP_DECK_INFO_FAST_FWD: u32 = 0x17;
pub const CEC_OP_DECK_INFO_FAST_REV: u32 = 0x18;
pub const CEC_OP_DECK_INFO_NO_MEDIA: u32 = 0x19;
pub const CEC_OP_DECK_INFO_STOP: u32 = 0x1a;
pub const CEC_OP_DECK_INFO_SKIP_FWD: u32 = 0x1b;
pub const CEC_OP_DECK_INFO_SKIP_REV: u32 = 0x1c;
pub const CEC_OP_DECK_INFO_INDEX_SEARCH_FWD: u32 = 0x1d;
pub const CEC_OP_DECK_INFO_INDEX_SEARCH_REV: u32 = 0x1e;
pub const CEC_OP_DECK_INFO_OTHER: u32 = 0x1f;

pub const CEC_MSG_GIVE_DECK_STATUS: u32 = 0x1a;
/* Status Request Operand (status_req) */
pub const CEC_OP_STATUS_REQ_ON: u32 = 1;
pub const CEC_OP_STATUS_REQ_OFF: u32 = 2;
pub const CEC_OP_STATUS_REQ_ONCE: u32 = 3;

pub const CEC_MSG_PLAY: u32 = 0x41;
/* Play Mode Operand (play_mode) */
pub const CEC_OP_PLAY_MODE_PLAY_FWD: u32 = 0x24;
pub const CEC_OP_PLAY_MODE_PLAY_REV: u32 = 0x20;
pub const CEC_OP_PLAY_MODE_PLAY_STILL: u32 = 0x25;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_FWD_MIN: u32 = 0x05;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_FWD_MED: u32 = 0x06;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_FWD_MAX: u32 = 0x07;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_REV_MIN: u32 = 0x09;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_REV_MED: u32 = 0x0a;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_REV_MAX: u32 = 0x0b;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_FWD_MIN: u32 = 0x15;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_FWD_MED: u32 = 0x16;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_FWD_MAX: u32 = 0x17;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_REV_MIN: u32 = 0x19;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_REV_MED: u32 = 0x1a;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_REV_MAX: u32 = 0x1b;


/* Tuner Control Feature */
pub const CEC_MSG_GIVE_TUNER_DEVICE_STATUS: u32 = 0x08;
pub const CEC_MSG_SELECT_ANALOGUE_SERVICE: u32 = 0x92;
pub const CEC_MSG_SELECT_DIGITAL_SERVICE: u32 = 0x93;
pub const CEC_MSG_TUNER_DEVICE_STATUS: u32 = 0x07;
/* Recording Flag Operand (rec_flag) */
pub const CEC_OP_REC_FLAG_NOT_USED: u32 = 0;
pub const CEC_OP_REC_FLAG_USED: u32 = 1;
/* Tuner Display Info Operand (tuner_display_info) */
pub const CEC_OP_TUNER_DISPLAY_INFO_DIGITAL: u32 = 0;
pub const CEC_OP_TUNER_DISPLAY_INFO_NONE: u32 = 1;
pub const CEC_OP_TUNER_DISPLAY_INFO_ANALOGUE: u32 = 2;

pub const CEC_MSG_TUNER_STEP_DECREMENT: u32 = 0x06;
pub const CEC_MSG_TUNER_STEP_INCREMENT: u32 = 0x05;


/* Vendor Specific Commands Feature */

/*
 * Has also:
 *	CEC_MSG_CEC_VERSION
 *	CEC_MSG_GET_CEC_VERSION
 */
pub const CEC_MSG_DEVICE_VENDOR_ID: u32 = 0x87;
pub const CEC_MSG_GIVE_DEVICE_VENDOR_ID: u32 = 0x8c;
pub const CEC_MSG_VENDOR_COMMAND: u32 = 0x89;
pub const CEC_MSG_VENDOR_COMMAND_WITH_ID: u32 = 0xa0;
pub const CEC_MSG_VENDOR_REMOTE_BUTTON_DOWN: u32 = 0x8a;
pub const CEC_MSG_VENDOR_REMOTE_BUTTON_UP: u32 = 0x8b;


/* OSD Display Feature */
pub const CEC_MSG_SET_OSD_STRING: u32 = 0x64;
/* Display Control Operand (disp_ctl) */
pub const CEC_OP_DISP_CTL_DEFAULT: u32 = 0x00;
pub const CEC_OP_DISP_CTL_UNTIL_CLEARED: u32 = 0x40;
pub const CEC_OP_DISP_CTL_CLEAR: u32 = 0x80;


/* Device OSD Transfer Feature */
pub const CEC_MSG_GIVE_OSD_NAME: u32 = 0x46;
pub const CEC_MSG_SET_OSD_NAME: u32 = 0x47;


/* Device Menu Control Feature */
pub const CEC_MSG_MENU_REQUEST: u32 = 0x8d;
/* Menu Request Type Operand (menu_req) */
pub const CEC_OP_MENU_REQUEST_ACTIVATE: u32 = 0x00;
pub const CEC_OP_MENU_REQUEST_DEACTIVATE: u32 = 0x01;
pub const CEC_OP_MENU_REQUEST_QUERY: u32 = 0x02;

pub const CEC_MSG_MENU_STATUS: u32 = 0x8e;
/* Menu State Operand (menu_state) */
pub const CEC_OP_MENU_STATE_ACTIVATED: u32 = 0x00;
pub const CEC_OP_MENU_STATE_DEACTIVATED: u32 = 0x01;

pub const CEC_MSG_USER_CONTROL_PRESSED: u32 = 0x44;
/* UI Command Operand (ui_cmd) */
pub const CEC_OP_UI_CMD_SELECT: u32 = 0x00;
pub const CEC_OP_UI_CMD_UP: u32 = 0x01;
pub const CEC_OP_UI_CMD_DOWN: u32 = 0x02;
pub const CEC_OP_UI_CMD_LEFT: u32 = 0x03;
pub const CEC_OP_UI_CMD_RIGHT: u32 = 0x04;
pub const CEC_OP_UI_CMD_RIGHT_UP: u32 = 0x05;
pub const CEC_OP_UI_CMD_RIGHT_DOWN: u32 = 0x06;
pub const CEC_OP_UI_CMD_LEFT_UP: u32 = 0x07;
pub const CEC_OP_UI_CMD_LEFT_DOWN: u32 = 0x08;
pub const CEC_OP_UI_CMD_DEVICE_ROOT_MENU: u32 = 0x09;
pub const CEC_OP_UI_CMD_DEVICE_SETUP_MENU: u32 = 0x0a;
pub const CEC_OP_UI_CMD_CONTENTS_MENU: u32 = 0x0b;
pub const CEC_OP_UI_CMD_FAVORITE_MENU: u32 = 0x0c;
pub const CEC_OP_UI_CMD_BACK: u32 = 0x0d;
pub const CEC_OP_UI_CMD_MEDIA_TOP_MENU: u32 = 0x10;
pub const CEC_OP_UI_CMD_MEDIA_CONTEXT_SENSITIVE_MENU: u32 = 0x11;
pub const CEC_OP_UI_CMD_NUMBER_ENTRY_MODE: u32 = 0x1d;
pub const CEC_OP_UI_CMD_NUMBER_11: u32 = 0x1e;
pub const CEC_OP_UI_CMD_NUMBER_12: u32 = 0x1f;
pub const CEC_OP_UI_CMD_NUMBER_0_OR_NUMBER_10: u32 = 0x20;
pub const CEC_OP_UI_CMD_NUMBER_1: u32 = 0x21;
pub const CEC_OP_UI_CMD_NUMBER_2: u32 = 0x22;
pub const CEC_OP_UI_CMD_NUMBER_3: u32 = 0x23;
pub const CEC_OP_UI_CMD_NUMBER_4: u32 = 0x24;
pub const CEC_OP_UI_CMD_NUMBER_5: u32 = 0x25;
pub const CEC_OP_UI_CMD_NUMBER_6: u32 = 0x26;
pub const CEC_OP_UI_CMD_NUMBER_7: u32 = 0x27;
pub const CEC_OP_UI_CMD_NUMBER_8: u32 = 0x28;
pub const CEC_OP_UI_CMD_NUMBER_9: u32 = 0x29;
pub const CEC_OP_UI_CMD_DOT: u32 = 0x2a;
pub const CEC_OP_UI_CMD_ENTER: u32 = 0x2b;
pub const CEC_OP_UI_CMD_CLEAR: u32 = 0x2c;
pub const CEC_OP_UI_CMD_NEXT_FAVORITE: u32 = 0x2f;
pub const CEC_OP_UI_CMD_CHANNEL_UP: u32 = 0x30;
pub const CEC_OP_UI_CMD_CHANNEL_DOWN: u32 = 0x31;
pub const CEC_OP_UI_CMD_PREVIOUS_CHANNEL: u32 = 0x32;
pub const CEC_OP_UI_CMD_SOUND_SELECT: u32 = 0x33;
pub const CEC_OP_UI_CMD_INPUT_SELECT: u32 = 0x34;
pub const CEC_OP_UI_CMD_DISPLAY_INFORMATION: u32 = 0x35;
pub const CEC_OP_UI_CMD_HELP: u32 = 0x36;
pub const CEC_OP_UI_CMD_PAGE_UP: u32 = 0x37;
pub const CEC_OP_UI_CMD_PAGE_DOWN: u32 = 0x38;
pub const CEC_OP_UI_CMD_POWER: u32 = 0x40;
pub const CEC_OP_UI_CMD_VOLUME_UP: u32 = 0x41;
pub const CEC_OP_UI_CMD_VOLUME_DOWN: u32 = 0x42;
pub const CEC_OP_UI_CMD_MUTE: u32 = 0x43;
pub const CEC_OP_UI_CMD_PLAY: u32 = 0x44;
pub const CEC_OP_UI_CMD_STOP: u32 = 0x45;
pub const CEC_OP_UI_CMD_PAUSE: u32 = 0x46;
pub const CEC_OP_UI_CMD_RECORD: u32 = 0x47;
pub const CEC_OP_UI_CMD_REWIND: u32 = 0x48;
pub const CEC_OP_UI_CMD_FAST_FORWARD: u32 = 0x49;
pub const CEC_OP_UI_CMD_EJECT: u32 = 0x4a;
pub const CEC_OP_UI_CMD_SKIP_FORWARD: u32 = 0x4b;
pub const CEC_OP_UI_CMD_SKIP_BACKWARD: u32 = 0x4c;
pub const CEC_OP_UI_CMD_STOP_RECORD: u32 = 0x4d;
pub const CEC_OP_UI_CMD_PAUSE_RECORD: u32 = 0x4e;
pub const CEC_OP_UI_CMD_ANGLE: u32 = 0x50;
pub const CEC_OP_UI_CMD_SUB_PICTURE: u32 = 0x51;
pub const CEC_OP_UI_CMD_VIDEO_ON_DEMAND: u32 = 0x52;
pub const CEC_OP_UI_CMD_ELECTRONIC_PROGRAM_GUIDE: u32 = 0x53;
pub const CEC_OP_UI_CMD_TIMER_PROGRAMMING: u32 = 0x54;
pub const CEC_OP_UI_CMD_INITIAL_CONFIGURATION: u32 = 0x55;
pub const CEC_OP_UI_CMD_SELECT_BROADCAST_TYPE: u32 = 0x56;
pub const CEC_OP_UI_CMD_SELECT_SOUND_PRESENTATION: u32 = 0x57;
pub const CEC_OP_UI_CMD_AUDIO_DESCRIPTION: u32 = 0x58;
pub const CEC_OP_UI_CMD_INTERNET: u32 = 0x59;
pub const CEC_OP_UI_CMD_3D_MODE: u32 = 0x5a;
pub const CEC_OP_UI_CMD_PLAY_FUNCTION: u32 = 0x60;
pub const CEC_OP_UI_CMD_PAUSE_PLAY_FUNCTION: u32 = 0x61;
pub const CEC_OP_UI_CMD_RECORD_FUNCTION: u32 = 0x62;
pub const CEC_OP_UI_CMD_PAUSE_RECORD_FUNCTION: u32 = 0x63;
pub const CEC_OP_UI_CMD_STOP_FUNCTION: u32 = 0x64;
pub const CEC_OP_UI_CMD_MUTE_FUNCTION: u32 = 0x65;
pub const CEC_OP_UI_CMD_RESTORE_VOLUME_FUNCTION: u32 = 0x66;
pub const CEC_OP_UI_CMD_TUNE_FUNCTION: u32 = 0x67;
pub const CEC_OP_UI_CMD_SELECT_MEDIA_FUNCTION: u32 = 0x68;
pub const CEC_OP_UI_CMD_SELECT_AV_INPUT_FUNCTION: u32 = 0x69;
pub const CEC_OP_UI_CMD_SELECT_AUDIO_INPUT_FUNCTION: u32 = 0x6a;
pub const CEC_OP_UI_CMD_POWER_TOGGLE_FUNCTION: u32 = 0x6b;
pub const CEC_OP_UI_CMD_POWER_OFF_FUNCTION: u32 = 0x6c;
pub const CEC_OP_UI_CMD_POWER_ON_FUNCTION: u32 = 0x6d;
pub const CEC_OP_UI_CMD_F1_BLUE: u32 = 0x71;
pub const CEC_OP_UI_CMD_F2_RED: u32 = 0x72;
pub const CEC_OP_UI_CMD_F3_GREEN: u32 = 0x73;
pub const CEC_OP_UI_CMD_F4_YELLOW: u32 = 0x74;
pub const CEC_OP_UI_CMD_F5: u32 = 0x75;
pub const CEC_OP_UI_CMD_DATA: u32 = 0x76;
/* UI Broadcast Type Operand (ui_bcast_type) */
pub const CEC_OP_UI_BCAST_TYPE_TOGGLE_ALL: u32 = 0x00;
pub const CEC_OP_UI_BCAST_TYPE_TOGGLE_DIG_ANA: u32 = 0x01;
pub const CEC_OP_UI_BCAST_TYPE_ANALOGUE: u32 = 0x10;
pub const CEC_OP_UI_BCAST_TYPE_ANALOGUE_T: u32 = 0x20;
pub const CEC_OP_UI_BCAST_TYPE_ANALOGUE_CABLE: u32 = 0x30;
pub const CEC_OP_UI_BCAST_TYPE_ANALOGUE_SAT: u32 = 0x40;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL: u32 = 0x50;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL_T: u32 = 0x60;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL_CABLE: u32 = 0x70;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL_SAT: u32 = 0x80;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL_COM_SAT: u32 = 0x90;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL_COM_SAT2: u32 = 0x91;
pub const CEC_OP_UI_BCAST_TYPE_IP: u32 = 0xa0;
/* UI Sound Presentation Control Operand (ui_snd_pres_ctl) */
pub const CEC_OP_UI_SND_PRES_CTL_DUAL_MONO: u32 = 0x10;
pub const CEC_OP_UI_SND_PRES_CTL_KARAOKE: u32 = 0x20;
pub const CEC_OP_UI_SND_PRES_CTL_DOWNMIX: u32 = 0x80;
pub const CEC_OP_UI_SND_PRES_CTL_REVERB: u32 = 0x90;
pub const CEC_OP_UI_SND_PRES_CTL_EQUALIZER: u32 = 0xa0;
pub const CEC_OP_UI_SND_PRES_CTL_BASS_UP: u32 = 0xb1;
pub const CEC_OP_UI_SND_PRES_CTL_BASS_NEUTRAL: u32 = 0xb2;
pub const CEC_OP_UI_SND_PRES_CTL_BASS_DOWN: u32 = 0xb3;
pub const CEC_OP_UI_SND_PRES_CTL_TREBLE_UP: u32 = 0xc1;
pub const CEC_OP_UI_SND_PRES_CTL_TREBLE_NEUTRAL: u32 = 0xc2;
pub const CEC_OP_UI_SND_PRES_CTL_TREBLE_DOWN: u32 = 0xc3;

pub const CEC_MSG_USER_CONTROL_RELEASED: u32 = 0x45;


/* Remote Control Passthrough Feature */

/*
 * Has also:
 *	CEC_MSG_USER_CONTROL_PRESSED
 *	CEC_MSG_USER_CONTROL_RELEASED
 */


/* Power Status Feature */
pub const CEC_MSG_GIVE_DEVICE_POWER_STATUS: u32 = 0x8f;
pub const CEC_MSG_REPORT_POWER_STATUS: u32 = 0x90;
/* Power Status Operand (pwr_state) */
pub const CEC_OP_POWER_STATUS_ON: u32 = 0;
pub const CEC_OP_POWER_STATUS_STANDBY: u32 = 1;
pub const CEC_OP_POWER_STATUS_TO_ON: u32 = 2;
pub const CEC_OP_POWER_STATUS_TO_STANDBY: u32 = 3;


/* General Protocol Messages */
pub const CEC_MSG_FEATURE_ABORT: u32 = 0x00;
/* Abort Reason Operand (reason) */
pub const CEC_OP_ABORT_UNRECOGNIZED_OP: u32 = 0;
pub const CEC_OP_ABORT_INCORRECT_MODE: u32 = 1;
pub const CEC_OP_ABORT_NO_SOURCE: u32 = 2;
pub const CEC_OP_ABORT_INVALID_OP: u32 = 3;
pub const CEC_OP_ABORT_REFUSED: u32 = 4;
pub const CEC_OP_ABORT_UNDETERMINED: u32 = 5;

pub const CEC_MSG_ABORT: u32 = 0xff;


/* System Audio Control Feature */

/*
 * Has also:
 *	CEC_MSG_USER_CONTROL_PRESSED
 *	CEC_MSG_USER_CONTROL_RELEASED
 */
pub const CEC_MSG_GIVE_AUDIO_STATUS: u32 = 0x71;
pub const CEC_MSG_GIVE_SYSTEM_AUDIO_MODE_STATUS: u32 = 0x7d;
pub const CEC_MSG_REPORT_AUDIO_STATUS: u32 = 0x7a;
/* Audio Mute Status Operand (aud_mute_status) */
pub const CEC_OP_AUD_MUTE_STATUS_OFF: u32 = 0;
pub const CEC_OP_AUD_MUTE_STATUS_ON: u32 = 1;

pub const CEC_MSG_REPORT_SHORT_AUDIO_DESCRIPTOR: u32 = 0xa3;
pub const CEC_MSG_REQUEST_SHORT_AUDIO_DESCRIPTOR: u32 = 0xa4;
pub const CEC_MSG_SET_SYSTEM_AUDIO_MODE: u32 = 0x72;
/* System Audio Status Operand (sys_aud_status) */
pub const CEC_OP_SYS_AUD_STATUS_OFF: u32 = 0;
pub const CEC_OP_SYS_AUD_STATUS_ON: u32 = 1;

pub const CEC_MSG_SYSTEM_AUDIO_MODE_REQUEST: u32 = 0x70;
pub const CEC_MSG_SYSTEM_AUDIO_MODE_STATUS: u32 = 0x7e;
/* Audio Format ID Operand (audio_format_id) */
pub const CEC_OP_AUD_FMT_ID_CEA861: u32 = 0;
pub const CEC_OP_AUD_FMT_ID_CEA861_CXT: u32 = 1;

pub const CEC_MSG_SET_AUDIO_VOLUME_LEVEL: u32 = 0x73	/* CEC 2.0 */;

/* Audio Rate Control Feature */
pub const CEC_MSG_SET_AUDIO_RATE: u32 = 0x9a;
/* Audio Rate Operand (audio_rate) */
pub const CEC_OP_AUD_RATE_OFF: u32 = 0;
pub const CEC_OP_AUD_RATE_WIDE_STD: u32 = 1;
pub const CEC_OP_AUD_RATE_WIDE_FAST: u32 = 2;
pub const CEC_OP_AUD_RATE_WIDE_SLOW: u32 = 3;
pub const CEC_OP_AUD_RATE_NARROW_STD: u32 = 4;
pub const CEC_OP_AUD_RATE_NARROW_FAST: u32 = 5;
pub const CEC_OP_AUD_RATE_NARROW_SLOW: u32 = 6;


/* Audio Return Channel Control Feature */
pub const CEC_MSG_INITIATE_ARC: u32 = 0xc0;
pub const CEC_MSG_REPORT_ARC_INITIATED: u32 = 0xc1;
pub const CEC_MSG_REPORT_ARC_TERMINATED: u32 = 0xc2;
pub const CEC_MSG_REQUEST_ARC_INITIATION: u32 = 0xc3;
pub const CEC_MSG_REQUEST_ARC_TERMINATION: u32 = 0xc4;
pub const CEC_MSG_TERMINATE_ARC: u32 = 0xc5;


/* Dynamic Audio Lipsync Feature */
pub const CEC_MSG_REQUEST_CURRENT_LATENCY: u32 = 0xa7;
pub const CEC_MSG_REPORT_CURRENT_LATENCY: u32 = 0xa8;
/* Low Latency Mode Operand (low_latency_mode) */
pub const CEC_OP_LOW_LATENCY_MODE_OFF: u32 = 0;
pub const CEC_OP_LOW_LATENCY_MODE_ON: u32 = 1;
/* Audio Output Compensated Operand (audio_out_compensated) */
pub const CEC_OP_AUD_OUT_COMPENSATED_NA: u32 = 0;
pub const CEC_OP_AUD_OUT_COMPENSATED_DELAY: u32 = 1;
pub const CEC_OP_AUD_OUT_COMPENSATED_NO_DELAY: u32 = 2;
pub const CEC_OP_AUD_OUT_COMPENSATED_PARTIAL_DELAY: u32 = 3;


/* Latency Indication Protocol Feature */
pub const CEC_MSG_REQUEST_LIP_SUPPORT: u32 = 0x50	/* CEC 2.0 */;
pub const CEC_MSG_REPORT_LIP_SUPPORT: u32 = 0x51	/* CEC 2.0 */;
pub const CEC_MSG_REQUEST_AUDIO_AND_VIDEO_LATENCY: u32 = 0x52	/* CEC 2.0 */;
/* HDR Format Operand (hdr_format) */
pub const CEC_OP_HDR_FORMAT_GAMMA_SDR: u32 = 0;
pub const CEC_OP_HDR_FORMAT_GAMMA_HDR: u32 = 1;
pub const CEC_OP_HDR_FORMAT_PQ: u32 = 2;
pub const CEC_OP_HDR_FORMAT_HLG: u32 = 3;
pub const CEC_OP_HDR_FORMAT_DYNAMIC_HDR_TYPE_1: u32 = 8;
pub const CEC_OP_HDR_FORMAT_DYNAMIC_HDR_TYPE_2: u32 = 9;
pub const CEC_OP_HDR_FORMAT_DYNAMIC_HDR_TYPE_4: u32 = 11;
pub const CEC_OP_HDR_FORMAT_DV_SINK_LED: u32 = 16;
pub const CEC_OP_HDR_FORMAT_DV_SOURCE_LED: u32 = 17;
pub const CEC_OP_HDR_FORMAT_HDR10PLUS: u32 = 24;
pub const CEC_OP_HDR_FORMAT_ETSI_TS_103_433: u32 = 32;
pub const CEC_MSG_REPORT_AUDIO_AND_VIDEO_LATENCY: u32 = 0x53	/* CEC 2.0 */;
pub const CEC_MSG_REQUEST_AUDIO_LATENCY: u32 = 0x54	/* CEC 2.0 */;
pub const CEC_MSG_REPORT_AUDIO_LATENCY: u32 = 0x55	/* CEC 2.0 */;
pub const CEC_MSG_REQUEST_VIDEO_LATENCY: u32 = 0x56	/* CEC 2.0 */;
pub const CEC_MSG_REPORT_VIDEO_LATENCY: u32 = 0x57	/* CEC 2.0 */;
pub const CEC_MSG_UPDATE_SQID: u32 = 0x58	/* CEC 2.0 */;


/* Capability Discovery and Control Feature */
pub const CEC_MSG_CDC_MESSAGE: u32 = 0xf8;
/* Ethernet-over-HDMI: nobody ever does this... */
pub const CEC_MSG_CDC_HEC_INQUIRE_STATE: u32 = 0x00;
pub const CEC_MSG_CDC_HEC_REPORT_STATE: u32 = 0x01;
/* HEC Functionality State Operand (hec_func_state) */
pub const CEC_OP_HEC_FUNC_STATE_NOT_SUPPORTED: u32 = 0;
pub const CEC_OP_HEC_FUNC_STATE_INACTIVE: u32 = 1;
pub const CEC_OP_HEC_FUNC_STATE_ACTIVE: u32 = 2;
pub const CEC_OP_HEC_FUNC_STATE_ACTIVATION_FIELD: u32 = 3;
/* Host Functionality State Operand (host_func_state) */
pub const CEC_OP_HOST_FUNC_STATE_NOT_SUPPORTED: u32 = 0;
pub const CEC_OP_HOST_FUNC_STATE_INACTIVE: u32 = 1;
pub const CEC_OP_HOST_FUNC_STATE_ACTIVE: u32 = 2;
/* ENC Functionality State Operand (enc_func_state) */
pub const CEC_OP_ENC_FUNC_STATE_EXT_CON_NOT_SUPPORTED: u32 = 0;
pub const CEC_OP_ENC_FUNC_STATE_EXT_CON_INACTIVE: u32 = 1;
pub const CEC_OP_ENC_FUNC_STATE_EXT_CON_ACTIVE: u32 = 2;
/* CDC Error Code Operand (cdc_errcode) */
pub const CEC_OP_CDC_ERROR_CODE_NONE: u32 = 0;
pub const CEC_OP_CDC_ERROR_CODE_CAP_UNSUPPORTED: u32 = 1;
pub const CEC_OP_CDC_ERROR_CODE_WRONG_STATE: u32 = 2;
pub const CEC_OP_CDC_ERROR_CODE_OTHER: u32 = 3;
/* HEC Support Operand (hec_support) */
pub const CEC_OP_HEC_SUPPORT_NO: u32 = 0;
pub const CEC_OP_HEC_SUPPORT_YES: u32 = 1;
/* HEC Activation Operand (hec_activation) */
pub const CEC_OP_HEC_ACTIVATION_ON: u32 = 0;
pub const CEC_OP_HEC_ACTIVATION_OFF: u32 = 1;

pub const CEC_MSG_CDC_HEC_SET_STATE_ADJACENT: u32 = 0x02;
pub const CEC_MSG_CDC_HEC_SET_STATE: u32 = 0x03;
/* HEC Set State Operand (hec_set_state) */
pub const CEC_OP_HEC_SET_STATE_DEACTIVATE: u32 = 0;
pub const CEC_OP_HEC_SET_STATE_ACTIVATE: u32 = 1;

pub const CEC_MSG_CDC_HEC_REQUEST_DEACTIVATION: u32 = 0x04;
pub const CEC_MSG_CDC_HEC_NOTIFY_ALIVE: u32 = 0x05;
pub const CEC_MSG_CDC_HEC_DISCOVER: u32 = 0x06;
/* Hotplug Detect messages */
pub const CEC_MSG_CDC_HPD_SET_STATE: u32 = 0x10;
/* HPD State Operand (hpd_state) */
pub const CEC_OP_HPD_STATE_CP_EDID_DISABLE: u32 = 0;
pub const CEC_OP_HPD_STATE_CP_EDID_ENABLE: u32 = 1;
pub const CEC_OP_HPD_STATE_CP_EDID_DISABLE_ENABLE: u32 = 2;
pub const CEC_OP_HPD_STATE_EDID_DISABLE: u32 = 3;
pub const CEC_OP_HPD_STATE_EDID_ENABLE: u32 = 4;
pub const CEC_OP_HPD_STATE_EDID_DISABLE_ENABLE: u32 = 5;
pub const CEC_MSG_CDC_HPD_REPORT_STATE: u32 = 0x11;
/* HPD Error Code Operand (hpd_error) */
pub const CEC_OP_HPD_ERROR_NONE: u32 = 0;
pub const CEC_OP_HPD_ERROR_INITIATOR_NOT_CAPABLE: u32 = 1;
pub const CEC_OP_HPD_ERROR_INITIATOR_WRONG_STATE: u32 = 2;
pub const CEC_OP_HPD_ERROR_OTHER: u32 = 3;
pub const CEC_OP_HPD_ERROR_NONE_NO_VIDEO: u32 = 4;

/* End of Messages */

/* Helper functions to identify the 'special' CEC devices */

pub unsafe fn int cec_is_2nd_tv(const pub struct cec_log_addrs (*las))
{
	/*
	 * It is a second TV if the logical address is 14 or 15 and the
	 * primary device type is a TV.
	 */
	return (*las).num_log_addrs &&
	       (*las).log_addr[0] >= CEC_LOG_ADDR_SPECIFIC &&
	       (*las).primary_device_type[0] == CEC_OP_PRIM_DEVTYPE_TV;
}

pub unsafe fn int cec_is_processor(const pub struct cec_log_addrs (*las))
{
	/*
	 * It is a processor if the logical address is 12-15 and the
	 * primary device type is a Processor.
	 */
	return (*las).num_log_addrs &&
	       (*las).log_addr[0] >= CEC_LOG_ADDR_BACKUP_1 &&
	       (*las).primary_device_type[0] == CEC_OP_PRIM_DEVTYPE_PROCESSOR;
}

pub unsafe fn int cec_is_switch(const pub struct cec_log_addrs (*las))
{
	/*
	 * It is a switch if the logical address is 15 and the
	 * primary device type is a Switch and the CDC-Only flag is not set.
	 */
	return (*las).num_log_addrs == 1 &&
	       (*las).log_addr[0] == CEC_LOG_ADDR_UNREGISTERED &&
	       (*las).primary_device_type[0] == CEC_OP_PRIM_DEVTYPE_SWITCH &&
	       !((*las).flags & CEC_LOG_ADDRS_FL_CDC_ONLY);
}

pub unsafe fn int cec_is_cdc_only(const pub struct cec_log_addrs (*las))
{
	/*
	 * It is a CDC-only device if the logical address is 15 and the
	 * primary device type is a Switch and the CDC-Only flag is set.
	 */
	return (*las).num_log_addrs == 1 &&
	       (*las).log_addr[0] == CEC_LOG_ADDR_UNREGISTERED &&
	       (*las).primary_device_type[0] == CEC_OP_PRIM_DEVTYPE_SWITCH &&
	       ((*las).flags & CEC_LOG_ADDRS_FL_CDC_ONLY);
}
// End of header

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
