/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * linux/can/netlink.h
 *
 * Definitions for the CAN netlink interface
 *
 * Copyright (c) 2009 Wolfgang Grandegger <wg@grandegger.com>
 * Copyright (c) 2021-2025 Vincent Mailhol <mailhol@kernel.org>
 */

/* Dependency equivalent of <linux/types.h>. */

/*
 * CAN bit-timing parameters
 *
 * For further information, please read chapter "8 BIT TIMING
 * REQUIREMENTS" of the "Bosch CAN Specification version 2.0"
 * at http://www.semiconductors.bosch.de/pdf/can2spec.pdf.
 */
#[repr(C)]
pub struct can_bittiming {
    pub bitrate: u32,       /* Bit-rate in bits/second */
    pub sample_point: u32,  /* Sample point in one-tenth of a percent */
    pub tq: u32,            /* Time quanta (TQ) in nanoseconds */
    pub prop_seg: u32,      /* Propagation segment in TQs */
    pub phase_seg1: u32,    /* Phase buffer segment 1 in TQs */
    pub phase_seg2: u32,    /* Phase buffer segment 2 in TQs */
    pub sjw: u32,            /* Synchronisation jump width in TQs */
    pub brp: u32,            /* Bit-rate prescaler */
}

/* CAN hardware-dependent bit-timing constant. */
#[repr(C)]
pub struct can_bittiming_const {
    pub name: [core::ffi::c_char; 16],
    pub tseg1_min: u32,
    pub tseg1_max: u32,
    pub tseg2_min: u32,
    pub tseg2_max: u32,
    pub sjw_max: u32,
    pub brp_min: u32,
    pub brp_max: u32,
    pub brp_inc: u32,
}

/* CAN clock parameters */
#[repr(C)]
pub struct can_clock {
    pub freq: u32,
}

/* CAN operational and error states */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum can_state {
    CAN_STATE_ERROR_ACTIVE = 0,
    CAN_STATE_ERROR_WARNING,
    CAN_STATE_ERROR_PASSIVE,
    CAN_STATE_BUS_OFF,
    CAN_STATE_STOPPED,
    CAN_STATE_SLEEPING,
    CAN_STATE_MAX,
}

/* CAN bus error counters */
#[repr(C)]
pub struct can_berr_counter {
    pub txerr: u16,
    pub rxerr: u16,
}

/* CAN controller mode */
#[repr(C)]
pub struct can_ctrlmode {
    pub mask: u32,
    pub flags: u32,
}

pub const CAN_CTRLMODE_LOOPBACK: u32 = 0x01;
pub const CAN_CTRLMODE_LISTENONLY: u32 = 0x02;
pub const CAN_CTRLMODE_3_SAMPLES: u32 = 0x04;
pub const CAN_CTRLMODE_ONE_SHOT: u32 = 0x08;
pub const CAN_CTRLMODE_BERR_REPORTING: u32 = 0x10;
pub const CAN_CTRLMODE_FD: u32 = 0x20;
pub const CAN_CTRLMODE_PRESUME_ACK: u32 = 0x40;
pub const CAN_CTRLMODE_FD_NON_ISO: u32 = 0x80;
pub const CAN_CTRLMODE_CC_LEN8_DLC: u32 = 0x100;
pub const CAN_CTRLMODE_TDC_AUTO: u32 = 0x200;
pub const CAN_CTRLMODE_TDC_MANUAL: u32 = 0x400;
pub const CAN_CTRLMODE_RESTRICTED: u32 = 0x800;
pub const CAN_CTRLMODE_XL: u32 = 0x1000;
pub const CAN_CTRLMODE_XL_TDC_AUTO: u32 = 0x2000;
pub const CAN_CTRLMODE_XL_TDC_MANUAL: u32 = 0x4000;
pub const CAN_CTRLMODE_XL_TMS: u32 = 0x8000;

/* CAN device statistics */
#[repr(C)]
pub struct can_device_stats {
    pub bus_error: u32,
    pub error_warning: u32,
    pub error_passive: u32,
    pub bus_off: u32,
    pub arbitration_lost: u32,
    pub restarts: u32,
}

/* CAN netlink interface */
pub const IFLA_CAN_UNSPEC: u32 = 0;
pub const IFLA_CAN_BITTIMING: u32 = 1;
pub const IFLA_CAN_BITTIMING_CONST: u32 = 2;
pub const IFLA_CAN_CLOCK: u32 = 3;
pub const IFLA_CAN_STATE: u32 = 4;
pub const IFLA_CAN_CTRLMODE: u32 = 5;
pub const IFLA_CAN_RESTART_MS: u32 = 6;
pub const IFLA_CAN_RESTART: u32 = 7;
pub const IFLA_CAN_BERR_COUNTER: u32 = 8;
pub const IFLA_CAN_DATA_BITTIMING: u32 = 9;
pub const IFLA_CAN_DATA_BITTIMING_CONST: u32 = 10;
pub const IFLA_CAN_TERMINATION: u32 = 11;
pub const IFLA_CAN_TERMINATION_CONST: u32 = 12;
pub const IFLA_CAN_BITRATE_CONST: u32 = 13;
pub const IFLA_CAN_DATA_BITRATE_CONST: u32 = 14;
pub const IFLA_CAN_BITRATE_MAX: u32 = 15;
pub const IFLA_CAN_TDC: u32 = 16;
pub const IFLA_CAN_CTRLMODE_EXT: u32 = 17;
pub const IFLA_CAN_XL_DATA_BITTIMING: u32 = 18;
pub const IFLA_CAN_XL_DATA_BITTIMING_CONST: u32 = 19;
pub const IFLA_CAN_XL_DATA_BITRATE_CONST: u32 = 20;
pub const IFLA_CAN_XL_TDC: u32 = 21;
pub const IFLA_CAN_XL_PWM: u32 = 22;
pub const __IFLA_CAN_MAX: u32 = 23;
pub const IFLA_CAN_MAX: u32 = __IFLA_CAN_MAX - 1;

/* CAN FD/XL Transmitter Delay Compensation (TDC) */
pub const IFLA_CAN_TDC_UNSPEC: u32 = 0;
pub const IFLA_CAN_TDC_TDCV_MIN: u32 = 1;
pub const IFLA_CAN_TDC_TDCV_MAX: u32 = 2;
pub const IFLA_CAN_TDC_TDCO_MIN: u32 = 3;
pub const IFLA_CAN_TDC_TDCO_MAX: u32 = 4;
pub const IFLA_CAN_TDC_TDCF_MIN: u32 = 5;
pub const IFLA_CAN_TDC_TDCF_MAX: u32 = 6;
pub const IFLA_CAN_TDC_TDCV: u32 = 7;
pub const IFLA_CAN_TDC_TDCO: u32 = 8;
pub const IFLA_CAN_TDC_TDCF: u32 = 9;
pub const __IFLA_CAN_TDC: u32 = 10;
pub const IFLA_CAN_TDC_MAX: u32 = __IFLA_CAN_TDC - 1;

/* IFLA_CAN_CTRLMODE_EXT nest: controller mode extended parameters */
pub const IFLA_CAN_CTRLMODE_UNSPEC: u32 = 0;
pub const IFLA_CAN_CTRLMODE_SUPPORTED: u32 = 1;
pub const __IFLA_CAN_CTRLMODE: u32 = 2;
pub const IFLA_CAN_CTRLMODE_MAX: u32 = __IFLA_CAN_CTRLMODE - 1;

/* CAN FD/XL Pulse-Width Modulation (PWM) */
pub const IFLA_CAN_PWM_UNSPEC: u32 = 0;
pub const IFLA_CAN_PWM_PWMS_MIN: u32 = 1;
pub const IFLA_CAN_PWM_PWMS_MAX: u32 = 2;
pub const IFLA_CAN_PWM_PWML_MIN: u32 = 3;
pub const IFLA_CAN_PWM_PWML_MAX: u32 = 4;
pub const IFLA_CAN_PWM_PWMO_MIN: u32 = 5;
pub const IFLA_CAN_PWM_PWMO_MAX: u32 = 6;
pub const IFLA_CAN_PWM_PWMS: u32 = 7;
pub const IFLA_CAN_PWM_PWML: u32 = 8;
pub const IFLA_CAN_PWM_PWMO: u32 = 9;
pub const __IFLA_CAN_PWM: u32 = 10;
pub const IFLA_CAN_PWM_MAX: u32 = __IFLA_CAN_PWM - 1;

/* u16 termination range: 1..65535 Ohms */
pub const CAN_TERMINATION_DISABLED: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
