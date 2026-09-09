/* SPDX-License-Identifier: ((GPL-2.0+ WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * Copyright (C) 2022-2024 OpenSynergy GmbH
 * Copyright (c) 2024 Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Translated from virtio_rtc.h. Linux __le* types are represented by their
// corresponding-width integer types; byte order is determined by the ABI.

pub const VIRTIO_RTC_F_ALARM: u32 = 0;

pub const VIRTIO_RTC_REQ_READ: u32 = 0x0001;
pub const VIRTIO_RTC_REQ_READ_CROSS: u32 = 0x0002;

pub const VIRTIO_RTC_REQ_CFG: u32 = 0x1000;
pub const VIRTIO_RTC_REQ_CLOCK_CAP: u32 = 0x1001;
pub const VIRTIO_RTC_REQ_CROSS_CAP: u32 = 0x1002;
pub const VIRTIO_RTC_REQ_READ_ALARM: u32 = 0x1003;
pub const VIRTIO_RTC_REQ_SET_ALARM: u32 = 0x1004;
pub const VIRTIO_RTC_REQ_SET_ALARM_ENABLED: u32 = 0x1005;

pub const VIRTIO_RTC_NOTIF_ALARM: u32 = 0x2000;

pub const VIRTIO_RTC_S_OK: u8 = 0;
pub const VIRTIO_RTC_S_EOPNOTSUPP: u8 = 2;
pub const VIRTIO_RTC_S_ENODEV: u8 = 3;
pub const VIRTIO_RTC_S_EINVAL: u8 = 4;
pub const VIRTIO_RTC_S_EIO: u8 = 5;

pub const VIRTIO_RTC_COUNTER_ARM_VCT: u8 = 0;
pub const VIRTIO_RTC_COUNTER_X86_TSC: u8 = 1;
pub const VIRTIO_RTC_COUNTER_INVALID: u8 = 0xFF;

pub const VIRTIO_RTC_CLOCK_UTC: u8 = 0;
pub const VIRTIO_RTC_CLOCK_TAI: u8 = 1;
pub const VIRTIO_RTC_CLOCK_MONOTONIC: u8 = 2;
pub const VIRTIO_RTC_CLOCK_UTC_SMEARED: u8 = 3;
pub const VIRTIO_RTC_CLOCK_UTC_MAYBE_SMEARED: u8 = 4;
pub const VIRTIO_RTC_SMEAR_UNSPECIFIED: u8 = 0;
pub const VIRTIO_RTC_SMEAR_NOON_LINEAR: u8 = 1;
pub const VIRTIO_RTC_SMEAR_UTC_SLS: u8 = 2;
pub const VIRTIO_RTC_FLAG_ALARM_CAP: u8 = 1 << 0;
pub const VIRTIO_RTC_FLAG_CROSS_CAP: u8 = 1 << 0;
pub const VIRTIO_RTC_FLAG_ALARM_ENABLED: u8 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_head { pub msg_type: u16, pub reserved: [u8; 6] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_head { pub status: u8, pub reserved: [u8; 7] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_notif_head { pub msg_type: u16, pub reserved: [u8; 6] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_read { pub head: virtio_rtc_req_head, pub clock_id: u16, pub reserved: [u8; 6] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_read { pub head: virtio_rtc_resp_head, pub clock_reading: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_read_cross { pub head: virtio_rtc_req_head, pub clock_id: u16, pub hw_counter: u8, pub reserved: [u8; 5] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_read_cross { pub head: virtio_rtc_resp_head, pub clock_reading: u64, pub counter_cycles: u64 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_cfg { pub head: virtio_rtc_req_head }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_cfg { pub head: virtio_rtc_resp_head, pub num_clocks: u16, pub reserved: [u8; 6] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_clock_cap { pub head: virtio_rtc_req_head, pub clock_id: u16, pub reserved: [u8; 6] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_clock_cap { pub head: virtio_rtc_resp_head, pub type_: u8, pub leap_second_smearing: u8, pub flags: u8, pub reserved: [u8; 5] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_cross_cap { pub head: virtio_rtc_req_head, pub clock_id: u16, pub hw_counter: u8, pub reserved: [u8; 5] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_cross_cap { pub head: virtio_rtc_resp_head, pub flags: u8, pub reserved: [u8; 7] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_read_alarm { pub head: virtio_rtc_req_head, pub clock_id: u16, pub reserved: [u8; 6] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_read_alarm { pub head: virtio_rtc_resp_head, pub alarm_time: u64, pub flags: u8, pub reserved: [u8; 7] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_set_alarm { pub head: virtio_rtc_req_head, pub alarm_time: u64, pub clock_id: u16, pub flags: u8, pub reserved: [u8; 5] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_set_alarm { pub head: virtio_rtc_resp_head }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_req_set_alarm_enabled { pub head: virtio_rtc_req_head, pub clock_id: u16, pub flags: u8, pub reserved: [u8; 5] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_resp_set_alarm_enabled { pub head: virtio_rtc_resp_head }

#[repr(C)]
pub union virtio_rtc_req_requestq {
    pub read: virtio_rtc_req_read,
    pub read_cross: virtio_rtc_req_read_cross,
    pub cfg: virtio_rtc_req_cfg,
    pub clock_cap: virtio_rtc_req_clock_cap,
    pub cross_cap: virtio_rtc_req_cross_cap,
    pub read_alarm: virtio_rtc_req_read_alarm,
    pub set_alarm: virtio_rtc_req_set_alarm,
    pub set_alarm_enabled: virtio_rtc_req_set_alarm_enabled,
}
#[repr(C)]
pub union virtio_rtc_resp_requestq {
    pub read: virtio_rtc_resp_read,
    pub read_cross: virtio_rtc_resp_read_cross,
    pub cfg: virtio_rtc_resp_cfg,
    pub clock_cap: virtio_rtc_resp_clock_cap,
    pub cross_cap: virtio_rtc_resp_cross_cap,
    pub read_alarm: virtio_rtc_resp_read_alarm,
    pub set_alarm: virtio_rtc_resp_set_alarm,
    pub set_alarm_enabled: virtio_rtc_resp_set_alarm_enabled,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_rtc_notif_alarm { pub head: virtio_rtc_notif_head, pub clock_id: u16, pub reserved: [u8; 6] }
#[repr(C)]
pub union virtio_rtc_notif_alarmq { pub alarm: virtio_rtc_notif_alarm }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
