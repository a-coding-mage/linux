// SPDX-License-Identifier: GPL-2.0
/*
 * Generic userspace implementations of gettimeofday() and similar.
 */

// Dependencies supplied by the surrounding vDSO implementation are intentionally external.

#[cfg(not(vdso_calc_ns))]
#[cfg(not(vdso_delta_nomask))]
const fn vdso_delta_mask(vc: *const vdso_clock) -> u64 { unsafe { (*vc).mask } }

#[cfg(not(vdso_calc_ns))]
#[inline(always)]
unsafe fn vdso_delta_ok(vc: *const vdso_clock, delta: u64) -> bool {
    #[cfg(CONFIG_GENERIC_VDSO_OVERFLOW_PROTECT)]
    { delta < (*vc).max_cycles }
    #[cfg(not(CONFIG_GENERIC_VDSO_OVERFLOW_PROTECT))]
    { let _ = (vc, delta); true }
}

#[cfg(not(vdso_calc_ns))]
#[cfg(not(vdso_shift_ns))]
#[inline(always)]
fn vdso_shift_ns(ns: u64, shift: u32) -> u64 { ns >> shift }

#[cfg(not(vdso_calc_ns))]
#[inline(always)]
unsafe fn vdso_calc_ns(vc: *const vdso_clock, cycles: u64, base: u64) -> u64 {
    let delta = (cycles.wrapping_sub((*vc).cycle_last)) & {
        #[cfg(VDSO_DELTA_NOMASK)] { u64::MAX }
        #[cfg(not(VDSO_DELTA_NOMASK))] { (*vc).mask }
    };
    if vdso_delta_ok(vc, delta) {
        vdso_shift_ns(delta.wrapping_mul((*vc).mult).wrapping_add(base), (*vc).shift)
    } else {
        mul_u64_u32_add_u64_shr(delta, (*vc).mult, base, (*vc).shift)
    }
}

#[cfg(not(__arch_vdso_hres_capable))]
#[inline]
fn __arch_vdso_hres_capable() -> bool { true }

#[cfg(not(vdso_clocksource_ok))]
#[inline]
unsafe fn vdso_clocksource_ok(vc: *const vdso_clock) -> bool { (*vc).clock_mode != VDSO_CLOCKMODE_NONE }

#[cfg(not(vdso_cycles_ok))]
#[inline]
fn vdso_cycles_ok(_cycles: u64) -> bool { true }

#[inline(always)]
fn vdso_clockid_valid(clock: clockid_t) -> bool { (clock as u32) <= CLOCK_AUX_LAST }

#[inline(always)]
unsafe fn vdso_set_timespec(ts: *mut __kernel_timespec, sec: u64, mut ns: u64) {
    (*ts).tv_sec = sec + __iter_div_u64_rem(ns, NSEC_PER_SEC, &mut ns);
    (*ts).tv_nsec = ns;
}

#[inline(always)]
unsafe fn vdso_get_timestamp(vd: *const vdso_time_data, vc: *const vdso_clock,
                             clkidx: u32, sec: *mut u64, ns: *mut u64) -> bool {
    let vdso_ts = &(*vc).basetime[clkidx as usize];
    if !vdso_clocksource_ok(vc) { return false; }
    let cycles = __arch_get_hw_counter((*vc).clock_mode, vd);
    if !vdso_cycles_ok(cycles) { return false; }
    *ns = vdso_calc_ns(vc, cycles, vdso_ts.nsec);
    *sec = vdso_ts.sec;
    true
}

#[inline(always)]
unsafe fn vdso_timens_data(vd: *const vdso_time_data) -> *const vdso_time_data {
    (vd as *const u8).add(PAGE_SIZE as usize) as *const vdso_time_data
}

#[inline(always)]
unsafe fn do_hres_timens(vdns: *const vdso_time_data, vcns: *const vdso_clock,
                         clk: clockid_t, ts: *mut __kernel_timespec) -> bool {
    let vd = vdso_timens_data(vdns);
    let offs = &(*vcns).offset[clk as usize];
    let mut vc = (*vd).clock_data;
    if clk != CLOCK_MONOTONIC_RAW { vc = vc.add(CS_HRES_COARSE as usize); }
    else { vc = vc.add(CS_RAW as usize); }
    let mut seq;
    let (mut sec, mut ns);
    loop {
        seq = vdso_read_begin(vc);
        sec = 0; ns = 0;
        if !vdso_get_timestamp(vd, vc, clk as u32, &mut sec, &mut ns) { return false; }
        if !vdso_read_retry(vc, seq) { break; }
    }
    sec = (sec as i64 + offs.sec) as u64; ns = ns.wrapping_add(offs.nsec);
    vdso_set_timespec(ts, sec, ns); true
}

#[inline(always)]
unsafe fn do_hres(vd: *const vdso_time_data, vc: *const vdso_clock,
                 clk: clockid_t, ts: *mut __kernel_timespec) -> bool {
    if !__arch_vdso_hres_capable() { return false; }
    let mut seq = 0; let (mut sec, mut ns) = (0, 0);
    loop {
        if vdso_read_begin_timens(vc, &mut seq) { return do_hres_timens(vd, vc, clk, ts); }
        if !vdso_get_timestamp(vd, vc, clk as u32, &mut sec, &mut ns) { return false; }
        if !vdso_read_retry(vc, seq) { break; }
    }
    vdso_set_timespec(ts, sec, ns); true
}

#[inline(always)]
unsafe fn do_coarse_timens(vdns: *const vdso_time_data, vcns: *const vdso_clock,
                           clk: clockid_t, ts: *mut __kernel_timespec) -> bool {
    let vd = vdso_timens_data(vdns); let offs = &(*vcns).offset[clk as usize];
    let vc = (*vd).clock_data; let t = &(*vc).basetime[clk as usize]; let mut seq;
    let (mut sec, mut ns); loop { seq = vdso_read_begin(vc); sec=t.sec; ns=t.nsec; if !vdso_read_retry(vc,seq) {break;} }
    vdso_set_timespec(ts, (sec as i64 + offs.sec) as u64, ns.wrapping_add(offs.nsec)); true
}

#[inline(always)]
unsafe fn do_coarse(vd: *const vdso_time_data, vc: *const vdso_clock,
                    clk: clockid_t, ts: *mut __kernel_timespec) -> bool {
    let t = &(*vc).basetime[clk as usize]; let mut seq=0;
    loop { if vdso_read_begin_timens(vc,&mut seq) { return do_coarse_timens(vd,vc,clk,ts); }
        (*ts).tv_sec=t.sec; (*ts).tv_nsec=t.nsec; if !vdso_read_retry(vc,seq){break;} }
    true
}

#[inline(always)]
unsafe fn do_aux(vd: *const vdso_time_data, clock: clockid_t, ts: *mut __kernel_timespec) -> bool {
    if !IS_ENABLED_CONFIG_POSIX_AUX_CLOCKS { return false; }
    let idx=(clock-CLOCK_AUX) as usize; let mut vc=(*vd).aux_clock_data.add(idx); let mut seq=0;
    loop { while vdso_read_begin_timens(vc,&mut seq) { vd=vdso_timens_data(vd); vc=(*vd).aux_clock_data.add(idx); }
        if (*vc).clock_mode==VDSO_CLOCKMODE_NONE {return false;} let (mut sec,mut ns)=(0,0);
        if !vdso_get_timestamp(vd,vc,VDSO_BASE_AUX,&mut sec,&mut ns){return false;} if !vdso_read_retry(vc,seq){vdso_set_timespec(ts,sec,ns);return true;} }
}

#[inline(always)]
unsafe fn __cvdso_clock_gettime_common(vd:*const vdso_time_data, clock:clockid_t, ts:*mut __kernel_timespec)->bool {
    if !vdso_clockid_valid(clock){return false;} let mut vc=(*vd).clock_data; let msk=1u32 << (clock as u32);
    if msk & VDSO_HRES != 0 {vc=vc.add(CS_HRES_COARSE as usize);} else if msk&VDSO_COARSE!=0{return do_coarse(vd,vc.add(CS_HRES_COARSE as usize),clock,ts)} else if msk&VDSO_RAW!=0{vc=vc.add(CS_RAW as usize)} else if msk&VDSO_AUX!=0{return do_aux(vd,clock,ts)} else{return false} do_hres(vd,vc,clock,ts)
}

fn __cvdso_clock_gettime_data(vd:*const vdso_time_data, clock:clockid_t, ts:*mut __kernel_timespec)->i32 { unsafe { if __cvdso_clock_gettime_common(vd,clock,ts){0}else{clock_gettime_fallback(clock,ts)} } }
#[allow(dead_code)]
fn __cvdso_clock_gettime(clock:clockid_t,ts:*mut __kernel_timespec)->i32 { unsafe { __cvdso_clock_gettime_data(__arch_get_vdso_u_time_data(),clock,ts) } }

#[cfg(BUILD_VDSO32)]
fn __cvdso_clock_gettime32_data(vd:*const vdso_time_data, clock:clockid_t, res:*mut old_timespec32)->i32 { unsafe {
    let mut ts=__kernel_timespec{tv_sec:0,tv_nsec:0};
    if !__cvdso_clock_gettime_common(vd,clock,&mut ts){return clock_gettime32_fallback(clock,res);}
    (*res).tv_sec=ts.tv_sec; (*res).tv_nsec=ts.tv_nsec; 0
} }
#[cfg(BUILD_VDSO32)]
fn __cvdso_clock_gettime32(clock:clockid_t,res:*mut old_timespec32)->i32 { unsafe{__cvdso_clock_gettime32_data(__arch_get_vdso_u_time_data(),clock,res)} }

fn __cvdso_gettimeofday_data(vd:*const vdso_time_data, tv:*mut __kernel_old_timeval, tz:*mut timezone)->i32 { unsafe {
    let vc=(*vd).clock_data;
    if !tv.is_null() { let mut ts=__kernel_timespec{tv_sec:0,tv_nsec:0}; if !do_hres(vd,vc.add(CS_HRES_COARSE as usize),CLOCK_REALTIME,&mut ts){return gettimeofday_fallback(tv,tz);} (*tv).tv_sec=ts.tv_sec; (*tv).tv_usec=(ts.tv_nsec as u32)/NSEC_PER_USEC; }
    if !tz.is_null() { let mut data=vd; if vdso_is_timens_clock(vc){data=vdso_timens_data(vd);} (*tz).tz_minuteswest=(*data).clock_data.add(CS_HRES_COARSE as usize).read().tz_minuteswest; (*tz).tz_dsttime=(*data).clock_data.add(CS_HRES_COARSE as usize).read().tz_dsttime; } 0
} }
#[allow(dead_code)]
fn __cvdso_gettimeofday(tv:*mut __kernel_old_timeval,tz:*mut timezone)->i32 { unsafe{__cvdso_gettimeofday_data(__arch_get_vdso_u_time_data(),tv,tz)} }

#[cfg(VDSO_HAS_TIME)]
fn __cvdso_time_data(vd:*const vdso_time_data,time:*mut __kernel_old_time_t)->__kernel_old_time_t { unsafe {
    let mut data=vd; let mut vc=(*vd).clock_data; if vdso_is_timens_clock(vc){data=vdso_timens_data(vd);vc=(*data).clock_data;}
    let t=core::ptr::read_volatile(&(*vc.add(CS_HRES_COARSE as usize)).basetime[CLOCK_REALTIME as usize].sec); if !time.is_null(){*time=t;} t
} }
#[cfg(VDSO_HAS_TIME)]
fn __cvdso_time(time:*mut __kernel_old_time_t)->__kernel_old_time_t { unsafe{__cvdso_time_data(__arch_get_vdso_u_time_data(),time)} }

#[cfg(VDSO_HAS_CLOCK_GETRES)]
unsafe fn __cvdso_clock_getres_common(vd:*const vdso_time_data,clock:clockid_t,res:*mut __kernel_timespec)->bool {
    if !vdso_clockid_valid(clock){return false;} let mut data=vd; let vc=(*vd).clock_data; if vdso_is_timens_clock(vc){data=vdso_timens_data(vd);}
    let msk=1u32<<(clock as u32); let ns=if msk&(VDSO_HRES|VDSO_RAW)!=0{core::ptr::read_volatile(&(*data).hrtimer_res)}else if msk&VDSO_COARSE!=0{LOW_RES_NSEC}else if msk&VDSO_AUX!=0{aux_clock_resolution_ns()}else{return false};
    if !res.is_null(){(*res).tv_sec=0;(*res).tv_nsec=ns;} true
}
#[cfg(VDSO_HAS_CLOCK_GETRES)]
fn __cvdso_clock_getres_data(vd:*const vdso_time_data,clock:clockid_t,res:*mut __kernel_timespec)->i32{unsafe{if __cvdso_clock_getres_common(vd,clock,res){0}else{clock_getres_fallback(clock,res)}}}
#[cfg(VDSO_HAS_CLOCK_GETRES)]
fn __cvdso_clock_getres(clock:clockid_t,res:*mut __kernel_timespec)->i32{unsafe{__cvdso_clock_getres_data(__arch_get_vdso_u_time_data(),clock,res)}}
#[cfg(all(VDSO_HAS_CLOCK_GETRES,BUILD_VDSO32))]
fn __cvdso_clock_getres_time32(clock:clockid_t,res:*mut old_timespec32)->i32{unsafe{let mut ts=__kernel_timespec{tv_sec:0,tv_nsec:0};if !__cvdso_clock_getres_common(__arch_get_vdso_u_time_data(),clock,&mut ts){return clock_getres32_fallback(clock,res);}if !res.is_null(){(*res).tv_sec=ts.tv_sec;(*res).tv_nsec=ts.tv_nsec;}0}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
