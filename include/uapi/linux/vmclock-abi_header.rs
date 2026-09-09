/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-2-Clause) */

/*
 * Rust translation of vmclock-abi.h. The integer fields retain their C
 * widths; little-endian interpretation is part of the ABI.
 */

#[repr(C)]
pub struct vmclock_abi {
    /* CONSTANT FIELDS */
    pub magic: u32,
    pub size: u32,                    /* Size of region containing this structure */
    pub version: u16,                 /* 1 */
    pub counter_id: u8,               /* Matches VIRTIO_RTC_COUNTER_xxx except INVALID */
    pub time_type: u8,                /* Matches VIRTIO_RTC_TYPE_xxx */

    /* NON-CONSTANT FIELDS PROTECTED BY SEQCOUNT LOCK */
    pub seq_count: u32,               /* Low bit means an update is in progress */
    /*
     * This field changes to another non-repeating value when the CPU
     * counter is disrupted, for example on live migration. This lets the
     * guest know that it should discard any calibration it has performed
     * of the counter against external sources (NTP/PTP/etc.).
     */
    pub disruption_marker: u64,
    pub flags: u64,
    pub pad: [u8; 2],
    pub clock_status: u8,
    pub leap_second_smearing_hint: u8, /* Matches VIRTIO_RTC_SUBTYPE_xxx */
    pub tai_offset_sec: u16,           /* Actually two's complement signed */
    pub leap_indicator: u8,
    /* Bit shift for counter_period_frac_sec and its error rate */
    pub counter_period_shift: u8,
    /* Paired values of counter and UTC at a given point in time. */
    pub counter_value: u64,
    /*
     * Counter period, and error margin of same. The unit of these
     * fields is 1/2^(64 + counter_period_shift) of a second.
     */
    pub counter_period_frac_sec: u64,
    pub counter_period_esterror_rate_frac_sec: u64,
    pub counter_period_maxerror_rate_frac_sec: u64,
    /* Time according to time_type field above. */
    pub time_sec: u64,                /* Seconds since time_type epoch */
    pub time_frac_sec: u64,           /* Units of 1/2^64 of a second */
    pub time_esterror_nanosec: u64,
    pub time_maxerror_nanosec: u64,
    /*
     * This field changes to another non-repeating value when the guest
     * has been loaded from a snapshot. In addition to handling a
     * disruption in time (which will also be signalled through the
     * disruption_marker field), a guest may wish to discard UUIDs,
     * reset network connections, reseed entropy, etc.
     */
    pub vm_generation_counter: u64,
}

pub const VMCLOCK_MAGIC: u32 = 0x4b4c4356; /* "VCLK" */
pub const VMCLOCK_COUNTER_ARM_VCNT: u8 = 0;
pub const VMCLOCK_COUNTER_X86_TSC: u8 = 1;
pub const VMCLOCK_COUNTER_INVALID: u8 = 0xff;
pub const VMCLOCK_TIME_UTC: u8 = 0; /* Since 1970-01-01 00:00:00z */
pub const VMCLOCK_TIME_TAI: u8 = 1; /* Since 1970-01-01 00:00:00z */
pub const VMCLOCK_TIME_MONOTONIC: u8 = 2; /* Since undefined epoch */
pub const VMCLOCK_TIME_INVALID_SMEARED: u8 = 3; /* Not supported */
pub const VMCLOCK_TIME_INVALID_MAYBE_SMEARED: u8 = 4; /* Not supported */

pub const VMCLOCK_FLAG_TAI_OFFSET_VALID: u64 = 1 << 0;
pub const VMCLOCK_FLAG_DISRUPTION_SOON: u64 = 1 << 1; /* About a day */
pub const VMCLOCK_FLAG_DISRUPTION_IMMINENT: u64 = 1 << 2; /* About an hour */
pub const VMCLOCK_FLAG_PERIOD_ESTERROR_VALID: u64 = 1 << 3;
pub const VMCLOCK_FLAG_PERIOD_MAXERROR_VALID: u64 = 1 << 4;
pub const VMCLOCK_FLAG_TIME_ESTERROR_VALID: u64 = 1 << 5;
pub const VMCLOCK_FLAG_TIME_MAXERROR_VALID: u64 = 1 << 6;
pub const VMCLOCK_FLAG_TIME_MONOTONIC: u64 = 1 << 7;
pub const VMCLOCK_FLAG_VM_GEN_COUNTER_PRESENT: u64 = 1 << 8;
pub const VMCLOCK_FLAG_NOTIFICATION_PRESENT: u64 = 1 << 9;

pub const VMCLOCK_STATUS_UNKNOWN: u8 = 0;
pub const VMCLOCK_STATUS_INITIALIZING: u8 = 1;
pub const VMCLOCK_STATUS_SYNCHRONIZED: u8 = 2;
pub const VMCLOCK_STATUS_FREERUNNING: u8 = 3;
pub const VMCLOCK_STATUS_UNRELIABLE: u8 = 4;

pub const VMCLOCK_SMEARING_STRICT: u8 = 0;
pub const VMCLOCK_SMEARING_NOON_LINEAR: u8 = 1;
pub const VMCLOCK_SMEARING_UTC_SLS: u8 = 2;

pub const VMCLOCK_LEAP_NONE: u8 = 0x00; /* No known nearby leap second */
pub const VMCLOCK_LEAP_PRE_POS: u8 = 0x01; /* Positive leap second at EOM */
pub const VMCLOCK_LEAP_PRE_NEG: u8 = 0x02; /* Negative leap second at EOM */
pub const VMCLOCK_LEAP_POS: u8 = 0x03; /* Set during 23:59:60 second */
pub const VMCLOCK_LEAP_POST_POS: u8 = 0x04;
pub const VMCLOCK_LEAP_POST_NEG: u8 = 0x05;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
