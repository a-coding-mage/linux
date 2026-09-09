/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2015-2017 Google, Inc
 *
 * USB Type-C Port Controller Interface.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/usb/typec.h, linux/usb/tcpm.h

pub const TCPC_VENDOR_ID: u32 = 0x0;
pub const TCPC_PRODUCT_ID: u32 = 0x2;
pub const TCPC_BCD_DEV: u32 = 0x4;
pub const TCPC_TC_REV: u32 = 0x6;
pub const TCPC_PD_REV: u32 = 0x8;
pub const TCPC_PD_INT_REV: u32 = 0xa;

pub const TCPC_ALERT: u32 = 0x10;
pub const TCPC_ALERT_EXTND: u32 = 1 << 14;
pub const TCPC_ALERT_EXTENDED_STATUS: u32 = 1 << 13;
pub const TCPC_ALERT_VBUS_DISCNCT: u32 = 1 << 11;
pub const TCPC_ALERT_RX_BUF_OVF: u32 = 1 << 10;
pub const TCPC_ALERT_FAULT: u32 = 1 << 9;
pub const TCPC_ALERT_V_ALARM_LO: u32 = 1 << 8;
pub const TCPC_ALERT_V_ALARM_HI: u32 = 1 << 7;
pub const TCPC_ALERT_TX_SUCCESS: u32 = 1 << 6;
pub const TCPC_ALERT_TX_DISCARDED: u32 = 1 << 5;
pub const TCPC_ALERT_TX_FAILED: u32 = 1 << 4;
pub const TCPC_ALERT_RX_HARD_RST: u32 = 1 << 3;
pub const TCPC_ALERT_RX_STATUS: u32 = 1 << 2;
pub const TCPC_ALERT_POWER_STATUS: u32 = 1 << 1;
pub const TCPC_ALERT_CC_STATUS: u32 = 1 << 0;

pub const TCPC_ALERT_MASK: u32 = 0x12;
pub const TCPC_POWER_STATUS_MASK: u32 = 0x14;
pub const TCPC_FAULT_STATUS_MASK: u32 = 0x15;
pub const TCPC_FAULT_STATUS_MASK_VCONN_OC: u32 = 1 << 1;
pub const TCPC_EXTENDED_STATUS_MASK: u32 = 0x16;
pub const TCPC_EXTENDED_STATUS_MASK_VSAFE0V: u32 = 1 << 0;
pub const TCPC_ALERT_EXTENDED_MASK: u32 = 0x17;
pub const TCPC_SINK_FAST_ROLE_SWAP: u32 = 1 << 0;
pub const TCPC_CONFIG_STD_OUTPUT: u32 = 0x18;
pub const TCPC_CONFIG_STD_OUTPUT_ORIENTATION_MASK: u32 = 1 << 0;
pub const TCPC_CONFIG_STD_OUTPUT_ORIENTATION_NORMAL: u32 = 0;
pub const TCPC_CONFIG_STD_OUTPUT_ORIENTATION_FLIPPED: u32 = 1;
pub const TCPC_TCPC_CTRL: u32 = 0x19;
pub const TCPC_TCPC_CTRL_ORIENTATION: u32 = 1 << 0;
pub const PLUG_ORNT_CC1: u32 = 0;
pub const PLUG_ORNT_CC2: u32 = 1;
pub const TCPC_TCPC_CTRL_BIST_TM: u32 = 1 << 1;
pub const TCPC_TCPC_CTRL_EN_LK4CONN_ALRT: u32 = 1 << 6;
pub const TCPC_EXTENDED_STATUS: u32 = 0x20;
pub const TCPC_EXTENDED_STATUS_VSAFE0V: u32 = 1 << 0;
pub const TCPC_ROLE_CTRL: u32 = 0x1a;
pub const TCPC_ROLE_CTRL_DRP: u32 = 1 << 6;
pub const TCPC_ROLE_CTRL_RP_VAL: u32 = (0x3 << 4);
pub const TCPC_ROLE_CTRL_RP_VAL_DEF: u32 = 0x0;
pub const TCPC_ROLE_CTRL_RP_VAL_1_5: u32 = 0x1;
pub const TCPC_ROLE_CTRL_RP_VAL_3_0: u32 = 0x2;
pub const TCPC_ROLE_CTRL_CC2: u32 = (0x3 << 2);
pub const TCPC_ROLE_CTRL_CC1: u32 = (0x3 << 0);
pub const TCPC_ROLE_CTRL_CC_RA: u32 = 0x0;
pub const TCPC_ROLE_CTRL_CC_RP: u32 = 0x1;
pub const TCPC_ROLE_CTRL_CC_RD: u32 = 0x2;
pub const TCPC_ROLE_CTRL_CC_OPEN: u32 = 0x3;
pub const TCPC_FAULT_CTRL: u32 = 0x1b;
pub const TCPC_POWER_CTRL: u32 = 0x1c;
pub const TCPC_POWER_CTRL_VCONN_ENABLE: u32 = 1 << 0;
pub const TCPC_POWER_CTRL_BLEED_DISCHARGE: u32 = 1 << 3;
pub const TCPC_POWER_CTRL_AUTO_DISCHARGE: u32 = 1 << 4;
pub const TCPC_DIS_VOLT_ALRM: u32 = 1 << 5;
pub const TCPC_POWER_CTRL_VBUS_VOLT_MON: u32 = 1 << 6;
pub const TCPC_FAST_ROLE_SWAP_EN: u32 = 1 << 7;
pub const TCPC_CC_STATUS: u32 = 0x1d;
pub const TCPC_CC_STATUS_TOGGLING: u32 = 1 << 5;
pub const TCPC_CC_STATUS_TERM: u32 = 1 << 4;
pub const TCPC_CC_STATUS_TERM_RP: u32 = 0;
pub const TCPC_CC_STATUS_TERM_RD: u32 = 1;
pub const TCPC_CC_STATUS_CC2: u32 = (0x3 << 2);
pub const TCPC_CC_STATUS_CC1: u32 = (0x3 << 0);
pub const TCPC_CC_STATE_SRC_OPEN: u32 = 0;
pub const TCPC_POWER_STATUS: u32 = 0x1e;
pub const TCPC_POWER_STATUS_DBG_ACC_CON: u32 = 1 << 7;
pub const TCPC_POWER_STATUS_UNINIT: u32 = 1 << 6;
pub const TCPC_POWER_STATUS_SOURCING_VBUS: u32 = 1 << 4;
pub const TCPC_POWER_STATUS_VBUS_DET: u32 = 1 << 3;
pub const TCPC_POWER_STATUS_VBUS_PRES: u32 = 1 << 2;
pub const TCPC_POWER_STATUS_VCONN_PRES: u32 = 1 << 1;
pub const TCPC_POWER_STATUS_SINKING_VBUS: u32 = 1 << 0;
pub const TCPC_FAULT_STATUS: u32 = 0x1f;
pub const TCPC_FAULT_STATUS_ALL_REG_RST_TO_DEFAULT: u32 = 1 << 7;
pub const TCPC_FAULT_STATUS_VCONN_OC: u32 = 1 << 1;
pub const TCPC_ALERT_EXTENDED: u32 = 0x21;
pub const TCPC_COMMAND: u32 = 0x23;
pub const TCPC_CMD_WAKE_I2C: u32 = 0x11;
pub const TCPC_CMD_DISABLE_VBUS_DETECT: u32 = 0x22;
pub const TCPC_CMD_ENABLE_VBUS_DETECT: u32 = 0x33;
pub const TCPC_CMD_DISABLE_SINK_VBUS: u32 = 0x44;
pub const TCPC_CMD_SINK_VBUS: u32 = 0x55;
pub const TCPC_CMD_DISABLE_SRC_VBUS: u32 = 0x66;
pub const TCPC_CMD_SRC_VBUS_DEFAULT: u32 = 0x77;
pub const TCPC_CMD_SRC_VBUS_HIGH: u32 = 0x88;
pub const TCPC_CMD_LOOK4CONNECTION: u32 = 0x99;
pub const TCPC_CMD_RXONEMORE: u32 = 0xAA;
pub const TCPC_CMD_I2C_IDLE: u32 = 0xFF;
pub const TCPC_DEV_CAP_1: u32 = 0x24;
pub const TCPC_DEV_CAP_2: u32 = 0x26;
pub const TCPC_STD_INPUT_CAP: u32 = 0x28;
pub const TCPC_STD_OUTPUT_CAP: u32 = 0x29;
pub const TCPC_STD_OUTPUT_CAP_ORIENTATION: u32 = 1 << 0;
pub const TCPC_MSG_HDR_INFO: u32 = 0x2e;
pub const TCPC_MSG_HDR_INFO_DATA_ROLE: u32 = 1 << 3;
pub const TCPC_MSG_HDR_INFO_REV: u32 = (0x3 << 1);
pub const TCPC_MSG_HDR_INFO_PWR_ROLE: u32 = 1 << 0;
pub const TCPC_RX_DETECT: u32 = 0x2f;
pub const TCPC_RX_DETECT_HARD_RESET: u32 = 1 << 5;
pub const TCPC_RX_DETECT_SOP: u32 = 1 << 0;
pub const TCPC_RX_DETECT_SOP1: u32 = 1 << 1;
pub const TCPC_RX_DETECT_SOP2: u32 = 1 << 2;
pub const TCPC_RX_DETECT_DBG1: u32 = 1 << 3;
pub const TCPC_RX_DETECT_DBG2: u32 = 1 << 4;
pub const TCPC_RX_BYTE_CNT: u32 = 0x30;
pub const TCPC_RX_BUF_FRAME_TYPE: u32 = 0x31;
pub const TCPC_RX_BUF_FRAME_TYPE_SOP: u32 = 0;
pub const TCPC_RX_BUF_FRAME_TYPE_SOP1: u32 = 1;
pub const TCPC_RX_BUF_FRAME_TYPE_MASK: u32 = (0x7 << 0);
pub const TCPC_RX_HDR: u32 = 0x32;
pub const TCPC_RX_DATA: u32 = 0x34;
pub const TCPC_TRANSMIT: u32 = 0x50;
pub const TCPC_TRANSMIT_RETRY: u32 = (0x3 << 4);
pub const TCPC_TRANSMIT_TYPE: u32 = (0x7 << 0);
pub const TCPC_TX_BYTE_CNT: u32 = 0x51;
pub const TCPC_TX_HDR: u32 = 0x52;
pub const TCPC_TX_DATA: u32 = 0x54;
pub const TCPC_VBUS_VOLTAGE: u32 = 0x70;
pub const TCPC_VBUS_VOLTAGE_MASK: u32 = 0x3ff;
pub const TCPC_VBUS_VOLTAGE_LSB_MV: u32 = 25;
pub const TCPC_VBUS_SINK_DISCONNECT_THRESH: u32 = 0x72;
pub const TCPC_VBUS_SINK_DISCONNECT_THRESH_LSB_MV: u32 = 25;
pub const TCPC_VBUS_SINK_DISCONNECT_THRESH_MAX: u32 = 0x3ff;
pub const TCPC_VBUS_STOP_DISCHARGE_THRESH: u32 = 0x74;
pub const TCPC_VBUS_VOLTAGE_ALARM_HI_CFG: u32 = 0x76;
pub const TCPC_VBUS_VOLTAGE_ALARM_LO_CFG: u32 = 0x78;

/* I2C_WRITE_BYTE_COUNT + 1 when TX_BUF_BYTE_x is only accessible I2C_WRITE_BYTE_COUNT */
pub const TCPC_TRANSMIT_BUFFER_MAX_LEN: u32 = 31;

#[inline]
pub const fn tcpc_presenting_rd(reg: u32, cc: u32) -> bool {
    (TCPC_ROLE_CTRL_DRP & reg) == 0 && (((cc & reg) >> cc.trailing_zeros()) == TCPC_ROLE_CTRL_CC_RD)
}

#[repr(C)]
pub struct tcpci;
#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct tcpm_port;

#[repr(C)]
pub struct tcpci_data {
    pub regmap: *mut regmap,
    pub TX_BUF_BYTE_x_hidden: u8,
    pub auto_discharge_disconnect: u8,
    pub vbus_vsafe0v: u8,
    pub cable_comm_capable: u8,
    pub set_orientation: u8,
    pub init: Option<unsafe extern "C" fn(*mut tcpci, *mut tcpci_data) -> i32>,
    pub set_vconn: Option<unsafe extern "C" fn(*mut tcpci, *mut tcpci_data, bool) -> i32>,
    pub start_drp_toggling: Option<unsafe extern "C" fn(*mut tcpci, *mut tcpci_data, typec_cc_status) -> i32>,
    pub set_vbus: Option<unsafe extern "C" fn(*mut tcpci, *mut tcpci_data, bool, bool) -> i32>,
    pub frs_sourcing_vbus: Option<unsafe extern "C" fn(*mut tcpci, *mut tcpci_data)>,
    pub set_partner_usb_comm_capable: Option<unsafe extern "C" fn(*mut tcpci, *mut tcpci_data, bool)>,
    pub check_contaminant: Option<unsafe extern "C" fn(*mut tcpci, *mut tcpci_data)>,
    pub attempt_vconn_swap_discovery: Option<unsafe extern "C" fn(*mut tcpci, *mut tcpci_data) -> bool>,
}

pub type irqreturn_t = i32;
pub type typec_cc_status = i32;

extern "C" {
    pub fn tcpci_register_port(dev: *mut device, data: *mut tcpci_data) -> *mut tcpci;
    pub fn tcpci_unregister_port(tcpci: *mut tcpci);
    pub fn tcpci_irq(tcpci: *mut tcpci) -> irqreturn_t;
    pub fn tcpci_get_tcpm_port(tcpci: *mut tcpci) -> *mut tcpm_port;
}

#[inline]
pub unsafe fn tcpci_to_typec_cc(cc: u32, sink: bool) -> typec_cc_status {
    match cc {
        0x1 => if sink { TYPEC_CC_RP_DEF } else { TYPEC_CC_RA },
        0x2 => if sink { TYPEC_CC_RP_1_5 } else { TYPEC_CC_RD },
        0x3 => {
            if sink { TYPEC_CC_RP_3_0 } else { TYPEC_CC_OPEN }
        }
        TCPC_CC_STATE_SRC_OPEN => TYPEC_CC_OPEN,
        _ => TYPEC_CC_OPEN,
    }
}

// External constants supplied by linux/usb/typec.h.
extern "C" {
    static TYPEC_CC_RP_DEF: typec_cc_status;
    static TYPEC_CC_RA: typec_cc_status;
    static TYPEC_CC_RP_1_5: typec_cc_status;
    static TYPEC_CC_RD: typec_cc_status;
    static TYPEC_CC_RP_3_0: typec_cc_status;
    static TYPEC_CC_OPEN: typec_cc_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
