/*
 * Direct Rust translation of linux/timex.h.
 * C header inclusion and guards are intentionally omitted; referenced
 * kernel types/constants are supplied by other translated dependencies.
 */

pub const ADJ_ADJTIME: u32 = 0x8000; // switch between adjtime/adjtimex modes
pub const ADJ_OFFSET_SINGLESHOT: u32 = 0x0001; // old-fashioned adjtime
pub const ADJ_OFFSET_READONLY: u32 = 0x2000; // read-only adjtime

unsafe extern "C" {
    pub fn random_get_entropy_fallback() -> usize;
}

/* Architectures may override random_get_entropy with get_cycles(). */
#[macro_export]
macro_rules! random_get_entropy {
    () => {{
        // The get_cycles() alternative is supplied by asm/timex.h when present.
        $crate::random_get_entropy_fallback()
    }};
}

pub const SHIFT_PLL: i32 = 2; // PLL frequency factor (shift)
pub const SHIFT_FLL: i32 = 2; // FLL frequency factor (shift)
pub const MAXTC: i32 = 10; // maximum time constant (shift)

pub const SHIFT_USEC: i32 = 16; // frequency offset scale (shift)
pub const PPM_SCALE: i64 = (NSEC_PER_USEC as i64) << (NTP_SCALE_SHIFT - SHIFT_USEC);
pub const PPM_SCALE_INV_SHIFT: i32 = 19;
pub const PPM_SCALE_INV: i64 =
    ((1_i64 << (PPM_SCALE_INV_SHIFT + NTP_SCALE_SHIFT)) / PPM_SCALE) + 1;

pub const MAXPHASE: i64 = 500000000; // max phase error (ns)
pub const MAXFREQ: i64 = 500000; // max frequency error (ns/s)
pub const MAXFREQ_SCALED: i64 = MAXFREQ << NTP_SCALE_SHIFT;
pub const MINSEC: i32 = 256; // min interval between updates (s)
pub const MAXSEC: i32 = 2048; // max interval between updates (s)
pub const NTP_PHASE_LIMIT: i64 = ((MAXPHASE / NSEC_PER_USEC as i64) << 5);

/* Required to safely shift negative values. */
#[macro_export]
macro_rules! shift_right {
    ($x:expr, $s:expr) => {{
        let __x = $x;
        let __s = $s;
        if __x < 0 { -((-__x) >> __s) } else { __x >> __s }
    }};
}

pub const NTP_SCALE_SHIFT: i32 = 32;
pub const NTP_INTERVAL_FREQ: u64 = HZ as u64;
pub const NTP_INTERVAL_LENGTH: u64 = NSEC_PER_SEC as u64 / NTP_INTERVAL_FREQ;

unsafe extern "C" {
    pub fn do_adjtimex(tx: *mut __kernel_timex) -> i32;
    pub fn do_clock_adjtime(which_clock: clockid_t, ktx: *mut __kernel_timex) -> i32;
    pub fn hardpps(t1: *const timespec64, t2: *const timespec64);
}

/* The clock frequency of the i8253/i8254 PIT. */
pub const PIT_TICK_RATE: u32 = 1193182;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
