/*
 * Rust translation of linux/timex.h.
 * The original include and header guard are intentionally omitted.
 * The declarations guarded by __KERNEL__ are represented under the
 * `not(feature = "kernel")` condition.
 */

pub const NTP_API: u32 = 4;

#[cfg(not(feature = "kernel"))]
#[repr(C)]
pub struct timex {
    pub modes: ::core::ffi::c_uint,
    pub offset: __kernel_long_t,
    pub freq: __kernel_long_t,
    pub maxerror: __kernel_long_t,
    pub esterror: __kernel_long_t,
    pub status: ::core::ffi::c_int,
    pub constant: __kernel_long_t,
    pub precision: __kernel_long_t,
    pub tolerance: __kernel_long_t,
    pub time: timeval,
    pub tick: __kernel_long_t,
    pub ppsfreq: __kernel_long_t,
    pub jitter: __kernel_long_t,
    pub shift: ::core::ffi::c_int,
    pub stabil: __kernel_long_t,
    pub jitcnt: __kernel_long_t,
    pub calcnt: __kernel_long_t,
    pub errcnt: __kernel_long_t,
    pub stbcnt: __kernel_long_t,
    pub tai: ::core::ffi::c_int,
    pub __reserved: [::core::ffi::c_int; 11],
}

#[repr(C)]
pub struct __kernel_timex_timeval {
    pub tv_sec: __kernel_time64_t,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct __kernel_timex {
    pub modes: ::core::ffi::c_uint,
    pub __pad0: ::core::ffi::c_int,
    pub offset: i64,
    pub freq: i64,
    pub maxerror: i64,
    pub esterror: i64,
    pub status: ::core::ffi::c_int,
    pub __pad1: ::core::ffi::c_int,
    pub constant: i64,
    pub precision: i64,
    pub tolerance: i64,
    pub time: __kernel_timex_timeval,
    pub tick: i64,
    pub ppsfreq: i64,
    pub jitter: i64,
    pub shift: ::core::ffi::c_int,
    pub __pad2: ::core::ffi::c_int,
    pub stabil: i64,
    pub jitcnt: i64,
    pub calcnt: i64,
    pub errcnt: i64,
    pub stbcnt: i64,
    pub tai: ::core::ffi::c_int,
    pub __reserved: [::core::ffi::c_int; 11],
}

pub const ADJ_OFFSET: u32 = 0x0001;
pub const ADJ_FREQUENCY: u32 = 0x0002;
pub const ADJ_MAXERROR: u32 = 0x0004;
pub const ADJ_ESTERROR: u32 = 0x0008;
pub const ADJ_STATUS: u32 = 0x0010;
pub const ADJ_TIMECONST: u32 = 0x0020;
pub const ADJ_TAI: u32 = 0x0080;
pub const ADJ_SETOFFSET: u32 = 0x0100;
pub const ADJ_MICRO: u32 = 0x1000;
pub const ADJ_NANO: u32 = 0x2000;
pub const ADJ_TICK: u32 = 0x4000;

#[cfg(not(feature = "kernel"))]
pub const ADJ_OFFSET_SINGLESHOT: u32 = 0x8001;
#[cfg(not(feature = "kernel"))]
pub const ADJ_OFFSET_SS_READ: u32 = 0xa001;

pub const MOD_OFFSET: u32 = ADJ_OFFSET;
pub const MOD_FREQUENCY: u32 = ADJ_FREQUENCY;
pub const MOD_MAXERROR: u32 = ADJ_MAXERROR;
pub const MOD_ESTERROR: u32 = ADJ_ESTERROR;
pub const MOD_STATUS: u32 = ADJ_STATUS;
pub const MOD_TIMECONST: u32 = ADJ_TIMECONST;
pub const MOD_TAI: u32 = ADJ_TAI;
pub const MOD_MICRO: u32 = ADJ_MICRO;
pub const MOD_NANO: u32 = ADJ_NANO;

pub const STA_PLL: u32 = 0x0001;
pub const STA_PPSFREQ: u32 = 0x0002;
pub const STA_PPSTIME: u32 = 0x0004;
pub const STA_FLL: u32 = 0x0008;
pub const STA_INS: u32 = 0x0010;
pub const STA_DEL: u32 = 0x0020;
pub const STA_UNSYNC: u32 = 0x0040;
pub const STA_FREQHOLD: u32 = 0x0080;
pub const STA_PPSSIGNAL: u32 = 0x0100;
pub const STA_PPSJITTER: u32 = 0x0200;
pub const STA_PPSWANDER: u32 = 0x0400;
pub const STA_PPSERROR: u32 = 0x0800;
pub const STA_CLOCKERR: u32 = 0x1000;
pub const STA_NANO: u32 = 0x2000;
pub const STA_MODE: u32 = 0x4000;
pub const STA_CLK: u32 = 0x8000;

pub const STA_RONLY: u32 = STA_PPSSIGNAL
    | STA_PPSJITTER
    | STA_PPSWANDER
    | STA_PPSERROR
    | STA_CLOCKERR
    | STA_NANO
    | STA_MODE
    | STA_CLK;

pub const TIME_OK: i32 = 0;
pub const TIME_INS: i32 = 1;
pub const TIME_DEL: i32 = 2;
pub const TIME_OOP: i32 = 3;
pub const TIME_WAIT: i32 = 4;
pub const TIME_ERROR: i32 = 5;
pub const TIME_BAD: i32 = TIME_ERROR;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
