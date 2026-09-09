/*
 * These are all interfaces based on the old time_t definition
 * that overflows in 2038 on 32-bit architectures. New code
 * should use the replacements based on time64_t and timespec64.
 *
 * Any interfaces in here that become unused as we migrate
 * code to time64_t should get removed.
 *
 * C dependencies: linux/time64.h, linux/timex.h, and vdso/time32.h.
 */

#[repr(C)]
pub struct old_itimerspec32 {
    pub it_interval: old_timespec32,
    pub it_value: old_timespec32,
}

#[repr(C)]
pub struct old_utimbuf32 {
    pub actime: old_time32_t,
    pub modtime: old_time32_t,
}

#[repr(C)]
pub struct old_timex32 {
    pub modes: u32,
    pub offset: i32,
    pub freq: i32,
    pub maxerror: i32,
    pub esterror: i32,
    pub status: i32,
    pub constant: i32,
    pub precision: i32,
    pub tolerance: i32,
    pub time: old_timeval32,
    pub tick: i32,
    pub ppsfreq: i32,
    pub jitter: i32,
    pub shift: i32,
    pub stabil: i32,
    pub jitcnt: i32,
    pub calcnt: i32,
    pub errcnt: i32,
    pub stbcnt: i32,
    pub tai: i32,
    // Eleven anonymous 32-bit fields from the C declaration: s32:32.
    pub __reserved: [i32; 11],
}

extern "C" {
    pub fn get_old_timespec32(
        ts: *mut timespec64,
        uts: *const core::ffi::c_void,
    ) -> i32;
    pub fn put_old_timespec32(
        ts: *const timespec64,
        uts: *mut core::ffi::c_void,
    ) -> i32;
    pub fn get_old_itimerspec32(
        its: *mut itimerspec64,
        uits: *const old_itimerspec32,
    ) -> i32;
    pub fn put_old_itimerspec32(
        its: *const itimerspec64,
        uits: *mut old_itimerspec32,
    ) -> i32;
}

extern "C" {
    pub fn get_old_timex32(
        timex: *mut __kernel_timex,
        old_timex: *const old_timex32,
    ) -> i32;
    pub fn put_old_timex32(
        old_timex: *mut old_timex32,
        timex: *const __kernel_timex,
    ) -> i32;

    /**
     * ns_to_kernel_old_timeval - Convert nanoseconds to timeval
     * @nsec: the nanoseconds value to be converted
     *
     * Returns the timeval representation of the nsec parameter.
     */
    pub fn ns_to_kernel_old_timeval(nsec: i64) -> __kernel_old_timeval;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
