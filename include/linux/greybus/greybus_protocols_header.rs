# # /* SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause) */
# /*
#  * Copyright(c) 2014 - 2015 Google Inc. All rights reserved.
#  * Copyright(c) 2014 - 2015 Linaro Ltd. All rights reserved.
#  */
# 
# #ifndef __GREYBUS_PROTOCOLS_H
# #define __GREYBUS_PROTOCOLS_H
# 
# 
# 
# /* Fixed IDs for control/svc protocols */
# 
# /* SVC switch-port device ids */
# #define GB_SVC_DEVICE_ID_SVC			0
# #define GB_SVC_DEVICE_ID_AP			1
# #define GB_SVC_DEVICE_ID_MIN			2
# #define GB_SVC_DEVICE_ID_MAX			31
# 
# #define GB_SVC_CPORT_ID				0
# #define GB_CONTROL_BUNDLE_ID			0
# #define GB_CONTROL_CPORT_ID			0
# 
# 
# /*
#  * All operation messages (both requests and responses) begin with
#  * a header that encodes the size of the message (header included).
#  * This header also contains a unique identifier, that associates a
#  * response message with its operation.  The header contains an
#  * operation type field, whose interpretation is dependent on what
#  * type of protocol is used over the connection.  The high bit
#  * (0x80) of the operation type field is used to indicate whether
#  * the message is a request (clear) or a response (set).
#  *
#  * Response messages include an additional result byte, which
#  * communicates the result of the corresponding request.  A zero
#  * result value means the operation completed successfully.  Any
#  * other value indicates an error; in this case, the payload of the
#  * response message (if any) is ignored.  The result byte must be
#  * zero in the header for a request message.
#  *
#  * The wire format for all numeric fields in the header is little
#  * endian.  Any operation-specific data begins immediately after the
#  * header.
#  */
# #[repr(C, packed)]
# pub gb_operation_msg_hdr {
# 	u16	size;		/* Size in bytes of header + payload */
# 	u16	operation_id;	/* Operation unique id */
# 	u8	type;		/* E.g GB_I2C_TYPE_* or GB_GPIO_TYPE_* */
# 	u8	result;		/* Result of request (in responses only) */
# 	u8	pad: [u8; 2];		/* must be zero (ignore when read) */
# }
# 
# 
# /* Generic request types */
# #define GB_REQUEST_TYPE_CPORT_SHUTDOWN		0x00
# #define GB_REQUEST_TYPE_INVALID			0x7f
# 
# #[repr(C, packed)]
# pub gb_cport_shutdown_request {
# 	u8 phase;
# }
# 
# 
# /* Control Protocol */
# 
# /* Greybus control request types */
# #define GB_CONTROL_TYPE_VERSION			0x01
# #define GB_CONTROL_TYPE_PROBE_AP		0x02
# #define GB_CONTROL_TYPE_GET_MANIFEST_SIZE	0x03
# #define GB_CONTROL_TYPE_GET_MANIFEST		0x04
# #define GB_CONTROL_TYPE_CONNECTED		0x05
# #define GB_CONTROL_TYPE_DISCONNECTED		0x06
# #define GB_CONTROL_TYPE_TIMESYNC_ENABLE		0x07
# #define GB_CONTROL_TYPE_TIMESYNC_DISABLE	0x08
# #define GB_CONTROL_TYPE_TIMESYNC_AUTHORITATIVE	0x09
# /*	Unused					0x0a */
# #define GB_CONTROL_TYPE_BUNDLE_VERSION		0x0b
# #define GB_CONTROL_TYPE_DISCONNECTING		0x0c
# #define GB_CONTROL_TYPE_TIMESYNC_GET_LAST_EVENT	0x0d
# #define GB_CONTROL_TYPE_MODE_SWITCH		0x0e
# #define GB_CONTROL_TYPE_BUNDLE_SUSPEND		0x0f
# #define GB_CONTROL_TYPE_BUNDLE_RESUME		0x10
# #define GB_CONTROL_TYPE_BUNDLE_DEACTIVATE	0x11
# #define GB_CONTROL_TYPE_BUNDLE_ACTIVATE		0x12
# #define GB_CONTROL_TYPE_INTF_SUSPEND_PREPARE		0x13
# #define GB_CONTROL_TYPE_INTF_DEACTIVATE_PREPARE	0x14
# #define GB_CONTROL_TYPE_INTF_HIBERNATE_ABORT	0x15
# 
# #[repr(C, packed)]
# pub gb_control_version_request {
# 	u8	major;
# 	u8	minor;
# }
# 
# #[repr(C, packed)]
# pub gb_control_version_response {
# 	u8	major;
# 	u8	minor;
# }
# 
# #[repr(C, packed)]
# pub gb_control_bundle_version_request {
# 	u8	bundle_id;
# }
# 
# #[repr(C, packed)]
# pub gb_control_bundle_version_response {
# 	u8	major;
# 	u8	minor;
# }
# 
# /* Control protocol manifest get size request has no payload*/
# #[repr(C, packed)]
# pub gb_control_get_manifest_size_response {
# 	u16			size;
# }
# 
# /* Control protocol manifest get request has no payload */
# #[repr(C, packed)]
# pub gb_control_get_manifest_response {
# 	u8			data: [u8; 0];
# }
# 
# /* Control protocol [dis]connected request */
# #[repr(C, packed)]
# pub gb_control_connected_request {
# 	u16			cport_id;
# }
# 
# #[repr(C, packed)]
# pub gb_control_disconnecting_request {
# 	u16			cport_id;
# }
# /* disconnecting response has no payload */
# 
# #[repr(C, packed)]
# pub gb_control_disconnected_request {
# 	u16			cport_id;
# }
# /* Control protocol [dis]connected response has no payload */
# 
# /*
#  * All Bundle power management operations use the same request and response
#  * layout and status codes.
#  */
# 
# #define GB_CONTROL_BUNDLE_PM_OK		0x00
# #define GB_CONTROL_BUNDLE_PM_INVAL	0x01
# #define GB_CONTROL_BUNDLE_PM_BUSY	0x02
# #define GB_CONTROL_BUNDLE_PM_FAIL	0x03
# #define GB_CONTROL_BUNDLE_PM_NA		0x04
# 
# #[repr(C, packed)]
# pub gb_control_bundle_pm_request {
# 	u8	bundle_id;
# }
# 
# #[repr(C, packed)]
# pub gb_control_bundle_pm_response {
# 	u8	status;
# }
# 
# /*
#  * Interface Suspend Prepare and Deactivate Prepare operations use the same
#  * response layout and error codes. Define a single response structure and reuse
#  * it. Both operations have no payload.
#  */
# 
# #define GB_CONTROL_INTF_PM_OK		0x00
# #define GB_CONTROL_INTF_PM_BUSY		0x01
# #define GB_CONTROL_INTF_PM_NA		0x02
# 
# #[repr(C, packed)]
# pub gb_control_intf_pm_response {
# 	u8	status;
# }
# 
# /* APBridge protocol */
# 
# /* request APB1 log */
# #define GB_APB_REQUEST_LOG			0x02
# 
# /* request to map a cport to bulk in and bulk out endpoints */
# #define GB_APB_REQUEST_EP_MAPPING		0x03
# 
# /* request to get the number of cports available */
# #define GB_APB_REQUEST_CPORT_COUNT		0x04
# 
# /* request to reset a cport state */
# #define GB_APB_REQUEST_RESET_CPORT		0x05
# 
# /* request to time the latency of messages on a given cport */
# #define GB_APB_REQUEST_LATENCY_TAG_EN		0x06
# #define GB_APB_REQUEST_LATENCY_TAG_DIS		0x07
# 
# /* request to control the CSI transmitter */
# #define GB_APB_REQUEST_CSI_TX_CONTROL		0x08
# 
# /* request to control audio streaming */
# #define GB_APB_REQUEST_AUDIO_CONTROL		0x09
# 
# /* TimeSync requests */
# #define GB_APB_REQUEST_TIMESYNC_ENABLE		0x0d
# #define GB_APB_REQUEST_TIMESYNC_DISABLE		0x0e
# #define GB_APB_REQUEST_TIMESYNC_AUTHORITATIVE	0x0f
# #define GB_APB_REQUEST_TIMESYNC_GET_LAST_EVENT	0x10
# 
# /* requests to set Greybus CPort flags */
# #define GB_APB_REQUEST_CPORT_FLAGS		0x11
# 
# /* ARPC request */
# #define GB_APB_REQUEST_ARPC_RUN			0x12
# 
# #[repr(C, packed)]
# pub gb_apb_request_cport_flags {
# 	u32	flags;
# #define GB_APB_CPORT_FLAG_CONTROL		0x01
# #define GB_APB_CPORT_FLAG_HIGH_PRIO		0x02
# }
# 
# 
# /* Firmware Download Protocol */
# 
# /* Request Types */
# #define GB_FW_DOWNLOAD_TYPE_FIND_FIRMWARE	0x01
# #define GB_FW_DOWNLOAD_TYPE_FETCH_FIRMWARE	0x02
# #define GB_FW_DOWNLOAD_TYPE_RELEASE_FIRMWARE	0x03
# 
# #define GB_FIRMWARE_TAG_MAX_SIZE		10
# 
# /* firmware download find firmware request/response */
# #[repr(C, packed)]
# pub gb_fw_download_find_firmware_request {
# 	u8			firmware_tag: [u8; GB_FIRMWARE_TAG_MAX_SIZE as usize];
# }
# 
# #[repr(C, packed)]
# pub gb_fw_download_find_firmware_response {
# 	u8			firmware_id;
# 	u32			size;
# }
# 
# /* firmware download fetch firmware request/response */
# #[repr(C, packed)]
# pub gb_fw_download_fetch_firmware_request {
# 	u8			firmware_id;
# 	u32			offset;
# 	u32			size;
# }
# 
# /* gb_fw_download_fetch_firmware_response contains no other data */
# 
# /* firmware download release firmware request */
# #[repr(C, packed)]
# pub gb_fw_download_release_firmware_request {
# 	u8			firmware_id;
# }
# /* firmware download release firmware response has no payload */
# 
# 
# /* Firmware Management Protocol */
# 
# /* Request Types */
# #define GB_FW_MGMT_TYPE_INTERFACE_FW_VERSION	0x01
# #define GB_FW_MGMT_TYPE_LOAD_AND_VALIDATE_FW	0x02
# #define GB_FW_MGMT_TYPE_LOADED_FW		0x03
# #define GB_FW_MGMT_TYPE_BACKEND_FW_VERSION	0x04
# #define GB_FW_MGMT_TYPE_BACKEND_FW_UPDATE	0x05
# #define GB_FW_MGMT_TYPE_BACKEND_FW_UPDATED	0x06
# 
# #define GB_FW_LOAD_METHOD_UNIPRO		0x01
# #define GB_FW_LOAD_METHOD_INTERNAL		0x02
# 
# #define GB_FW_LOAD_STATUS_FAILED		0x00
# #define GB_FW_LOAD_STATUS_UNVALIDATED		0x01
# #define GB_FW_LOAD_STATUS_VALIDATED		0x02
# #define GB_FW_LOAD_STATUS_VALIDATION_FAILED	0x03
# 
# #define GB_FW_BACKEND_FW_STATUS_SUCCESS		0x01
# #define GB_FW_BACKEND_FW_STATUS_FAIL_FIND	0x02
# #define GB_FW_BACKEND_FW_STATUS_FAIL_FETCH	0x03
# #define GB_FW_BACKEND_FW_STATUS_FAIL_WRITE	0x04
# #define GB_FW_BACKEND_FW_STATUS_INT		0x05
# #define GB_FW_BACKEND_FW_STATUS_RETRY		0x06
# #define GB_FW_BACKEND_FW_STATUS_NOT_SUPPORTED	0x07
# 
# #define GB_FW_BACKEND_VERSION_STATUS_SUCCESS		0x01
# #define GB_FW_BACKEND_VERSION_STATUS_NOT_AVAILABLE	0x02
# #define GB_FW_BACKEND_VERSION_STATUS_NOT_SUPPORTED	0x03
# #define GB_FW_BACKEND_VERSION_STATUS_RETRY		0x04
# #define GB_FW_BACKEND_VERSION_STATUS_FAIL_INT		0x05
# 
# /* firmware management interface firmware version request has no payload */
# #[repr(C, packed)]
# pub gb_fw_mgmt_interface_fw_version_response {
# 	u8			firmware_tag: [u8; GB_FIRMWARE_TAG_MAX_SIZE as usize];
# 	u16			major;
# 	u16			minor;
# }
# 
# /* firmware management load and validate firmware request/response */
# #[repr(C, packed)]
# pub gb_fw_mgmt_load_and_validate_fw_request {
# 	u8			request_id;
# 	u8			load_method;
# 	u8			firmware_tag: [u8; GB_FIRMWARE_TAG_MAX_SIZE as usize];
# }
# /* firmware management load and validate firmware response has no payload*/
# 
# /* firmware management loaded firmware request */
# #[repr(C, packed)]
# pub gb_fw_mgmt_loaded_fw_request {
# 	u8			request_id;
# 	u8			status;
# 	u16			major;
# 	u16			minor;
# }
# /* firmware management loaded firmware response has no payload */
# 
# /* firmware management backend firmware version request/response */
# #[repr(C, packed)]
# pub gb_fw_mgmt_backend_fw_version_request {
# 	u8			firmware_tag: [u8; GB_FIRMWARE_TAG_MAX_SIZE as usize];
# }
# 
# #[repr(C, packed)]
# pub gb_fw_mgmt_backend_fw_version_response {
# 	u16			major;
# 	u16			minor;
# 	u8			status;
# }
# 
# /* firmware management backend firmware update request */
# #[repr(C, packed)]
# pub gb_fw_mgmt_backend_fw_update_request {
# 	u8			request_id;
# 	u8			firmware_tag: [u8; GB_FIRMWARE_TAG_MAX_SIZE as usize];
# }
# /* firmware management backend firmware update response has no payload */
# 
# /* firmware management backend firmware updated request */
# #[repr(C, packed)]
# pub gb_fw_mgmt_backend_fw_updated_request {
# 	u8			request_id;
# 	u8			status;
# }
# /* firmware management backend firmware updated response has no payload */
# 
# 
# /* Component Authentication Protocol (CAP) */
# 
# /* Request Types */
# #define GB_CAP_TYPE_GET_ENDPOINT_UID	0x01
# #define GB_CAP_TYPE_GET_IMS_CERTIFICATE	0x02
# #define GB_CAP_TYPE_AUTHENTICATE	0x03
# 
# /* CAP get endpoint uid request has no payload */
# #[repr(C, packed)]
# pub gb_cap_get_endpoint_uid_response {
# 	u8			uid: [u8; 8];
# }
# 
# /* CAP get endpoint ims certificate request/response */
# #[repr(C, packed)]
# pub gb_cap_get_ims_certificate_request {
# 	u32			certificate_class;
# 	u32			certificate_id;
# }
# 
# #[repr(C, packed)]
# pub gb_cap_get_ims_certificate_response {
# 	u8			result_code;
# 	u8			certificate: [u8; 0];
# }
# 
# /* CAP authenticate request/response */
# #[repr(C, packed)]
# pub gb_cap_authenticate_request {
# 	u32			auth_type;
# 	u8			uid: [u8; 8];
# 	u8			challenge: [u8; 32];
# }
# 
# #[repr(C, packed)]
# pub gb_cap_authenticate_response {
# 	u8			result_code;
# 	u8			response: [u8; 64];
# 	u8			signature: [u8; 0];
# }
# 
# 
# /* Bootrom Protocol */
# 
# /* Version of the Greybus bootrom protocol we support */
# #define GB_BOOTROM_VERSION_MAJOR		0x00
# #define GB_BOOTROM_VERSION_MINOR		0x01
# 
# /* Greybus bootrom request types */
# #define GB_BOOTROM_TYPE_VERSION			0x01
# #define GB_BOOTROM_TYPE_FIRMWARE_SIZE		0x02
# #define GB_BOOTROM_TYPE_GET_FIRMWARE		0x03
# #define GB_BOOTROM_TYPE_READY_TO_BOOT		0x04
# #define GB_BOOTROM_TYPE_AP_READY		0x05	/* Request with no-payload */
# #define GB_BOOTROM_TYPE_GET_VID_PID		0x06	/* Request with no-payload */
# 
# /* Greybus bootrom boot stages */
# #define GB_BOOTROM_BOOT_STAGE_ONE		0x01 /* Reserved for the boot ROM */
# #define GB_BOOTROM_BOOT_STAGE_TWO		0x02 /* Bootrom package to be loaded by the boot ROM */
# #define GB_BOOTROM_BOOT_STAGE_THREE		0x03 /* Module personality package loaded by Stage 2 firmware */
# 
# /* Greybus bootrom ready to boot status */
# #define GB_BOOTROM_BOOT_STATUS_INVALID		0x00 /* Firmware blob could not be validated */
# #define GB_BOOTROM_BOOT_STATUS_INSECURE		0x01 /* Firmware blob is valid but insecure */
# #define GB_BOOTROM_BOOT_STATUS_SECURE		0x02 /* Firmware blob is valid and secure */
# 
# /* Max bootrom data fetch size in bytes */
# #define GB_BOOTROM_FETCH_MAX			2000
# 
# #[repr(C, packed)]
# pub gb_bootrom_version_request {
# 	u8	major;
# 	u8	minor;
# }
# 
# #[repr(C, packed)]
# pub gb_bootrom_version_response {
# 	u8	major;
# 	u8	minor;
# }
# 
# /* Bootrom protocol firmware size request/response */
# #[repr(C, packed)]
# pub gb_bootrom_firmware_size_request {
# 	u8			stage;
# }
# 
# #[repr(C, packed)]
# pub gb_bootrom_firmware_size_response {
# 	u32			size;
# }
# 
# /* Bootrom protocol get firmware request/response */
# #[repr(C, packed)]
# pub gb_bootrom_get_firmware_request {
# 	u32			offset;
# 	u32			size;
# }
# 
# /* gb_bootrom_get_firmware_response contains no other data */
# 
# /* Bootrom protocol Ready to boot request */
# #[repr(C, packed)]
# pub gb_bootrom_ready_to_boot_request {
# 	u8			status;
# }
# /* Bootrom protocol Ready to boot response has no payload */
# 
# /* Bootrom protocol get VID/PID request has no payload */
# #[repr(C, packed)]
# pub gb_bootrom_get_vid_pid_response {
# 	u32			vendor_id;
# 	u32			product_id;
# }
# 
# 
# /* Power Supply */
# 
# /* Greybus power supply request types */
# #define GB_POWER_SUPPLY_TYPE_GET_SUPPLIES		0x02
# #define GB_POWER_SUPPLY_TYPE_GET_DESCRIPTION		0x03
# #define GB_POWER_SUPPLY_TYPE_GET_PROP_DESCRIPTORS	0x04
# #define GB_POWER_SUPPLY_TYPE_GET_PROPERTY		0x05
# #define GB_POWER_SUPPLY_TYPE_SET_PROPERTY		0x06
# #define GB_POWER_SUPPLY_TYPE_EVENT			0x07
# 
# /* Greybus power supply battery technologies types */
# #define GB_POWER_SUPPLY_TECH_UNKNOWN			0x0000
# #define GB_POWER_SUPPLY_TECH_NiMH			0x0001
# #define GB_POWER_SUPPLY_TECH_LION			0x0002
# #define GB_POWER_SUPPLY_TECH_LIPO			0x0003
# #define GB_POWER_SUPPLY_TECH_LiFe			0x0004
# #define GB_POWER_SUPPLY_TECH_NiCd			0x0005
# #define GB_POWER_SUPPLY_TECH_LiMn			0x0006
# 
# /* Greybus power supply types */
# #define GB_POWER_SUPPLY_UNKNOWN_TYPE			0x0000
# #define GB_POWER_SUPPLY_BATTERY_TYPE			0x0001
# #define GB_POWER_SUPPLY_UPS_TYPE			0x0002
# #define GB_POWER_SUPPLY_MAINS_TYPE			0x0003
# #define GB_POWER_SUPPLY_USB_TYPE			0x0004
# #define GB_POWER_SUPPLY_USB_DCP_TYPE			0x0005
# #define GB_POWER_SUPPLY_USB_CDP_TYPE			0x0006
# #define GB_POWER_SUPPLY_USB_ACA_TYPE			0x0007
# 
# /* Greybus power supply health values */
# #define GB_POWER_SUPPLY_HEALTH_UNKNOWN			0x0000
# #define GB_POWER_SUPPLY_HEALTH_GOOD			0x0001
# #define GB_POWER_SUPPLY_HEALTH_OVERHEAT			0x0002
# #define GB_POWER_SUPPLY_HEALTH_DEAD			0x0003
# #define GB_POWER_SUPPLY_HEALTH_OVERVOLTAGE		0x0004
# #define GB_POWER_SUPPLY_HEALTH_UNSPEC_FAILURE		0x0005
# #define GB_POWER_SUPPLY_HEALTH_COLD			0x0006
# #define GB_POWER_SUPPLY_HEALTH_WATCHDOG_TIMER_EXPIRE	0x0007
# #define GB_POWER_SUPPLY_HEALTH_SAFETY_TIMER_EXPIRE	0x0008
# 
# /* Greybus power supply status values */
# #define GB_POWER_SUPPLY_STATUS_UNKNOWN			0x0000
# #define GB_POWER_SUPPLY_STATUS_CHARGING			0x0001
# #define GB_POWER_SUPPLY_STATUS_DISCHARGING		0x0002
# #define GB_POWER_SUPPLY_STATUS_NOT_CHARGING		0x0003
# #define GB_POWER_SUPPLY_STATUS_FULL			0x0004
# 
# /* Greybus power supply capacity level values */
# #define GB_POWER_SUPPLY_CAPACITY_LEVEL_UNKNOWN		0x0000
# #define GB_POWER_SUPPLY_CAPACITY_LEVEL_CRITICAL		0x0001
# #define GB_POWER_SUPPLY_CAPACITY_LEVEL_LOW		0x0002
# #define GB_POWER_SUPPLY_CAPACITY_LEVEL_NORMAL		0x0003
# #define GB_POWER_SUPPLY_CAPACITY_LEVEL_HIGH		0x0004
# #define GB_POWER_SUPPLY_CAPACITY_LEVEL_FULL		0x0005
# 
# /* Greybus power supply scope values */
# #define GB_POWER_SUPPLY_SCOPE_UNKNOWN			0x0000
# #define GB_POWER_SUPPLY_SCOPE_SYSTEM			0x0001
# #define GB_POWER_SUPPLY_SCOPE_DEVICE			0x0002
# 
# #[repr(C, packed)]
# pub gb_power_supply_get_supplies_response {
# 	u8	supplies_count;
# }
# 
# #[repr(C, packed)]
# pub gb_power_supply_get_description_request {
# 	u8	psy_id;
# }
# 
# #[repr(C, packed)]
# pub gb_power_supply_get_description_response {
# 	u8	manufacturer: [u8; 32];
# 	u8	model: [u8; 32];
# 	u8	serial_number: [u8; 32];
# 	u16	type;
# 	u8	properties_count;
# }
# 
# #[repr(C, packed)]
# pub gb_power_supply_props_desc {
# 	u8	property;
# #define GB_POWER_SUPPLY_PROP_STATUS				0x00
# #define GB_POWER_SUPPLY_PROP_CHARGE_TYPE			0x01
# #define GB_POWER_SUPPLY_PROP_HEALTH				0x02
# #define GB_POWER_SUPPLY_PROP_PRESENT				0x03
# #define GB_POWER_SUPPLY_PROP_ONLINE				0x04
# #define GB_POWER_SUPPLY_PROP_AUTHENTIC				0x05
# #define GB_POWER_SUPPLY_PROP_TECHNOLOGY				0x06
# #define GB_POWER_SUPPLY_PROP_CYCLE_COUNT			0x07
# #define GB_POWER_SUPPLY_PROP_VOLTAGE_MAX			0x08
# #define GB_POWER_SUPPLY_PROP_VOLTAGE_MIN			0x09
# #define GB_POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN			0x0A
# #define GB_POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN			0x0B
# #define GB_POWER_SUPPLY_PROP_VOLTAGE_NOW			0x0C
# #define GB_POWER_SUPPLY_PROP_VOLTAGE_AVG			0x0D
# #define GB_POWER_SUPPLY_PROP_VOLTAGE_OCV			0x0E
# #define GB_POWER_SUPPLY_PROP_VOLTAGE_BOOT			0x0F
# #define GB_POWER_SUPPLY_PROP_CURRENT_MAX			0x10
# #define GB_POWER_SUPPLY_PROP_CURRENT_NOW			0x11
# #define GB_POWER_SUPPLY_PROP_CURRENT_AVG			0x12
# #define GB_POWER_SUPPLY_PROP_CURRENT_BOOT			0x13
# #define GB_POWER_SUPPLY_PROP_POWER_NOW				0x14
# #define GB_POWER_SUPPLY_PROP_POWER_AVG				0x15
# #define GB_POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN			0x16
# #define GB_POWER_SUPPLY_PROP_CHARGE_EMPTY_DESIGN		0x17
# #define GB_POWER_SUPPLY_PROP_CHARGE_FULL			0x18
# #define GB_POWER_SUPPLY_PROP_CHARGE_EMPTY			0x19
# #define GB_POWER_SUPPLY_PROP_CHARGE_NOW				0x1A
# #define GB_POWER_SUPPLY_PROP_CHARGE_AVG				0x1B
# #define GB_POWER_SUPPLY_PROP_CHARGE_COUNTER			0x1C
# #define GB_POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT		0x1D
# #define GB_POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX	0x1E
# #define GB_POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE		0x1F
# #define GB_POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX	0x20
# #define GB_POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT		0x21
# #define GB_POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT_MAX		0x22
# #define GB_POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT		0x23
# #define GB_POWER_SUPPLY_PROP_ENERGY_FULL_DESIGN			0x24
# #define GB_POWER_SUPPLY_PROP_ENERGY_EMPTY_DESIGN		0x25
# #define GB_POWER_SUPPLY_PROP_ENERGY_FULL			0x26
# #define GB_POWER_SUPPLY_PROP_ENERGY_EMPTY			0x27
# #define GB_POWER_SUPPLY_PROP_ENERGY_NOW				0x28
# #define GB_POWER_SUPPLY_PROP_ENERGY_AVG				0x29
# #define GB_POWER_SUPPLY_PROP_CAPACITY				0x2A
# #define GB_POWER_SUPPLY_PROP_CAPACITY_ALERT_MIN			0x2B
# #define GB_POWER_SUPPLY_PROP_CAPACITY_ALERT_MAX			0x2C
# #define GB_POWER_SUPPLY_PROP_CAPACITY_LEVEL			0x2D
# #define GB_POWER_SUPPLY_PROP_TEMP				0x2E
# #define GB_POWER_SUPPLY_PROP_TEMP_MAX				0x2F
# #define GB_POWER_SUPPLY_PROP_TEMP_MIN				0x30
# #define GB_POWER_SUPPLY_PROP_TEMP_ALERT_MIN			0x31
# #define GB_POWER_SUPPLY_PROP_TEMP_ALERT_MAX			0x32
# #define GB_POWER_SUPPLY_PROP_TEMP_AMBIENT			0x33
# #define GB_POWER_SUPPLY_PROP_TEMP_AMBIENT_ALERT_MIN		0x34
# #define GB_POWER_SUPPLY_PROP_TEMP_AMBIENT_ALERT_MAX		0x35
# #define GB_POWER_SUPPLY_PROP_TIME_TO_EMPTY_NOW			0x36
# #define GB_POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG			0x37
# #define GB_POWER_SUPPLY_PROP_TIME_TO_FULL_NOW			0x38
# #define GB_POWER_SUPPLY_PROP_TIME_TO_FULL_AVG			0x39
# #define GB_POWER_SUPPLY_PROP_TYPE				0x3A
# #define GB_POWER_SUPPLY_PROP_SCOPE				0x3B
# #define GB_POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT		0x3C
# #define GB_POWER_SUPPLY_PROP_CALIBRATE				0x3D
# 	u8	is_writeable;
# }
# 
# #[repr(C, packed)]
# pub gb_power_supply_get_property_descriptors_request {
# 	u8	psy_id;
# }
# 
# #[repr(C, packed)]
# pub gb_power_supply_get_property_descriptors_response {
# 	u8	properties_count;
# 	gb_power_supply_props_desc props: [u8; 0];
# }
# 
# #[repr(C, packed)]
# pub gb_power_supply_get_property_request {
# 	u8	psy_id;
# 	u8	property;
# }
# 
# #[repr(C, packed)]
# pub gb_power_supply_get_property_response {
# 	u32	prop_val;
# };
# 
# #[repr(C, packed)]
# pub gb_power_supply_set_property_request {
# 	u8	psy_id;
# 	u8	property;
# 	u32	prop_val;
# }
# 
# #[repr(C, packed)]
# pub gb_power_supply_event_request {
# 	u8	psy_id;
# 	u8	event;
# #define GB_POWER_SUPPLY_UPDATE		0x01
# }
# 
# 
# /* HID */
# 
# /* Greybus HID operation types */
# #define GB_HID_TYPE_GET_DESC		0x02
# #define GB_HID_TYPE_GET_REPORT_DESC	0x03
# #define GB_HID_TYPE_PWR_ON		0x04
# #define GB_HID_TYPE_PWR_OFF		0x05
# #define GB_HID_TYPE_GET_REPORT		0x06
# #define GB_HID_TYPE_SET_REPORT		0x07
# #define GB_HID_TYPE_IRQ_EVENT		0x08
# 
# /* Report type */
# #define GB_HID_INPUT_REPORT		0
# #define GB_HID_OUTPUT_REPORT		1
# #define GB_HID_FEATURE_REPORT		2
# 
# /* Different request/response structures */
# /* HID get descriptor response */
# #[repr(C, packed)]
# pub gb_hid_desc_response {
# 	u8				bLength;
# 	u16				wReportDescLength;
# 	u16				bcdHID;
# 	u16				wProductID;
# 	u16				wVendorID;
# 	u8				bCountryCode;
# }
# 
# /* HID get report request/response */
# #[repr(C, packed)]
# pub gb_hid_get_report_request {
# 	u8				report_type;
# 	u8				report_id;
# }
# 
# /* HID set report request */
# #[repr(C, packed)]
# pub gb_hid_set_report_request {
# 	u8				report_type;
# 	u8				report_id;
# 	u8				report: [u8; 0];
# }
# 
# /* HID input report request, via interrupt pipe */
# #[repr(C, packed)]
# pub gb_hid_input_report_request {
# 	u8				report: [u8; 0];
# }
# 
# 
# /* I2C */
# 
# /* Greybus i2c request types */
# #define GB_I2C_TYPE_FUNCTIONALITY	0x02
# #define GB_I2C_TYPE_TRANSFER		0x05
# 
# /* functionality request has no payload */
# #[repr(C, packed)]
# pub gb_i2c_functionality_response {
# 	u32	functionality;
# }
# 
# /*
#  * Outgoing data immediately follows the op count and ops array.
#  * The data for each write (master -> slave) op in the array is sent
#  * in order, with no (e.g. pad) bytes separating them.
#  *
#  * Short reads cause the entire transfer request to fail So response
#  * payload consists only of bytes read, and the number of bytes is
#  * exactly what was specified in the corresponding op.  Like
#  * outgoing data, the incoming data is in order and contiguous.
#  */
# #[repr(C, packed)]
# pub gb_i2c_transfer_op {
# 	u16	addr;
# 	u16	flags;
# 	u16	size;
# }
# 
# #[repr(C, packed)]
# pub gb_i2c_transfer_request {
# 	u16				op_count;
# 	gb_i2c_transfer_op	ops: [u8; 0];		/* op_count of these */
# }
# #[repr(C, packed)]
# pub gb_i2c_transfer_response {
# 	u8				data: [u8; 0];	/* inbound data */
# }
# 
# 
# /* GPIO */
# 
# /* Greybus GPIO request types */
# #define GB_GPIO_TYPE_LINE_COUNT		0x02
# #define GB_GPIO_TYPE_ACTIVATE		0x03
# #define GB_GPIO_TYPE_DEACTIVATE		0x04
# #define GB_GPIO_TYPE_GET_DIRECTION	0x05
# #define GB_GPIO_TYPE_DIRECTION_IN	0x06
# #define GB_GPIO_TYPE_DIRECTION_OUT	0x07
# #define GB_GPIO_TYPE_GET_VALUE		0x08
# #define GB_GPIO_TYPE_SET_VALUE		0x09
# #define GB_GPIO_TYPE_SET_DEBOUNCE	0x0a
# #define GB_GPIO_TYPE_IRQ_TYPE		0x0b
# #define GB_GPIO_TYPE_IRQ_MASK		0x0c
# #define GB_GPIO_TYPE_IRQ_UNMASK		0x0d
# #define GB_GPIO_TYPE_IRQ_EVENT		0x0e
# 
# #define GB_GPIO_IRQ_TYPE_NONE		0x00
# #define GB_GPIO_IRQ_TYPE_EDGE_RISING	0x01
# #define GB_GPIO_IRQ_TYPE_EDGE_FALLING	0x02
# #define GB_GPIO_IRQ_TYPE_EDGE_BOTH	0x03
# #define GB_GPIO_IRQ_TYPE_LEVEL_HIGH	0x04
# #define GB_GPIO_IRQ_TYPE_LEVEL_LOW	0x08
# 
# /* line count request has no payload */
# #[repr(C, packed)]
# pub gb_gpio_line_count_response {
# 	u8	count;
# }
# 
# #[repr(C, packed)]
# pub gb_gpio_activate_request {
# 	u8	which;
# }
# /* activate response has no payload */
# 
# #[repr(C, packed)]
# pub gb_gpio_deactivate_request {
# 	u8	which;
# }
# /* deactivate response has no payload */
# 
# #[repr(C, packed)]
# pub gb_gpio_get_direction_request {
# 	u8	which;
# }
# #[repr(C, packed)]
# pub gb_gpio_get_direction_response {
# 	u8	direction;
# }
# 
# #[repr(C, packed)]
# pub gb_gpio_direction_in_request {
# 	u8	which;
# }
# /* direction in response has no payload */
# 
# #[repr(C, packed)]
# pub gb_gpio_direction_out_request {
# 	u8	which;
# 	u8	value;
# }
# /* direction out response has no payload */
# 
# #[repr(C, packed)]
# pub gb_gpio_get_value_request {
# 	u8	which;
# }
# #[repr(C, packed)]
# pub gb_gpio_get_value_response {
# 	u8	value;
# }
# 
# #[repr(C, packed)]
# pub gb_gpio_set_value_request {
# 	u8	which;
# 	u8	value;
# }
# /* set value response has no payload */
# 
# #[repr(C, packed)]
# pub gb_gpio_set_debounce_request {
# 	u8	which;
# 	u16	usec;
# }
# /* debounce response has no payload */
# 
# #[repr(C, packed)]
# pub gb_gpio_irq_type_request {
# 	u8	which;
# 	u8	type;
# }
# /* irq type response has no payload */
# 
# #[repr(C, packed)]
# pub gb_gpio_irq_mask_request {
# 	u8	which;
# }
# /* irq mask response has no payload */
# 
# #[repr(C, packed)]
# pub gb_gpio_irq_unmask_request {
# 	u8	which;
# }
# /* irq unmask response has no payload */
# 
# /* irq event requests originate on another module and are handled on the AP */
# #[repr(C, packed)]
# pub gb_gpio_irq_event_request {
# 	u8	which;
# }
# /* irq event has no response */
# 
# 
# /* PWM */
# 
# /* Greybus PWM operation types */
# #define GB_PWM_TYPE_PWM_COUNT		0x02
# #define GB_PWM_TYPE_ACTIVATE		0x03
# #define GB_PWM_TYPE_DEACTIVATE		0x04
# #define GB_PWM_TYPE_CONFIG		0x05
# #define GB_PWM_TYPE_POLARITY		0x06
# #define GB_PWM_TYPE_ENABLE		0x07
# #define GB_PWM_TYPE_DISABLE		0x08
# 
# /* pwm count request has no payload */
# #[repr(C, packed)]
# pub gb_pwm_count_response {
# 	u8	count;
# }
# 
# #[repr(C, packed)]
# pub gb_pwm_activate_request {
# 	u8	which;
# }
# 
# #[repr(C, packed)]
# pub gb_pwm_deactivate_request {
# 	u8	which;
# }
# 
# #[repr(C, packed)]
# pub gb_pwm_config_request {
# 	u8	which;
# 	u32	duty;
# 	u32	period;
# }
# 
# #[repr(C, packed)]
# pub gb_pwm_polarity_request {
# 	u8	which;
# 	u8	polarity;
# }
# 
# #[repr(C, packed)]
# pub gb_pwm_enable_request {
# 	u8	which;
# }
# 
# #[repr(C, packed)]
# pub gb_pwm_disable_request {
# 	u8	which;
# }
# 
# /* SPI */
# 
# /* Should match up with modes in linux/spi/spi.h */
# #define GB_SPI_MODE_CPHA		0x01		/* clock phase */
# #define GB_SPI_MODE_CPOL		0x02		/* clock polarity */
# #define GB_SPI_MODE_MODE_0		(0 | 0)		/* (original MicroWire) */
# #define GB_SPI_MODE_MODE_1		(0 | GB_SPI_MODE_CPHA)
# #define GB_SPI_MODE_MODE_2		(GB_SPI_MODE_CPOL | 0)
# #define GB_SPI_MODE_MODE_3		(GB_SPI_MODE_CPOL | GB_SPI_MODE_CPHA)
# #define GB_SPI_MODE_CS_HIGH		0x04		/* chipselect active high? */
# #define GB_SPI_MODE_LSB_FIRST		0x08		/* per-word bits-on-wire */
# #define GB_SPI_MODE_3WIRE		0x10		/* SI/SO signals shared */
# #define GB_SPI_MODE_LOOP		0x20		/* loopback mode */
# #define GB_SPI_MODE_NO_CS		0x40		/* 1 dev/bus, no chipselect */
# #define GB_SPI_MODE_READY		0x80		/* slave pulls low to pause */
# 
# /* Should match up with flags in linux/spi/spi.h */
# #define GB_SPI_FLAG_HALF_DUPLEX		 (1u32 << 0)		/* can't do full duplex */
# #define GB_SPI_FLAG_NO_RX		 (1u32 << 1)		/* can't do buffer read */
# #define GB_SPI_FLAG_NO_TX		 (1u32 << 2)		/* can't do buffer write */
# 
# /* Greybus spi operation types */
# #define GB_SPI_TYPE_MASTER_CONFIG	0x02
# #define GB_SPI_TYPE_DEVICE_CONFIG	0x03
# #define GB_SPI_TYPE_TRANSFER		0x04
# 
# /* mode request has no payload */
# #[repr(C, packed)]
# pub gb_spi_master_config_response {
# 	u32	bits_per_word_mask;
# 	u32	min_speed_hz;
# 	u32	max_speed_hz;
# 	u16	mode;
# 	u16	flags;
# 	u8	num_chipselect;
# }
# 
# #[repr(C, packed)]
# pub gb_spi_device_config_request {
# 	u8	chip_select;
# }
# 
# #[repr(C, packed)]
# pub gb_spi_device_config_response {
# 	u16	mode;
# 	u8	bits_per_word;
# 	u32	max_speed_hz;
# 	u8	device_type;
# #define GB_SPI_SPI_DEV		0x00
# #define GB_SPI_SPI_NOR		0x01
# #define GB_SPI_SPI_MODALIAS	0x02
# 	u8	name: [u8; 32];
# }
# 
# /**
#  * gb_spi_transfer - a read/write buffer pair
#  * @speed_hz: Select a speed other than the device default for this transfer. If
#  *	0 the default (from @spi_device) is used.
#  * @len: size of rx and tx buffers (in bytes)
#  * @delay_usecs: microseconds to delay after this transfer before (optionally)
#  * 	changing the chipselect status, then starting the next transfer or
#  * 	completing this spi_message.
#  * @cs_change: affects chipselect after this transfer completes
#  * @bits_per_word: select a bits_per_word other than the device default for this
#  *	transfer. If 0 the default (from @spi_device) is used.
#  */
# #[repr(C, packed)]
# pub gb_spi_transfer {
# 	u32		speed_hz;
# 	u32		len;
# 	u16		delay_usecs;
# 	u8		cs_change;
# 	u8		bits_per_word;
# 	u8		xfer_flags;
# #define GB_SPI_XFER_READ	0x01
# #define GB_SPI_XFER_WRITE	0x02
# #define GB_SPI_XFER_INPROGRESS	0x04
# }
# 
# #[repr(C, packed)]
# pub gb_spi_transfer_request {
# 	u8			chip_select;	/* of the spi device */
# 	u8			mode;		/* of the spi device */
# 	u16			count;
# 	gb_spi_transfer	transfers: [u8; 0];	/* count of these */
# }
# 
# #[repr(C, packed)]
# pub gb_spi_transfer_response {
# 	u8			data: [u8; 0];	/* inbound data */
# }
# 
# /* Version of the Greybus SVC protocol we support */
# #define GB_SVC_VERSION_MAJOR		0x00
# #define GB_SVC_VERSION_MINOR		0x01
# 
# /* Greybus SVC request types */
# #define GB_SVC_TYPE_PROTOCOL_VERSION		0x01
# #define GB_SVC_TYPE_SVC_HELLO			0x02
# #define GB_SVC_TYPE_INTF_DEVICE_ID		0x03
# #define GB_SVC_TYPE_INTF_RESET			0x06
# #define GB_SVC_TYPE_CONN_CREATE			0x07
# #define GB_SVC_TYPE_CONN_DESTROY		0x08
# #define GB_SVC_TYPE_DME_PEER_GET		0x09
# #define GB_SVC_TYPE_DME_PEER_SET		0x0a
# #define GB_SVC_TYPE_ROUTE_CREATE		0x0b
# #define GB_SVC_TYPE_ROUTE_DESTROY		0x0c
# #define GB_SVC_TYPE_TIMESYNC_ENABLE		0x0d
# #define GB_SVC_TYPE_TIMESYNC_DISABLE		0x0e
# #define GB_SVC_TYPE_TIMESYNC_AUTHORITATIVE	0x0f
# #define GB_SVC_TYPE_INTF_SET_PWRM		0x10
# #define GB_SVC_TYPE_INTF_EJECT			0x11
# #define GB_SVC_TYPE_PING			0x13
# #define GB_SVC_TYPE_PWRMON_RAIL_COUNT_GET	0x14
# #define GB_SVC_TYPE_PWRMON_RAIL_NAMES_GET	0x15
# #define GB_SVC_TYPE_PWRMON_SAMPLE_GET		0x16
# #define GB_SVC_TYPE_PWRMON_INTF_SAMPLE_GET	0x17
# #define GB_SVC_TYPE_TIMESYNC_WAKE_PINS_ACQUIRE	0x18
# #define GB_SVC_TYPE_TIMESYNC_WAKE_PINS_RELEASE	0x19
# #define GB_SVC_TYPE_TIMESYNC_PING		0x1a
# #define GB_SVC_TYPE_MODULE_INSERTED		0x1f
# #define GB_SVC_TYPE_MODULE_REMOVED		0x20
# #define GB_SVC_TYPE_INTF_VSYS_ENABLE		0x21
# #define GB_SVC_TYPE_INTF_VSYS_DISABLE		0x22
# #define GB_SVC_TYPE_INTF_REFCLK_ENABLE		0x23
# #define GB_SVC_TYPE_INTF_REFCLK_DISABLE		0x24
# #define GB_SVC_TYPE_INTF_UNIPRO_ENABLE		0x25
# #define GB_SVC_TYPE_INTF_UNIPRO_DISABLE		0x26
# #define GB_SVC_TYPE_INTF_ACTIVATE		0x27
# #define GB_SVC_TYPE_INTF_RESUME			0x28
# #define GB_SVC_TYPE_INTF_MAILBOX_EVENT		0x29
# #define GB_SVC_TYPE_INTF_OOPS			0x2a
# 
# /* Greybus SVC protocol status values */
# #define GB_SVC_OP_SUCCESS			0x00
# #define GB_SVC_OP_UNKNOWN_ERROR			0x01
# #define GB_SVC_INTF_NOT_DETECTED		0x02
# #define GB_SVC_INTF_NO_UPRO_LINK		0x03
# #define GB_SVC_INTF_UPRO_NOT_DOWN		0x04
# #define GB_SVC_INTF_UPRO_NOT_HIBERNATED		0x05
# #define GB_SVC_INTF_NO_V_SYS			0x06
# #define GB_SVC_INTF_V_CHG			0x07
# #define GB_SVC_INTF_WAKE_BUSY			0x08
# #define GB_SVC_INTF_NO_REFCLK			0x09
# #define GB_SVC_INTF_RELEASING			0x0a
# #define GB_SVC_INTF_NO_ORDER			0x0b
# #define GB_SVC_INTF_MBOX_SET			0x0c
# #define GB_SVC_INTF_BAD_MBOX			0x0d
# #define GB_SVC_INTF_OP_TIMEOUT			0x0e
# #define GB_SVC_PWRMON_OP_NOT_PRESENT		0x0f
# 
# #[repr(C, packed)]
# pub gb_svc_version_request {
# 	u8	major;
# 	u8	minor;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_version_response {
# 	u8	major;
# 	u8	minor;
# }
# 
# /* SVC protocol hello request */
# #[repr(C, packed)]
# pub gb_svc_hello_request {
# 	u16			endo_id;
# 	u8			interface_id;
# }
# /* hello response has no payload */
# 
# #[repr(C, packed)]
# pub gb_svc_intf_device_id_request {
# 	u8	intf_id;
# 	u8	device_id;
# }
# /* device id response has no payload */
# 
# #[repr(C, packed)]
# pub gb_svc_intf_reset_request {
# 	u8	intf_id;
# }
# /* interface reset response has no payload */
# 
# #[repr(C, packed)]
# pub gb_svc_intf_eject_request {
# 	u8	intf_id;
# }
# /* interface eject response has no payload */
# 
# #[repr(C, packed)]
# pub gb_svc_conn_create_request {
# 	u8	intf1_id;
# 	u16	cport1_id;
# 	u8	intf2_id;
# 	u16	cport2_id;
# 	u8	tc;
# 	u8	flags;
# }
# /* connection create response has no payload */
# 
# #[repr(C, packed)]
# pub gb_svc_conn_destroy_request {
# 	u8	intf1_id;
# 	u16	cport1_id;
# 	u8	intf2_id;
# 	u16	cport2_id;
# }
# /* connection destroy response has no payload */
# 
# #[repr(C, packed)]
# pub gb_svc_dme_peer_get_request {
# 	u8	intf_id;
# 	u16	attr;
# 	u16	selector;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_dme_peer_get_response {
# 	u16	result_code;
# 	u32	attr_value;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_dme_peer_set_request {
# 	u8	intf_id;
# 	u16	attr;
# 	u16	selector;
# 	u32	value;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_dme_peer_set_response {
# 	u16	result_code;
# }
# 
# /* Greybus init-status values, currently retrieved using DME peer gets. */
# #define GB_INIT_SPI_BOOT_STARTED			0x02
# #define GB_INIT_TRUSTED_SPI_BOOT_FINISHED		0x03
# #define GB_INIT_UNTRUSTED_SPI_BOOT_FINISHED		0x04
# #define GB_INIT_BOOTROM_UNIPRO_BOOT_STARTED		0x06
# #define GB_INIT_BOOTROM_FALLBACK_UNIPRO_BOOT_STARTED	0x09
# #define GB_INIT_S2_LOADER_BOOT_STARTED			0x0D
# 
# #[repr(C, packed)]
# pub gb_svc_route_create_request {
# 	u8	intf1_id;
# 	u8	dev1_id;
# 	u8	intf2_id;
# 	u8	dev2_id;
# }
# /* route create response has no payload */
# 
# #[repr(C, packed)]
# pub gb_svc_route_destroy_request {
# 	u8	intf1_id;
# 	u8	intf2_id;
# }
# /* route destroy response has no payload */
# 
# /* used for svc_intf_vsys_{enable,disable} */
# #[repr(C, packed)]
# pub gb_svc_intf_vsys_request {
# 	u8	intf_id;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_intf_vsys_response {
# 	u8	result_code;
# #define GB_SVC_INTF_VSYS_OK				0x00
# 	/* 0x01 is reserved */
# #define GB_SVC_INTF_VSYS_FAIL				0x02
# }
# 
# /* used for svc_intf_refclk_{enable,disable} */
# #[repr(C, packed)]
# pub gb_svc_intf_refclk_request {
# 	u8	intf_id;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_intf_refclk_response {
# 	u8	result_code;
# #define GB_SVC_INTF_REFCLK_OK				0x00
# 	/* 0x01 is reserved */
# #define GB_SVC_INTF_REFCLK_FAIL				0x02
# }
# 
# /* used for svc_intf_unipro_{enable,disable} */
# #[repr(C, packed)]
# pub gb_svc_intf_unipro_request {
# 	u8	intf_id;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_intf_unipro_response {
# 	u8	result_code;
# #define GB_SVC_INTF_UNIPRO_OK				0x00
# 	/* 0x01 is reserved */
# #define GB_SVC_INTF_UNIPRO_FAIL				0x02
# #define GB_SVC_INTF_UNIPRO_NOT_OFF			0x03
# }
# 
# #define GB_SVC_UNIPRO_FAST_MODE			0x01
# #define GB_SVC_UNIPRO_SLOW_MODE			0x02
# #define GB_SVC_UNIPRO_FAST_AUTO_MODE		0x04
# #define GB_SVC_UNIPRO_SLOW_AUTO_MODE		0x05
# #define GB_SVC_UNIPRO_MODE_UNCHANGED		0x07
# #define GB_SVC_UNIPRO_HIBERNATE_MODE		0x11
# #define GB_SVC_UNIPRO_OFF_MODE			0x12
# 
# #define GB_SVC_SMALL_AMPLITUDE          0x01
# #define GB_SVC_LARGE_AMPLITUDE          0x02
# 
# #define GB_SVC_NO_DE_EMPHASIS           0x00
# #define GB_SVC_SMALL_DE_EMPHASIS        0x01
# #define GB_SVC_LARGE_DE_EMPHASIS        0x02
# 
# #define GB_SVC_PWRM_RXTERMINATION		0x01
# #define GB_SVC_PWRM_TXTERMINATION		0x02
# #define GB_SVC_PWRM_LINE_RESET			0x04
# #define GB_SVC_PWRM_SCRAMBLING			0x20
# 
# #define GB_SVC_PWRM_QUIRK_HSSER			0x00000001
# 
# #define GB_SVC_UNIPRO_HS_SERIES_A		0x01
# #define GB_SVC_UNIPRO_HS_SERIES_B		0x02
# 
# #define GB_SVC_SETPWRM_PWR_OK           0x00
# #define GB_SVC_SETPWRM_PWR_LOCAL        0x01
# #define GB_SVC_SETPWRM_PWR_REMOTE       0x02
# #define GB_SVC_SETPWRM_PWR_BUSY         0x03
# #define GB_SVC_SETPWRM_PWR_ERROR_CAP    0x04
# #define GB_SVC_SETPWRM_PWR_FATAL_ERROR  0x05
# 
# #[repr(C, packed)]
# pub gb_svc_l2_timer_cfg {
# 	u16 tsb_fc0_protection_timeout;
# 	u16 tsb_tc0_replay_timeout;
# 	u16 tsb_afc0_req_timeout;
# 	u16 tsb_fc1_protection_timeout;
# 	u16 tsb_tc1_replay_timeout;
# 	u16 tsb_afc1_req_timeout;
# 	u16 reserved_for_tc2: [u8; 3];
# 	u16 reserved_for_tc3: [u8; 3];
# }
# 
# #[repr(C, packed)]
# pub gb_svc_intf_set_pwrm_request {
# 	u8	intf_id;
# 	u8	hs_series;
# 	u8	tx_mode;
# 	u8	tx_gear;
# 	u8	tx_nlanes;
# 	u8	tx_amplitude;
# 	u8	tx_hs_equalizer;
# 	u8	rx_mode;
# 	u8	rx_gear;
# 	u8	rx_nlanes;
# 	u8	flags;
# 	u32	quirks;
# 	gb_svc_l2_timer_cfg local_l2timerdata, remote_l2timerdata;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_intf_set_pwrm_response {
# 	u8	result_code;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_key_event_request {
# 	u16  key_code;
# #define GB_KEYCODE_ARA         0x00
# 
# 	u8    key_event;
# #define GB_SVC_KEY_RELEASED    0x00
# #define GB_SVC_KEY_PRESSED     0x01
# }
# 
# #define GB_SVC_PWRMON_MAX_RAIL_COUNT		254
# 
# #[repr(C, packed)]
# pub gb_svc_pwrmon_rail_count_get_response {
# 	u8	rail_count;
# }
# 
# #define GB_SVC_PWRMON_RAIL_NAME_BUFSIZE		32
# 
# #[repr(C, packed)]
# pub gb_svc_pwrmon_rail_names_get_response {
# 	u8	status;
# 	u8	name: [u8; 0][GB_SVC_PWRMON_RAIL_NAME_BUFSIZE];
# }
# 
# #define GB_SVC_PWRMON_TYPE_CURR			0x01
# #define GB_SVC_PWRMON_TYPE_VOL			0x02
# #define GB_SVC_PWRMON_TYPE_PWR			0x03
# 
# #define GB_SVC_PWRMON_GET_SAMPLE_OK		0x00
# #define GB_SVC_PWRMON_GET_SAMPLE_INVAL		0x01
# #define GB_SVC_PWRMON_GET_SAMPLE_NOSUPP		0x02
# #define GB_SVC_PWRMON_GET_SAMPLE_HWERR		0x03
# 
# #[repr(C, packed)]
# pub gb_svc_pwrmon_sample_get_request {
# 	u8	rail_id;
# 	u8	measurement_type;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_pwrmon_sample_get_response {
# 	u8	result;
# 	u32	measurement;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_pwrmon_intf_sample_get_request {
# 	u8	intf_id;
# 	u8	measurement_type;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_pwrmon_intf_sample_get_response {
# 	u8	result;
# 	u32	measurement;
# }
# 
# #define GB_SVC_MODULE_INSERTED_FLAG_NO_PRIMARY	0x0001
# 
# #[repr(C, packed)]
# pub gb_svc_module_inserted_request {
# 	u8	primary_intf_id;
# 	u8	intf_count;
# 	u16	flags;
# }
# /* module_inserted response has no payload */
# 
# #[repr(C, packed)]
# pub gb_svc_module_removed_request {
# 	u8	primary_intf_id;
# }
# /* module_removed response has no payload */
# 
# #[repr(C, packed)]
# pub gb_svc_intf_activate_request {
# 	u8	intf_id;
# }
# 
# #define GB_SVC_INTF_TYPE_UNKNOWN		0x00
# #define GB_SVC_INTF_TYPE_DUMMY			0x01
# #define GB_SVC_INTF_TYPE_UNIPRO			0x02
# #define GB_SVC_INTF_TYPE_GREYBUS		0x03
# 
# #[repr(C, packed)]
# pub gb_svc_intf_activate_response {
# 	u8	status;
# 	u8	intf_type;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_intf_resume_request {
# 	u8	intf_id;
# }
# 
# #[repr(C, packed)]
# pub gb_svc_intf_resume_response {
# 	u8	status;
# }
# 
# #define GB_SVC_INTF_MAILBOX_NONE		0x00
# #define GB_SVC_INTF_MAILBOX_AP			0x01
# #define GB_SVC_INTF_MAILBOX_GREYBUS		0x02
# 
# #[repr(C, packed)]
# pub gb_svc_intf_mailbox_event_request {
# 	u8	intf_id;
# 	u16	result_code;
# 	u32	mailbox;
# }
# /* intf_mailbox_event response has no payload */
# 
# #[repr(C, packed)]
# pub gb_svc_intf_oops_request {
# 	u8	intf_id;
# 	u8	reason;
# }
# /* intf_oops response has no payload */
# 
# 
# /* RAW */
# 
# /* Greybus raw request types */
# #define	GB_RAW_TYPE_SEND			0x02
# 
# #[repr(C, packed)]
# pub gb_raw_send_request {
# 	u32	len;
# 	u8	data: [u8; 0];
# }
# 
# 
# /* UART */
# 
# /* Greybus UART operation types */
# #define GB_UART_TYPE_SEND_DATA			0x02
# #define GB_UART_TYPE_RECEIVE_DATA		0x03	/* Unsolicited data */
# #define GB_UART_TYPE_SET_LINE_CODING		0x04
# #define GB_UART_TYPE_SET_CONTROL_LINE_STATE	0x05
# #define GB_UART_TYPE_SEND_BREAK			0x06
# #define GB_UART_TYPE_SERIAL_STATE		0x07	/* Unsolicited data */
# #define GB_UART_TYPE_RECEIVE_CREDITS		0x08
# #define GB_UART_TYPE_FLUSH_FIFOS		0x09
# 
# /* Represents data from AP -> Module */
# #[repr(C, packed)]
# pub gb_uart_send_data_request {
# 	u16	size;
# 	u8	data: [u8; 0];
# }
# 
# /* recv-data-request flags */
# #define GB_UART_RECV_FLAG_FRAMING		0x01	/* Framing error */
# #define GB_UART_RECV_FLAG_PARITY		0x02	/* Parity error */
# #define GB_UART_RECV_FLAG_OVERRUN		0x04	/* Overrun error */
# #define GB_UART_RECV_FLAG_BREAK			0x08	/* Break */
# 
# /* Represents data from Module -> AP */
# #[repr(C, packed)]
# pub gb_uart_recv_data_request {
# 	u16	size;
# 	u8	flags;
# 	u8	data: [u8; 0];
# }
# 
# #[repr(C, packed)]
# pub gb_uart_receive_credits_request {
# 	u16  count;
# }
# 
# #[repr(C, packed)]
# pub gb_uart_set_line_coding_request {
# 	u32	rate;
# 	u8	format;
# #define GB_SERIAL_1_STOP_BITS			0
# #define GB_SERIAL_1_5_STOP_BITS			1
# #define GB_SERIAL_2_STOP_BITS			2
# 
# 	u8	parity;
# #define GB_SERIAL_NO_PARITY			0
# #define GB_SERIAL_ODD_PARITY			1
# #define GB_SERIAL_EVEN_PARITY			2
# #define GB_SERIAL_MARK_PARITY			3
# #define GB_SERIAL_SPACE_PARITY			4
# 
# 	u8	data_bits;
# 
# 	u8	flow_control;
# #define GB_SERIAL_AUTO_RTSCTS_EN		0x1
# }
# 
# /* output control lines */
# #define GB_UART_CTRL_DTR			0x01
# #define GB_UART_CTRL_RTS			0x02
# 
# #[repr(C, packed)]
# pub gb_uart_set_control_line_state_request {
# 	u8	control;
# }
# 
# #[repr(C, packed)]
# pub gb_uart_set_break_request {
# 	u8	state;
# }
# 
# /* input control lines and line errors */
# #define GB_UART_CTRL_DCD			0x01
# #define GB_UART_CTRL_DSR			0x02
# #define GB_UART_CTRL_RI				0x04
# 
# #[repr(C, packed)]
# pub gb_uart_serial_state_request {
# 	u8	control;
# }
# 
# #[repr(C, packed)]
# pub gb_uart_serial_flush_request {
# 	u8    flags;
# #define GB_SERIAL_FLAG_FLUSH_TRANSMITTER	0x01
# #define GB_SERIAL_FLAG_FLUSH_RECEIVER		0x02
# }
# 
# /* Loopback */
# 
# /* Greybus loopback request types */
# #define GB_LOOPBACK_TYPE_PING			0x02
# #define GB_LOOPBACK_TYPE_TRANSFER		0x03
# #define GB_LOOPBACK_TYPE_SINK			0x04
# 
# /*
#  * Loopback request/response header format should be identical
#  * to simplify bandwidth and data movement analysis.
#  */
# #[repr(C, packed)]
# pub gb_loopback_transfer_request {
# 	u32	len;
# 	u32  reserved0;
# 	u32  reserved1;
# 	u8	data: [u8; 0];
# }
# 
# #[repr(C, packed)]
# pub gb_loopback_transfer_response {
# 	u32	len;
# 	u32	reserved0;
# 	u32	reserved1;
# 	u8	data: [u8; 0];
# }
# 
# /* SDIO */
# /* Greybus SDIO operation types */
# #define GB_SDIO_TYPE_GET_CAPABILITIES		0x02
# #define GB_SDIO_TYPE_SET_IOS			0x03
# #define GB_SDIO_TYPE_COMMAND			0x04
# #define GB_SDIO_TYPE_TRANSFER			0x05
# #define GB_SDIO_TYPE_EVENT			0x06
# 
# /* get caps response: request has no payload */
# #[repr(C, packed)]
# pub gb_sdio_get_caps_response {
# 	u32	caps;
# #define GB_SDIO_CAP_NONREMOVABLE	0x00000001
# #define GB_SDIO_CAP_4_BIT_DATA		0x00000002
# #define GB_SDIO_CAP_8_BIT_DATA		0x00000004
# #define GB_SDIO_CAP_MMC_HS		0x00000008
# #define GB_SDIO_CAP_SD_HS		0x00000010
# #define GB_SDIO_CAP_ERASE		0x00000020
# #define GB_SDIO_CAP_1_2V_DDR		0x00000040
# #define GB_SDIO_CAP_1_8V_DDR		0x00000080
# #define GB_SDIO_CAP_POWER_OFF_CARD	0x00000100
# #define GB_SDIO_CAP_UHS_SDR12		0x00000200
# #define GB_SDIO_CAP_UHS_SDR25		0x00000400
# #define GB_SDIO_CAP_UHS_SDR50		0x00000800
# #define GB_SDIO_CAP_UHS_SDR104		0x00001000
# #define GB_SDIO_CAP_UHS_DDR50		0x00002000
# #define GB_SDIO_CAP_DRIVER_TYPE_A	0x00004000
# #define GB_SDIO_CAP_DRIVER_TYPE_C	0x00008000
# #define GB_SDIO_CAP_DRIVER_TYPE_D	0x00010000
# #define GB_SDIO_CAP_HS200_1_2V		0x00020000
# #define GB_SDIO_CAP_HS200_1_8V		0x00040000
# #define GB_SDIO_CAP_HS400_1_2V		0x00080000
# #define GB_SDIO_CAP_HS400_1_8V		0x00100000
# 
# 	/* see possible values below at vdd */
# 	u32 ocr;
# 	u32 f_min;
# 	u32 f_max;
# 	u16 max_blk_count;
# 	u16 max_blk_size;
# }
# 
# /* set ios request: response has no payload */
# #[repr(C, packed)]
# pub gb_sdio_set_ios_request {
# 	u32	clock;
# 	u32	vdd;
# #define GB_SDIO_VDD_165_195	0x00000001
# #define GB_SDIO_VDD_20_21	0x00000002
# #define GB_SDIO_VDD_21_22	0x00000004
# #define GB_SDIO_VDD_22_23	0x00000008
# #define GB_SDIO_VDD_23_24	0x00000010
# #define GB_SDIO_VDD_24_25	0x00000020
# #define GB_SDIO_VDD_25_26	0x00000040
# #define GB_SDIO_VDD_26_27	0x00000080
# #define GB_SDIO_VDD_27_28	0x00000100
# #define GB_SDIO_VDD_28_29	0x00000200
# #define GB_SDIO_VDD_29_30	0x00000400
# #define GB_SDIO_VDD_30_31	0x00000800
# #define GB_SDIO_VDD_31_32	0x00001000
# #define GB_SDIO_VDD_32_33	0x00002000
# #define GB_SDIO_VDD_33_34	0x00004000
# #define GB_SDIO_VDD_34_35	0x00008000
# #define GB_SDIO_VDD_35_36	0x00010000
# 
# 	u8	bus_mode;
# #define GB_SDIO_BUSMODE_OPENDRAIN	0x00
# #define GB_SDIO_BUSMODE_PUSHPULL	0x01
# 
# 	u8	power_mode;
# #define GB_SDIO_POWER_OFF	0x00
# #define GB_SDIO_POWER_UP	0x01
# #define GB_SDIO_POWER_ON	0x02
# #define GB_SDIO_POWER_UNDEFINED	0x03
# 
# 	u8	bus_width;
# #define GB_SDIO_BUS_WIDTH_1	0x00
# #define GB_SDIO_BUS_WIDTH_4	0x02
# #define GB_SDIO_BUS_WIDTH_8	0x03
# 
# 	u8	timing;
# #define GB_SDIO_TIMING_LEGACY		0x00
# #define GB_SDIO_TIMING_MMC_HS		0x01
# #define GB_SDIO_TIMING_SD_HS		0x02
# #define GB_SDIO_TIMING_UHS_SDR12	0x03
# #define GB_SDIO_TIMING_UHS_SDR25	0x04
# #define GB_SDIO_TIMING_UHS_SDR50	0x05
# #define GB_SDIO_TIMING_UHS_SDR104	0x06
# #define GB_SDIO_TIMING_UHS_DDR50	0x07
# #define GB_SDIO_TIMING_MMC_DDR52	0x08
# #define GB_SDIO_TIMING_MMC_HS200	0x09
# #define GB_SDIO_TIMING_MMC_HS400	0x0A
# 
# 	u8	signal_voltage;
# #define GB_SDIO_SIGNAL_VOLTAGE_330	0x00
# #define GB_SDIO_SIGNAL_VOLTAGE_180	0x01
# #define GB_SDIO_SIGNAL_VOLTAGE_120	0x02
# 
# 	u8	drv_type;
# #define GB_SDIO_SET_DRIVER_TYPE_B	0x00
# #define GB_SDIO_SET_DRIVER_TYPE_A	0x01
# #define GB_SDIO_SET_DRIVER_TYPE_C	0x02
# #define GB_SDIO_SET_DRIVER_TYPE_D	0x03
# }
# 
# /* command request */
# #[repr(C, packed)]
# pub gb_sdio_command_request {
# 	u8	cmd;
# 	u8	cmd_flags;
# #define GB_SDIO_RSP_NONE		0x00
# #define GB_SDIO_RSP_PRESENT		0x01
# #define GB_SDIO_RSP_136			0x02
# #define GB_SDIO_RSP_CRC			0x04
# #define GB_SDIO_RSP_BUSY		0x08
# #define GB_SDIO_RSP_OPCODE		0x10
# 
# 	u8	cmd_type;
# #define GB_SDIO_CMD_AC		0x00
# #define GB_SDIO_CMD_ADTC	0x01
# #define GB_SDIO_CMD_BC		0x02
# #define GB_SDIO_CMD_BCR		0x03
# 
# 	u32	cmd_arg;
# 	u16	data_blocks;
# 	u16	data_blksz;
# }
# 
# #[repr(C, packed)]
# pub gb_sdio_command_response {
# 	u32	resp: [u8; 4];
# }
# 
# /* transfer request */
# #[repr(C, packed)]
# pub gb_sdio_transfer_request {
# 	u8	data_flags;
# #define GB_SDIO_DATA_WRITE	0x01
# #define GB_SDIO_DATA_READ	0x02
# #define GB_SDIO_DATA_STREAM	0x04
# 
# 	u16	data_blocks;
# 	u16	data_blksz;
# 	u8	data: [u8; 0];
# }
# 
# #[repr(C, packed)]
# pub gb_sdio_transfer_response {
# 	u16	data_blocks;
# 	u16	data_blksz;
# 	u8	data: [u8; 0];
# }
# 
# /* event request: generated by module and is defined as unidirectional */
# #[repr(C, packed)]
# pub gb_sdio_event_request {
# 	u8	event;
# #define GB_SDIO_CARD_INSERTED	0x01
# #define GB_SDIO_CARD_REMOVED	0x02
# #define GB_SDIO_WP		0x04
# }
# 
# /* Camera */
# 
# /* Greybus Camera request types */
# #define GB_CAMERA_TYPE_CAPABILITIES		0x02
# #define GB_CAMERA_TYPE_CONFIGURE_STREAMS	0x03
# #define GB_CAMERA_TYPE_CAPTURE			0x04
# #define GB_CAMERA_TYPE_FLUSH			0x05
# #define GB_CAMERA_TYPE_METADATA			0x06
# 
# #define GB_CAMERA_MAX_STREAMS			4
# #define GB_CAMERA_MAX_SETTINGS_SIZE		8192
# 
# /* Greybus Camera Configure Streams request payload */
# #[repr(C, packed)]
# pub gb_camera_stream_config_request {
# 	u16 width;
# 	u16 height;
# 	u16 format;
# 	u16 padding;
# }
# 
# #[repr(C, packed)]
# pub gb_camera_configure_streams_request {
# 	u8 num_streams;
# 	u8 flags;
# #define GB_CAMERA_CONFIGURE_STREAMS_TEST_ONLY	0x01
# 	u16 padding;
# 	gb_camera_stream_config_request config: [u8; 0];
# }
# 
# /* Greybus Camera Configure Streams response payload */
# #[repr(C, packed)]
# pub gb_camera_stream_config_response {
# 	u16 width;
# 	u16 height;
# 	u16 format;
# 	u8 virtual_channel;
# 	u8 data_type: [u8; 2];
# 	u16 max_pkt_size;
# 	u8 padding;
# 	u32 max_size;
# }
# 
# #[repr(C, packed)]
# pub gb_camera_configure_streams_response {
# 	u8 num_streams;
# #define GB_CAMERA_CONFIGURE_STREAMS_ADJUSTED	0x01
# 	u8 flags;
# 	u8 padding: [u8; 2];
# 	u32 data_rate;
# 	gb_camera_stream_config_response config: [u8; 0];
# };
# 
# /* Greybus Camera Capture request payload - response has no payload */
# #[repr(C, packed)]
# pub gb_camera_capture_request {
# 	u32 request_id;
# 	u8 streams;
# 	u8 padding;
# 	u16 num_frames;
# 	u8 settings: [u8; 0];
# }
# 
# /* Greybus Camera Flush response payload - request has no payload */
# #[repr(C, packed)]
# pub gb_camera_flush_response {
# 	u32 request_id;
# }
# 
# /* Greybus Camera Metadata request payload - operation has no response */
# #[repr(C, packed)]
# pub gb_camera_metadata_request {
# 	u32 request_id;
# 	u16 frame_number;
# 	u8 stream;
# 	u8 padding;
# 	u8 metadata: [u8; 0];
# }
# 
# /* Lights */
# 
# /* Greybus Lights request types */
# #define GB_LIGHTS_TYPE_GET_LIGHTS		0x02
# #define GB_LIGHTS_TYPE_GET_LIGHT_CONFIG		0x03
# #define GB_LIGHTS_TYPE_GET_CHANNEL_CONFIG	0x04
# #define GB_LIGHTS_TYPE_GET_CHANNEL_FLASH_CONFIG	0x05
# #define GB_LIGHTS_TYPE_SET_BRIGHTNESS		0x06
# #define GB_LIGHTS_TYPE_SET_BLINK		0x07
# #define GB_LIGHTS_TYPE_SET_COLOR		0x08
# #define GB_LIGHTS_TYPE_SET_FADE			0x09
# #define GB_LIGHTS_TYPE_EVENT			0x0A
# #define GB_LIGHTS_TYPE_SET_FLASH_INTENSITY	0x0B
# #define GB_LIGHTS_TYPE_SET_FLASH_STROBE		0x0C
# #define GB_LIGHTS_TYPE_SET_FLASH_TIMEOUT	0x0D
# #define GB_LIGHTS_TYPE_GET_FLASH_FAULT		0x0E
# 
# /* Greybus Light modes */
# 
# /*
#  * if you add any specific mode below, update also the
#  * GB_CHANNEL_MODE_DEFINED_RANGE value accordingly
#  */
# #define GB_CHANNEL_MODE_NONE		0x00000000
# #define GB_CHANNEL_MODE_BATTERY		0x00000001
# #define GB_CHANNEL_MODE_POWER		0x00000002
# #define GB_CHANNEL_MODE_WIRELESS	0x00000004
# #define GB_CHANNEL_MODE_BLUETOOTH	0x00000008
# #define GB_CHANNEL_MODE_KEYBOARD	0x00000010
# #define GB_CHANNEL_MODE_BUTTONS		0x00000020
# #define GB_CHANNEL_MODE_NOTIFICATION	0x00000040
# #define GB_CHANNEL_MODE_ATTENTION	0x00000080
# #define GB_CHANNEL_MODE_FLASH		0x00000100
# #define GB_CHANNEL_MODE_TORCH		0x00000200
# #define GB_CHANNEL_MODE_INDICATOR	0x00000400
# 
# /* Lights Mode valid bit values */
# #define GB_CHANNEL_MODE_DEFINED_RANGE	0x000004FF
# #define GB_CHANNEL_MODE_VENDOR_RANGE	0x00F00000
# 
# /* Greybus Light Channels Flags */
# #define GB_LIGHT_CHANNEL_MULTICOLOR	0x00000001
# #define GB_LIGHT_CHANNEL_FADER		0x00000002
# #define GB_LIGHT_CHANNEL_BLINK		0x00000004
# 
# /* get count of lights in module */
# #[repr(C, packed)]
# pub gb_lights_get_lights_response {
# 	u8	lights_count;
# }
# 
# /* light config request payload */
# #[repr(C, packed)]
# pub gb_lights_get_light_config_request {
# 	u8	id;
# }
# 
# /* light config response payload */
# #[repr(C, packed)]
# pub gb_lights_get_light_config_response {
# 	u8	channel_count;
# 	u8	name: [u8; 32];
# }
# 
# /* channel config request payload */
# #[repr(C, packed)]
# pub gb_lights_get_channel_config_request {
# 	u8	light_id;
# 	u8	channel_id;
# }
# 
# /* channel flash config request payload */
# #[repr(C, packed)]
# pub gb_lights_get_channel_flash_config_request {
# 	u8	light_id;
# 	u8	channel_id;
# }
# 
# /* channel config response payload */
# #[repr(C, packed)]
# pub gb_lights_get_channel_config_response {
# 	u8	max_brightness;
# 	u32	flags;
# 	u32	color;
# 	u8	color_name: [u8; 32];
# 	u32	mode;
# 	u8	mode_name: [u8; 32];
# }
# 
# /* channel flash config response payload */
# #[repr(C, packed)]
# pub gb_lights_get_channel_flash_config_response {
# 	u32	intensity_min_uA;
# 	u32	intensity_max_uA;
# 	u32	intensity_step_uA;
# 	u32	timeout_min_us;
# 	u32	timeout_max_us;
# 	u32	timeout_step_us;
# }
# 
# /* blink request payload: response have no payload */
# #[repr(C, packed)]
# pub gb_lights_blink_request {
# 	u8	light_id;
# 	u8	channel_id;
# 	u16	time_on_ms;
# 	u16	time_off_ms;
# }
# 
# /* set brightness request payload: response have no payload */
# #[repr(C, packed)]
# pub gb_lights_set_brightness_request {
# 	u8	light_id;
# 	u8	channel_id;
# 	u8	brightness;
# }
# 
# /* set color request payload: response have no payload */
# #[repr(C, packed)]
# pub gb_lights_set_color_request {
# 	u8	light_id;
# 	u8	channel_id;
# 	u32	color;
# }
# 
# /* set fade request payload: response have no payload */
# #[repr(C, packed)]
# pub gb_lights_set_fade_request {
# 	u8	light_id;
# 	u8	channel_id;
# 	u8	fade_in;
# 	u8	fade_out;
# }
# 
# /* event request: generated by module */
# #[repr(C, packed)]
# pub gb_lights_event_request {
# 	u8	light_id;
# 	u8	event;
# #define GB_LIGHTS_LIGHT_CONFIG		0x01
# }
# 
# /* set flash intensity request payload: response have no payload */
# #[repr(C, packed)]
# pub gb_lights_set_flash_intensity_request {
# 	u8	light_id;
# 	u8	channel_id;
# 	u32	intensity_uA;
# }
# 
# /* set flash strobe state request payload: response have no payload */
# #[repr(C, packed)]
# pub gb_lights_set_flash_strobe_request {
# 	u8	light_id;
# 	u8	channel_id;
# 	u8	state;
# }
# 
# /* set flash timeout request payload: response have no payload */
# #[repr(C, packed)]
# pub gb_lights_set_flash_timeout_request {
# 	u8	light_id;
# 	u8	channel_id;
# 	u32	timeout_us;
# }
# 
# /* get flash fault request payload */
# #[repr(C, packed)]
# pub gb_lights_get_flash_fault_request {
# 	u8	light_id;
# 	u8	channel_id;
# }
# 
# /* get flash fault response payload */
# #[repr(C, packed)]
# pub gb_lights_get_flash_fault_response {
# 	u32	fault;
# #define GB_LIGHTS_FLASH_FAULT_OVER_VOLTAGE		0x00000000
# #define GB_LIGHTS_FLASH_FAULT_TIMEOUT			0x00000001
# #define GB_LIGHTS_FLASH_FAULT_OVER_TEMPERATURE		0x00000002
# #define GB_LIGHTS_FLASH_FAULT_SHORT_CIRCUIT		0x00000004
# #define GB_LIGHTS_FLASH_FAULT_OVER_CURRENT		0x00000008
# #define GB_LIGHTS_FLASH_FAULT_INDICATOR			0x00000010
# #define GB_LIGHTS_FLASH_FAULT_UNDER_VOLTAGE		0x00000020
# #define GB_LIGHTS_FLASH_FAULT_INPUT_VOLTAGE		0x00000040
# #define GB_LIGHTS_FLASH_FAULT_LED_OVER_TEMPERATURE	0x00000080
# }
# 
# /* Audio */
# 
# #define GB_AUDIO_TYPE_GET_TOPOLOGY_SIZE		0x02
# #define GB_AUDIO_TYPE_GET_TOPOLOGY		0x03
# #define GB_AUDIO_TYPE_GET_CONTROL		0x04
# #define GB_AUDIO_TYPE_SET_CONTROL		0x05
# #define GB_AUDIO_TYPE_ENABLE_WIDGET		0x06
# #define GB_AUDIO_TYPE_DISABLE_WIDGET		0x07
# #define GB_AUDIO_TYPE_GET_PCM			0x08
# #define GB_AUDIO_TYPE_SET_PCM			0x09
# #define GB_AUDIO_TYPE_SET_TX_DATA_SIZE		0x0a
# 						/* 0x0b unused */
# #define GB_AUDIO_TYPE_ACTIVATE_TX		0x0c
# #define GB_AUDIO_TYPE_DEACTIVATE_TX		0x0d
# #define GB_AUDIO_TYPE_SET_RX_DATA_SIZE		0x0e
# 						/* 0x0f unused */
# #define GB_AUDIO_TYPE_ACTIVATE_RX		0x10
# #define GB_AUDIO_TYPE_DEACTIVATE_RX		0x11
# #define GB_AUDIO_TYPE_JACK_EVENT		0x12
# #define GB_AUDIO_TYPE_BUTTON_EVENT		0x13
# #define GB_AUDIO_TYPE_STREAMING_EVENT		0x14
# #define GB_AUDIO_TYPE_SEND_DATA			0x15
# 
# /* Module must be able to buffer 10ms of audio data, minimum */
# #define GB_AUDIO_SAMPLE_BUFFER_MIN_US		10000
# 
# #define GB_AUDIO_PCM_NAME_MAX			32
# #define AUDIO_DAI_NAME_MAX			32
# #define AUDIO_CONTROL_NAME_MAX			32
# #define AUDIO_CTL_ELEM_NAME_MAX			44
# #define AUDIO_ENUM_NAME_MAX			64
# #define AUDIO_WIDGET_NAME_MAX			32
# 
# /* See SNDRV_PCM_FMTBIT_* in Linux source */
# #define GB_AUDIO_PCM_FMT_S8			 (1u32 << 0)
# #define GB_AUDIO_PCM_FMT_U8			 (1u32 << 1)
# #define GB_AUDIO_PCM_FMT_S16_LE			 (1u32 << 2)
# #define GB_AUDIO_PCM_FMT_S16_BE			 (1u32 << 3)
# #define GB_AUDIO_PCM_FMT_U16_LE			 (1u32 << 4)
# #define GB_AUDIO_PCM_FMT_U16_BE			 (1u32 << 5)
# #define GB_AUDIO_PCM_FMT_S24_LE			 (1u32 << 6)
# #define GB_AUDIO_PCM_FMT_S24_BE			 (1u32 << 7)
# #define GB_AUDIO_PCM_FMT_U24_LE			 (1u32 << 8)
# #define GB_AUDIO_PCM_FMT_U24_BE			 (1u32 << 9)
# #define GB_AUDIO_PCM_FMT_S32_LE			 (1u32 << 10)
# #define GB_AUDIO_PCM_FMT_S32_BE			 (1u32 << 11)
# #define GB_AUDIO_PCM_FMT_U32_LE			 (1u32 << 12)
# #define GB_AUDIO_PCM_FMT_U32_BE			 (1u32 << 13)
# 
# /* See SNDRV_PCM_RATE_* in Linux source */
# #define GB_AUDIO_PCM_RATE_5512			 (1u32 << 0)
# #define GB_AUDIO_PCM_RATE_8000			 (1u32 << 1)
# #define GB_AUDIO_PCM_RATE_11025			 (1u32 << 2)
# #define GB_AUDIO_PCM_RATE_16000			 (1u32 << 3)
# #define GB_AUDIO_PCM_RATE_22050			 (1u32 << 4)
# #define GB_AUDIO_PCM_RATE_32000			 (1u32 << 5)
# #define GB_AUDIO_PCM_RATE_44100			 (1u32 << 6)
# #define GB_AUDIO_PCM_RATE_48000			 (1u32 << 7)
# #define GB_AUDIO_PCM_RATE_64000			 (1u32 << 8)
# #define GB_AUDIO_PCM_RATE_88200			 (1u32 << 9)
# #define GB_AUDIO_PCM_RATE_96000			 (1u32 << 10)
# #define GB_AUDIO_PCM_RATE_176400		 (1u32 << 11)
# #define GB_AUDIO_PCM_RATE_192000		 (1u32 << 12)
# 
# #define GB_AUDIO_STREAM_TYPE_CAPTURE		0x1
# #define GB_AUDIO_STREAM_TYPE_PLAYBACK		0x2
# 
# #define GB_AUDIO_CTL_ELEM_ACCESS_READ		 (1u32 << 0)
# #define GB_AUDIO_CTL_ELEM_ACCESS_WRITE		 (1u32 << 1)
# 
# /* See SNDRV_CTL_ELEM_TYPE_* in Linux source */
# #define GB_AUDIO_CTL_ELEM_TYPE_BOOLEAN		0x01
# #define GB_AUDIO_CTL_ELEM_TYPE_INTEGER		0x02
# #define GB_AUDIO_CTL_ELEM_TYPE_ENUMERATED	0x03
# #define GB_AUDIO_CTL_ELEM_TYPE_INTEGER64	0x06
# 
# /* See SNDRV_CTL_ELEM_IFACE_* in Linux source */
# #define GB_AUDIO_CTL_ELEM_IFACE_CARD		0x00
# #define GB_AUDIO_CTL_ELEM_IFACE_HWDEP		0x01
# #define GB_AUDIO_CTL_ELEM_IFACE_MIXER		0x02
# #define GB_AUDIO_CTL_ELEM_IFACE_PCM		0x03
# #define GB_AUDIO_CTL_ELEM_IFACE_RAWMIDI		0x04
# #define GB_AUDIO_CTL_ELEM_IFACE_TIMER		0x05
# #define GB_AUDIO_CTL_ELEM_IFACE_SEQUENCER	0x06
# 
# /* SNDRV_CTL_ELEM_ACCESS_* in Linux source */
# #define GB_AUDIO_ACCESS_READ			 (1u32 << 0)
# #define GB_AUDIO_ACCESS_WRITE			 (1u32 << 1)
# #define GB_AUDIO_ACCESS_VOLATILE		 (1u32 << 2)
# #define GB_AUDIO_ACCESS_TIMESTAMP		 (1u32 << 3)
# #define GB_AUDIO_ACCESS_TLV_READ		 (1u32 << 4)
# #define GB_AUDIO_ACCESS_TLV_WRITE		 (1u32 << 5)
# #define GB_AUDIO_ACCESS_TLV_COMMAND		 (1u32 << 6)
# #define GB_AUDIO_ACCESS_INACTIVE		 (1u32 << 7)
# #define GB_AUDIO_ACCESS_LOCK			 (1u32 << 8)
# #define GB_AUDIO_ACCESS_OWNER			 (1u32 << 9)
# 
# /* enum snd_soc_dapm_type */
# #define GB_AUDIO_WIDGET_TYPE_INPUT		0x0
# #define GB_AUDIO_WIDGET_TYPE_OUTPUT		0x1
# #define GB_AUDIO_WIDGET_TYPE_MUX		0x2
# #define GB_AUDIO_WIDGET_TYPE_VIRT_MUX		0x3
# #define GB_AUDIO_WIDGET_TYPE_VALUE_MUX		0x4
# #define GB_AUDIO_WIDGET_TYPE_MIXER		0x5
# #define GB_AUDIO_WIDGET_TYPE_MIXER_NAMED_CTL	0x6
# #define GB_AUDIO_WIDGET_TYPE_PGA		0x7
# #define GB_AUDIO_WIDGET_TYPE_OUT_DRV		0x8
# #define GB_AUDIO_WIDGET_TYPE_ADC		0x9
# #define GB_AUDIO_WIDGET_TYPE_DAC		0xa
# #define GB_AUDIO_WIDGET_TYPE_MICBIAS		0xb
# #define GB_AUDIO_WIDGET_TYPE_MIC		0xc
# #define GB_AUDIO_WIDGET_TYPE_HP			0xd
# #define GB_AUDIO_WIDGET_TYPE_SPK		0xe
# #define GB_AUDIO_WIDGET_TYPE_LINE		0xf
# #define GB_AUDIO_WIDGET_TYPE_SWITCH		0x10
# #define GB_AUDIO_WIDGET_TYPE_VMID		0x11
# #define GB_AUDIO_WIDGET_TYPE_PRE		0x12
# #define GB_AUDIO_WIDGET_TYPE_POST		0x13
# #define GB_AUDIO_WIDGET_TYPE_SUPPLY		0x14
# #define GB_AUDIO_WIDGET_TYPE_REGULATOR_SUPPLY	0x15
# #define GB_AUDIO_WIDGET_TYPE_CLOCK_SUPPLY	0x16
# #define GB_AUDIO_WIDGET_TYPE_AIF_IN		0x17
# #define GB_AUDIO_WIDGET_TYPE_AIF_OUT		0x18
# #define GB_AUDIO_WIDGET_TYPE_SIGGEN		0x19
# #define GB_AUDIO_WIDGET_TYPE_DAI_IN		0x1a
# #define GB_AUDIO_WIDGET_TYPE_DAI_OUT		0x1b
# #define GB_AUDIO_WIDGET_TYPE_DAI_LINK		0x1c
# 
# #define GB_AUDIO_WIDGET_STATE_DISABLED		0x01
# #define GB_AUDIO_WIDGET_STATE_ENAABLED		0x02
# 
# #define GB_AUDIO_JACK_EVENT_INSERTION		0x1
# #define GB_AUDIO_JACK_EVENT_REMOVAL		0x2
# 
# #define GB_AUDIO_BUTTON_EVENT_PRESS		0x1
# #define GB_AUDIO_BUTTON_EVENT_RELEASE		0x2
# 
# #define GB_AUDIO_STREAMING_EVENT_UNSPECIFIED	0x1
# #define GB_AUDIO_STREAMING_EVENT_HALT		0x2
# #define GB_AUDIO_STREAMING_EVENT_INTERNAL_ERROR	0x3
# #define GB_AUDIO_STREAMING_EVENT_PROTOCOL_ERROR	0x4
# #define GB_AUDIO_STREAMING_EVENT_FAILURE	0x5
# #define GB_AUDIO_STREAMING_EVENT_UNDERRUN	0x6
# #define GB_AUDIO_STREAMING_EVENT_OVERRUN	0x7
# #define GB_AUDIO_STREAMING_EVENT_CLOCKING	0x8
# #define GB_AUDIO_STREAMING_EVENT_DATA_LEN	0x9
# 
# #define GB_AUDIO_INVALID_INDEX			0xff
# 
# /* enum snd_jack_types */
# #define GB_AUDIO_JACK_HEADPHONE			0x0000001
# #define GB_AUDIO_JACK_MICROPHONE		0x0000002
# #define GB_AUDIO_JACK_HEADSET			(GB_AUDIO_JACK_HEADPHONE | \
# 						 GB_AUDIO_JACK_MICROPHONE)
# #define GB_AUDIO_JACK_LINEOUT			0x0000004
# #define GB_AUDIO_JACK_MECHANICAL		0x0000008
# #define GB_AUDIO_JACK_VIDEOOUT			0x0000010
# #define GB_AUDIO_JACK_AVOUT			(GB_AUDIO_JACK_LINEOUT | \
# 						 GB_AUDIO_JACK_VIDEOOUT)
# #define GB_AUDIO_JACK_LINEIN			0x0000020
# #define GB_AUDIO_JACK_OC_HPHL			0x0000040
# #define GB_AUDIO_JACK_OC_HPHR			0x0000080
# #define GB_AUDIO_JACK_MICROPHONE2		0x0000200
# #define GB_AUDIO_JACK_ANC_HEADPHONE		(GB_AUDIO_JACK_HEADPHONE | \
# 						 GB_AUDIO_JACK_MICROPHONE | \
# 						 GB_AUDIO_JACK_MICROPHONE2)
# /* Kept separate from switches to facilitate implementation */
# #define GB_AUDIO_JACK_BTN_0			0x4000000
# #define GB_AUDIO_JACK_BTN_1			0x2000000
# #define GB_AUDIO_JACK_BTN_2			0x1000000
# #define GB_AUDIO_JACK_BTN_3			0x0800000
# 
# #[repr(C, packed)]
# pub gb_audio_pcm {
# 	u8	stream_name: [u8; GB_AUDIO_PCM_NAME_MAX as usize];
# 	u32	formats;	/* GB_AUDIO_PCM_FMT_* */
# 	u32	rates;		/* GB_AUDIO_PCM_RATE_* */
# 	u8	chan_min;
# 	u8	chan_max;
# 	u8	sig_bits;	/* number of bits of content */
# }
# 
# #[repr(C, packed)]
# pub gb_audio_dai {
# 	u8			name: [u8; AUDIO_DAI_NAME_MAX as usize];
# 	u16			data_cport;
# 	gb_audio_pcm	capture;
# 	gb_audio_pcm	playback;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_integer {
# 	u32	min;
# 	u32	max;
# 	u32	step;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_integer64 {
# 	u64	min;
# 	u64	max;
# 	u64	step;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_enumerated {
# 	u32	items;
# 	u16	names_length;
# 	u8	names: [u8; 0];
# }
# 
# #[repr(C, packed)]
# pub gb_audio_ctl_elem_info { /* See snd_ctl_elem_info in Linux source */
# 	u8		type;		/* GB_AUDIO_CTL_ELEM_TYPE_* */
# 	u16		dimen: [u8; 4];
# 	#[repr(C)]
# pub union {
# 		gb_audio_integer		integer;
# 		gb_audio_integer64	integer64;
# 		gb_audio_enumerated	enumerated;
# 	} value;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_ctl_elem_value { /* See snd_ctl_elem_value in Linux source */
# 	u64				timestamp; /* XXX needed? */
# 	#[repr(C)]
# pub union {
# 		u32	integer_value: [u8; 2];	/* consider CTL_DOUBLE_xxx */
# 		u64	integer64_value: [u8; 2];
# 		u32	enumerated_item: [u8; 2];
# 	} value;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_control {
# 	u8	name: [u8; AUDIO_CONTROL_NAME_MAX as usize];
# 	u8	id;		/* 0-63 */
# 	u8	iface;		/* GB_AUDIO_IFACE_* */
# 	u16	data_cport;
# 	u32	access;		/* GB_AUDIO_ACCESS_* */
# 	u8    count;		/* count of same elements */
# 	u8	count_values;	/* count of values, max=2 for CTL_DOUBLE_xxx */
# 	gb_audio_ctl_elem_info	info;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_widget {
# 	u8	name: [u8; AUDIO_WIDGET_NAME_MAX as usize];
# 	u8	sname: [u8; AUDIO_WIDGET_NAME_MAX as usize];
# 	u8	id;
# 	u8	type;		/* GB_AUDIO_WIDGET_TYPE_* */
# 	u8	state;		/* GB_AUDIO_WIDGET_STATE_* */
# 	u8	ncontrols;
# 	gb_audio_control	ctl: [u8; 0];	/* 'ncontrols' entries */
# }
# 
# #[repr(C, packed)]
# pub gb_audio_route {
# 	u8	source_id;	/* widget id */
# 	u8	destination_id;	/* widget id */
# 	u8	control_id;	/* 0-63 */
# 	u8	index;		/* Selection within the control */
# }
# 
# #[repr(C, packed)]
# pub gb_audio_topology {
# 	u8	num_dais;
# 	u8	num_controls;
# 	u8	num_widgets;
# 	u8	num_routes;
# 	u32	size_dais;
# 	u32	size_controls;
# 	u32	size_widgets;
# 	u32	size_routes;
# 	u32	jack_type;
# 	/*
# 	 * gb_audio_dai		dai: [u8; num_dais as usize];
# 	 * gb_audio_control	controls: [u8; num_controls as usize];
# 	 * gb_audio_widget	widgets: [u8; num_widgets as usize];
# 	 * gb_audio_route	routes: [u8; num_routes as usize];
# 	 */
# 	u8	data: [u8; 0];
# }
# 
# #[repr(C, packed)]
# pub gb_audio_get_topology_size_response {
# 	u16	size;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_get_topology_response {
# 	gb_audio_topology	topology;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_get_control_request {
# 	u8	control_id;
# 	u8	index;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_get_control_response {
# 	gb_audio_ctl_elem_value	value;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_set_control_request {
# 	u8	control_id;
# 	u8	index;
# 	gb_audio_ctl_elem_value	value;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_enable_widget_request {
# 	u8	widget_id;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_disable_widget_request {
# 	u8	widget_id;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_get_pcm_request {
# 	u16	data_cport;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_get_pcm_response {
# 	u32	format;
# 	u32	rate;
# 	u8	channels;
# 	u8	sig_bits;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_set_pcm_request {
# 	u16	data_cport;
# 	u32	format;
# 	u32	rate;
# 	u8	channels;
# 	u8	sig_bits;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_set_tx_data_size_request {
# 	u16	data_cport;
# 	u16	size;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_activate_tx_request {
# 	u16	data_cport;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_deactivate_tx_request {
# 	u16	data_cport;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_set_rx_data_size_request {
# 	u16	data_cport;
# 	u16	size;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_activate_rx_request {
# 	u16	data_cport;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_deactivate_rx_request {
# 	u16	data_cport;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_jack_event_request {
# 	u8	widget_id;
# 	u8	jack_attribute;
# 	u8	event;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_button_event_request {
# 	u8	widget_id;
# 	u8	button_id;
# 	u8	event;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_streaming_event_request {
# 	u16	data_cport;
# 	u8	event;
# }
# 
# #[repr(C, packed)]
# pub gb_audio_send_data_request {
# 	u64	timestamp;
# 	u8	data: [u8; 0];
# }
# 
# 
# /* Log */
# 
# /* operations */
# #define GB_LOG_TYPE_SEND_LOG	0x02
# 
# /* length */
# #define GB_LOG_MAX_LEN		1024
# 
# #[repr(C, packed)]
# pub gb_log_send_log_request {
# 	u16	len;
# 	u8    msg: [u8; 0];
# }
# 
# 
# 

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
