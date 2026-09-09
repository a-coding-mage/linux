/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// asm/lowcore.h, asm/cpu_mf.h, and asm/idle.h

extern "C" {
    static mut mt_cycles: [u64; 8];
}

#[inline]
pub unsafe fn update_timer_sys() {
    let lc: *mut lowcore = get_lowcore();

    (*lc).system_timer = (*lc).system_timer
        .wrapping_add((*lc).last_update_timer.wrapping_sub((*lc).exit_timer));
    (*lc).user_timer = (*lc).user_timer
        .wrapping_add((*lc).exit_timer.wrapping_sub((*lc).sys_enter_timer));
    (*lc).last_update_timer = (*lc).sys_enter_timer;
}

#[inline]
pub unsafe fn update_timer_mcck() {
    let lc: *mut lowcore = get_lowcore();

    (*lc).system_timer = (*lc).system_timer
        .wrapping_add((*lc).last_update_timer.wrapping_sub((*lc).exit_timer));
    (*lc).user_timer = (*lc).user_timer
        .wrapping_add((*lc).exit_timer.wrapping_sub((*lc).mcck_enter_timer));
    (*lc).last_update_timer = (*lc).mcck_enter_timer;
}

#[inline]
pub unsafe fn update_timer_idle() {
    let idle: *mut s390_idle_data = this_cpu_ptr(&raw mut s390_idle);
    let lc: *mut lowcore = get_lowcore();
    let mut cycles_new = [0u64; 8];
    let mut i: i32;
    let mtid: i32;

    mtid = smp_cpu_mtid;
    if mtid != 0 {
        stcctm(MT_DIAG, mtid, cycles_new.as_mut_ptr());
        i = 0;
        while i < mtid {
            let index = i as usize;
            let delta = cycles_new[index].wrapping_sub((*idle).mt_cycles_enter[index]);
            mt_cycles[index] = mt_cycles[index].wrapping_add(delta);
            i += 1;
        }
    }
    /*
     * This is a bit subtle: Forward last_update_clock so it excludes idle
     * time. For correct steal time calculation in do_account_vtime() add
     * passed wall time before idle_enter to steal_timer:
     * During the passed wall time before idle_enter CPU time may have
     * been accounted to system, hardirq, softirq, etc. lowcore fields.
     * The accounted CPU times will be subtracted again from steal_timer
     * when accumulated steal time is calculated in do_account_vtime().
     */
    (*lc).steal_timer = (*lc).steal_timer.wrapping_add(
        (*idle).clock_idle_enter.tod.wrapping_sub((*lc).last_update_clock),
    );
    (*lc).last_update_clock = (*lc).int_clock.tod;
    (*lc).system_timer = (*lc).system_timer
        .wrapping_add((*lc).last_update_timer.wrapping_sub((*idle).timer_idle_enter));
    (*lc).last_update_timer = (*lc).sys_enter_timer;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
