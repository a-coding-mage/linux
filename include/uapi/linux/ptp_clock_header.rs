/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* PTP 1588 clock support - user space interface. */

/* Dependencies supplied by the surrounding translated headers: ioctl macros,
 * __kernel_clockid_t, and the fixed-width integer aliases. */

pub const PTP_ENABLE_FEATURE: u32 = 1 << 0;
pub const PTP_RISING_EDGE: u32 = 1 << 1;
pub const PTP_FALLING_EDGE: u32 = 1 << 2;
pub const PTP_STRICT_FLAGS: u32 = 1 << 3;
pub const PTP_EXT_OFFSET: u32 = 1 << 4;
pub const PTP_EXTTS_EDGES: u32 = PTP_RISING_EDGE | PTP_FALLING_EDGE;
pub const PTP_EXTTS_VALID_FLAGS: u32 = PTP_ENABLE_FEATURE | PTP_RISING_EDGE | PTP_FALLING_EDGE | PTP_STRICT_FLAGS | PTP_EXT_OFFSET;
pub const PTP_EXTTS_V1_VALID_FLAGS: u32 = PTP_ENABLE_FEATURE | PTP_RISING_EDGE | PTP_FALLING_EDGE;
pub const PTP_EXTTS_EVENT_VALID: u32 = PTP_ENABLE_FEATURE;

pub const PTP_PEROUT_ONE_SHOT: u32 = 1 << 0;
pub const PTP_PEROUT_DUTY_CYCLE: u32 = 1 << 1;
pub const PTP_PEROUT_PHASE: u32 = 1 << 2;
pub const PTP_PEROUT_VALID_FLAGS: u32 = PTP_PEROUT_ONE_SHOT | PTP_PEROUT_DUTY_CYCLE | PTP_PEROUT_PHASE;
pub const PTP_PEROUT_V1_VALID_FLAGS: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_clock_time { pub sec: i64, pub nsec: u32, pub reserved: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_clock_caps {
    pub max_adj: i32, pub n_alarm: i32, pub n_ext_ts: i32, pub n_per_out: i32,
    pub pps: i32, pub n_pins: i32, pub cross_timestamping: i32, pub adjust_phase: i32,
    pub max_phase_adj: i32, pub rsv: [i32; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_extts_request { pub index: u32, pub flags: u32, pub rsv: [u32; 2] }

#[repr(C)]
pub union ptp_perout_request_start_phase { pub start: ptp_clock_time, pub phase: ptp_clock_time }
#[repr(C)]
pub union ptp_perout_request_on_rsv { pub on: ptp_clock_time, pub rsv: [u32; 4] }
#[repr(C)]
pub struct ptp_perout_request {
    pub start_phase: ptp_perout_request_start_phase,
    pub period: ptp_clock_time,
    pub index: u32,
    pub flags: u32,
    pub on_rsv: ptp_perout_request_on_rsv,
}

pub const PTP_MAX_SAMPLES: usize = 25;
#[repr(C)]
pub struct ptp_sys_offset { pub n_samples: u32, pub rsv: [u32; 3], pub ts: [ptp_clock_time; 2 * PTP_MAX_SAMPLES + 1] }
#[repr(C)]
pub struct ptp_sys_offset_extended { pub n_samples: u32, pub clockid: __kernel_clockid_t, pub rsv: [u32; 2], pub ts: [[ptp_clock_time; 3]; PTP_MAX_SAMPLES] }
#[repr(C)]
pub struct ptp_sys_offset_precise { pub device: ptp_clock_time, pub sys_realtime: ptp_clock_time, pub sys_monoraw: ptp_clock_time, pub rsv: [u32; 4] }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ptp_pin_function { PTP_PF_NONE, PTP_PF_EXTTS, PTP_PF_PEROUT, PTP_PF_PHYSYNC }
#[repr(C)]
pub struct ptp_pin_desc { pub name: [i8; 64], pub index: u32, pub func: u32, pub chan: u32, pub rsv: [u32; 5] }

pub const PTP_CLK_MAGIC: u8 = b'=';
pub const PTP_CLOCK_GETCAPS: _ = _IOR(PTP_CLK_MAGIC, 1, ptp_clock_caps);
pub const PTP_EXTTS_REQUEST: _ = _IOW(PTP_CLK_MAGIC, 2, ptp_extts_request);
pub const PTP_PEROUT_REQUEST: _ = _IOW(PTP_CLK_MAGIC, 3, ptp_perout_request);
pub const PTP_ENABLE_PPS: _ = _IOW(PTP_CLK_MAGIC, 4, i32);
pub const PTP_SYS_OFFSET: _ = _IOW(PTP_CLK_MAGIC, 5, ptp_sys_offset);
pub const PTP_PIN_GETFUNC: _ = _IOWR(PTP_CLK_MAGIC, 6, ptp_pin_desc);
pub const PTP_PIN_SETFUNC: _ = _IOW(PTP_CLK_MAGIC, 7, ptp_pin_desc);
pub const PTP_SYS_OFFSET_PRECISE: _ = _IOWR(PTP_CLK_MAGIC, 8, ptp_sys_offset_precise);
pub const PTP_SYS_OFFSET_EXTENDED: _ = _IOWR(PTP_CLK_MAGIC, 9, ptp_sys_offset_extended);
pub const PTP_CLOCK_GETCAPS2: _ = _IOR(PTP_CLK_MAGIC, 10, ptp_clock_caps);
pub const PTP_EXTTS_REQUEST2: _ = _IOW(PTP_CLK_MAGIC, 11, ptp_extts_request);
pub const PTP_PEROUT_REQUEST2: _ = _IOW(PTP_CLK_MAGIC, 12, ptp_perout_request);
pub const PTP_ENABLE_PPS2: _ = _IOW(PTP_CLK_MAGIC, 13, i32);
pub const PTP_SYS_OFFSET2: _ = _IOW(PTP_CLK_MAGIC, 14, ptp_sys_offset);
pub const PTP_PIN_GETFUNC2: _ = _IOWR(PTP_CLK_MAGIC, 15, ptp_pin_desc);
pub const PTP_PIN_SETFUNC2: _ = _IOW(PTP_CLK_MAGIC, 16, ptp_pin_desc);
pub const PTP_SYS_OFFSET_PRECISE2: _ = _IOWR(PTP_CLK_MAGIC, 17, ptp_sys_offset_precise);
pub const PTP_SYS_OFFSET_EXTENDED2: _ = _IOWR(PTP_CLK_MAGIC, 18, ptp_sys_offset_extended);
pub const PTP_MASK_CLEAR_ALL: _ = _IO(PTP_CLK_MAGIC, 19);
pub const PTP_MASK_EN_SINGLE: _ = _IOW(PTP_CLK_MAGIC, 20, u32);
pub const PTP_SYS_OFFSET_PRECISE_CYCLES: _ = _IOWR(PTP_CLK_MAGIC, 21, ptp_sys_offset_precise);
pub const PTP_SYS_OFFSET_EXTENDED_CYCLES: _ = _IOWR(PTP_CLK_MAGIC, 22, ptp_sys_offset_extended);

#[repr(C)]
pub struct ptp_extts_event { pub t: ptp_clock_time, pub index: u32, pub flags: u32, pub rsv: [u32; 2] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
