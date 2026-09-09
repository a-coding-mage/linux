// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of ntp.c. Kernel-provided types, constants, and
// functions referenced below are intentionally left as external dependencies.

#[repr(C)]
pub struct ntp_data {
    pub tick_usec: libc::c_ulong, pub tick_length: u64, pub time_state: i32,
    pub time_status: i32, pub time_offset: i64, pub skew_delta: i64,
    pub time_constant: libc::c_long, pub time_maxerror: libc::c_long,
    pub time_esterror: libc::c_long, pub time_freq: i64, pub time_reftime: i64,
    pub time_adjust: libc::c_long, pub time_adjust_frac: i64,
    pub ntp_tick_adj: i64, pub cs_tick_adj: i64, pub ntp_next_leap_sec: i64,
    #[cfg(CONFIG_NTP_PPS)] pub pps_valid: i32,
    #[cfg(CONFIG_NTP_PPS)] pub pps_tf: [libc::c_long; 3],
    #[cfg(CONFIG_NTP_PPS)] pub pps_jitter: libc::c_long,
    #[cfg(CONFIG_NTP_PPS)] pub pps_fbase: timespec64,
    #[cfg(CONFIG_NTP_PPS)] pub pps_shift: i32,
    #[cfg(CONFIG_NTP_PPS)] pub pps_intcnt: i32,
    #[cfg(CONFIG_NTP_PPS)] pub pps_freq: i64,
    #[cfg(CONFIG_NTP_PPS)] pub pps_stabil: libc::c_long,
    #[cfg(CONFIG_NTP_PPS)] pub pps_calcnt: libc::c_long,
    #[cfg(CONFIG_NTP_PPS)] pub pps_jitcnt: libc::c_long,
    #[cfg(CONFIG_NTP_PPS)] pub pps_stbcnt: libc::c_long,
    #[cfg(CONFIG_NTP_PPS)] pub pps_errcnt: libc::c_long,
}

#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: libc::c_long }
extern "C" {
    static mut tk_ntp_data: [ntp_data; TIMEKEEPERS_MAX as usize];
    fn shift_right(x: i64, s: i32) -> i64;
    fn div_s64(x: i64, y: i64) -> i64;
    fn div_u64(x: u64, y: u64) -> u64;
    fn ktime_get_ntp_seconds(p: *const ntp_data) -> i64;
}

// Build-time kernel constants and helpers are supplied by the surrounding tree.
extern "C" { static TIMEKEEPERS_MAX: u32; }
const SECS_PER_DAY: i64 = 86400;
const MAX_TICKADJ: i64 = 500;

#[inline] unsafe fn signof(x: i64) -> i64 { if x < 0 { -1 } else { 1 } }

#[cfg(not(CONFIG_NTP_PPS))]
#[inline] unsafe fn ntp_offset_chunk(n: *mut ntp_data, o: i64) -> i64 { shift_right(o, SHIFT_PLL + (*n).time_constant as i32) }
#[cfg(not(CONFIG_NTP_PPS))] #[inline] unsafe fn pps_reset_freq_interval(_: *mut ntp_data) {}
#[cfg(not(CONFIG_NTP_PPS))] #[inline] unsafe fn pps_clear(_: *mut ntp_data) {}
#[cfg(not(CONFIG_NTP_PPS))] #[inline] unsafe fn pps_dec_valid(_: *mut ntp_data) {}
#[cfg(not(CONFIG_NTP_PPS))] #[inline] unsafe fn pps_set_freq(_: *mut ntp_data) {}
#[cfg(not(CONFIG_NTP_PPS))] #[inline] unsafe fn is_error_status(s: i32) -> bool { (s & (STA_UNSYNC|STA_CLOCKERR)) != 0 }

unsafe fn ntp_update_frequency(n: *mut ntp_data) {
    let mut x = ((*n).tick_usec as u64) * NSEC_PER_USEC * USER_HZ as u64;
    x <<= NTP_SCALE_SHIFT; x = x.wrapping_add((*n).ntp_tick_adj as u64)
        .wrapping_add((*n).cs_tick_adj as u64).wrapping_add((*n).time_freq as u64);
    (*n).tick_length = div_u64(x, NTP_INTERVAL_FREQ as u64);
}

unsafe fn ntp_update_offset_fll(n: *mut ntp_data, o: i64, secs: i64) -> i64 {
    (*n).time_status &= !STA_MODE; if secs < MINSEC { return 0; }
    if (*n).time_status & STA_FLL == 0 && secs <= MAXSEC { return 0; }
    (*n).time_status |= STA_MODE; div_s64(o << (NTP_SCALE_SHIFT - SHIFT_FLL), secs)
}

unsafe fn ntp_update_offset(n: *mut ntp_data, mut offset: i64) {
    if (*n).time_status & STA_PLL == 0 { return; }
    if (*n).time_status & STA_NANO == 0 { offset = offset.clamp(-USEC_PER_SEC, USEC_PER_SEC) * NSEC_PER_USEC; }
    offset = offset.clamp(-MAXPHASE, MAXPHASE);
    let real = ktime_get_ntp_seconds(n); let mut secs = real - (*n).time_reftime;
    if (*n).time_status & STA_FREQHOLD != 0 { secs = 0; } (*n).time_reftime = real;
    let o = offset; let mut f = ntp_update_offset_fll(n, o, secs);
    let lim = 1i64 << (SHIFT_PLL + 1 + (*n).time_constant as i32); if secs > lim { secs = lim; }
    f += (o * secs) << (NTP_SCALE_SHIFT - 2 * (SHIFT_PLL + 2 + (*n).time_constant as i32));
    (*n).time_freq = (f + (*n).time_freq).min(MAXFREQ_SCALED).max(-MAXFREQ_SCALED);
    (*n).time_offset = div_s64(o << NTP_SCALE_SHIFT, NTP_INTERVAL_FREQ as i64);
}

unsafe fn __ntp_clear(n: *mut ntp_data) {
    (*n).time_adjust = 0; (*n).time_adjust_frac = 0; (*n).time_status |= STA_UNSYNC;
    (*n).time_maxerror = NTP_PHASE_LIMIT; (*n).time_esterror = NTP_PHASE_LIMIT;
    ntp_update_frequency(n); (*n).time_offset = 0; (*n).skew_delta = 0;
    (*n).ntp_next_leap_sec = TIME64_MAX; pps_clear(n);
}

#[no_mangle] pub unsafe extern "C" fn ntp_clear(tkid: u32, adj: i64) { tk_ntp_data[tkid as usize].cs_tick_adj = adj; __ntp_clear(&mut tk_ntp_data[tkid as usize]); }
#[no_mangle] pub unsafe extern "C" fn ntp_tick_length(tkid: u32) -> u64 { tk_ntp_data[tkid as usize].tick_length }
#[no_mangle] pub unsafe extern "C" fn ntp_get_skew_delta(tkid: u32) -> i64 { tk_ntp_data[tkid as usize].skew_delta }

unsafe fn ntp_drain_time_offset(tkid: u32, mut amount: i64) -> i64 {
    let n=&mut tk_ntp_data[tkid as usize]; if amount==0 || signof(amount)!=signof(n.time_offset) { return amount; }
    if amount.abs()>n.time_offset.abs() { let x=amount-n.time_offset; n.time_offset=0; return x; }
    n.time_offset-=amount; 0
}
#[no_mangle] pub unsafe extern "C" fn ntp_drain_skew(tkid:u32, amount:i64, _shift:u32)->i64 { amount-ntp_drain_time_offset(tkid,amount) }

#[no_mangle] pub unsafe extern "C" fn second_overflow(tkid:u32, _secs:i64)->i32 {
    let n=&mut tk_ntp_data[tkid as usize]; pps_dec_valid(n); n.time_maxerror += MAXFREQ/NSEC_PER_USEC;
    if n.time_maxerror>NTP_PHASE_LIMIT { n.time_maxerror=NTP_PHASE_LIMIT; n.time_status|=STA_UNSYNC; }
    n.skew_delta=0; if n.time_offset!=0 { let x=ntp_offset_chunk(n,n.time_offset); n.skew_delta=if x==0 { signof(n.time_offset) } else { div_s64(x,NTP_INTERVAL_FREQ as i64) }; } 0
}

unsafe fn process_adj_status(n:*mut ntp_data, status:i32) { if (*n).time_status&STA_PLL!=0 && status&STA_PLL==0 { (*n).time_state=TIME_OK; (*n).time_status=STA_UNSYNC; (*n).ntp_next_leap_sec=TIME64_MAX; pps_reset_freq_interval(n); } (*n).time_status = ((*n).time_status & STA_RONLY) | (status & !STA_RONLY); }

#[no_mangle] pub unsafe extern "C" fn ntp_init() { for i in 0..TIMEKEEPERS_MAX as usize { __ntp_clear(&mut tk_ntp_data[i]); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
