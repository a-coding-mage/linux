// SPDX-License-Identifier: GPL-2.0+
/*
 * USB 338x super/high/full speed USB device controller.
 * Unlike many such controllers, this one talks PCI.
 *
 * Copyright (C) 2002 NetChip Technology, Inc. (http://www.netchip.com)
 * Copyright (C) 2003 David Brownell
 * Copyright (C) 2014 Ricardo Ribalda - Qtechnology/AS
 */

// Dependency supplied by linux/usb/net2280.h.

pub const SCRATCH: u32 = 0x0b;
pub const DEFECT7374_FSM_FIELD: u32 = 28;
pub const SUPER_SPEED: u32 = 8;
pub const DMA_REQUEST_OUTSTANDING: u32 = 5;
pub const DMA_PAUSE_DONE_INTERRUPT: u32 = 26;
pub const SET_ISOCHRONOUS_DELAY: u32 = 24;
pub const SET_SEL: u32 = 22;
pub const SUPER_SPEED_MODE: u32 = 8;

pub const MAX_BURST_SIZE: u32 = 24;
pub const EP_FIFO_BYTE_COUNT: u32 = 16;
pub const IN_ENDPOINT_ENABLE: u32 = 14;
pub const IN_ENDPOINT_TYPE: u32 = 12;
pub const OUT_ENDPOINT_ENABLE: u32 = 10;
pub const OUT_ENDPOINT_TYPE: u32 = 8;
pub const USB3380_EP_CFG_MASK_IN: u32 = (0x3 << IN_ENDPOINT_TYPE) | (1 << IN_ENDPOINT_ENABLE);
pub const USB3380_EP_CFG_MASK_OUT: u32 = (0x3 << OUT_ENDPOINT_TYPE) | (1 << OUT_ENDPOINT_ENABLE);

#[repr(C, packed)]
pub struct usb338x_usb_ext_regs {
    pub usbclass: u32,
    pub ss_sel: u32,
    pub ss_del: u32,
    pub usb2lpm: u32,
    pub usb3belt: u32,
    pub usbctl2: u32,
    pub in_timeout: u32,
    pub isodelay: u32,
}

pub const DEVICE_PROTOCOL: u32 = 16;
pub const DEVICE_SUB_CLASS: u32 = 8;
pub const DEVICE_CLASS: u32 = 0;
pub const U2_SYSTEM_EXIT_LATENCY: u32 = 8;
pub const U1_SYSTEM_EXIT_LATENCY: u32 = 0;
pub const U2_DEVICE_EXIT_LATENCY: u32 = 8;
pub const U1_DEVICE_EXIT_LATENCY: u32 = 0;
pub const USB_L1_LPM_HIRD: u32 = 2;
pub const USB_L1_LPM_REMOTE_WAKE: u32 = 1;
pub const USB_L1_LPM_SUPPORT: u32 = 0;
pub const BELT_MULTIPLIER: u32 = 10;
pub const BEST_EFFORT_LATENCY_TOLERANCE: u32 = 0;
pub const LTM_ENABLE: u32 = 7;
pub const U2_ENABLE: u32 = 6;
pub const U1_ENABLE: u32 = 5;
pub const FUNCTION_SUSPEND: u32 = 4;
pub const USB3_CORE_ENABLE: u32 = 3;
pub const USB2_CORE_ENABLE: u32 = 2;
pub const SERIAL_NUMBER_STRING_ENABLE: u32 = 0;
pub const GPEP3_TIMEOUT: u32 = 19;
pub const GPEP2_TIMEOUT: u32 = 18;
pub const GPEP1_TIMEOUT: u32 = 17;
pub const GPEP0_TIMEOUT: u32 = 16;
pub const GPEP3_TIMEOUT_VALUE: u32 = 13;
pub const GPEP3_TIMEOUT_ENABLE: u32 = 12;
pub const GPEP2_TIMEOUT_VALUE: u32 = 9;
pub const GPEP2_TIMEOUT_ENABLE: u32 = 8;
pub const GPEP1_TIMEOUT_VALUE: u32 = 5;
pub const GPEP1_TIMEOUT_ENABLE: u32 = 4;
pub const GPEP0_TIMEOUT_VALUE: u32 = 1;
pub const GPEP0_TIMEOUT_ENABLE: u32 = 0;
pub const ISOCHRONOUS_DELAY: u32 = 0;

#[repr(C, packed)]
pub struct usb338x_fifo_regs {
    pub ep_fifo_size_base: u32,
    pub ep_fifo_out_wrptr: u32,
    pub ep_fifo_out_rdptr: u32,
    pub ep_fifo_in_wrptr: u32,
    pub ep_fifo_in_rdptr: u32,
    pub unused: [u32; 3],
}

pub const IN_FIFO_BASE_ADDRESS: u32 = 22;
pub const IN_FIFO_SIZE: u32 = 16;
pub const OUT_FIFO_BASE_ADDRESS: u32 = 6;
pub const OUT_FIFO_SIZE: u32 = 0;

/* Link layer */
#[repr(C, packed)]
pub struct usb338x_ll_regs {
    pub ll_ltssm_ctrl1: u32,
    pub ll_ltssm_ctrl2: u32,
    pub ll_ltssm_ctrl3: u32,
    pub unused1: u32,
    pub unused2: u32,
    pub ll_general_ctrl0: u32,
    pub ll_general_ctrl1: u32,
    pub ll_general_ctrl2: u32,
    pub ll_general_ctrl3: u32,
    pub ll_general_ctrl4: u32,
    pub ll_error_gen: u32,
    pub unused3: u32,
    pub unused4: [u32; 4],
    pub unused5: [u32; 2],
    pub ll_lfps_5: u32,
    pub ll_lfps_6: u32,
    pub unused6: [u32; 8],
    pub unused7: [u32; 3],
    pub ll_tsn_counters_2: u32,
    pub ll_tsn_counters_3: u32,
    pub unused8: [u32; 3],
    pub unused9: u32,
    pub ll_lfps_timers_2: u32,
    pub unused10: u32,
    pub ll_tsn_chicken_bit: u32,
}

pub const PM_U3_AUTO_EXIT: u32 = 29;
pub const PM_U2_AUTO_EXIT: u32 = 28;
pub const PM_U1_AUTO_EXIT: u32 = 27;
pub const PM_FORCE_U2_ENTRY: u32 = 26;
pub const PM_FORCE_U1_ENTRY: u32 = 25;
pub const PM_LGO_COLLISION_SEND_LAU: u32 = 24;
pub const PM_DIR_LINK_REJECT: u32 = 23;
pub const PM_FORCE_LINK_ACCEPT: u32 = 22;
pub const PM_DIR_ENTRY_U3: u32 = 20;
pub const PM_DIR_ENTRY_U2: u32 = 19;
pub const PM_DIR_ENTRY_U1: u32 = 18;
pub const PM_U2_ENABLE: u32 = 17;
pub const PM_U1_ENABLE: u32 = 16;
pub const SKP_THRESHOLD_ADJUST_FMW: u32 = 8;
pub const RESEND_DPP_ON_LRTY_FMW: u32 = 7;
pub const DL_BIT_VALUE_FMW: u32 = 6;
pub const FORCE_DL_BIT: u32 = 5;
pub const SELECT_INVERT_LANE_POLARITY: u32 = 7;
pub const FORCE_INVERT_LANE_POLARITY: u32 = 6;
pub const TIMER_LFPS_6US: u32 = 16;
pub const TIMER_LFPS_80US: u32 = 0;
pub const HOT_TX_NORESET_TS2: u32 = 24;
pub const HOT_RX_RESET_TS2: u32 = 0;
pub const LFPS_TIMERS_2_WORKAROUND_VALUE: u32 = 0x084d;
pub const RECOVERY_IDLE_TO_RECOVER_FMW: u32 = 3;

/* protocol layer */
#[repr(C, packed)]
pub struct usb338x_pl_regs {
    pub pl_reg_1: u32,
    pub pl_reg_2: u32,
    pub pl_reg_3: u32,
    pub pl_reg_4: u32,
    pub pl_ep_ctrl: u32,
    pub pl_reg_6: u32,
    pub pl_reg_7: u32,
    pub pl_reg_8: u32,
    pub pl_ep_status_1: u32,
    pub pl_ep_status_2: u32,
    pub pl_ep_status_3: u32,
    pub pl_ep_status_4: u32,
    pub pl_ep_cfg_4: u32,
}

pub const PL_EP_CTRL: u32 = 0x810;
pub const ENDPOINT_SELECT: u32 = 0;
pub const EP_INITIALIZED: u32 = 16;
pub const SEQUENCE_NUMBER_RESET: u32 = 17;
pub const CLEAR_ACK_ERROR_CODE: u32 = 20;
pub const PL_EP_STATUS_1: u32 = 0x820;
pub const STATE: u32 = 16;
pub const ACK_GOOD_NORMAL: u32 = 0x11;
pub const ACK_GOOD_MORE_ACKS_TO_COME: u32 = 0x16;
pub const PL_EP_STATUS_3: u32 = 0x828;
pub const SEQUENCE_NUMBER: u32 = 0;
pub const PL_EP_STATUS_4: u32 = 0x82c;
pub const PL_EP_CFG_4: u32 = 0x830;
pub const NON_CTRL_IN_TOLERATE_BAD_DIR: u32 = 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
