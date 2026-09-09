// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 ARM Ltd.
 *
 * Generic implementation of update_vsyscall and update_vsyscall_tz.
 *
 * Based on the x86 specific implementation.
 */

// Kernel/VDSO dependencies supplied by other translation units.

#[inline]
unsafe fn fill_clock_configuration(vc: *mut vdso_clock, base: *const tk_read_base) {
    (*vc).cycle_last = (*base).cycle_last;
    // CONFIG_GENERIC_VDSO_OVERFLOW_PROTECT
    #[cfg(CONFIG_GENERIC_VDSO_OVERFLOW_PROTECT)]
    {
        (*vc).max_cycles = (*(*base).clock).max_cycles;
    }
    (*vc).mask = (*base).mask;
    (*vc).mult = (*base).mult;
    (*vc).shift = (*base).shift;
}

#[inline]
unsafe fn update_vdso_time_data(vdata: *mut vdso_time_data, tk: *mut timekeeper) {
    let vc = (*vdata).clock_data;
    let mut vdso_ts: *mut vdso_timestamp;
    let mut nsec: u64;
    let mut sec: u64;

    fill_clock_configuration(vc.add(CS_HRES_COARSE), &(*tk).tkr_mono);
    fill_clock_configuration(vc.add(CS_RAW), &(*tk).tkr_raw);

    /* CLOCK_MONOTONIC */
    vdso_ts = &mut (*vc.add(CS_HRES_COARSE)).basetime[CLOCK_MONOTONIC];
    (*vdso_ts).sec = (*tk).xtime_sec + (*tk).wall_to_monotonic.tv_sec;

    nsec = (*tk).tkr_mono.xtime_nsec;
    nsec += ((*tk).wall_to_monotonic.tv_nsec as u64) << (*tk).tkr_mono.shift;
    while nsec >= ((NSEC_PER_SEC as u64) << (*tk).tkr_mono.shift) {
        nsec -= (NSEC_PER_SEC as u64) << (*tk).tkr_mono.shift;
        (*vdso_ts).sec += 1;
    }
    (*vdso_ts).nsec = nsec;

    /* Copy MONOTONIC time for BOOTTIME */
    sec = (*vdso_ts).sec;
    /* Add the boot offset */
    sec += (*tk).monotonic_to_boot.tv_sec;
    nsec += ((*tk).monotonic_to_boot.tv_nsec as u64) << (*tk).tkr_mono.shift;

    /* CLOCK_BOOTTIME */
    vdso_ts = &mut (*vc.add(CS_HRES_COARSE)).basetime[CLOCK_BOOTTIME];
    (*vdso_ts).sec = sec;

    while nsec >= ((NSEC_PER_SEC as u64) << (*tk).tkr_mono.shift) {
        nsec -= (NSEC_PER_SEC as u64) << (*tk).tkr_mono.shift;
        (*vdso_ts).sec += 1;
    }
    (*vdso_ts).nsec = nsec;

    /* CLOCK_MONOTONIC_RAW */
    vdso_ts = &mut (*vc.add(CS_RAW)).basetime[CLOCK_MONOTONIC_RAW];
    (*vdso_ts).sec = (*tk).raw_sec;
    (*vdso_ts).nsec = (*tk).tkr_raw.xtime_nsec;

    /* CLOCK_TAI */
    vdso_ts = &mut (*vc.add(CS_HRES_COARSE)).basetime[CLOCK_TAI];
    (*vdso_ts).sec = (*tk).xtime_sec + (*tk).tai_offset as i64;
    (*vdso_ts).nsec = (*tk).tkr_mono.xtime_nsec;
}

pub unsafe fn update_vsyscall(tk: *mut timekeeper) {
    let vdata = vdso_k_time_data;
    let vc = (*vdata).clock_data;
    let mut vdso_ts: *mut vdso_timestamp;
    let clock_mode: i32;
    let mut nsec: u64;

    /* copy vsyscall data */
    vdso_write_begin(vdata);

    clock_mode = (*(*tk).tkr_mono.clock).vdso_clock_mode;
    (*vc.add(CS_HRES_COARSE)).clock_mode = clock_mode;
    (*vc.add(CS_RAW)).clock_mode = clock_mode;

    /* CLOCK_REALTIME also required for time() */
    vdso_ts = &mut (*vc.add(CS_HRES_COARSE)).basetime[CLOCK_REALTIME];
    (*vdso_ts).sec = (*tk).xtime_sec;
    (*vdso_ts).nsec = (*tk).tkr_mono.xtime_nsec;

    /* CLOCK_REALTIME_COARSE */
    vdso_ts = &mut (*vc.add(CS_HRES_COARSE)).basetime[CLOCK_REALTIME_COARSE];
    (*vdso_ts).sec = (*tk).xtime_sec;
    (*vdso_ts).nsec = (*tk).coarse_nsec;

    /* CLOCK_MONOTONIC_COARSE */
    vdso_ts = &mut (*vc.add(CS_HRES_COARSE)).basetime[CLOCK_MONOTONIC_COARSE];
    (*vdso_ts).sec = (*tk).xtime_sec + (*tk).wall_to_monotonic.tv_sec;
    nsec = (*tk).coarse_nsec;
    nsec = nsec + (*tk).wall_to_monotonic.tv_nsec as u64;
    (*vdso_ts).sec += __iter_div_u64_rem(nsec, NSEC_PER_SEC, &mut (*vdso_ts).nsec);

    /* Read without the seqlock held by clock_getres(). */
    WRITE_ONCE((*vdata).hrtimer_res, hrtimer_resolution);

    /* If the current clocksource is not VDSO capable, spare the update of the high resolution parts. */
    if clock_mode != VDSO_CLOCKMODE_NONE {
        update_vdso_time_data(vdata, tk);
    }

    __arch_update_vdso_clock(vc.add(CS_HRES_COARSE));
    __arch_update_vdso_clock(vc.add(CS_RAW));
    vdso_write_end(vdata);
    __arch_sync_vdso_time_data(vdata);
}

pub unsafe fn update_vsyscall_tz() {
    let vdata = vdso_k_time_data;
    (*vdata).tz_minuteswest = sys_tz.tz_minuteswest;
    (*vdata).tz_dsttime = sys_tz.tz_dsttime;
    __arch_sync_vdso_time_data(vdata);
}

// CONFIG_POSIX_AUX_CLOCKS
#[cfg(CONFIG_POSIX_AUX_CLOCKS)]
pub unsafe fn vdso_time_update_aux(tk: *mut timekeeper) {
    let vdata = vdso_k_time_data;
    let vdso_ts: *mut vdso_timestamp;
    let vc: *mut vdso_clock;
    let mut clock_mode: i32;
    let mut nsec: u64;

    vc = &mut (*vdata).aux_clock_data[((*tk).id - TIMEKEEPER_AUX_FIRST) as usize];
    vdso_ts = &mut (*vc).basetime[VDSO_BASE_AUX];
    clock_mode = (*(*tk).tkr_mono.clock).vdso_clock_mode;
    if !(*tk).clock_valid {
        clock_mode = VDSO_CLOCKMODE_NONE;
    }

    vdso_write_begin_clock(vc);
    (*vc).clock_mode = clock_mode;

    if clock_mode != VDSO_CLOCKMODE_NONE {
        fill_clock_configuration(vc, &(*tk).tkr_mono);
        (*vdso_ts).sec = (*tk).xtime_sec + (*tk).monotonic_to_aux.tv_sec;
        nsec = (*tk).tkr_mono.xtime_nsec >> (*tk).tkr_mono.shift;
        nsec += (*tk).monotonic_to_aux.tv_nsec as u64;
        (*vdso_ts).sec += __iter_div_u64_rem(nsec, NSEC_PER_SEC, &mut nsec);
        nsec <<= (*tk).tkr_mono.shift;
        (*vdso_ts).nsec = nsec;
    }

    __arch_update_vdso_clock(vc);
    vdso_write_end_clock(vc);
    __arch_sync_vdso_time_data(vdata);
}

pub unsafe fn vdso_update_begin() -> c_ulong {
    let vdata = vdso_k_time_data;
    let flags = timekeeper_lock_irqsave();
    vdso_write_begin(vdata);
    flags
}

pub unsafe fn vdso_update_end(flags: c_ulong) {
    let vdata = vdso_k_time_data;
    vdso_write_end(vdata);
    __arch_sync_vdso_time_data(vdata);
    timekeeper_unlock_irqrestore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
