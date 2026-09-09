/* SPDX-License-Identifier: GPL-2.0 */
/*
   BlueZ - Bluetooth protocol stack for Linux
   Copyright (C) 2000-2001 Qualcomm Incorporated
   Copyright 2023-2024 NXP

   Written 2000,2001 by Maxim Krasnyansky <maxk@qualcomm.com>

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
   OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
   IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
   CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
   WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
   ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
   OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

   ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
   COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
   SOFTWARE IS DISCLAIMED.
*/

// #ifndef __HCI_H
// #define __HCI_H

pub const HCI_MAX_ACL_SIZE: _ = 1024;
pub const HCI_MAX_SCO_SIZE: _ = 255;
pub const HCI_MAX_ISO_SIZE: _ = 251;
pub const HCI_MAX_ISO_BIS: _ = 31;
pub const HCI_MAX_EVENT_SIZE: _ = 260;
pub const HCI_MAX_FRAME_SIZE: _ = (HCI_MAX_ACL_SIZE + 4);

pub const HCI_LINK_KEY_SIZE: _ = 16;

pub const HCI_MAX_CPB_DATA_SIZE: _ = 252;

/* HCI dev events */
pub const HCI_DEV_REG: _ = 1;
pub const HCI_DEV_UNREG: _ = 2;
pub const HCI_DEV_UP: _ = 3;
pub const HCI_DEV_DOWN: _ = 4;
pub const HCI_DEV_SUSPEND: _ = 5;
pub const HCI_DEV_RESUME: _ = 6;
pub const HCI_DEV_OPEN: _ = 7;
pub const HCI_DEV_CLOSE: _ = 8;
pub const HCI_DEV_SETUP: _ = 9;

/* HCI notify events */
pub const HCI_NOTIFY_CONN_ADD: _ = 1;
pub const HCI_NOTIFY_CONN_DEL: _ = 2;
pub const HCI_NOTIFY_VOICE_SETTING: _ = 3;
pub const HCI_NOTIFY_ENABLE_SCO_CVSD: _ = 4;
pub const HCI_NOTIFY_ENABLE_SCO_TRANSP: _ = 5;
pub const HCI_NOTIFY_DISABLE_SCO: _ = 6;

/* HCI bus types */
pub const HCI_VIRTUAL: _ = 0;
pub const HCI_USB: _ = 1;
pub const HCI_PCCARD: _ = 2;
pub const HCI_UART: _ = 3;
pub const HCI_RS232: _ = 4;
pub const HCI_PCI: _ = 5;
pub const HCI_SDIO: _ = 6;
pub const HCI_SPI: _ = 7;
pub const HCI_I2C: _ = 8;
pub const HCI_SMD: _ = 9;
pub const HCI_VIRTIO: _ = 10;
pub const HCI_IPC: _ = 11;

/* HCI device quirks */
enum {
	/* When this quirk is set, the HCI Reset command is send when
	 * closing the transport instead of when opening it.
	 *
	 * This quirk must be set before hci_register_dev is called.
	 */
	HCI_QUIRK_RESET_ON_CLOSE,

	/* When this quirk is set, the device is turned into a raw-only
	 * device and it will stay in unconfigured state.
	 *
	 * This quirk must be set before hci_register_dev is called.
	 */
	HCI_QUIRK_RAW_DEVICE,

	/* When this quirk is set, the buffer sizes reported by
	 * HCI Read Buffer Size command are corrected if invalid.
	 *
	 * This quirk must be set before hci_register_dev is called.
	 */
	HCI_QUIRK_FIXUP_BUFFER_SIZE,

	/* When this quirk is set, then a controller that does not
	 * indicate support for Inquiry Result with RSSI is assumed to
	 * support it anyway. Some early Bluetooth 1.2 controllers had
	 * wrongly configured local features that will require forcing
	 * them to enable this mode. Getting RSSI information with the
	 * inquiry responses is preferred since it allows for a better
	 * user experience.
	 *
	 * This quirk must be set before hci_register_dev is called.
	 */
	HCI_QUIRK_FIXUP_INQUIRY_MODE,

	/* When this quirk is set, then the HCI Read Local Supported
	 * Commands command is not supported. In general Bluetooth 1.2
	 * and later controllers should support this command. However
	 * some controllers indicate Bluetooth 1.2 support, but do
	 * not support this command.
	 *
	 * This quirk must be set before hci_register_dev is called.
	 */
	HCI_QUIRK_BROKEN_LOCAL_COMMANDS,

	/* When this quirk is set, then no stored link key handling
	 * is performed. This is mainly due to the fact that the
	 * HCI Delete Stored Link Key command is advertised, but
	 * not supported.
	 *
	 * This quirk must be set before hci_register_dev is called.
	 */
	HCI_QUIRK_BROKEN_STORED_LINK_KEY,

	/* When this quirk is set, an external configuration step
	 * is required and will be indicated with the controller
	 * configuration.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_EXTERNAL_CONFIG,

	/* When this quirk is set, the public Bluetooth address
	 * initially reported by HCI Read BD Address command
	 * is considered invalid. Controller configuration is
	 * required before this device can be used.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_INVALID_BDADDR,

	/* When this quirk is set, the public Bluetooth address
	 * initially reported by HCI Read BD Address command
	 * is considered invalid. The public BD Address can be
	 * specified in the fwnode property 'local-bd-address'.
	 * If this property does not exist or is invalid controller
	 * configuration is required before this device can be used.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_USE_BDADDR_PROPERTY,

	/* When this quirk is set, the Bluetooth Device Address provided by
	 * the 'local-bd-address' fwnode property is incorrectly specified in
	 * big-endian order.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_BDADDR_PROPERTY_BROKEN,

	/* When this quirk is set, the duplicate filtering during
	 * scanning is based on Bluetooth devices addresses. To allow
	 * RSSI based updates, restart scanning if needed.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_STRICT_DUPLICATE_FILTER,

	/* When this quirk is set, LE scan and BR/EDR inquiry is done
	 * simultaneously, otherwise it's interleaved.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_SIMULTANEOUS_DISCOVERY,

	/* When this quirk is set, the enabling of diagnostic mode is
	 * not persistent over HCI Reset. Every time the controller
	 * is brought up it needs to be reprogrammed.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_NON_PERSISTENT_DIAG,

	/* When this quirk is set, setup() would be run after every
	 * open() and not just after the first open().
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 *
	 */
	HCI_QUIRK_NON_PERSISTENT_SETUP,

	/* When this quirk is set, wide band speech is supported by
	 * the driver since no reliable mechanism exist to report
	 * this from the hardware, a driver flag is use to convey
	 * this support
	 *
	 * This quirk must be set before hci_register_dev is called.
	 */
	HCI_QUIRK_WIDEBAND_SPEECH_SUPPORTED,

	/* When this quirk is set consider Sync Flow Control as supported by
	 * the driver.
	 *
	 * This quirk must be set before hci_register_dev is called.
	 */
	HCI_QUIRK_SYNC_FLOWCTL_SUPPORTED,

	/* When this quirk is set, the LE states reported through the
	 * HCI_LE_READ_SUPPORTED_STATES are invalid/broken.
	 *
	 * This mechanism is necessary as many controllers have been seen has
	 * having trouble initiating a connectable advertisement despite the
	 * state combination being reported as supported.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_BROKEN_LE_STATES,

	/* When this quirk is set, then erroneous data reporting
	 * is ignored. This is mainly due to the fact that the HCI
	 * Read Default Erroneous Data Reporting command is advertised,
	 * but not supported; these controllers often reply with unknown
	 * command and tend to lock up randomly. Needing a hard reset.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_BROKEN_ERR_DATA_REPORTING,

	/*
	 * When this quirk is set, then the hci_suspend_notifier is not
	 * registered. This is intended for devices which drop completely
	 * from the bus on system-suspend and which will show up as a new
	 * HCI after resume.
	 */
	HCI_QUIRK_NO_SUSPEND_NOTIFIER,

	/*
	 * When this quirk is set, LE tx power is not queried on startup
	 * and the min/max tx power values default to HCI_TX_POWER_INVALID.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_BROKEN_READ_TRANSMIT_POWER,

	/* When this quirk is set, HCI_OP_SET_EVENT_FLT requests with
	 * HCI_FLT_CLEAR_ALL are ignored and event filtering is
	 * completely avoided. A subset of the CSR controller
	 * clones struggle with this and instantly lock up.
	 *
	 * Note that devices using this must (separately) disable
	 * runtime suspend, because event filtering takes place there.
	 */
	HCI_QUIRK_BROKEN_FILTER_CLEAR_ALL,

	/*
	 * When this quirk is set, disables the use of
	 * HCI_OP_ENHANCED_SETUP_SYNC_CONN command to setup SCO connections.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_BROKEN_ENHANCED_SETUP_SYNC_CONN,

	/*
	 * When this quirk is set, the HCI_OP_LE_SET_EXT_SCAN_ENABLE command is
	 * disabled. This is required for some Broadcom controllers which
	 * erroneously claim to support extended scanning.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_BROKEN_EXT_SCAN,

	/*
	 * When this quirk is set, the HCI_OP_GET_MWS_TRANSPORT_CONFIG command is
	 * disabled. This is required for some Broadcom controllers which
	 * erroneously claim to support MWS Transport Layer Configuration.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_BROKEN_MWS_TRANSPORT_CONFIG,

	/* When this quirk is set, max_page for local extended features
	 * is set to 1, even if controller reports higher number. Some
	 * controllers (e.g. RTL8723CS) report more pages, but they
	 * don't actually support features declared there.
	 */
	HCI_QUIRK_BROKEN_LOCAL_EXT_FEATURES_PAGE_2,

	/*
	 * When this quirk is set, the HCI_OP_LE_SET_RPA_TIMEOUT command is
	 * skipped during initialization. This is required for the Actions
	 * Semiconductor ATS2851 based controllers, which erroneously claims
	 * to support it.
	 */
	HCI_QUIRK_BROKEN_SET_RPA_TIMEOUT,

	/*
	 * When this quirk is set, the HCI_OP_LE_EXT_CREATE_CONN command is
	 * disabled. This is required for the Actions Semiconductor ATS2851
	 * based controllers, which erroneously claims to support it.
	 */
	HCI_QUIRK_BROKEN_EXT_CREATE_CONN,

	/*
	 * When this quirk is set, the command WRITE_AUTH_PAYLOAD_TIMEOUT is
	 * skipped. This is required for the Actions Semiconductor ATS2851
	 * based controllers, due to a race condition in pairing process.
	 */
	HCI_QUIRK_BROKEN_WRITE_AUTH_PAYLOAD_TIMEOUT,

	/* When this quirk is set, MSFT extension monitor tracking by
	 * address filter is supported. Since tracking quantity of each
	 * pattern is limited, this feature supports tracking multiple
	 * devices concurrently if controller supports multiple
	 * address filters.
	 *
	 * This quirk must be set before hci_register_dev is called.
	 */
	HCI_QUIRK_USE_MSFT_EXT_ADDRESS_FILTER,

	/*
	 * When this quirk is set, LE Coded PHY shall not be used. This is
	 * required for some Intel controllers which erroneously claim to
	 * support it but it causes problems with extended scanning.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_BROKEN_LE_CODED,

	/*
	 * When this quirk is set, the HCI_OP_READ_ENC_KEY_SIZE command is
	 * skipped during an HCI_EV_ENCRYPT_CHANGE event. This is required
	 * for Actions Semiconductor ATS2851 based controllers, which erroneously
	 * claim to support it.
	 */
	HCI_QUIRK_BROKEN_READ_ENC_KEY_SIZE,

	/*
	 * When this quirk is set, the reserved bits of Primary/Secondary_PHY
	 * inside the LE Extended Advertising Report events are discarded.
	 * This is required for some Apple/Broadcom controllers which
	 * abuse these reserved bits for unrelated flags.
	 *
	 * This quirk can be set before hci_register_dev is called or
	 * during the hdev->setup vendor callback.
	 */
	HCI_QUIRK_FIXUP_LE_EXT_ADV_REPORT_PHY,

	/* When this quirk is set, the HCI_OP_READ_VOICE_SETTING command is
	 * skipped. This is required for a subset of the CSR controller clones
	 * which erroneously claim to support it.
	 *
	 * This quirk must be set before hci_register_dev is called.
	 */
	HCI_QUIRK_BROKEN_READ_VOICE_SETTING,

	/* When this quirk is set, the HCI_OP_READ_PAGE_SCAN_TYPE command is
	 * skipped. This is required for a subset of the CSR controller clones
	 * which erroneously claim to support it.
	 *
	 * This quirk must be set before hci_register_dev is called.
	 */
	HCI_QUIRK_BROKEN_READ_PAGE_SCAN_TYPE,

	__HCI_NUM_QUIRKS,
};

/* HCI device flags */
enum {
	HCI_UP,
	HCI_INIT,
	HCI_RUNNING,

	HCI_PSCAN,
	HCI_ISCAN,
	HCI_AUTH,
	HCI_ENCRYPT,
	HCI_INQUIRY,

	HCI_RAW,

	HCI_RESET,
};

/* HCI socket flags */
enum {
	HCI_SOCK_TRUSTED,
	HCI_MGMT_INDEX_EVENTS,
	HCI_MGMT_UNCONF_INDEX_EVENTS,
	HCI_MGMT_EXT_INDEX_EVENTS,
	HCI_MGMT_EXT_INFO_EVENTS,
	HCI_MGMT_OPTION_EVENTS,
	HCI_MGMT_SETTING_EVENTS,
	HCI_MGMT_DEV_CLASS_EVENTS,
	HCI_MGMT_LOCAL_NAME_EVENTS,
	HCI_MGMT_OOB_DATA_EVENTS,
	HCI_MGMT_EXP_FEATURE_EVENTS,
};

/*
 * BR/EDR and/or LE controller flags: the flags defined here should represent
 * states from the controller.
 */
enum {
	HCI_SETUP,
	HCI_CONFIG,
	HCI_DEBUGFS_CREATED,
	HCI_POWERING_DOWN,
	HCI_AUTO_OFF,
	HCI_RFKILLED,
	HCI_MGMT,
	HCI_BONDABLE,
	HCI_SERVICE_CACHE,
	HCI_KEEP_DEBUG_KEYS,
	HCI_USE_DEBUG_KEYS,
	HCI_UNREGISTER,
	HCI_UNCONFIGURED,
	HCI_USER_CHANNEL,
	HCI_EXT_CONFIGURED,
	HCI_LE_ADV,
	HCI_LE_ADV_0,
	HCI_LE_PER_ADV,
	HCI_LE_SCAN,
	HCI_SSP_ENABLED,
	HCI_SC_ENABLED,
	HCI_SC_ONLY,
	HCI_PRIVACY,
	HCI_LIMITED_PRIVACY,
	HCI_RPA_EXPIRED,
	HCI_RPA_RESOLVING,
	HCI_LE_ENABLED,
	HCI_ADVERTISING,
	HCI_ADVERTISING_CONNECTABLE,
	HCI_CONNECTABLE,
	HCI_DISCOVERABLE,
	HCI_LIMITED_DISCOVERABLE,
	HCI_LINK_SECURITY,
	HCI_PERIODIC_INQ,
	HCI_FAST_CONNECTABLE,
	HCI_BREDR_ENABLED,
	HCI_LE_SCAN_INTERRUPTED,
	HCI_WIDEBAND_SPEECH_ENABLED,
	HCI_EVENT_FILTER_CONFIGURED,
	HCI_PA_SYNC,
	HCI_SCO_FLOWCTL,

	HCI_DUT_MODE,
	HCI_VENDOR_DIAG,
	HCI_FORCE_BREDR_SMP,
	HCI_FORCE_STATIC_ADDR,
	HCI_LL_RPA_RESOLUTION,
	HCI_CMD_PENDING,
	HCI_FORCE_NO_MITM,
	HCI_QUALITY_REPORT,
	HCI_OFFLOAD_CODECS_ENABLED,
	HCI_LE_SIMULTANEOUS_ROLES,
	HCI_CMD_DRAIN_WORKQUEUE,

	HCI_MESH_EXPERIMENTAL,
	HCI_MESH,
	HCI_MESH_SENDING,

	__HCI_NUM_FLAGS,
};

/* HCI timeouts */
pub const HCI_DISCONN_TIMEOUT: _ = msecs_to_jiffies(2000)	/* 2 seconds */;
pub const HCI_PAIRING_TIMEOUT: _ = msecs_to_jiffies(60000)	/* 60 seconds */;
pub const HCI_INIT_TIMEOUT: _ = msecs_to_jiffies(10000)	/* 10 seconds */;
pub const HCI_CMD_TIMEOUT: _ = msecs_to_jiffies(2000)	/* 2 seconds */;
pub const HCI_NCMD_TIMEOUT: _ = msecs_to_jiffies(4000)	/* 4 seconds */;
pub const HCI_ACL_TX_TIMEOUT: _ = msecs_to_jiffies(45000)	/* 45 seconds */;
pub const HCI_AUTO_OFF_TIMEOUT: _ = msecs_to_jiffies(2000)	/* 2 seconds */;
pub const HCI_ACL_CONN_TIMEOUT: _ = msecs_to_jiffies(20000)	/* 20 seconds */;
pub const HCI_LE_CONN_TIMEOUT: _ = msecs_to_jiffies(20000)	/* 20 seconds */;
pub const HCI_ISO_TX_TIMEOUT: _ = usecs_to_jiffies(0x7fffff) /* 8388607 usecs */;

/* HCI data types */
pub const HCI_COMMAND_PKT: _ = 0x01;
pub const HCI_ACLDATA_PKT: _ = 0x02;
pub const HCI_SCODATA_PKT: _ = 0x03;
pub const HCI_EVENT_PKT: _ = 0x04;
pub const HCI_ISODATA_PKT: _ = 0x05;
pub const HCI_DIAG_PKT: _ = 0xf0;
pub const HCI_DRV_PKT: _ = 0xf1;
pub const HCI_VENDOR_PKT: _ = 0xff;

/* HCI packet types */
pub const HCI_DM1: _ = 0x0008;
pub const HCI_DM3: _ = 0x0400;
pub const HCI_DM5: _ = 0x4000;
pub const HCI_DH1: _ = 0x0010;
pub const HCI_DH3: _ = 0x0800;
pub const HCI_DH5: _ = 0x8000;

/* HCI packet types inverted masks */
pub const HCI_2DH1: _ = 0x0002;
pub const HCI_3DH1: _ = 0x0004;
pub const HCI_2DH3: _ = 0x0100;
pub const HCI_3DH3: _ = 0x0200;
pub const HCI_2DH5: _ = 0x1000;
pub const HCI_3DH5: _ = 0x2000;

pub const HCI_HV1: _ = 0x0020;
pub const HCI_HV2: _ = 0x0040;
pub const HCI_HV3: _ = 0x0080;

pub const SCO_PTYPE_MASK: _ = (HCI_HV1 | HCI_HV2 | HCI_HV3);
pub const ACL_PTYPE_MASK: _ = (~SCO_PTYPE_MASK);

/* eSCO packet types */
pub const ESCO_HV1: _ = 0x0001;
pub const ESCO_HV2: _ = 0x0002;
pub const ESCO_HV3: _ = 0x0004;
pub const ESCO_EV3: _ = 0x0008;
pub const ESCO_EV4: _ = 0x0010;
pub const ESCO_EV5: _ = 0x0020;
pub const ESCO_2EV3: _ = 0x0040;
pub const ESCO_3EV3: _ = 0x0080;
pub const ESCO_2EV5: _ = 0x0100;
pub const ESCO_3EV5: _ = 0x0200;

pub const SCO_ESCO_MASK: _ = (ESCO_HV1 | ESCO_HV2 | ESCO_HV3);
pub const EDR_ESCO_MASK: _ = (ESCO_2EV3 | ESCO_3EV3 | ESCO_2EV5 | ESCO_3EV5);

/* ACL flags */
pub const ACL_START_NO_FLUSH: _ = 0x00;
pub const ACL_CONT: _ = 0x01;
pub const ACL_START: _ = 0x02;
pub const ACL_COMPLETE: _ = 0x03;
pub const ACL_ACTIVE_BCAST: _ = 0x04;
pub const ACL_PICO_BCAST: _ = 0x08;

/* ISO PB flags */
pub const ISO_START: _ = 0x00;
pub const ISO_CONT: _ = 0x01;
pub const ISO_SINGLE: _ = 0x02;
pub const ISO_END: _ = 0x03;

/* ISO TS flags */
pub const ISO_TS: _ = 0x01;

/* Baseband links */
pub const SCO_LINK: _ = 0x00;
pub const ACL_LINK: _ = 0x01;
pub const ESCO_LINK: _ = 0x02;
/* Low Energy links do not have defined link type. Use invented one */
pub const LE_LINK: _ = 0x80;
pub const CIS_LINK: _ = 0x82;
pub const BIS_LINK: _ = 0x83;
pub const PA_LINK: _ = 0x84;
pub const INVALID_LINK: _ = 0xff;

/* LMP features */
pub const LMP_3SLOT: _ = 0x01;
pub const LMP_5SLOT: _ = 0x02;
pub const LMP_ENCRYPT: _ = 0x04;
pub const LMP_SOFFSET: _ = 0x08;
pub const LMP_TACCURACY: _ = 0x10;
pub const LMP_RSWITCH: _ = 0x20;
pub const LMP_HOLD: _ = 0x40;
pub const LMP_SNIFF: _ = 0x80;

pub const LMP_PARK: _ = 0x01;
pub const LMP_RSSI: _ = 0x02;
pub const LMP_QUALITY: _ = 0x04;
pub const LMP_SCO: _ = 0x08;
pub const LMP_HV2: _ = 0x10;
pub const LMP_HV3: _ = 0x20;
pub const LMP_ULAW: _ = 0x40;
pub const LMP_ALAW: _ = 0x80;

pub const LMP_CVSD: _ = 0x01;
pub const LMP_PSCHEME: _ = 0x02;
pub const LMP_PCONTROL: _ = 0x04;
pub const LMP_TRANSPARENT: _ = 0x08;

pub const LMP_EDR_2M: _ = 0x02;
pub const LMP_EDR_3M: _ = 0x04;
pub const LMP_RSSI_INQ: _ = 0x40;
pub const LMP_ESCO: _ = 0x80;

pub const LMP_EV4: _ = 0x01;
pub const LMP_EV5: _ = 0x02;
pub const LMP_NO_BREDR: _ = 0x20;
pub const LMP_LE: _ = 0x40;
pub const LMP_EDR_3SLOT: _ = 0x80;

pub const LMP_EDR_5SLOT: _ = 0x01;
pub const LMP_SNIFF_SUBR: _ = 0x02;
pub const LMP_PAUSE_ENC: _ = 0x04;
pub const LMP_EDR_ESCO_2M: _ = 0x20;
pub const LMP_EDR_ESCO_3M: _ = 0x40;
pub const LMP_EDR_3S_ESCO: _ = 0x80;

pub const LMP_EXT_INQ: _ = 0x01;
pub const LMP_SIMUL_LE_BR: _ = 0x02;
pub const LMP_SIMPLE_PAIR: _ = 0x08;
pub const LMP_ERR_DATA_REPORTING: _ = 0x20;
pub const LMP_NO_FLUSH: _ = 0x40;

pub const LMP_LSTO: _ = 0x01;
pub const LMP_INQ_TX_PWR: _ = 0x02;
pub const LMP_EXTFEATURES: _ = 0x80;

/* Extended LMP features */
pub const LMP_CPB_CENTRAL: _ = 0x01;
pub const LMP_CPB_PERIPHERAL: _ = 0x02;
pub const LMP_SYNC_TRAIN: _ = 0x04;
pub const LMP_SYNC_SCAN: _ = 0x08;

pub const LMP_SC: _ = 0x01;
pub const LMP_PING: _ = 0x02;

/* Host features */
pub const LMP_HOST_SSP: _ = 0x01;
pub const LMP_HOST_LE: _ = 0x02;
pub const LMP_HOST_LE_BREDR: _ = 0x04;
pub const LMP_HOST_SC: _ = 0x08;

/* LE features */
pub const HCI_LE_ENCRYPTION: _ = 0x01;
pub const HCI_LE_CONN_PARAM_REQ_PROC: _ = 0x02;
pub const HCI_LE_PERIPHERAL_FEATURES: _ = 0x08;
pub const HCI_LE_PING: _ = 0x10;
pub const HCI_LE_DATA_LEN_EXT: _ = 0x20;
pub const HCI_LE_LL_PRIVACY: _ = 0x40;
pub const HCI_LE_EXT_SCAN_POLICY: _ = 0x80;
pub const HCI_LE_PHY_2M: _ = 0x01;
pub const HCI_LE_PHY_CODED: _ = 0x08;
pub const HCI_LE_EXT_ADV: _ = 0x10;
pub const HCI_LE_PERIODIC_ADV: _ = 0x20;
pub const HCI_LE_CHAN_SEL_ALG2: _ = 0x40;
pub const HCI_LE_PAST_SENDER: _ = 0x01;
pub const HCI_LE_PAST_RECEIVER: _ = 0x02;
pub const HCI_LE_CIS_CENTRAL: _ = 0x10;
pub const HCI_LE_CIS_PERIPHERAL: _ = 0x20;
pub const HCI_LE_ISO_BROADCASTER: _ = 0x40;
pub const HCI_LE_ISO_SYNC_RECEIVER: _ = 0x80;
pub const HCI_LE_LL_EXT_FEATURE: _ = 0x80;
pub const HCI_LE_CS: _ = 0x40;
pub const HCI_LE_CS_HOST: _ = 0x80;
pub const HCI_LE_SCI: _ = 0x01	/* byte 9 - Shorter Connection Intervals */;
pub const HCI_LE_SCI_HOST: _ = 0x02	/* byte 9 - Shorter Connection Intervals (Host) */;

/* Connection modes */
pub const HCI_CM_ACTIVE: _ = 0x0000;
pub const HCI_CM_HOLD: _ = 0x0001;
pub const HCI_CM_SNIFF: _ = 0x0002;
pub const HCI_CM_PARK: _ = 0x0003;

/* Link policies */
pub const HCI_LP_RSWITCH: _ = 0x0001;
pub const HCI_LP_HOLD: _ = 0x0002;
pub const HCI_LP_SNIFF: _ = 0x0004;
pub const HCI_LP_PARK: _ = 0x0008;

/* Link modes */
pub const HCI_LM_ACCEPT: _ = 0x8000;
pub const HCI_LM_MASTER: _ = 0x0001;
pub const HCI_LM_AUTH: _ = 0x0002;
pub const HCI_LM_ENCRYPT: _ = 0x0004;
pub const HCI_LM_TRUSTED: _ = 0x0008;
pub const HCI_LM_RELIABLE: _ = 0x0010;
pub const HCI_LM_SECURE: _ = 0x0020;
pub const HCI_LM_FIPS: _ = 0x0040;

/* Authentication types */
pub const HCI_AT_NO_BONDING: _ = 0x00;
pub const HCI_AT_NO_BONDING_MITM: _ = 0x01;
pub const HCI_AT_DEDICATED_BONDING: _ = 0x02;
pub const HCI_AT_DEDICATED_BONDING_MITM: _ = 0x03;
pub const HCI_AT_GENERAL_BONDING: _ = 0x04;
pub const HCI_AT_GENERAL_BONDING_MITM: _ = 0x05;

/* I/O capabilities */
pub const HCI_IO_DISPLAY_ONLY: _ = 0x00;
pub const HCI_IO_DISPLAY_YESNO: _ = 0x01;
pub const HCI_IO_KEYBOARD_ONLY: _ = 0x02;
pub const HCI_IO_NO_INPUT_OUTPUT: _ = 0x03;

/* Link Key types */
pub const HCI_LK_COMBINATION: _ = 0x00;
pub const HCI_LK_LOCAL_UNIT: _ = 0x01;
pub const HCI_LK_REMOTE_UNIT: _ = 0x02;
pub const HCI_LK_DEBUG_COMBINATION: _ = 0x03;
pub const HCI_LK_UNAUTH_COMBINATION_P192: _ = 0x04;
pub const HCI_LK_AUTH_COMBINATION_P192: _ = 0x05;
pub const HCI_LK_CHANGED_COMBINATION: _ = 0x06;
pub const HCI_LK_UNAUTH_COMBINATION_P256: _ = 0x07;
pub const HCI_LK_AUTH_COMBINATION_P256: _ = 0x08;

/* ---- HCI Error Codes ---- */
pub const HCI_ERROR_UNKNOWN_CONN_ID: _ = 0x02;
pub const HCI_ERROR_AUTH_FAILURE: _ = 0x05;
pub const HCI_ERROR_PIN_OR_KEY_MISSING: _ = 0x06;
pub const HCI_ERROR_MEMORY_EXCEEDED: _ = 0x07;
pub const HCI_ERROR_CONNECTION_TIMEOUT: _ = 0x08;
pub const HCI_ERROR_COMMAND_DISALLOWED: _ = 0x0c;
pub const HCI_ERROR_REJ_LIMITED_RESOURCES: _ = 0x0d;
pub const HCI_ERROR_REJ_BAD_ADDR: _ = 0x0f;
pub const HCI_ERROR_INVALID_PARAMETERS: _ = 0x12;
pub const HCI_ERROR_REMOTE_USER_TERM: _ = 0x13;
pub const HCI_ERROR_REMOTE_LOW_RESOURCES: _ = 0x14;
pub const HCI_ERROR_REMOTE_POWER_OFF: _ = 0x15;
pub const HCI_ERROR_LOCAL_HOST_TERM: _ = 0x16;
pub const HCI_ERROR_PAIRING_NOT_ALLOWED: _ = 0x18;
pub const HCI_ERROR_UNSUPPORTED_REMOTE_FEATURE: _ = 0x1a;
pub const HCI_ERROR_INVALID_LL_PARAMS: _ = 0x1e;
pub const HCI_ERROR_UNSPECIFIED: _ = 0x1f;
pub const HCI_ERROR_ADVERTISING_TIMEOUT: _ = 0x3c;
pub const HCI_ERROR_CANCELLED_BY_HOST: _ = 0x44;

/* Flow control modes */
pub const HCI_FLOW_CTL_MODE_PACKET_BASED: _ = 0x00;
pub const HCI_FLOW_CTL_MODE_BLOCK_BASED: _ = 0x01;

/* The core spec defines 127 as the "not available" value */
pub const HCI_TX_POWER_INVALID: _ = 127;
pub const HCI_RSSI_INVALID: _ = 127;

pub const HCI_SYNC_HANDLE_INVALID: _ = 0xffff;
pub const HCI_SID_INVALID: _ = 0xff;

pub const HCI_ROLE_MASTER: _ = 0x00;
pub const HCI_ROLE_SLAVE: _ = 0x01;

/* Extended Inquiry Response field types */
pub const EIR_FLAGS: _ = 0x01 /* flags */;
pub const EIR_UUID16_SOME: _ = 0x02 /* 16-bit UUID, more available */;
pub const EIR_UUID16_ALL: _ = 0x03 /* 16-bit UUID, all listed */;
pub const EIR_UUID32_SOME: _ = 0x04 /* 32-bit UUID, more available */;
pub const EIR_UUID32_ALL: _ = 0x05 /* 32-bit UUID, all listed */;
pub const EIR_UUID128_SOME: _ = 0x06 /* 128-bit UUID, more available */;
pub const EIR_UUID128_ALL: _ = 0x07 /* 128-bit UUID, all listed */;
pub const EIR_NAME_SHORT: _ = 0x08 /* shortened local name */;
pub const EIR_NAME_COMPLETE: _ = 0x09 /* complete local name */;
pub const EIR_TX_POWER: _ = 0x0A /* transmit power level */;
pub const EIR_CLASS_OF_DEV: _ = 0x0D /* Class of Device */;
pub const EIR_SSP_HASH_C192: _ = 0x0E /* Simple Pairing Hash C-192 */;
pub const EIR_SSP_RAND_R192: _ = 0x0F /* Simple Pairing Randomizer R-192 */;
pub const EIR_DEVICE_ID: _ = 0x10 /* device ID */;
pub const EIR_APPEARANCE: _ = 0x19 /* Device appearance */;
pub const EIR_SERVICE_DATA: _ = 0x16 /* Service Data */;
pub const EIR_LE_BDADDR: _ = 0x1B /* LE Bluetooth device address */;
pub const EIR_LE_ROLE: _ = 0x1C /* LE role */;
pub const EIR_SSP_HASH_C256: _ = 0x1D /* Simple Pairing Hash C-256 */;
pub const EIR_SSP_RAND_R256: _ = 0x1E /* Simple Pairing Rand R-256 */;
pub const EIR_LE_SC_CONFIRM: _ = 0x22 /* LE SC Confirmation Value */;
pub const EIR_LE_SC_RANDOM: _ = 0x23 /* LE SC Random Value */;

/* Low Energy Advertising Flags */
pub const LE_AD_LIMITED: _ = 0x01 /* Limited Discoverable */;
pub const LE_AD_GENERAL: _ = 0x02 /* General Discoverable */;
pub const LE_AD_NO_BREDR: _ = 0x04 /* BR/EDR not supported */;
pub const LE_AD_SIM_LE_BREDR_CTRL: _ = 0x08 /* Simultaneous LE & BR/EDR Controller */;
pub const LE_AD_SIM_LE_BREDR_HOST: _ = 0x10 /* Simultaneous LE & BR/EDR Host */;

/* -----  HCI Commands ---- */
pub const HCI_OP_NOP: _ = 0x0000;

pub const HCI_OP_INQUIRY: _ = 0x0401;
#[repr(C, packed)]
pub struct hci_cp_inquiry {
	u8     lap[3];
	u8     length;
	u8     num_rsp;
} __packed;

pub const HCI_OP_INQUIRY_CANCEL: _ = 0x0402;

pub const HCI_OP_PERIODIC_INQ: _ = 0x0403;

pub const HCI_OP_EXIT_PERIODIC_INQ: _ = 0x0404;

pub const HCI_OP_CREATE_CONN: _ = 0x0405;
#[repr(C, packed)]
pub struct hci_cp_create_conn {
	bdaddr_t bdaddr;
	u16   pkt_type;
	u8     pscan_rep_mode;
	u8     pscan_mode;
	u16   clock_offset;
	u8     role_switch;
} __packed;

pub const HCI_OP_DISCONNECT: _ = 0x0406;
#[repr(C, packed)]
pub struct hci_cp_disconnect {
	u16   handle;
	u8     reason;
} __packed;

pub const HCI_OP_ADD_SCO: _ = 0x0407;
#[repr(C, packed)]
pub struct hci_cp_add_sco {
	u16   handle;
	u16   pkt_type;
} __packed;

pub const HCI_OP_CREATE_CONN_CANCEL: _ = 0x0408;
#[repr(C, packed)]
pub struct hci_cp_create_conn_cancel {
	bdaddr_t bdaddr;
} __packed;

pub const HCI_OP_ACCEPT_CONN_REQ: _ = 0x0409;
#[repr(C, packed)]
pub struct hci_cp_accept_conn_req {
	bdaddr_t bdaddr;
	u8     role;
} __packed;

pub const HCI_OP_REJECT_CONN_REQ: _ = 0x040a;
#[repr(C, packed)]
pub struct hci_cp_reject_conn_req {
	bdaddr_t bdaddr;
	u8     reason;
} __packed;

pub const HCI_OP_LINK_KEY_REPLY: _ = 0x040b;
#[repr(C, packed)]
pub struct hci_cp_link_key_reply {
	bdaddr_t bdaddr;
	u8     link_key[HCI_LINK_KEY_SIZE];
} __packed;

pub const HCI_OP_LINK_KEY_NEG_REPLY: _ = 0x040c;
#[repr(C, packed)]
pub struct hci_cp_link_key_neg_reply {
	bdaddr_t bdaddr;
} __packed;

pub const HCI_OP_PIN_CODE_REPLY: _ = 0x040d;
#[repr(C, packed)]
pub struct hci_cp_pin_code_reply {
	bdaddr_t bdaddr;
	u8     pin_len;
	u8     pin_code[16];
} __packed;
#[repr(C, packed)]
pub struct hci_rp_pin_code_reply {
	u8     status;
	bdaddr_t bdaddr;
} __packed;

pub const HCI_OP_PIN_CODE_NEG_REPLY: _ = 0x040e;
#[repr(C, packed)]
pub struct hci_cp_pin_code_neg_reply {
	bdaddr_t bdaddr;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_pin_code_neg_reply {
	u8     status;
	bdaddr_t bdaddr;
} __packed;

pub const HCI_OP_CHANGE_CONN_PTYPE: _ = 0x040f;
#[repr(C, packed)]
pub struct hci_cp_change_conn_ptype {
	u16   handle;
	u16   pkt_type;
} __packed;

pub const HCI_OP_AUTH_REQUESTED: _ = 0x0411;
#[repr(C, packed)]
pub struct hci_cp_auth_requested {
	u16   handle;
} __packed;

pub const HCI_OP_SET_CONN_ENCRYPT: _ = 0x0413;
#[repr(C, packed)]
pub struct hci_cp_set_conn_encrypt {
	u16   handle;
	u8     encrypt;
} __packed;

pub const HCI_OP_CHANGE_CONN_LINK_KEY: _ = 0x0415;
#[repr(C, packed)]
pub struct hci_cp_change_conn_link_key {
	u16   handle;
} __packed;

pub const HCI_OP_REMOTE_NAME_REQ: _ = 0x0419;
#[repr(C, packed)]
pub struct hci_cp_remote_name_req {
	bdaddr_t bdaddr;
	u8     pscan_rep_mode;
	u8     pscan_mode;
	u16   clock_offset;
} __packed;

pub const HCI_OP_REMOTE_NAME_REQ_CANCEL: _ = 0x041a;
#[repr(C, packed)]
pub struct hci_cp_remote_name_req_cancel {
	bdaddr_t bdaddr;
} __packed;

#[repr(C, packed)]
pub struct hci_rp_remote_name_req_cancel {
	u8     status;
	bdaddr_t bdaddr;
} __packed;

pub const HCI_OP_READ_REMOTE_FEATURES: _ = 0x041b;
#[repr(C, packed)]
pub struct hci_cp_read_remote_features {
	u16   handle;
} __packed;

pub const HCI_OP_READ_REMOTE_EXT_FEATURES: _ = 0x041c;
#[repr(C, packed)]
pub struct hci_cp_read_remote_ext_features {
	u16   handle;
	u8     page;
} __packed;

pub const HCI_OP_READ_REMOTE_VERSION: _ = 0x041d;
#[repr(C, packed)]
pub struct hci_cp_read_remote_version {
	u16   handle;
} __packed;

pub const HCI_OP_READ_CLOCK_OFFSET: _ = 0x041f;
#[repr(C, packed)]
pub struct hci_cp_read_clock_offset {
	u16   handle;
} __packed;

pub const HCI_OP_SETUP_SYNC_CONN: _ = 0x0428;
#[repr(C, packed)]
pub struct hci_cp_setup_sync_conn {
	u16   handle;
	u32   tx_bandwidth;
	u32   rx_bandwidth;
	u16   max_latency;
	u16   voice_setting;
	u8     retrans_effort;
	u16   pkt_type;
} __packed;

pub const HCI_OP_ACCEPT_SYNC_CONN_REQ: _ = 0x0429;
#[repr(C, packed)]
pub struct hci_cp_accept_sync_conn_req {
	bdaddr_t bdaddr;
	u32   tx_bandwidth;
	u32   rx_bandwidth;
	u16   max_latency;
	u16   content_format;
	u8     retrans_effort;
	u16   pkt_type;
} __packed;

pub const HCI_OP_REJECT_SYNC_CONN_REQ: _ = 0x042a;
#[repr(C, packed)]
pub struct hci_cp_reject_sync_conn_req {
	bdaddr_t bdaddr;
	u8     reason;
} __packed;

pub const HCI_OP_IO_CAPABILITY_REPLY: _ = 0x042b;
#[repr(C, packed)]
pub struct hci_cp_io_capability_reply {
	bdaddr_t bdaddr;
	u8     capability;
	u8     oob_data;
	u8     authentication;
} __packed;

pub const HCI_OP_USER_CONFIRM_REPLY: _ = 0x042c;
#[repr(C, packed)]
pub struct hci_cp_user_confirm_reply {
	bdaddr_t bdaddr;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_user_confirm_reply {
	u8     status;
	bdaddr_t bdaddr;
} __packed;

pub const HCI_OP_USER_CONFIRM_NEG_REPLY: _ = 0x042d;

pub const HCI_OP_USER_PASSKEY_REPLY: _ = 0x042e;
#[repr(C, packed)]
pub struct hci_cp_user_passkey_reply {
	bdaddr_t bdaddr;
	u32	passkey;
} __packed;

pub const HCI_OP_USER_PASSKEY_NEG_REPLY: _ = 0x042f;

pub const HCI_OP_REMOTE_OOB_DATA_REPLY: _ = 0x0430;
#[repr(C, packed)]
pub struct hci_cp_remote_oob_data_reply {
	bdaddr_t bdaddr;
	u8     hash[16];
	u8     rand[16];
} __packed;

pub const HCI_OP_REMOTE_OOB_DATA_NEG_REPLY: _ = 0x0433;
#[repr(C, packed)]
pub struct hci_cp_remote_oob_data_neg_reply {
	bdaddr_t bdaddr;
} __packed;

pub const HCI_OP_IO_CAPABILITY_NEG_REPLY: _ = 0x0434;
#[repr(C, packed)]
pub struct hci_cp_io_capability_neg_reply {
	bdaddr_t bdaddr;
	u8     reason;
} __packed;

pub const HCI_OP_ENHANCED_SETUP_SYNC_CONN: _ = 0x043d;
#[repr(C, packed)]
pub struct hci_coding_format {
	u8	id;
	u16	cid;
	u16	vid;
} __packed;

#[repr(C, packed)]
pub struct hci_cp_enhanced_setup_sync_conn {
	u16   handle;
	u32   tx_bandwidth;
	u32   rx_bandwidth;
	struct	 hci_coding_format tx_coding_format;
	struct	 hci_coding_format rx_coding_format;
	u16	 tx_codec_frame_size;
	u16	 rx_codec_frame_size;
	u32	 in_bandwidth;
	u32	 out_bandwidth;
	struct	 hci_coding_format in_coding_format;
	struct	 hci_coding_format out_coding_format;
	u16   in_coded_data_size;
	u16	 out_coded_data_size;
	u8	 in_pcm_data_format;
	u8	 out_pcm_data_format;
	u8	 in_pcm_sample_payload_msb_pos;
	u8	 out_pcm_sample_payload_msb_pos;
	u8	 in_data_path;
	u8	 out_data_path;
	u8	 in_transport_unit_size;
	u8	 out_transport_unit_size;
	u16   max_latency;
	u16   pkt_type;
	u8     retrans_effort;
} __packed;

#[repr(C, packed)]
pub struct hci_rp_logical_link_cancel {
	u8     status;
	u8     phy_handle;
	u8     flow_spec_id;
} __packed;

pub const HCI_OP_SET_CPB: _ = 0x0441;
#[repr(C, packed)]
pub struct hci_cp_set_cpb {
	u8	enable;
	u8	lt_addr;
	u8	lpo_allowed;
	u16	packet_type;
	u16	interval_min;
	u16	interval_max;
	u16	cpb_sv_tout;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_set_cpb {
	u8	status;
	u8	lt_addr;
	u16	interval;
} __packed;

pub const HCI_OP_START_SYNC_TRAIN: _ = 0x0443;

pub const HCI_OP_REMOTE_OOB_EXT_DATA_REPLY: _ = 0x0445;
#[repr(C, packed)]
pub struct hci_cp_remote_oob_ext_data_reply {
	bdaddr_t bdaddr;
	u8     hash192[16];
	u8     rand192[16];
	u8     hash256[16];
	u8     rand256[16];
} __packed;

pub const HCI_OP_SNIFF_MODE: _ = 0x0803;
#[repr(C, packed)]
pub struct hci_cp_sniff_mode {
	u16   handle;
	u16   max_interval;
	u16   min_interval;
	u16   attempt;
	u16   timeout;
} __packed;

pub const HCI_OP_EXIT_SNIFF_MODE: _ = 0x0804;
#[repr(C, packed)]
pub struct hci_cp_exit_sniff_mode {
	u16   handle;
} __packed;

pub const HCI_OP_ROLE_DISCOVERY: _ = 0x0809;
#[repr(C, packed)]
pub struct hci_cp_role_discovery {
	u16   handle;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_role_discovery {
	u8     status;
	u16   handle;
	u8     role;
} __packed;

pub const HCI_OP_SWITCH_ROLE: _ = 0x080b;
#[repr(C, packed)]
pub struct hci_cp_switch_role {
	bdaddr_t bdaddr;
	u8     role;
} __packed;

pub const HCI_OP_READ_LINK_POLICY: _ = 0x080c;
#[repr(C, packed)]
pub struct hci_cp_read_link_policy {
	u16   handle;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_read_link_policy {
	u8     status;
	u16   handle;
	u16   policy;
} __packed;

pub const HCI_OP_WRITE_LINK_POLICY: _ = 0x080d;
#[repr(C, packed)]
pub struct hci_cp_write_link_policy {
	u16   handle;
	u16   policy;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_write_link_policy {
	u8     status;
	u16   handle;
} __packed;

pub const HCI_OP_READ_DEF_LINK_POLICY: _ = 0x080e;
#[repr(C, packed)]
pub struct hci_rp_read_def_link_policy {
	u8     status;
	u16   policy;
} __packed;

pub const HCI_OP_WRITE_DEF_LINK_POLICY: _ = 0x080f;
#[repr(C, packed)]
pub struct hci_cp_write_def_link_policy {
	u16   policy;
} __packed;

pub const HCI_OP_SNIFF_SUBRATE: _ = 0x0811;
#[repr(C, packed)]
pub struct hci_cp_sniff_subrate {
	u16   handle;
	u16   max_latency;
	u16   min_remote_timeout;
	u16   min_local_timeout;
} __packed;

pub const HCI_OP_SET_EVENT_MASK: _ = 0x0c01;

pub const HCI_OP_RESET: _ = 0x0c03;

pub const HCI_OP_SET_EVENT_FLT: _ = 0x0c05;
pub const HCI_SET_EVENT_FLT_SIZE: _ = 9;
#[repr(C, packed)]
pub struct hci_cp_set_event_filter {
	u8		flt_type;
	u8		cond_type;
	struct {
		bdaddr_t bdaddr;
		u8 auto_accept;
	} __packed	addr_conn_flt;
} __packed;

/* Filter types */
pub const HCI_FLT_CLEAR_ALL: _ = 0x00;
pub const HCI_FLT_INQ_RESULT: _ = 0x01;
pub const HCI_FLT_CONN_SETUP: _ = 0x02;

/* CONN_SETUP Condition types */
pub const HCI_CONN_SETUP_ALLOW_ALL: _ = 0x00;
pub const HCI_CONN_SETUP_ALLOW_CLASS: _ = 0x01;
pub const HCI_CONN_SETUP_ALLOW_BDADDR: _ = 0x02;

/* CONN_SETUP Conditions */
pub const HCI_CONN_SETUP_AUTO_OFF: _ = 0x01;
pub const HCI_CONN_SETUP_AUTO_ON: _ = 0x02;
pub const HCI_CONN_SETUP_AUTO_ON_WITH_RS: _ = 0x03;

pub const HCI_OP_READ_STORED_LINK_KEY: _ = 0x0c0d;
#[repr(C, packed)]
pub struct hci_cp_read_stored_link_key {
	bdaddr_t bdaddr;
	u8     read_all;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_read_stored_link_key {
	u8     status;
	u16   max_keys;
	u16   num_keys;
} __packed;

pub const HCI_OP_DELETE_STORED_LINK_KEY: _ = 0x0c12;
#[repr(C, packed)]
pub struct hci_cp_delete_stored_link_key {
	bdaddr_t bdaddr;
	u8     delete_all;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_delete_stored_link_key {
	u8     status;
	u16   num_keys;
} __packed;

pub const HCI_MAX_NAME_LENGTH: _ = 248;

pub const HCI_OP_WRITE_LOCAL_NAME: _ = 0x0c13;
#[repr(C, packed)]
pub struct hci_cp_write_local_name {
	u8     name[HCI_MAX_NAME_LENGTH];
} __packed;

pub const HCI_OP_READ_LOCAL_NAME: _ = 0x0c14;
#[repr(C, packed)]
pub struct hci_rp_read_local_name {
	u8     status;
	u8     name[HCI_MAX_NAME_LENGTH];
} __packed;

pub const HCI_OP_WRITE_CA_TIMEOUT: _ = 0x0c16;

pub const HCI_OP_WRITE_PG_TIMEOUT: _ = 0x0c18;

pub const HCI_OP_WRITE_SCAN_ENABLE: _ = 0x0c1a;
	#define SCAN_DISABLED		0x00
	#define SCAN_INQUIRY		0x01
	#define SCAN_PAGE		0x02

pub const HCI_OP_READ_AUTH_ENABLE: _ = 0x0c1f;

pub const HCI_OP_WRITE_AUTH_ENABLE: _ = 0x0c20;
	#define AUTH_DISABLED		0x00
	#define AUTH_ENABLED		0x01

pub const HCI_OP_READ_ENCRYPT_MODE: _ = 0x0c21;

pub const HCI_OP_WRITE_ENCRYPT_MODE: _ = 0x0c22;
	#define ENCRYPT_DISABLED	0x00
	#define ENCRYPT_P2P		0x01
	#define ENCRYPT_BOTH		0x02

pub const HCI_OP_READ_CLASS_OF_DEV: _ = 0x0c23;
#[repr(C, packed)]
pub struct hci_rp_read_class_of_dev {
	u8     status;
	u8     dev_class[3];
} __packed;

pub const HCI_OP_WRITE_CLASS_OF_DEV: _ = 0x0c24;
#[repr(C, packed)]
pub struct hci_cp_write_class_of_dev {
	u8     dev_class[3];
} __packed;

pub const HCI_OP_READ_VOICE_SETTING: _ = 0x0c25;
#[repr(C, packed)]
pub struct hci_rp_read_voice_setting {
	u8     status;
	u16   voice_setting;
} __packed;

pub const HCI_OP_WRITE_VOICE_SETTING: _ = 0x0c26;
#[repr(C, packed)]
pub struct hci_cp_write_voice_setting {
	u16   voice_setting;
} __packed;

pub const HCI_OP_HOST_BUFFER_SIZE: _ = 0x0c33;
#[repr(C, packed)]
pub struct hci_cp_host_buffer_size {
	u16   acl_mtu;
	u8     sco_mtu;
	u16   acl_max_pkt;
	u16   sco_max_pkt;
} __packed;

pub const HCI_OP_READ_NUM_SUPPORTED_IAC: _ = 0x0c38;
#[repr(C, packed)]
pub struct hci_rp_read_num_supported_iac {
	u8	status;
	u8	num_iac;
} __packed;

pub const HCI_OP_READ_CURRENT_IAC_LAP: _ = 0x0c39;

pub const HCI_OP_WRITE_CURRENT_IAC_LAP: _ = 0x0c3a;
#[repr(C, packed)]
pub struct hci_cp_write_current_iac_lap {
	u8	num_iac;
	u8	iac_lap[6];
} __packed;

pub const HCI_OP_WRITE_INQUIRY_MODE: _ = 0x0c45;

pub const HCI_MAX_EIR_LENGTH: _ = 240;

pub const HCI_OP_WRITE_EIR: _ = 0x0c52;
#[repr(C, packed)]
pub struct hci_cp_write_eir {
	u8	fec;
	u8	data[HCI_MAX_EIR_LENGTH];
} __packed;

pub const HCI_OP_READ_SSP_MODE: _ = 0x0c55;
#[repr(C, packed)]
pub struct hci_rp_read_ssp_mode {
	u8     status;
	u8     mode;
} __packed;

pub const HCI_OP_WRITE_SSP_MODE: _ = 0x0c56;
#[repr(C, packed)]
pub struct hci_cp_write_ssp_mode {
	u8     mode;
} __packed;

pub const HCI_OP_READ_LOCAL_OOB_DATA: _ = 0x0c57;
#[repr(C, packed)]
pub struct hci_rp_read_local_oob_data {
	u8     status;
	u8     hash[16];
	u8     rand[16];
} __packed;

pub const HCI_OP_READ_INQ_RSP_TX_POWER: _ = 0x0c58;
#[repr(C, packed)]
pub struct hci_rp_read_inq_rsp_tx_power {
	u8     status;
	i8     tx_power;
} __packed;

pub const HCI_OP_READ_DEF_ERR_DATA_REPORTING: _ = 0x0c5a;
	#define ERR_DATA_REPORTING_DISABLED	0x00
	#define ERR_DATA_REPORTING_ENABLED	0x01
#[repr(C, packed)]
pub struct hci_rp_read_def_err_data_reporting {
	u8     status;
	u8     err_data_reporting;
} __packed;

pub const HCI_OP_WRITE_DEF_ERR_DATA_REPORTING: _ = 0x0c5b;
#[repr(C, packed)]
pub struct hci_cp_write_def_err_data_reporting {
	u8     err_data_reporting;
} __packed;

pub const HCI_OP_SET_EVENT_MASK_PAGE_2: _ = 0x0c63;

pub const HCI_OP_READ_LOCATION_DATA: _ = 0x0c64;

pub const HCI_OP_READ_FLOW_CONTROL_MODE: _ = 0x0c66;
#[repr(C, packed)]
pub struct hci_rp_read_flow_control_mode {
	u8     status;
	u8     mode;
} __packed;

pub const HCI_OP_WRITE_LE_HOST_SUPPORTED: _ = 0x0c6d;
#[repr(C, packed)]
pub struct hci_cp_write_le_host_supported {
	u8	le;
	u8	simul;
} __packed;

pub const HCI_OP_SET_RESERVED_LT_ADDR: _ = 0x0c74;
#[repr(C, packed)]
pub struct hci_cp_set_reserved_lt_addr {
	u8	lt_addr;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_set_reserved_lt_addr {
	u8	status;
	u8	lt_addr;
} __packed;

pub const HCI_OP_DELETE_RESERVED_LT_ADDR: _ = 0x0c75;
#[repr(C, packed)]
pub struct hci_cp_delete_reserved_lt_addr {
	u8	lt_addr;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_delete_reserved_lt_addr {
	u8	status;
	u8	lt_addr;
} __packed;

pub const HCI_OP_SET_CPB_DATA: _ = 0x0c76;
#[repr(C, packed)]
pub struct hci_cp_set_cpb_data {
	u8	lt_addr;
	u8	fragment;
	u8	data_length;
	u8	data[HCI_MAX_CPB_DATA_SIZE];
} __packed;
#[repr(C, packed)]
pub struct hci_rp_set_cpb_data {
	u8	status;
	u8	lt_addr;
} __packed;

pub const HCI_OP_READ_SYNC_TRAIN_PARAMS: _ = 0x0c77;

pub const HCI_OP_WRITE_SYNC_TRAIN_PARAMS: _ = 0x0c78;
#[repr(C, packed)]
pub struct hci_cp_write_sync_train_params {
	u16	interval_min;
	u16	interval_max;
	u32	sync_train_tout;
	u8	service_data;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_write_sync_train_params {
	u8	status;
	u16	sync_train_int;
} __packed;

pub const HCI_OP_READ_SC_SUPPORT: _ = 0x0c79;
#[repr(C, packed)]
pub struct hci_rp_read_sc_support {
	u8	status;
	u8	support;
} __packed;

pub const HCI_OP_WRITE_SC_SUPPORT: _ = 0x0c7a;
#[repr(C, packed)]
pub struct hci_cp_write_sc_support {
	u8	support;
} __packed;

pub const HCI_OP_READ_AUTH_PAYLOAD_TO: _ = 0x0c7b;
#[repr(C, packed)]
pub struct hci_cp_read_auth_payload_to {
	u16  handle;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_read_auth_payload_to {
	u8    status;
	u16  handle;
	u16  timeout;
} __packed;

pub const HCI_OP_WRITE_AUTH_PAYLOAD_TO: _ = 0x0c7c;
#[repr(C, packed)]
pub struct hci_cp_write_auth_payload_to {
	u16  handle;
	u16  timeout;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_write_auth_payload_to {
	u8    status;
	u16  handle;
} __packed;

pub const HCI_OP_READ_LOCAL_OOB_EXT_DATA: _ = 0x0c7d;
#[repr(C, packed)]
pub struct hci_rp_read_local_oob_ext_data {
	u8     status;
	u8     hash192[16];
	u8     rand192[16];
	u8     hash256[16];
	u8     rand256[16];
} __packed;

pub const HCI_CONFIGURE_DATA_PATH: _ = 0x0c83;
#[repr(C, packed)]
pub struct hci_op_configure_data_path {
	u8	direction;
	u8	data_path_id;
	u8	vnd_len;
	u8	vnd_data[];
} __packed;

pub const HCI_OP_READ_LOCAL_VERSION: _ = 0x1001;
#[repr(C, packed)]
pub struct hci_rp_read_local_version {
	u8     status;
	u8     hci_ver;
	u16   hci_rev;
	u8     lmp_ver;
	u16   manufacturer;
	u16   lmp_subver;
} __packed;

pub const HCI_OP_READ_LOCAL_COMMANDS: _ = 0x1002;
#[repr(C, packed)]
pub struct hci_rp_read_local_commands {
	u8     status;
	u8     commands[64];
} __packed;

pub const HCI_OP_READ_LOCAL_FEATURES: _ = 0x1003;
#[repr(C, packed)]
pub struct hci_rp_read_local_features {
	u8     status;
	u8     features[8];
} __packed;

pub const HCI_OP_READ_LOCAL_EXT_FEATURES: _ = 0x1004;
#[repr(C, packed)]
pub struct hci_cp_read_local_ext_features {
	u8     page;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_read_local_ext_features {
	u8     status;
	u8     page;
	u8     max_page;
	u8     features[8];
} __packed;

pub const HCI_OP_READ_BUFFER_SIZE: _ = 0x1005;
#[repr(C, packed)]
pub struct hci_rp_read_buffer_size {
	u8     status;
	u16   acl_mtu;
	u8     sco_mtu;
	u16   acl_max_pkt;
	u16   sco_max_pkt;
} __packed;

pub const HCI_OP_READ_BD_ADDR: _ = 0x1009;
#[repr(C, packed)]
pub struct hci_rp_read_bd_addr {
	u8     status;
	bdaddr_t bdaddr;
} __packed;

pub const HCI_OP_READ_DATA_BLOCK_SIZE: _ = 0x100a;
#[repr(C, packed)]
pub struct hci_rp_read_data_block_size {
	u8     status;
	u16   max_acl_len;
	u16   block_len;
	u16   num_blocks;
} __packed;

pub const HCI_OP_READ_LOCAL_CODECS: _ = 0x100b;
#[repr(C, packed)]
pub struct hci_std_codecs_hdr {
	u8	num;
} __packed;

#[repr(C, packed)]
pub struct hci_std_codecs {
	struct hci_std_codecs_hdr;
	u8	codec[];
} __packed;

#[repr(C, packed)]
pub struct hci_vnd_codec {
	/* company id */
	u16	cid;
	/* vendor codec id */
	u16	vid;
} __packed;

#[repr(C, packed)]
pub struct hci_vnd_codecs {
	u8	num;
	struct hci_vnd_codec codec[];
} __packed;

#[repr(C, packed)]
pub struct hci_rp_read_local_supported_codecs {
	u8	status;
	struct hci_std_codecs_hdr std_codecs;
	struct hci_vnd_codecs vnd_codecs;
} __packed;

pub const HCI_OP_READ_LOCAL_PAIRING_OPTS: _ = 0x100c;
#[repr(C, packed)]
pub struct hci_rp_read_local_pairing_opts {
	u8     status;
	u8     pairing_opts;
	u8     max_key_size;
} __packed;

pub const HCI_OP_READ_LOCAL_CODECS_V2: _ = 0x100d;
#[repr(C, packed)]
pub struct hci_std_codec_v2 {
	u8	id;
	u8	transport;
} __packed;

#[repr(C, packed)]
pub struct hci_std_codecs_v2_hdr {
	u8	num;
} __packed;

#[repr(C, packed)]
pub struct hci_std_codecs_v2 {
	struct hci_std_codecs_v2_hdr;
	struct hci_std_codec_v2 codec[];
} __packed;

#[repr(C, packed)]
pub struct hci_vnd_codec_v2 {
	u16	cid;
	u16	vid;
	u8	transport;
} __packed;

#[repr(C, packed)]
pub struct hci_vnd_codecs_v2 {
	u8	num;
	struct hci_vnd_codec_v2 codec[];
} __packed;

#[repr(C, packed)]
pub struct hci_rp_read_local_supported_codecs_v2 {
	u8	status;
	struct hci_std_codecs_v2_hdr std_codecs;
	struct hci_vnd_codecs_v2 vendor_codecs;
} __packed;

pub const HCI_OP_READ_LOCAL_CODEC_CAPS: _ = 0x100e;
#[repr(C, packed)]
pub struct hci_op_read_local_codec_caps {
	u8	id;
	u16	cid;
	u16	vid;
	u8	transport;
	u8	direction;
} __packed;

#[repr(C, packed)]
pub struct hci_codec_caps {
	u8	len;
	u8	data[];
} __packed;

#[repr(C, packed)]
pub struct hci_rp_read_local_codec_caps {
	u8	status;
	u8	num_caps;
} __packed;

pub const HCI_OP_READ_PAGE_SCAN_ACTIVITY: _ = 0x0c1b;
#[repr(C, packed)]
pub struct hci_rp_read_page_scan_activity {
	u8     status;
	u16   interval;
	u16   window;
} __packed;

pub const HCI_OP_WRITE_PAGE_SCAN_ACTIVITY: _ = 0x0c1c;
#[repr(C, packed)]
pub struct hci_cp_write_page_scan_activity {
	u16   interval;
	u16   window;
} __packed;

pub const HCI_OP_READ_TX_POWER: _ = 0x0c2d;
#[repr(C, packed)]
pub struct hci_cp_read_tx_power {
	u16   handle;
	u8     type;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_read_tx_power {
	u8     status;
	u16   handle;
	i8     tx_power;
} __packed;

pub const HCI_OP_WRITE_SYNC_FLOWCTL: _ = 0x0c2f;
#[repr(C, packed)]
pub struct hci_cp_write_sync_flowctl {
	u8     enable;
} __packed;

pub const HCI_OP_READ_PAGE_SCAN_TYPE: _ = 0x0c46;
#[repr(C, packed)]
pub struct hci_rp_read_page_scan_type {
	u8     status;
	u8     type;
} __packed;

pub const HCI_OP_WRITE_PAGE_SCAN_TYPE: _ = 0x0c47;
	#define PAGE_SCAN_TYPE_STANDARD		0x00
	#define PAGE_SCAN_TYPE_INTERLACED	0x01

pub const HCI_OP_READ_RSSI: _ = 0x1405;
#[repr(C, packed)]
pub struct hci_cp_read_rssi {
	u16   handle;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_read_rssi {
	u8     status;
	u16   handle;
	i8     rssi;
} __packed;

pub const HCI_OP_READ_CLOCK: _ = 0x1407;
#[repr(C, packed)]
pub struct hci_cp_read_clock {
	u16   handle;
	u8     which;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_read_clock {
	u8     status;
	u16   handle;
	u32   clock;
	u16   accuracy;
} __packed;

pub const HCI_OP_READ_ENC_KEY_SIZE: _ = 0x1408;
#[repr(C, packed)]
pub struct hci_cp_read_enc_key_size {
	u16   handle;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_read_enc_key_size {
	u8     status;
	u16   handle;
	u8     key_size;
} __packed;

pub const HCI_OP_GET_MWS_TRANSPORT_CONFIG: _ = 0x140c;

pub const HCI_OP_ENABLE_DUT_MODE: _ = 0x1803;

pub const HCI_OP_WRITE_SSP_DEBUG_MODE: _ = 0x1804;

pub const HCI_OP_LE_SET_EVENT_MASK: _ = 0x2001;
#[repr(C, packed)]
pub struct hci_cp_le_set_event_mask {
	u8     mask[8];
} __packed;

/* BLUETOOTH CORE SPECIFICATION Version 5.4 | Vol 4, Part E
 * 7.8.2 LE Read Buffer Size command
 * MAX_LE_MTU is 0xffff.
 * 0 is also valid. It means that no dedicated LE Buffer exists.
 * It should use the HCI_Read_Buffer_Size command and mtu is shared
 * between BR/EDR and LE.
 */
pub const HCI_MIN_LE_MTU: _ = 0x001b;

pub const HCI_OP_LE_READ_BUFFER_SIZE: _ = 0x2002;
#[repr(C, packed)]
pub struct hci_rp_le_read_buffer_size {
	u8     status;
	u16   le_mtu;
	u8     le_max_pkt;
} __packed;

pub const HCI_OP_LE_READ_LOCAL_FEATURES: _ = 0x2003;
#[repr(C, packed)]
pub struct hci_rp_le_read_local_features {
	u8     status;
	u8     features[8];
} __packed;

pub const HCI_OP_LE_SET_RANDOM_ADDR: _ = 0x2005;

pub const HCI_OP_LE_SET_ADV_PARAM: _ = 0x2006;
#[repr(C, packed)]
pub struct hci_cp_le_set_adv_param {
	u16   min_interval;
	u16   max_interval;
	u8     type;
	u8     own_address_type;
	u8     direct_addr_type;
	bdaddr_t direct_addr;
	u8     channel_map;
	u8     filter_policy;
} __packed;

pub const HCI_OP_LE_READ_ADV_TX_POWER: _ = 0x2007;
#[repr(C, packed)]
pub struct hci_rp_le_read_adv_tx_power {
	u8	status;
	i8	tx_power;
} __packed;

pub const HCI_MAX_AD_LENGTH: _ = 31;

pub const HCI_OP_LE_SET_ADV_DATA: _ = 0x2008;
#[repr(C, packed)]
pub struct hci_cp_le_set_adv_data {
	u8	length;
	u8	data[HCI_MAX_AD_LENGTH];
} __packed;

pub const HCI_OP_LE_SET_SCAN_RSP_DATA: _ = 0x2009;
#[repr(C, packed)]
pub struct hci_cp_le_set_scan_rsp_data {
	u8	length;
	u8	data[HCI_MAX_AD_LENGTH];
} __packed;

pub const HCI_OP_LE_SET_ADV_ENABLE: _ = 0x200a;

pub const LE_SCAN_PASSIVE: _ = 0x00;
pub const LE_SCAN_ACTIVE: _ = 0x01;

pub const HCI_OP_LE_SET_SCAN_PARAM: _ = 0x200b;
#[repr(C, packed)]
pub struct hci_cp_le_set_scan_param {
	u8    type;
	u16  interval;
	u16  window;
	u8    own_address_type;
	u8    filter_policy;
} __packed;

pub const LE_SCAN_DISABLE: _ = 0x00;
pub const LE_SCAN_ENABLE: _ = 0x01;
pub const LE_SCAN_FILTER_DUP_DISABLE: _ = 0x00;
pub const LE_SCAN_FILTER_DUP_ENABLE: _ = 0x01;

pub const HCI_OP_LE_SET_SCAN_ENABLE: _ = 0x200c;
#[repr(C, packed)]
pub struct hci_cp_le_set_scan_enable {
	u8     enable;
	u8     filter_dup;
} __packed;

pub const HCI_LE_USE_PEER_ADDR: _ = 0x00;
pub const HCI_LE_USE_ACCEPT_LIST: _ = 0x01;

pub const HCI_OP_LE_CREATE_CONN: _ = 0x200d;
#[repr(C, packed)]
pub struct hci_cp_le_create_conn {
	u16   scan_interval;
	u16   scan_window;
	u8     filter_policy;
	u8     peer_addr_type;
	bdaddr_t peer_addr;
	u8     own_address_type;
	u16   conn_interval_min;
	u16   conn_interval_max;
	u16   conn_latency;
	u16   supervision_timeout;
	u16   min_ce_len;
	u16   max_ce_len;
} __packed;

pub const HCI_OP_LE_CREATE_CONN_CANCEL: _ = 0x200e;

pub const HCI_OP_LE_READ_ACCEPT_LIST_SIZE: _ = 0x200f;
#[repr(C, packed)]
pub struct hci_rp_le_read_accept_list_size {
	u8	status;
	u8	size;
} __packed;

pub const HCI_OP_LE_CLEAR_ACCEPT_LIST: _ = 0x2010;

pub const HCI_OP_LE_ADD_TO_ACCEPT_LIST: _ = 0x2011;
#[repr(C, packed)]
pub struct hci_cp_le_add_to_accept_list {
	u8     bdaddr_type;
	bdaddr_t bdaddr;
} __packed;

pub const HCI_OP_LE_DEL_FROM_ACCEPT_LIST: _ = 0x2012;
#[repr(C, packed)]
pub struct hci_cp_le_del_from_accept_list {
	u8     bdaddr_type;
	bdaddr_t bdaddr;
} __packed;

pub const HCI_OP_LE_CONN_UPDATE: _ = 0x2013;
#[repr(C, packed)]
pub struct hci_cp_le_conn_update {
	u16   handle;
	u16   conn_interval_min;
	u16   conn_interval_max;
	u16   conn_latency;
	u16   supervision_timeout;
	u16   min_ce_len;
	u16   max_ce_len;
} __packed;

pub const HCI_OP_LE_READ_REMOTE_FEATURES: _ = 0x2016;
#[repr(C, packed)]
pub struct hci_cp_le_read_remote_features {
	u16	 handle;
} __packed;

pub const HCI_OP_LE_START_ENC: _ = 0x2019;
#[repr(C, packed)]
pub struct hci_cp_le_start_enc {
	u16	handle;
	u64	rand;
	u16	ediv;
	u8	ltk[16];
} __packed;

pub const HCI_OP_LE_LTK_REPLY: _ = 0x201a;
#[repr(C, packed)]
pub struct hci_cp_le_ltk_reply {
	u16	handle;
	u8	ltk[16];
} __packed;
#[repr(C, packed)]
pub struct hci_rp_le_ltk_reply {
	u8	status;
	u16	handle;
} __packed;

pub const HCI_OP_LE_LTK_NEG_REPLY: _ = 0x201b;
#[repr(C, packed)]
pub struct hci_cp_le_ltk_neg_reply {
	u16	handle;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_le_ltk_neg_reply {
	u8	status;
	u16	handle;
} __packed;

pub const HCI_OP_LE_READ_SUPPORTED_STATES: _ = 0x201c;
#[repr(C, packed)]
pub struct hci_rp_le_read_supported_states {
	u8	status;
	u8	le_states[8];
} __packed;

pub const HCI_OP_LE_CONN_PARAM_REQ_REPLY: _ = 0x2020;
#[repr(C, packed)]
pub struct hci_cp_le_conn_param_req_reply {
	u16	handle;
	u16	interval_min;
	u16	interval_max;
	u16	latency;
	u16	timeout;
	u16	min_ce_len;
	u16	max_ce_len;
} __packed;

pub const HCI_OP_LE_CONN_PARAM_REQ_NEG_REPLY: _ = 0x2021;
#[repr(C, packed)]
pub struct hci_cp_le_conn_param_req_neg_reply {
	u16	handle;
	u8	reason;
} __packed;

pub const HCI_OP_LE_SET_DATA_LEN: _ = 0x2022;
#[repr(C, packed)]
pub struct hci_cp_le_set_data_len {
	u16	handle;
	u16	tx_len;
	u16	tx_time;
} __packed;
#[repr(C, packed)]
pub struct hci_rp_le_set_data_len {
	u8	status;
	u16	handle;
} __packed;

pub const HCI_OP_LE_READ_DEF_DATA_LEN: _ = 0x2023;
#[repr(C, packed)]
pub struct hci_rp_le_read_def_data_len {
	u8	status;
	u16	tx_len;
	u16	tx_time;
} __packed;

pub const HCI_OP_LE_WRITE_DEF_DATA_LEN: _ = 0x2024;
#[repr(C, packed)]
pub struct hci_cp_le_write_def_data_len {
	u16	tx_len;
	u16	tx_time;
} __packed;

pub const HCI_OP_LE_ADD_TO_RESOLV_LIST: _ = 0x2027;
#[repr(C, packed)]
pub struct hci_cp_le_add_to_resolv_list {
	u8	 bdaddr_type;
	bdaddr_t bdaddr;
	u8	 peer_irk[16];
	u8	 local_irk[16];
} __packed;

pub const HCI_OP_LE_DEL_FROM_RESOLV_LIST: _ = 0x2028;
#[repr(C, packed)]
pub struct hci_cp_le_del_from_resolv_list {
	u8	 bdaddr_type;
	bdaddr_t bdaddr;
} __packed;

pub const HCI_OP_LE_CLEAR_RESOLV_LIST: _ = 0x2029;

pub const HCI_OP_LE_READ_RESOLV_LIST_SIZE: _ = 0x202a;
#[repr(C, packed)]
pub struct hci_rp_le_read_resolv_list_size {
	u8	status;
	u8	size;
} __packed;

pub const HCI_OP_LE_SET_ADDR_RESOLV_ENABLE: _ = 0x202d;

pub const HCI_OP_LE_SET_RPA_TIMEOUT: _ = 0x202e;

pub const HCI_OP_LE_READ_MAX_DATA_LEN: _ = 0x202f;
#[repr(C, packed)]
pub struct hci_rp_le_read_max_data_len {
	u8	status;
	u16	tx_len;
	u16	tx_time;
	u16	rx_len;
	u16	rx_time;
} __packed;

pub const HCI_OP_LE_SET_DEFAULT_PHY: _ = 0x2031;
#[repr(C, packed)]
pub struct hci_cp_le_set_default_phy {
	u8    all_phys;
	u8    tx_phys;
	u8    rx_phys;
} __packed;

pub const HCI_LE_SET_PHY_1M: _ = 0x01;
pub const HCI_LE_SET_PHY_2M: _ = 0x02;
pub const HCI_LE_SET_PHY_CODED: _ = 0x04;

pub const HCI_OP_LE_SET_PHY: _ = 0x2032;
#[repr(C, packed)]
pub struct hci_cp_le_set_phy {
	u16  handle;
	u8    all_phys;
	u8    tx_phys;
	u8    rx_phys;
	u16  phy_opts;
} __packed;

pub const HCI_OP_LE_SET_EXT_SCAN_PARAMS: _ = 0x2041;
#[repr(C, packed)]
pub struct hci_cp_le_set_ext_scan_params {
	u8    own_addr_type;
	u8    filter_policy;
	u8    scanning_phys;
	u8    data[];
} __packed;

pub const LE_SCAN_PHY_1M: _ = 0x01;
pub const LE_SCAN_PHY_2M: _ = 0x02;
pub const LE_SCAN_PHY_CODED: _ = 0x04;

#[repr(C, packed)]
pub struct hci_cp_le_scan_phy_params {
	u8    type;
	u16  interval;
	u16  window;
} __packed;

pub const HCI_OP_LE_SET_EXT_SCAN_ENABLE: _ = 0x2042;
#[repr(C, packed)]
pub struct hci_cp_le_set_ext_scan_enable {
	u8    enable;
	u8    filter_dup;
	u16  duration;
	u16  period;
} __packed;

pub const HCI_OP_LE_EXT_CREATE_CONN: _ = 0x2043;
#[repr(C, packed)]
pub struct hci_cp_le_ext_create_conn {
	u8      filter_policy;
	u8      own_addr_type;
	u8      peer_addr_type;
	bdaddr_t  peer_addr;
	u8      phys;
	u8      data[];
} __packed;

#[repr(C, packed)]
pub struct hci_cp_le_ext_conn_param {
	u16 scan_interval;
	u16 scan_window;
	u16 conn_interval_min;
	u16 conn_interval_max;
	u16 conn_latency;
	u16 supervision_timeout;
	u16 min_ce_len;
	u16 max_ce_len;
} __packed;

pub const HCI_OP_LE_PA_CREATE_SYNC: _ = 0x2044;
#[repr(C, packed)]
pub struct hci_cp_le_pa_create_sync {
	u8      options;
	u8      sid;
	u8      addr_type;
	bdaddr_t  addr;
	u16    skip;
	u16    sync_timeout;
	u8      sync_cte_type;
} __packed;

pub const HCI_OP_LE_PA_CREATE_SYNC_CANCEL: _ = 0x2045;

pub const HCI_OP_LE_PA_TERM_SYNC: _ = 0x2046;
#[repr(C, packed)]
pub struct hci_cp_le_pa_term_sync {
	u16    handle;
} __packed;

pub const HCI_OP_LE_READ_NUM_SUPPORTED_ADV_SETS: _ = 0x203b;
#[repr(C, packed)]
pub struct hci_rp_le_read_num_supported_adv_sets {
	u8  status;
	u8  num_of_sets;
} __packed;

pub const HCI_OP_LE_SET_EXT_ADV_PARAMS: _ = 0x2036;
#[repr(C, packed)]
pub struct hci_cp_le_set_ext_adv_params {
	u8      handle;
	u16    evt_properties;
	u8      min_interval[3];
	u8      max_interval[3];
	u8      channel_map;
	u8      own_addr_type;
	u8      peer_addr_type;
	bdaddr_t  peer_addr;
	u8      filter_policy;
	u8      tx_power;
	u8      primary_phy;
	u8      secondary_max_skip;
	u8      secondary_phy;
	u8      sid;
	u8      notif_enable;
} __packed;

pub const HCI_ADV_PHY_1M: _ = 0X01;
pub const HCI_ADV_PHY_2M: _ = 0x02;
pub const HCI_ADV_PHY_CODED: _ = 0x03;

#[repr(C, packed)]
pub struct hci_rp_le_set_ext_adv_params {
	u8  status;
	u8  tx_power;
} __packed;

#[repr(C, packed)]
pub struct hci_cp_ext_adv_set {
	u8  handle;
	u16 duration;
	u8  max_events;
} __packed;

pub const HCI_MAX_EXT_AD_LENGTH: _ = 251;

pub const HCI_OP_LE_SET_EXT_ADV_DATA: _ = 0x2037;
#[repr(C, packed)]
pub struct hci_cp_le_set_ext_adv_data {
	u8  handle;
	u8  operation;
	u8  frag_pref;
	u8  length;
	u8  data[] __counted_by(length);
} __packed;

pub const HCI_OP_LE_SET_EXT_SCAN_RSP_DATA: _ = 0x2038;
#[repr(C, packed)]
pub struct hci_cp_le_set_ext_scan_rsp_data {
	u8  handle;
	u8  operation;
	u8  frag_pref;
	u8  length;
	u8  data[] __counted_by(length);
} __packed;

pub const HCI_OP_LE_SET_EXT_ADV_ENABLE: _ = 0x2039;
#[repr(C, packed)]
pub struct hci_cp_le_set_ext_adv_enable {
	u8  enable;
	u8  num_of_sets;
	u8  data[];
} __packed;

pub const HCI_OP_LE_SET_PER_ADV_PARAMS: _ = 0x203e;
#[repr(C, packed)]
pub struct hci_cp_le_set_per_adv_params {
	u8      handle;
	u16    min_interval;
	u16    max_interval;
	u16    periodic_properties;
} __packed;

pub const HCI_MAX_PER_AD_LENGTH: _ = 252;
pub const HCI_MAX_PER_AD_TOT_LEN: _ = 1650;

pub const HCI_OP_LE_SET_PER_ADV_DATA: _ = 0x203f;
#[repr(C, packed)]
pub struct hci_cp_le_set_per_adv_data {
	u8  handle;
	u8  operation;
	u8  length;
	u8  data[] __counted_by(length);
} __packed;

pub const HCI_OP_LE_SET_PER_ADV_ENABLE: _ = 0x2040;
#[repr(C, packed)]
pub struct hci_cp_le_set_per_adv_enable {
	u8  enable;
	u8  handle;
} __packed;

pub const LE_SET_ADV_DATA_OP_COMPLETE: _ = 0x03;

pub const LE_SET_ADV_DATA_NO_FRAG: _ = 0x01;

pub const HCI_OP_LE_REMOVE_ADV_SET: _ = 0x203c;

pub const HCI_OP_LE_CLEAR_ADV_SETS: _ = 0x203d;

pub const HCI_OP_LE_SET_ADV_SET_RAND_ADDR: _ = 0x2035;
#[repr(C, packed)]
pub struct hci_cp_le_set_adv_set_rand_addr {
	u8  handle;
	bdaddr_t  bdaddr;
} __packed;

pub const HCI_OP_LE_READ_TRANSMIT_POWER: _ = 0x204b;
#[repr(C, packed)]
pub struct hci_rp_le_read_transmit_power {
	u8  status;
	i8  min_le_tx_power;
	i8  max_le_tx_power;
} __packed;

pub const HCI_NETWORK_PRIVACY: _ = 0x00;
pub const HCI_DEVICE_PRIVACY: _ = 0x01;

pub const HCI_OP_LE_SET_PRIVACY_MODE: _ = 0x204e;
#[repr(C, packed)]
pub struct hci_cp_le_set_privacy_mode {
	u8  bdaddr_type;
	bdaddr_t  bdaddr;
	u8  mode;
} __packed;

pub const HCI_OP_LE_PAST: _ = 0x205a;
#[repr(C, packed)]
pub struct hci_cp_le_past {
	u16 handle;
	u16 service_data;
	u16 sync_handle;
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_past {
	u8   status;
	u16 handle;
} __packed;

pub const HCI_OP_LE_PAST_SET_INFO: _ = 0x205b;
#[repr(C, packed)]
pub struct hci_cp_le_past_set_info {
	u16 handle;
	u16 service_data;
	u8   adv_handle;
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_past_set_info {
	u8   status;
	u16 handle;
} __packed;

pub const HCI_OP_LE_PAST_PARAMS: _ = 0x205c;
#[repr(C, packed)]
pub struct hci_cp_le_past_params {
	u16  handle;
	u8    mode;
	u16  skip;
	u16  sync_timeout;
	u8    cte_type;
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_past_params {
	u8   status;
	u16 handle;
} __packed;

pub const HCI_OP_LE_READ_BUFFER_SIZE_V2: _ = 0x2060;
#[repr(C, packed)]
pub struct hci_rp_le_read_buffer_size_v2 {
	u8    status;
	u16  acl_mtu;
	u8    acl_max_pkt;
	u16  iso_mtu;
	u8    iso_max_pkt;
} __packed;

pub const HCI_OP_LE_READ_ISO_TX_SYNC: _ = 0x2061;
#[repr(C, packed)]
pub struct hci_cp_le_read_iso_tx_sync {
	u16  handle;
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_read_iso_tx_sync {
	u8    status;
	u16  handle;
	u16  seq;
	u32  imestamp;
	u8    offset[3];
} __packed;

pub const HCI_OP_LE_SET_CIG_PARAMS: _ = 0x2062;
#[repr(C, packed)]
pub struct hci_cis_params {
	u8    cis_id;
	u16  c_sdu;
	u16  p_sdu;
	u8    c_phys;
	u8    p_phys;
	u8    c_rtn;
	u8    p_rtn;
} __packed;

#[repr(C, packed)]
pub struct hci_cp_le_set_cig_params {
	u8    cig_id;
	u8    c_interval[3];
	u8    p_interval[3];
	u8    sca;
	u8    packing;
	u8    framing;
	u16  c_latency;
	u16  p_latency;
	u8    num_cis;
	struct hci_cis_params cis[] __counted_by(num_cis);
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_set_cig_params {
	u8    status;
	u8    cig_id;
	u8    num_handles;
	u16  handle[];
} __packed;

pub const HCI_OP_LE_CREATE_CIS: _ = 0x2064;
#[repr(C, packed)]
pub struct hci_cis {
	u16  cis_handle;
	u16  acl_handle;
} __packed;

#[repr(C, packed)]
pub struct hci_cp_le_create_cis {
	u8    num_cis;
	struct hci_cis cis[] __counted_by(num_cis);
} __packed;

pub const HCI_OP_LE_REMOVE_CIG: _ = 0x2065;
#[repr(C, packed)]
pub struct hci_cp_le_remove_cig {
	u8    cig_id;
} __packed;

pub const HCI_OP_LE_ACCEPT_CIS: _ = 0x2066;
#[repr(C, packed)]
pub struct hci_cp_le_accept_cis {
	u16  handle;
} __packed;

pub const HCI_OP_LE_REJECT_CIS: _ = 0x2067;
#[repr(C, packed)]
pub struct hci_cp_le_reject_cis {
	u16  handle;
	u8    reason;
} __packed;

pub const HCI_OP_LE_CREATE_BIG: _ = 0x2068;
#[repr(C, packed)]
pub struct hci_bis {
	u8    sdu_interval[3];
	u16  sdu;
	u16  latency;
	u8    rtn;
	u8    phy;
	u8    packing;
	u8    framing;
	u8    encryption;
	u8    bcode[16];
} __packed;

#[repr(C, packed)]
pub struct hci_cp_le_create_big {
	u8    handle;
	u8    adv_handle;
	u8    num_bis;
	struct hci_bis bis;
} __packed;

pub const HCI_OP_LE_TERM_BIG: _ = 0x206a;
#[repr(C, packed)]
pub struct hci_cp_le_term_big {
	u8    handle;
	u8    reason;
} __packed;

pub const HCI_OP_LE_BIG_CREATE_SYNC: _ = 0x206b;
#[repr(C, packed)]
pub struct hci_cp_le_big_create_sync {
	u8    handle;
	u16  sync_handle;
	u8    encryption;
	u8    bcode[16];
	u8    mse;
	u16  timeout;
	u8    num_bis;
	u8    bis[] __counted_by(num_bis);
} __packed;

pub const HCI_OP_LE_BIG_TERM_SYNC: _ = 0x206c;
#[repr(C, packed)]
pub struct hci_cp_le_big_term_sync {
	u8    handle;
} __packed;

pub const HCI_OP_LE_SETUP_ISO_PATH: _ = 0x206e;
#[repr(C, packed)]
pub struct hci_cp_le_setup_iso_path {
	u16  handle;
	u8    direction;
	u8    path;
	u8    codec;
	u16  codec_cid;
	u16  codec_vid;
	u8    delay[3];
	u8    codec_cfg_len;
	u8    codec_cfg[];
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_setup_iso_path {
	u8    status;
	u16  handle;
} __packed;

pub const HCI_OP_LE_SET_HOST_FEATURE: _ = 0x2074;
#[repr(C, packed)]
pub struct hci_cp_le_set_host_feature {
	u8     bit_number;
	u8     bit_value;
} __packed;

pub const HCI_OP_LE_READ_ALL_LOCAL_FEATURES: _ = 0x2087;
#[repr(C, packed)]
pub struct hci_rp_le_read_all_local_features {
	u8    status;
	u8    page;
	u8    features[248];
} __packed;

pub const HCI_OP_LE_READ_ALL_REMOTE_FEATURES: _ = 0x2088;
#[repr(C, packed)]
pub struct hci_cp_le_read_all_remote_features {
	u16	 handle;
	u8	 pages;
} __packed;

/* Channel Sounding Commands */
pub const HCI_OP_LE_CS_RD_LOCAL_SUPP_CAP: _ = 0x2089;
#[repr(C, packed)]
pub struct hci_rp_le_cs_rd_local_supp_cap {
	u8	status;
	u8	num_config_supported;
	u16	max_consecutive_procedures_supported;
	u8	num_antennas_supported;
	u8	max_antenna_paths_supported;
	u8	roles_supported;
	u8	modes_supported;
	u8	rtt_capability;
	u8	rtt_aa_only_n;
	u8	rtt_sounding_n;
	u8	rtt_random_payload_n;
	u16	nadm_sounding_capability;
	u16	nadm_random_capability;
	u8	cs_sync_phys_supported;
	u16	subfeatures_supported;
	u16	t_ip1_times_supported;
	u16	t_ip2_times_supported;
	u16	t_fcs_times_supported;
	u16	t_pm_times_supported;
	u8	t_sw_time_supported;
	u8	tx_snr_capability;
} __packed;

pub const HCI_OP_LE_CS_RD_RMT_SUPP_CAP: _ = 0x208A;
#[repr(C, packed)]
pub struct hci_cp_le_cs_rd_local_supp_cap {
	u16	handle;
} __packed;

pub const HCI_OP_LE_CS_WR_CACHED_RMT_SUPP_CAP: _ = 0x208B;
#[repr(C, packed)]
pub struct hci_cp_le_cs_wr_cached_rmt_supp_cap {
	u16	handle;
	u8	num_config_supported;
	u16	max_consecutive_procedures_supported;
	u8	num_antennas_supported;
	u8	max_antenna_paths_supported;
	u8	roles_supported;
	u8	modes_supported;
	u8	rtt_capability;
	u8	rtt_aa_only_n;
	u8	rtt_sounding_n;
	u8	rtt_random_payload_n;
	u16	nadm_sounding_capability;
	u16	nadm_random_capability;
	u8	cs_sync_phys_supported;
	u16	subfeatures_supported;
	u16	t_ip1_times_supported;
	u16	t_ip2_times_supported;
	u16	t_fcs_times_supported;
	u16	t_pm_times_supported;
	u8	t_sw_time_supported;
	u8	tx_snr_capability;
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_cs_wr_cached_rmt_supp_cap {
	u8	status;
	u16	handle;
} __packed;

pub const HCI_OP_LE_CS_SEC_ENABLE: _ = 0x208C;
#[repr(C, packed)]
pub struct hci_cp_le_cs_sec_enable {
	u16	handle;
} __packed;

pub const HCI_OP_LE_CS_SET_DEFAULT_SETTINGS: _ = 0x208D;
#[repr(C, packed)]
pub struct hci_cp_le_cs_set_default_settings {
	u16	handle;
	u8	role_enable;
	u8	cs_sync_ant_sel;
	i8	max_tx_power;
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_cs_set_default_settings {
	u8	status;
	u16	handle;
} __packed;

pub const HCI_OP_LE_CS_RD_RMT_FAE_TABLE: _ = 0x208E;
#[repr(C, packed)]
pub struct hci_cp_le_cs_rd_rmt_fae_table {
	u16	handle;
} __packed;

pub const HCI_OP_LE_CS_WR_CACHED_RMT_FAE_TABLE: _ = 0x208F;
#[repr(C, packed)]
pub struct hci_cp_le_cs_wr_rmt_cached_fae_table {
	u16	handle;
	u8	remote_fae_table[72];
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_cs_wr_rmt_cached_fae_table {
	u8	status;
	u16	handle;
} __packed;

pub const HCI_OP_LE_CS_CREATE_CONFIG: _ = 0x2090;
#[repr(C, packed)]
pub struct hci_cp_le_cs_create_config {
	u16	handle;
	u8	config_id;
	u8	create_context;
	u8	main_mode_type;
	u8	sub_mode_type;
	u8	min_main_mode_steps;
	u8	max_main_mode_steps;
	u8	main_mode_repetition;
	u8	mode_0_steps;
	u8	role;
	u8	rtt_type;
	u8	cs_sync_phy;
	u8	channel_map[10];
	u8	channel_map_repetition;
	u8	channel_selection_type;
	u8	ch3c_shape;
	u8	ch3c_jump;
	u8	reserved;
} __packed;

pub const HCI_OP_LE_CS_REMOVE_CONFIG: _ = 0x2091;
#[repr(C, packed)]
pub struct hci_cp_le_cs_remove_config {
	u16	handle;
	u8	config_id;
} __packed;

pub const HCI_OP_LE_CS_SET_CH_CLASSIFICATION: _ = 0x2092;
#[repr(C, packed)]
pub struct hci_cp_le_cs_set_ch_classification {
	u8	ch_classification[10];
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_cs_set_ch_classification {
	u8	status;
} __packed;

pub const HCI_OP_LE_CS_SET_PROC_PARAM: _ = 0x2093;
#[repr(C, packed)]
pub struct hci_cp_le_cs_set_proc_param {
	u16	handle;
	u8	config_id;
	u16	max_procedure_len;
	u16	min_procedure_interval;
	u16	max_procedure_interval;
	u16	max_procedure_count;
	u8	min_subevent_len[3];
	u8	max_subevent_len[3];
	u8	tone_antenna_config_selection;
	u8	phy;
	u8	tx_power_delta;
	u8	preferred_peer_antenna;
	u8	snr_control_initiator;
	u8	snr_control_reflector;
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_cs_set_proc_param {
	u8	status;
	u16	handle;
} __packed;

pub const HCI_OP_LE_CS_SET_PROC_ENABLE: _ = 0x2094;
#[repr(C, packed)]
pub struct hci_cp_le_cs_set_proc_enable {
	u16	handle;
	u8	config_id;
	u8	enable;
} __packed;

pub const HCI_OP_LE_CS_TEST: _ = 0x2095;
#[repr(C, packed)]
pub struct hci_cp_le_cs_test {
	u8	main_mode_type;
	u8	sub_mode_type;
	u8	main_mode_repetition;
	u8	mode_0_steps;
	u8	role;
	u8	rtt_type;
	u8	cs_sync_phy;
	u8	cs_sync_antenna_selection;
	u8	subevent_len[3];
	u16	subevent_interval;
	u8	max_num_subevents;
	u8	transmit_power_level;
	u8	t_ip1_time;
	u8	t_ip2_time;
	u8	t_fcs_time;
	u8	t_pm_time;
	u8	t_sw_time;
	u8	tone_antenna_config_selection;
	u8	reserved;
	u8	snr_control_initiator;
	u8	snr_control_reflector;
	u16	drbg_nonce;
	u8	channel_map_repetition;
	u16	override_config;
	u8	override_parameters_length;
	u8	override_parameters_data[];
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_cs_test {
	u8	status;
} __packed;

pub const HCI_OP_LE_CS_TEST_END: _ = 0x2096;

pub const HCI_OP_LE_SET_HOST_FEATURE_V2: _ = 0x2097;
#[repr(C, packed)]
pub struct hci_cp_le_set_host_feature_v2 {
	u16	bit_number;
	u8	bit_value;
} __packed;

pub const HCI_OP_LE_CONN_RATE: _ = 0x20a1;
#[repr(C, packed)]
pub struct hci_cp_le_conn_rate {
	u16	handle;
	u16	interval_min;
	u16	interval_max;
	u16	subrate_min;
	u16	subrate_max;
	u16	max_latency;
	u16	cont_num;
	u16	supv_timeout;
	u16	min_ce_len;
	u16	max_ce_len;
} __packed;

pub const HCI_OP_LE_SET_DEF_RATE: _ = 0x20a2;
#[repr(C, packed)]
pub struct hci_cp_le_set_def_rate {
	u16	interval_min;
	u16	interval_max;
	u16	subrate_min;
	u16	subrate_max;
	u16	max_latency;
	u16	cont_num;
	u16	supv_timeout;
	u16	min_ce_len;
	u16	max_ce_len;
} __packed;

pub const HCI_OP_LE_READ_CONN_INTERVAL: _ = 0x20a3;
#[repr(C, packed)]
pub struct hci_le_conn_interval_group {
	u16	min;
	u16	max;
	u16	stride;
} __packed;

#[repr(C, packed)]
pub struct hci_rp_le_read_conn_interval {
	u8	status;
	u8	num_grps;
	struct hci_le_conn_interval_group grps[];
} __packed;

/* ---- HCI Events ---- */
#[repr(C, packed)]
pub struct hci_ev_status {
	u8    status;
} __packed;

pub const HCI_EV_INQUIRY_COMPLETE: _ = 0x01;

pub const HCI_EV_INQUIRY_RESULT: _ = 0x02;
#[repr(C, packed)]
pub struct inquiry_info {
	bdaddr_t bdaddr;
	u8     pscan_rep_mode;
	u8     pscan_period_mode;
	u8     pscan_mode;
	u8     dev_class[3];
	u16   clock_offset;
} __packed;

#[repr(C, packed)]
pub struct hci_ev_inquiry_result {
	u8    num;
	struct inquiry_info info[];
};

pub const HCI_EV_CONN_COMPLETE: _ = 0x03;
#[repr(C, packed)]
pub struct hci_ev_conn_complete {
	u8     status;
	u16   handle;
	bdaddr_t bdaddr;
	u8     link_type;
	u8     encr_mode;
} __packed;

pub const HCI_EV_CONN_REQUEST: _ = 0x04;
#[repr(C, packed)]
pub struct hci_ev_conn_request {
	bdaddr_t bdaddr;
	u8     dev_class[3];
	u8     link_type;
} __packed;

pub const HCI_EV_DISCONN_COMPLETE: _ = 0x05;
#[repr(C, packed)]
pub struct hci_ev_disconn_complete {
	u8     status;
	u16   handle;
	u8     reason;
} __packed;

pub const HCI_EV_AUTH_COMPLETE: _ = 0x06;
#[repr(C, packed)]
pub struct hci_ev_auth_complete {
	u8     status;
	u16   handle;
} __packed;

pub const HCI_EV_REMOTE_NAME: _ = 0x07;
#[repr(C, packed)]
pub struct hci_ev_remote_name {
	u8     status;
	bdaddr_t bdaddr;
	u8     name[HCI_MAX_NAME_LENGTH];
} __packed;

pub const HCI_EV_ENCRYPT_CHANGE: _ = 0x08;
#[repr(C, packed)]
pub struct hci_ev_encrypt_change {
	u8     status;
	u16   handle;
	u8     encrypt;
} __packed;

pub const HCI_EV_CHANGE_LINK_KEY_COMPLETE: _ = 0x09;
#[repr(C, packed)]
pub struct hci_ev_change_link_key_complete {
	u8     status;
	u16   handle;
} __packed;

pub const HCI_EV_REMOTE_FEATURES: _ = 0x0b;
#[repr(C, packed)]
pub struct hci_ev_remote_features {
	u8     status;
	u16   handle;
	u8     features[8];
} __packed;

pub const HCI_EV_REMOTE_VERSION: _ = 0x0c;
#[repr(C, packed)]
pub struct hci_ev_remote_version {
	u8     status;
	u16   handle;
	u8     lmp_ver;
	u16   manufacturer;
	u16   lmp_subver;
} __packed;

pub const HCI_EV_QOS_SETUP_COMPLETE: _ = 0x0d;
#[repr(C, packed)]
pub struct hci_qos {
	u8     service_type;
	u32    token_rate;
	u32    peak_bandwidth;
	u32    latency;
	u32    delay_variation;
} __packed;
#[repr(C, packed)]
pub struct hci_ev_qos_setup_complete {
	u8     status;
	u16   handle;
	struct   hci_qos qos;
} __packed;

pub const HCI_EV_CMD_COMPLETE: _ = 0x0e;
#[repr(C, packed)]
pub struct hci_ev_cmd_complete {
	u8     ncmd;
	u16   opcode;
} __packed;

pub const HCI_EV_CMD_STATUS: _ = 0x0f;
#[repr(C, packed)]
pub struct hci_ev_cmd_status {
	u8     status;
	u8     ncmd;
	u16   opcode;
} __packed;

pub const HCI_EV_HARDWARE_ERROR: _ = 0x10;
#[repr(C, packed)]
pub struct hci_ev_hardware_error {
	u8     code;
} __packed;

pub const HCI_EV_ROLE_CHANGE: _ = 0x12;
#[repr(C, packed)]
pub struct hci_ev_role_change {
	u8     status;
	bdaddr_t bdaddr;
	u8     role;
} __packed;

pub const HCI_EV_NUM_COMP_PKTS: _ = 0x13;
#[repr(C, packed)]
pub struct hci_comp_pkts_info {
	u16   handle;
	u16   count;
} __packed;

#[repr(C, packed)]
pub struct hci_ev_num_comp_pkts {
	u8     num;
	struct hci_comp_pkts_info handles[];
} __packed;

pub const HCI_EV_MODE_CHANGE: _ = 0x14;
#[repr(C, packed)]
pub struct hci_ev_mode_change {
	u8     status;
	u16   handle;
	u8     mode;
	u16   interval;
} __packed;

pub const HCI_EV_PIN_CODE_REQ: _ = 0x16;
#[repr(C, packed)]
pub struct hci_ev_pin_code_req {
	bdaddr_t bdaddr;
} __packed;

pub const HCI_EV_LINK_KEY_REQ: _ = 0x17;
#[repr(C, packed)]
pub struct hci_ev_link_key_req {
	bdaddr_t bdaddr;
} __packed;

pub const HCI_EV_LINK_KEY_NOTIFY: _ = 0x18;
#[repr(C, packed)]
pub struct hci_ev_link_key_notify {
	bdaddr_t bdaddr;
	u8     link_key[HCI_LINK_KEY_SIZE];
	u8     key_type;
} __packed;

pub const HCI_EV_CLOCK_OFFSET: _ = 0x1c;
#[repr(C, packed)]
pub struct hci_ev_clock_offset {
	u8     status;
	u16   handle;
	u16   clock_offset;
} __packed;

pub const HCI_EV_PKT_TYPE_CHANGE: _ = 0x1d;
#[repr(C, packed)]
pub struct hci_ev_pkt_type_change {
	u8     status;
	u16   handle;
	u16   pkt_type;
} __packed;

pub const HCI_EV_PSCAN_REP_MODE: _ = 0x20;
#[repr(C, packed)]
pub struct hci_ev_pscan_rep_mode {
	bdaddr_t bdaddr;
	u8     pscan_rep_mode;
} __packed;

pub const HCI_EV_INQUIRY_RESULT_WITH_RSSI: _ = 0x22;
#[repr(C, packed)]
pub struct inquiry_info_rssi {
	bdaddr_t bdaddr;
	u8     pscan_rep_mode;
	u8     pscan_period_mode;
	u8     dev_class[3];
	u16   clock_offset;
	i8     rssi;
} __packed;
#[repr(C, packed)]
pub struct inquiry_info_rssi_pscan {
	bdaddr_t bdaddr;
	u8     pscan_rep_mode;
	u8     pscan_period_mode;
	u8     pscan_mode;
	u8     dev_class[3];
	u16   clock_offset;
	i8     rssi;
} __packed;
#[repr(C, packed)]
pub struct hci_ev_inquiry_result_rssi {
	u8     num;
	u8     data[];
} __packed;

pub const HCI_EV_REMOTE_EXT_FEATURES: _ = 0x23;
#[repr(C, packed)]
pub struct hci_ev_remote_ext_features {
	u8     status;
	u16   handle;
	u8     page;
	u8     max_page;
	u8     features[8];
} __packed;

pub const HCI_EV_SYNC_CONN_COMPLETE: _ = 0x2c;
#[repr(C, packed)]
pub struct hci_ev_sync_conn_complete {
	u8     status;
	u16   handle;
	bdaddr_t bdaddr;
	u8     link_type;
	u8     tx_interval;
	u8     retrans_window;
	u16   rx_pkt_len;
	u16   tx_pkt_len;
	u8     air_mode;
} __packed;

pub const HCI_EV_SYNC_CONN_CHANGED: _ = 0x2d;
#[repr(C, packed)]
pub struct hci_ev_sync_conn_changed {
	u8     status;
	u16   handle;
	u8     tx_interval;
	u8     retrans_window;
	u16   rx_pkt_len;
	u16   tx_pkt_len;
} __packed;

pub const HCI_EV_SNIFF_SUBRATE: _ = 0x2e;
#[repr(C, packed)]
pub struct hci_ev_sniff_subrate {
	u8     status;
	u16   handle;
	u16   max_tx_latency;
	u16   max_rx_latency;
	u16   max_remote_timeout;
	u16   max_local_timeout;
} __packed;

pub const HCI_EV_EXTENDED_INQUIRY_RESULT: _ = 0x2f;
#[repr(C, packed)]
pub struct extended_inquiry_info {
	bdaddr_t bdaddr;
	u8     pscan_rep_mode;
	u8     pscan_period_mode;
	u8     dev_class[3];
	u16   clock_offset;
	i8     rssi;
	u8     data[240];
} __packed;

#[repr(C, packed)]
pub struct hci_ev_ext_inquiry_result {
	u8     num;
	struct extended_inquiry_info info[];
} __packed;

pub const HCI_EV_KEY_REFRESH_COMPLETE: _ = 0x30;
#[repr(C, packed)]
pub struct hci_ev_key_refresh_complete {
	u8	status;
	u16	handle;
} __packed;

pub const HCI_EV_IO_CAPA_REQUEST: _ = 0x31;
#[repr(C, packed)]
pub struct hci_ev_io_capa_request {
	bdaddr_t bdaddr;
} __packed;

pub const HCI_EV_IO_CAPA_REPLY: _ = 0x32;
#[repr(C, packed)]
pub struct hci_ev_io_capa_reply {
	bdaddr_t bdaddr;
	u8     capability;
	u8     oob_data;
	u8     authentication;
} __packed;

pub const HCI_EV_USER_CONFIRM_REQUEST: _ = 0x33;
#[repr(C, packed)]
pub struct hci_ev_user_confirm_req {
	bdaddr_t	bdaddr;
	u32		passkey;
} __packed;

pub const HCI_EV_USER_PASSKEY_REQUEST: _ = 0x34;
#[repr(C, packed)]
pub struct hci_ev_user_passkey_req {
	bdaddr_t	bdaddr;
} __packed;

pub const HCI_EV_REMOTE_OOB_DATA_REQUEST: _ = 0x35;
#[repr(C, packed)]
pub struct hci_ev_remote_oob_data_request {
	bdaddr_t bdaddr;
} __packed;

pub const HCI_EV_SIMPLE_PAIR_COMPLETE: _ = 0x36;
#[repr(C, packed)]
pub struct hci_ev_simple_pair_complete {
	u8     status;
	bdaddr_t bdaddr;
} __packed;

pub const HCI_EV_USER_PASSKEY_NOTIFY: _ = 0x3b;
#[repr(C, packed)]
pub struct hci_ev_user_passkey_notify {
	bdaddr_t	bdaddr;
	u32		passkey;
} __packed;

pub const HCI_KEYPRESS_STARTED: _ = 0;
pub const HCI_KEYPRESS_ENTERED: _ = 1;
pub const HCI_KEYPRESS_ERASED: _ = 2;
pub const HCI_KEYPRESS_CLEARED: _ = 3;
pub const HCI_KEYPRESS_COMPLETED: _ = 4;

pub const HCI_EV_KEYPRESS_NOTIFY: _ = 0x3c;
#[repr(C, packed)]
pub struct hci_ev_keypress_notify {
	bdaddr_t	bdaddr;
	u8		type;
} __packed;

pub const HCI_EV_REMOTE_HOST_FEATURES: _ = 0x3d;
#[repr(C, packed)]
pub struct hci_ev_remote_host_features {
	bdaddr_t bdaddr;
	u8     features[8];
} __packed;

pub const HCI_EV_LE_META: _ = 0x3e;
#[repr(C, packed)]
pub struct hci_ev_le_meta {
	u8     subevent;
} __packed;

pub const HCI_EV_PHY_LINK_COMPLETE: _ = 0x40;
#[repr(C, packed)]
pub struct hci_ev_phy_link_complete {
	u8     status;
	u8     phy_handle;
} __packed;

pub const HCI_EV_CHANNEL_SELECTED: _ = 0x41;
#[repr(C, packed)]
pub struct hci_ev_channel_selected {
	u8     phy_handle;
} __packed;

pub const HCI_EV_DISCONN_PHY_LINK_COMPLETE: _ = 0x42;
#[repr(C, packed)]
pub struct hci_ev_disconn_phy_link_complete {
	u8     status;
	u8     phy_handle;
	u8     reason;
} __packed;

pub const HCI_EV_LOGICAL_LINK_COMPLETE: _ = 0x45;
#[repr(C, packed)]
pub struct hci_ev_logical_link_complete {
	u8     status;
	u16   handle;
	u8     phy_handle;
	u8     flow_spec_id;
} __packed;

pub const HCI_EV_DISCONN_LOGICAL_LINK_COMPLETE: _ = 0x46;
#[repr(C, packed)]
pub struct hci_ev_disconn_logical_link_complete {
	u8     status;
	u16   handle;
	u8     reason;
} __packed;

pub const HCI_EV_NUM_COMP_BLOCKS: _ = 0x48;
#[repr(C, packed)]
pub struct hci_comp_blocks_info {
	u16   handle;
	u16   pkts;
	u16   blocks;
} __packed;

#[repr(C, packed)]
pub struct hci_ev_num_comp_blocks {
	u16   num_blocks;
	u8     num_hndl;
	struct hci_comp_blocks_info handles[];
} __packed;

pub const HCI_EV_SYNC_TRAIN_COMPLETE: _ = 0x4F;
#[repr(C, packed)]
pub struct hci_ev_sync_train_complete {
	u8	status;
} __packed;

pub const HCI_EV_PERIPHERAL_PAGE_RESP_TIMEOUT: _ = 0x54;

pub const HCI_EV_LE_CONN_COMPLETE: _ = 0x01;
#[repr(C, packed)]
pub struct hci_ev_le_conn_complete {
	u8     status;
	u16   handle;
	u8     role;
	u8     bdaddr_type;
	bdaddr_t bdaddr;
	u16   interval;
	u16   latency;
	u16   supervision_timeout;
	u8     clk_accurancy;
} __packed;

/* Advertising report event types */
pub const LE_ADV_IND: _ = 0x00;
pub const LE_ADV_DIRECT_IND: _ = 0x01;
pub const LE_ADV_SCAN_IND: _ = 0x02;
pub const LE_ADV_NONCONN_IND: _ = 0x03;
pub const LE_ADV_SCAN_RSP: _ = 0x04;
pub const LE_ADV_INVALID: _ = 0x05;

/* Legacy event types in extended adv report */
pub const LE_LEGACY_ADV_IND: _ = 0x0013;
pub const LE_LEGACY_ADV_DIRECT_IND: _ = 0x0015;
pub const LE_LEGACY_ADV_SCAN_IND: _ = 0x0012;
pub const LE_LEGACY_NONCONN_IND: _ = 0x0010;
pub const LE_LEGACY_SCAN_RSP_ADV: _ = 0x001b;
pub const LE_LEGACY_SCAN_RSP_ADV_SCAN: _ = 0x001a;

/* Extended Advertising event types */
pub const LE_EXT_ADV_NON_CONN_IND: _ = 0x0000;
pub const LE_EXT_ADV_CONN_IND: _ = 0x0001;
pub const LE_EXT_ADV_SCAN_IND: _ = 0x0002;
pub const LE_EXT_ADV_DIRECT_IND: _ = 0x0004;
pub const LE_EXT_ADV_SCAN_RSP: _ = 0x0008;
pub const LE_EXT_ADV_LEGACY_PDU: _ = 0x0010;
pub const LE_EXT_ADV_DATA_STATUS_MASK: _ = 0x0060;
pub const LE_EXT_ADV_EVT_TYPE_MASK: _ = 0x007f;

pub const ADDR_LE_DEV_PUBLIC: _ = 0x00;
pub const ADDR_LE_DEV_RANDOM: _ = 0x01;
pub const ADDR_LE_DEV_PUBLIC_RESOLVED: _ = 0x02;
pub const ADDR_LE_DEV_RANDOM_RESOLVED: _ = 0x03;

pub const HCI_EV_LE_ADVERTISING_REPORT: _ = 0x02;
#[repr(C, packed)]
pub struct hci_ev_le_advertising_info {
	u8	 type;
	u8	 bdaddr_type;
	bdaddr_t bdaddr;
	u8	 length;
	u8	 data[];
} __packed;

#[repr(C, packed)]
pub struct hci_ev_le_advertising_report {
	u8    num;
	struct hci_ev_le_advertising_info info[];
} __packed;

pub const HCI_EV_LE_CONN_UPDATE_COMPLETE: _ = 0x03;
#[repr(C, packed)]
pub struct hci_ev_le_conn_update_complete {
	u8     status;
	u16   handle;
	u16   interval;
	u16   latency;
	u16   supervision_timeout;
} __packed;

pub const HCI_EV_LE_REMOTE_FEAT_COMPLETE: _ = 0x04;
#[repr(C, packed)]
pub struct hci_ev_le_remote_feat_complete {
	u8     status;
	u16   handle;
	u8     features[8];
} __packed;

pub const HCI_EV_LE_LTK_REQ: _ = 0x05;
#[repr(C, packed)]
pub struct hci_ev_le_ltk_req {
	u16	handle;
	u64	rand;
	u16	ediv;
} __packed;

pub const HCI_EV_LE_REMOTE_CONN_PARAM_REQ: _ = 0x06;
#[repr(C, packed)]
pub struct hci_ev_le_remote_conn_param_req {
	u16 handle;
	u16 interval_min;
	u16 interval_max;
	u16 latency;
	u16 timeout;
} __packed;

pub const HCI_EV_LE_DATA_LEN_CHANGE: _ = 0x07;
#[repr(C, packed)]
pub struct hci_ev_le_data_len_change {
	u16	handle;
	u16	tx_len;
	u16	tx_time;
	u16	rx_len;
	u16	rx_time;
} __packed;

pub const HCI_EV_LE_DIRECT_ADV_REPORT: _ = 0x0B;
#[repr(C, packed)]
pub struct hci_ev_le_direct_adv_info {
	u8	 type;
	u8	 bdaddr_type;
	bdaddr_t bdaddr;
	u8	 direct_addr_type;
	bdaddr_t direct_addr;
	i8	 rssi;
} __packed;

#[repr(C, packed)]
pub struct hci_ev_le_direct_adv_report {
	u8	 num;
	struct hci_ev_le_direct_adv_info info[];
} __packed;

pub const HCI_EV_LE_PHY_UPDATE_COMPLETE: _ = 0x0c;
#[repr(C, packed)]
pub struct hci_ev_le_phy_update_complete {
	u8  status;
	u16 handle;
	u8  tx_phy;
	u8  rx_phy;
} __packed;

pub const HCI_EV_LE_EXT_ADV_REPORT: _ = 0x0d;
#[repr(C, packed)]
pub struct hci_ev_le_ext_adv_info {
	u16   type;
	u8	 bdaddr_type;
	bdaddr_t bdaddr;
	u8	 primary_phy;
	u8	 secondary_phy;
	u8	 sid;
	u8	 tx_power;
	i8	 rssi;
	u16   interval;
	u8     direct_addr_type;
	bdaddr_t direct_addr;
	u8     length;
	u8     data[];
} __packed;

#[repr(C, packed)]
pub struct hci_ev_le_ext_adv_report {
	u8     num;
	struct hci_ev_le_ext_adv_info info[];
} __packed;

pub const HCI_EV_LE_PA_SYNC_ESTABLISHED: _ = 0x0e;
#[repr(C, packed)]
pub struct hci_ev_le_pa_sync_established {
	u8      status;
	u16    handle;
	u8      sid;
	u8      bdaddr_type;
	bdaddr_t  bdaddr;
	u8      phy;
	u16    interval;
	u8      clock_accuracy;
} __packed;

pub const HCI_EV_LE_ENHANCED_CONN_COMPLETE: _ = 0x0a;
#[repr(C, packed)]
pub struct hci_ev_le_enh_conn_complete {
	u8      status;
	u16    handle;
	u8      role;
	u8      bdaddr_type;
	bdaddr_t  bdaddr;
	bdaddr_t  local_rpa;
	bdaddr_t  peer_rpa;
	u16    interval;
	u16    latency;
	u16    supervision_timeout;
	u8      clk_accurancy;
} __packed;

pub const HCI_EV_LE_PER_ADV_REPORT: _ = 0x0f;
#[repr(C, packed)]
pub struct hci_ev_le_per_adv_report {
	u16	 sync_handle;
	u8	 tx_power;
	u8	 rssi;
	u8	 cte_type;
	u8	 data_status;
	u8     length;
	u8     data[];
} __packed;

pub const HCI_EV_LE_PA_SYNC_LOST: _ = 0x10;
#[repr(C, packed)]
pub struct hci_ev_le_pa_sync_lost {
	u16 handle;
} __packed;

pub const LE_PA_DATA_COMPLETE: _ = 0x00;
pub const LE_PA_DATA_MORE_TO_COME: _ = 0x01;
pub const LE_PA_DATA_TRUNCATED: _ = 0x02;

pub const HCI_EV_LE_EXT_ADV_SET_TERM: _ = 0x12;
#[repr(C, packed)]
pub struct hci_evt_le_ext_adv_set_term {
	u8	status;
	u8	handle;
	u16	conn_handle;
	u8	num_evts;
} __packed;

pub const HCI_EV_LE_PAST_RECEIVED: _ = 0x18;
#[repr(C, packed)]
pub struct hci_ev_le_past_received {
	u8   status;
	u16 handle;
	u16 service_data;
	u16 sync_handle;
	u8   sid;
	u8   bdaddr_type;
	bdaddr_t  bdaddr;
	u8   phy;
	u16 interval;
	u8   clock_accuracy;
} __packed;

pub const HCI_EVT_LE_CIS_ESTABLISHED: _ = 0x19;
#[repr(C, packed)]
pub struct hci_evt_le_cis_established {
	u8  status;
	u16 handle;
	u8  cig_sync_delay[3];
	u8  cis_sync_delay[3];
	u8  c_latency[3];
	u8  p_latency[3];
	u8  c_phy;
	u8  p_phy;
	u8  nse;
	u8  c_bn;
	u8  p_bn;
	u8  c_ft;
	u8  p_ft;
	u16 c_mtu;
	u16 p_mtu;
	u16 interval;
} __packed;

pub const HCI_EVT_LE_CIS_REQ: _ = 0x1a;
#[repr(C, packed)]
pub struct hci_evt_le_cis_req {
	u16 acl_handle;
	u16 cis_handle;
	u8  cig_id;
	u8  cis_id;
} __packed;

pub const HCI_EVT_LE_CREATE_BIG_COMPLETE: _ = 0x1b;
#[repr(C, packed)]
pub struct hci_evt_le_create_big_complete {
	u8    status;
	u8    handle;
	u8    sync_delay[3];
	u8    transport_delay[3];
	u8    phy;
	u8    nse;
	u8    bn;
	u8    pto;
	u8    irc;
	u16  max_pdu;
	u16  interval;
	u8    num_bis;
	u16  bis_handle[];
} __packed;

pub const HCI_EVT_LE_BIG_SYNC_ESTABLISHED: _ = 0x1d;
#[repr(C, packed)]
pub struct hci_evt_le_big_sync_established {
	u8    status;
	u8    handle;
	u8    latency[3];
	u8    nse;
	u8    bn;
	u8    pto;
	u8    irc;
	u16  max_pdu;
	u16  interval;
	u8    num_bis;
	u16  bis[];
} __packed;

pub const HCI_EVT_LE_BIG_SYNC_LOST: _ = 0x1e;
#[repr(C, packed)]
pub struct hci_evt_le_big_sync_lost {
	u8    handle;
	u8    reason;
} __packed;

pub const HCI_EVT_LE_BIG_INFO_ADV_REPORT: _ = 0x22;
#[repr(C, packed)]
pub struct hci_evt_le_big_info_adv_report {
	u16  sync_handle;
	u8    num_bis;
	u8    nse;
	u16  iso_interval;
	u8    bn;
	u8    pto;
	u8    irc;
	u16  max_pdu;
	u8    sdu_interval[3];
	u16  max_sdu;
	u8    phy;
	u8    framing;
	u8    encryption;
} __packed;

pub const HCI_EVT_LE_ALL_REMOTE_FEATURES_COMPLETE: _ = 0x2b;
#[repr(C, packed)]
pub struct hci_evt_le_read_all_remote_features_complete {
	u8    status;
	u16  handle;
	u8    max_pages;
	u8    valid_pages;
	u8    features[248];
} __packed;

/* Channel Sounding Events */
pub const HCI_EVT_LE_CS_READ_RMT_SUPP_CAP_COMPLETE: _ = 0x2C;
#[repr(C, packed)]
pub struct hci_evt_le_cs_read_rmt_supp_cap_complete {
	u8	status;
	u16	handle;
	u8	num_configs_supp;
	u16	max_consec_proc_supp;
	u8	num_ant_supp;
	u8	max_ant_path_supp;
	u8	roles_supp;
	u8	modes_supp;
	u8	rtt_cap;
	u8	rtt_aa_only_n;
	u8	rtt_sounding_n;
	u8	rtt_rand_payload_n;
	u16	nadm_sounding_cap;
	u16	nadm_rand_cap;
	u8	cs_sync_phys_supp;
	u16	sub_feat_supp;
	u16	t_ip1_times_supp;
	u16	t_ip2_times_supp;
	u16	t_fcs_times_supp;
	u16	t_pm_times_supp;
	u8	t_sw_times_supp;
	u8	tx_snr_cap;
} __packed;

pub const HCI_EVT_LE_CS_READ_RMT_FAE_TABLE_COMPLETE: _ = 0x2D;
#[repr(C, packed)]
pub struct hci_evt_le_cs_read_rmt_fae_table_complete {
	u8	status;
	u16	handle;
	u8	remote_fae_table[72];
} __packed;

pub const HCI_EVT_LE_CS_SECURITY_ENABLE_COMPLETE: _ = 0x2E;
#[repr(C, packed)]
pub struct hci_evt_le_cs_security_enable_complete {
	u8	status;
	u16	handle;
} __packed;

pub const HCI_EVT_LE_CS_CONFIG_COMPLETE: _ = 0x2F;
#[repr(C, packed)]
pub struct hci_evt_le_cs_config_complete {
	u8	status;
	u16	handle;
	u8	config_id;
	u8	action;
	u8	main_mode_type;
	u8	sub_mode_type;
	u8	min_main_mode_steps;
	u8	max_main_mode_steps;
	u8	main_mode_rep;
	u8	mode_0_steps;
	u8	role;
	u8	rtt_type;
	u8	cs_sync_phy;
	u8	channel_map[10];
	u8	channel_map_rep;
	u8	channel_sel_type;
	u8	ch3c_shape;
	u8	ch3c_jump;
	u8	reserved;
	u8	t_ip1_time;
	u8	t_ip2_time;
	u8	t_fcs_time;
	u8	t_pm_time;
} __packed;

pub const HCI_EVT_LE_CS_PROCEDURE_ENABLE_COMPLETE: _ = 0x30;
#[repr(C, packed)]
pub struct hci_evt_le_cs_procedure_enable_complete {
	u8	status;
	u16	handle;
	u8	config_id;
	u8	state;
	u8	tone_ant_config_sel;
	i8	sel_tx_pwr;
	u8	sub_evt_len[3];
	u8	sub_evts_per_evt;
	u16	sub_evt_intrvl;
	u16	evt_intrvl;
	u16	proc_intrvl;
	u16	proc_counter;
	u16	max_proc_len;
} __packed;

pub const HCI_EVT_LE_CS_SUBEVENT_RESULT: _ = 0x31;
#[repr(C, packed)]
pub struct hci_evt_le_cs_subevent_result {
	u16	handle;
	u8	config_id;
	u16	start_acl_conn_evt_counter;
	u16	proc_counter;
	u16	freq_comp;
	u8	ref_pwr_lvl;
	u8	proc_done_status;
	u8	subevt_done_status;
	u8	abort_reason;
	u8	num_ant_paths;
	u8	num_steps_reported;
	u8	step_mode[0]; /* depends on num_steps_reported */
	u8	step_channel[0]; /* depends on num_steps_reported */
	u8	step_data_length[0]; /* depends on num_steps_reported */
	u8	step_data[0]; /* depends on num_steps_reported */
} __packed;

pub const HCI_EVT_LE_CS_SUBEVENT_RESULT_CONTINUE: _ = 0x32;
#[repr(C, packed)]
pub struct hci_evt_le_cs_subevent_result_continue {
	u16	handle;
	u8	config_id;
	u8	proc_done_status;
	u8	subevt_done_status;
	u8	abort_reason;
	u8	num_ant_paths;
	u8	num_steps_reported;
	u8	step_mode[0]; /* depends on num_steps_reported */
	u8	step_channel[0]; /* depends on num_steps_reported */
	u8	step_data_length[0]; /* depends on num_steps_reported */
	u8	step_data[0]; /* depends on num_steps_reported */
} __packed;

pub const HCI_EVT_LE_CS_TEST_END_COMPLETE: _ = 0x33;
#[repr(C, packed)]
pub struct hci_evt_le_cs_test_end_complete {
	u8	status;
} __packed;

pub const HCI_EVT_LE_CONN_RATE_CHANGE: _ = 0x37;
#[repr(C, packed)]
pub struct hci_evt_le_conn_rate_change {
	u8	status;
	u16	handle;
	u16	interval;
	u16	subrate;
	u16	latency;
	u16	cont_number;
	u16	supv_timeout;
} __packed;

pub const HCI_EV_VENDOR: _ = 0xff;

/* Internal events generated by Bluetooth stack */
pub const HCI_EV_STACK_INTERNAL: _ = 0xfd;
#[repr(C, packed)]
pub struct hci_ev_stack_internal {
	u16    type;
	u8     data[];
} __packed;

pub const HCI_EV_SI_DEVICE: _ = 0x01;
#[repr(C, packed)]
pub struct hci_ev_si_device {
	u16    event;
	u16    dev_id;
} __packed;

pub const HCI_EV_SI_SECURITY: _ = 0x02;
#[repr(C, packed)]
pub struct hci_ev_si_security {
	u16    event;
	u16    proto;
	u16    subproto;
	u8     incoming;
} __packed;

/* ---- HCI Packet structures ---- */
pub const HCI_COMMAND_HDR_SIZE: _ = 3;
pub const HCI_EVENT_HDR_SIZE: _ = 2;
pub const HCI_MAX_EVENT_PLEN: _ = 255;
pub const HCI_ACL_HDR_SIZE: _ = 4;
pub const HCI_SCO_HDR_SIZE: _ = 3;
pub const HCI_ISO_HDR_SIZE: _ = 4;

#[repr(C, packed)]
pub struct hci_command_hdr {
	u16	opcode;		/* OCF & OGF */
	u8	plen;
} __packed;

#[repr(C, packed)]
pub struct hci_event_hdr {
	u8	evt;
	u8	plen;
} __packed;

#[repr(C, packed)]
pub struct hci_acl_hdr {
	u16	handle;		/* Handle & Flags(PB, BC) */
	u16	dlen;
} __packed;

#[repr(C, packed)]
pub struct hci_sco_hdr {
	u16	handle;
	u8	dlen;
} __packed;

#[repr(C, packed)]
pub struct hci_iso_hdr {
	u16	handle;
	u16	dlen;
	u8	data[];
} __packed;

/* ISO data packet status flags */
pub const HCI_ISO_STATUS_VALID: _ = 0x00;
pub const HCI_ISO_STATUS_INVALID: _ = 0x01;
pub const HCI_ISO_STATUS_NOP: _ = 0x02;

pub const HCI_ISO_DATA_HDR_SIZE: _ = 4;
#[repr(C, packed)]
pub struct hci_iso_data_hdr {
	u16	sn;
	u16	slen;
};

pub const HCI_ISO_TS_DATA_HDR_SIZE: _ = 8;
#[repr(C, packed)]
pub struct hci_iso_ts_data_hdr {
	u32	ts;
	u16	sn;
	u16	slen;
};

static inline struct hci_event_hdr *hci_event_hdr(const struct sk_buff *skb)
{
	return (struct hci_event_hdr *) skb->data;
}

static inline struct hci_acl_hdr *hci_acl_hdr(const struct sk_buff *skb)
{
	return (struct hci_acl_hdr *) skb->data;
}

static inline struct hci_sco_hdr *hci_sco_hdr(const struct sk_buff *skb)
{
	return (struct hci_sco_hdr *) skb->data;
}

static inline struct hci_iso_hdr *hci_iso_hdr(const struct sk_buff *skb)
{
	return (struct hci_iso_hdr *)skb->data;
}

/* Command opcode pack/unpack */
// #define hci_opcode_pack(ogf, ocf)	((__u16) ((ocf & 0x03ff)|(ogf << 10)))
// #define hci_opcode_ogf(op)		(op >> 10)
// #define hci_opcode_ocf(op)		(op & 0x03ff)

/* ACL handle and flags pack/unpack */
// #define hci_handle_pack(h, f)	((__u16) ((h & 0x0fff)|(f << 12)))
// #define hci_handle(h)		(h & 0x0fff)
// #define hci_flags(h)		(h >> 12)

static inline u16 hci_acl_handle(const struct sk_buff *skb)
{
	return hci_handle(u16_to_cpu(hci_acl_hdr(skb)->handle));
}

static inline u16 hci_acl_dlen(const struct sk_buff *skb)
{
	return u16_to_cpu(hci_acl_hdr(skb)->dlen);
}

/* ISO handle and flags pack/unpack */
// #define hci_iso_flags_pb(f)		(f & 0x0003)
// #define hci_iso_flags_ts(f)		((f >> 2) & 0x0001)
// #define hci_iso_flags_pack(pb, ts)	((pb & 0x03) | ((ts & 0x01) << 2))

/* ISO data length and flags pack/unpack */
// #define hci_iso_data_len_pack(h, f)	((__u16) (((h) & 0x0fff) | \
						  (((f) & 0x3) << 14)))
// #define hci_iso_data_len(h)		((h) & 0x0fff)
// #define hci_iso_data_flags(h)		((h) >> 14)

/* codec transport types */
pub const HCI_TRANSPORT_SCO_ESCO: _ = 0x01;

/* le24 support */
static inline void hci_cpu_to_le24(u32 val, u8 dst[3])
{
	dst[0] = val & 0xff;
	dst[1] = (val & 0xff00) >> 8;
	dst[2] = (val & 0xff0000) >> 16;
}

// #endif /* __HCI_H */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
